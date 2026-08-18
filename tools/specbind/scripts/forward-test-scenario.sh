#!/usr/bin/env sh
# Builds the starting state one forward-test scenario needs.
#
# The fixture builder produces the common baseline. Several scenarios start from
# something further along — an uncommitted edit, a broken steering document, a
# milestone already scoped — and doing that by hand is where a run silently
# becomes a different run. Every recipe here verifies what it built and fails
# loudly rather than handing over a fixture that only looks right.
#
# This script owns starting state only. The request to give the agent and the
# expectations to check afterwards live in docs/skill-forward-tests.md, which
# stays the contract.
#
# Usage: forward-test-scenario.sh <scenario> <target-directory> [en|ja]
#
# Scenarios:
#   base   the fixture as built, nothing added
#   d9     base plus an uncommitted edit to an owned file
#   d12    base plus a steering document that cannot be parsed
#   r1     milestone scoping a new `order` Spec, with its brief written
#   r3     milestone scoping a `cart` update that removes behavior, brief written
#   r4     milestone scoping the `cart` quantity cap, brief written
#   r5     r4 with the requirements gate already approved
#   c2     base plus a Git adapter carrying real policy

set -eu

scenario=${1:?usage: forward-test-scenario.sh <scenario> <target-directory> [en|ja]}
target=${2:?usage: forward-test-scenario.sh <scenario> <target-directory> [en|ja]}
language=${3:-en}

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)

sh "$script_dir/forward-test-fixture.sh" "$target" "$language" >/dev/null
cd "$target"
PATH="$(CDPATH= cd -- .specbind/bin && pwd):$PATH"
export PATH

fail() {
    echo "forward-test-scenario: $1" >&2
    exit 1
}

# Every recipe ends by proving its precondition holds. A precondition that did
# not apply is the failure mode worth spending a check on.
expect() {
    if ! eval "$2" >/dev/null 2>&1; then
        fail "$1"
    fi
}

milestone() {
    printf '%s' "$1" | specbind milestone create --scope - >/dev/null \
        || fail "milestone create rejected the scope document"
}

brief() {
    spec=$1
    problem=$2
    [ -d ".specbind/specs/$spec" ] || fail "no spec directory for $spec"
    {
        echo "---"
        echo "type: SpecBind Brief"
        echo "---"
        echo
        echo "# Brief"
        echo
        echo "## Problem"
        echo
        echo "$problem"
        echo
        echo "## Desired outcome"
        echo
        echo "$problem"
        echo
        echo "## Scope boundaries"
        echo
        echo "Only this change."
        echo
        echo "## Known dependencies"
        echo
        echo "None."
    } > ".specbind/specs/$spec/brief.md"
    specbind artifact read "$spec" brief >/dev/null \
        || fail "the brief written for $spec is not a readable artifact"
}

leave_dirty=no

case "$scenario" in
base)
    ;;

d9)
    leave_dirty=yes
    printf '\n# pending experiment\n' >> src/cart.py
    expect "the uncommitted edit did not apply" \
        'test -n "$(git status --porcelain src/cart.py)"'
    ;;

d12)
    { echo "stray prose before the front matter"; cat .specbind/steering/structure.md; } \
        > steering.tmp
    mv steering.tmp .specbind/steering/structure.md
    git add -A
    git -c user.name=Fixture -c user.email=fixture@example.invalid \
        commit --quiet -m "Break a steering document"
    expect "steering still parses; the break did not apply" \
        '! specbind steering list'
    ;;

r1)
    milestone '{"schemaVersion":1,"workItems":{"newSpecs":[{"spec":"order","summary":"Let a customer cancel an order they placed."}]}}'
    brief order "Customers cannot cancel an order once placed."
    expect "order did not reach the requirements state" \
        'specbind spec status order | grep -q "State: requirements"'
    ;;

r3)
    milestone '{"schemaVersion":1,"workItems":{"specUpdates":[{"spec":"cart","summary":"Remove cart reporting."}]}}'
    brief cart "Cart reporting is no longer offered."
    expect "cart did not reach the requirements state" \
        'specbind spec status cart | grep -q "State: requirements"'
    ;;

r4 | r5)
    milestone '{"schemaVersion":1,"workItems":{"specUpdates":[{"spec":"cart","summary":"Cap cart quantities at 99 per SKU."}]}}'
    brief cart "A cart has no upper bound per SKU."
    if [ "$scenario" = r5 ]; then
        # The cap has to exist as an approved requirement, or a later request to
        # change it has nothing to change and the scenario measures the fixture
        # instead of the skill.
        awk '{ print }
             /^3\. A quantity below one is rejected/ {
               print "4. Adding a SKU where the resulting held quantity would exceed 99 is rejected and states the largest accepted quantity."
             }' .specbind/specs/cart/requirements.md > requirements.tmp
        mv requirements.tmp .specbind/specs/cart/requirements.md
        expect "the cap criterion was not added" \
            'specbind artifact read cart requirements | grep -q "exceed 99"'
        specbind spec requirements approve cart \
            --approval-mode explicit --requirement-ids 1.1,1.2,1.3,1.4 >/dev/null \
            || fail "could not approve the requirements gate"
        expect "the requirements gate is not approved" \
            'specbind spec status cart | grep -q "requirements=fresh"'
        expect "the approved set does not carry the cap criterion" \
            'grep -q "\"1.4\"" .specbind/specs/cart/spec.yaml'
    fi
    ;;

c2)
    # Real policy, so the checkpoint has something to obey. Removing the
    # instruction comments is what makes this the project's own writing rather
    # than the scaffold as installed.
    {
        echo "---"
        echo "type: SpecBind Git Adapter"
        echo "---"
        echo
        echo "# Git adapter"
        echo
        echo "## When to checkpoint"
        echo
        echo "Commit after each approved gate. Do not commit unapproved work."
        echo
        echo "## What to include"
        echo
        echo "Only the paths the run produced."
        echo
        echo "## Commit messages"
        echo
        echo 'Prefix every message with `spec:`, for example `spec: approve cart requirements`.'
        echo
        echo "## Branches and pushing"
        echo
        echo "Stay on the current branch. Never push."
    } > .specbind/settings/adapters/git.md
    git add -A
    git -c user.name=Fixture -c user.email=fixture@example.invalid \
        commit --quiet -m "State the project commit policy"
    expect "the adapter still carries its scaffold comments" \
        '! specbind adapter read git | grep -q "specbind:instruction"'
    ;;

*)
    fail "unknown scenario: $scenario"
    ;;
esac

# A recipe that leaves machine state uncommitted hands over a fixture whose next
# guarded operation is blocked: gate invalidation refuses a dirty `spec.yaml`,
# and nothing in a default install is authorized to commit. The scenario would
# then measure that instead of the skill. Recipes end committed, except the ones
# whose precondition is precisely an uncommitted worktree.
if [ "$leave_dirty" = yes ]; then
    expect "the scenario needs an uncommitted change and has none" \
        'test -n "$(git status --porcelain)"'
else
    if [ -n "$(git status --porcelain)" ]; then
        git add -A
        git -c user.name=Fixture -c user.email=fixture@example.invalid \
            commit --quiet -m "Set up the $scenario scenario"
    fi
    expect "the fixture did not end on a clean worktree" \
        'test -z "$(git status --porcelain)"'
fi

echo "Scenario $scenario ready at $target"
echo "  language: $language"
echo
echo "Put the CLI on PATH before starting the session:"
echo
echo "    export PATH=\"$(CDPATH= cd -- .specbind/bin && pwd):\$PATH\""
echo
echo "The request and the expectations are in docs/skill-forward-tests.md."
