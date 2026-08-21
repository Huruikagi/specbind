#!/usr/bin/env sh
# Prepares and judges the deliberately expensive end-to-end forward-test journey.
#
# Usage:
#   forward-test-journey.sh prepare hp1 <target-directory> [en|ja]
#   forward-test-journey.sh judge   hp1 <target-directory>

set -eu

action=${1:?usage: forward-test-journey.sh <prepare|judge> hp1 <target-directory> [en|ja]}
journey=${2:?usage: forward-test-journey.sh <prepare|judge> hp1 <target-directory> [en|ja]}
target=${3:?usage: forward-test-journey.sh <prepare|judge> hp1 <target-directory> [en|ja]}
language=${4:-en}

if [ "$journey" != hp1 ]; then
    echo "forward-test-journey: unknown journey: $journey" >&2
    exit 1
fi

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)

fail() {
    echo "forward-test-journey: $1" >&2
    exit 1
}

expect() {
    label=$1
    command=$2
    if ! eval "$command" >/dev/null 2>&1; then
        fail "$label"
    fi
    echo "  PASS $label"
}

python_runner() {
    for candidate in python py python3; do
        if "$candidate" -c "import sys; sys.exit(0)" >/dev/null 2>&1; then
            echo "$candidate"
            return 0
        fi
    done
    fail "no working Python interpreter found"
}

prepare_hp1() {
    if [ "$language" != en ] && [ "$language" != ja ]; then
        fail "language must be en or ja"
    fi

    sh "$script_dir/forward-test-fixture.sh" \
        "$target" "$language" --instrument-dispatch >/dev/null
    cd "$target"
    PATH="$(CDPATH= cd -- .specbind/bin && pwd):$PATH"
    export PATH

    cat > .specbind/settings/adapters/release.md <<'EOF'
---
type: SpecBind Release Adapter
---

# Release adapter

## Prepare

Run `sh scripts/test.sh` and require a passing suite from the clean release tree.

## Publish

Record the current HEAD, then create an annotated local tag named exactly after
the bound release version and pointing to that recorded commit.

## Verify

Resolve the tag independently, require it to point to the recorded commit, then
check out the tagged tree in a temporary detached worktree and run
`sh scripts/test.sh` there. Remove only that temporary worktree afterwards.

## After finalize

Nothing.
EOF

    cat >> .specbind/steering/conventions.md <<'EOF'

## Verification

The canonical project test command is `sh scripts/test.sh`. Product changes add
or update that command with automated coverage for their observable behavior.
EOF

    cat >> .specbind/settings/adapters/git.md <<'EOF'

## Release-version binding

Treat a confirmed release-version binding as one eligible workflow unit. Commit
only the Roadmap change immediately after binding, before completion validation.
Never push that checkpoint.
EOF

    git add .specbind/settings/adapters/release.md \
        .specbind/settings/adapters/git.md .specbind/steering/conventions.md
    git commit --quiet -m "Configure the forward-test release journey"

    expect "Release adapter is active" \
        '! grep -q "specbind:adapter-scaffold" .specbind/settings/adapters/release.md'
    expect "release binding has a checkpoint policy" \
        'grep -q "release-version binding" .specbind/settings/adapters/git.md'
    expect "fixture has no active milestone" \
        'specbind milestone status | grep -q "NO_ACTIVE_MILESTONE"'
    expect "fixture has no release tag" \
        '! git rev-parse --verify --quiet refs/tags/v1.4.0'
    expect "fixture has no remote" \
        '! git remote | grep -q .'
    expect "fixture starts clean" \
        'test -z "$(git status --porcelain)"'

    shell_target=$(pwd)
    if native_target=$(pwd -W 2>/dev/null); then
        :
    else
        native_target=$shell_target
    fi

    echo
    echo "Journey hp1 ready"
    echo "  native path: $native_target"
    if [ "$shell_target" != "$native_target" ]; then
        echo "  shell path: $shell_target"
    fi
    echo "Put the fixture CLI first on PATH:"
    echo
    echo "    export PATH=\"$(CDPATH= cd -- .specbind/bin && pwd):\$PATH\""
    echo
    echo "Drive the exact conversation in:"
    echo "    docs/skill-forward-tests/journey-scenarios.md"
}

probe_cart() {
    PYTHONDONTWRITEBYTECODE=1 "$runner" - <<'PY'
import sys

sys.path.insert(0, "src")
from cart import add_item

assert add_item({"book": 97}, "book", 2) == {"book": 99}

for cart, quantity, bound in [({}, 0, "1"), ({"book": 99}, 1, "99")]:
    before = dict(cart)
    try:
        add_item(cart, "book", quantity)
    except ValueError as error:
        assert bound in str(error)
    else:
        raise AssertionError(f"quantity {quantity} was accepted")
    assert cart == before
PY
}

judge_hp1() {
    [ -d "$target/.git" ] || fail "$target is not a prepared Git fixture"
    cd "$target"
    PATH="$(CDPATH= cd -- .specbind/bin && pwd):$PATH"
    export PATH

    runner=$(python_runner)

    expect "project test command passes" 'sh scripts/test.sh'
    if ! probe_cart >/dev/null 2>&1; then
        fail "cart bounds or rejection atomicity are wrong"
    fi
    echo "  PASS cart bounds and rejection atomicity hold"
    expect "active milestone is finalized" \
        'specbind milestone status | grep -q "NO_ACTIVE_MILESTONE"'
    expect "cart returned to idle" \
        'specbind spec status cart | grep -q "State: idle"'
    expect "transient cart artifacts were removed" \
        '! test -e .specbind/specs/cart/brief.md && ! test -e .specbind/specs/cart/tasks.yaml'
    expect "cart release log records v1.4.0" \
        'grep -q "Release v1.4.0" .specbind/specs/cart/log.md'
    expect "roadmap archive exists" \
        'test -f .specbind/releases/v1.4.0-roadmap.md'
    expect "contract-review archive exists" \
        'test -f .specbind/releases/v1.4.0-contract-review.md'
    expect "v1.4.0 is an annotated tag" \
        'test "$(git cat-file -t refs/tags/v1.4.0)" = tag'

    tagged_commit=$(git rev-list -n 1 v1.4.0)
    final_commit=$(git rev-parse HEAD)
    expect "tag points before the finalization checkpoint" \
        'test "$tagged_commit" != "$final_commit" && git merge-base --is-ancestor "$tagged_commit" "$final_commit"'
    expect "fixture still has no remote" \
        '! git remote | grep -q .'
    expect "final worktree is clean" \
        'test -z "$(git status --porcelain)"'
    expect "fresh-context dispatch occurred" \
        'test -f .forward-test/agents.log && test "$(wc -l < .forward-test/agents.log)" -gt 1'

    contexts=$(wc -l < .forward-test/agents.log | tr -d ' ')
    echo
    echo "PASS hp1"
    echo "  final commit: $final_commit"
    echo "  tagged commit: $tagged_commit"
    echo "  dispatch contexts: $contexts"
}

case "$action" in
prepare)
    prepare_hp1
    ;;
judge)
    judge_hp1
    ;;
*)
    fail "unknown action: $action"
    ;;
esac
