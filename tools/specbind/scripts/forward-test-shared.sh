#!/usr/bin/env sh
# Decision 0209 shared-resource starting states; one fresh directory per run.
set -eu
scenario=${1:?usage: forward-test-shared.sh sh1|sh2|sh3 target}
target=${2:?target required}
case "$scenario" in sh1|sh2|sh3) ;; *) echo 'unknown scenario' >&2; exit 1;; esac
script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
if [ "$scenario" = sh3 ]; then
    sh "$script_dir/forward-test-scenario.sh" a4 "$target" en
else
    sh "$script_dir/forward-test-fixture.sh" "$target" en
fi
cd "$target"
PATH="$(CDPATH= cd -- .specbind/bin && pwd):$PATH"
export PATH
if [ "$scenario" = sh3 ]; then
    cat > .specbind/specs/shared-contract.yaml <<'EOF'
schema_version: 1
resources:
  - id: project-overview
    description: Shared overview of the bookshop project.
    paths: [README.md]
    change_policy: Maintainers may update project overview prose while preserving the documented project purpose.
    invariants: []
EOF
    cat >> .specbind/adoption/reverse-discovery.yaml <<'EOF'
shared_resources:
  - id: project-overview
    paths: [README.md]
    evidence: README.md
    disposition: Confirmed shared project documentation agreement; no independent behavior or implementation change.
EOF
    git add .specbind/specs/shared-contract.yaml .specbind/adoption/reverse-discovery.yaml
    git commit --quiet -m 'Record confirmed reverse shared agreement'
    specbind adoption preflight | grep -q 'ADOPTION_RESUME_READY'
    specbind milestone status --json | grep -q 'reverse_resume'
    test -z "$(git status --porcelain)"
    exit 0
fi
mkdir -p locales
printf '%s\n' '{"common.title":"Bookshp"}' > locales/en.json
printf '%s\n' '{"common.title":"Bookshop"}' > locales/ja.json
if [ "$scenario" = sh2 ]; then
    cat > .specbind/specs/shared-contract.yaml <<'EOF'
schema_version: 1
resources:
  - id: translations
    description: English and Japanese translation catalogs.
    paths: [locales/en.json, locales/ja.json]
    change_policy: Features may update their own namespace. Direct maintenance may correct existing copy without changing keys or behavior.
    invariants:
      - Both catalogs expose matching keys.
      - Corresponding messages use matching interpolation variables.
EOF
fi
git add locales .specbind
git commit --quiet -m 'Seed translation catalogs'
if [ "$scenario" = sh1 ]; then
    printf '%s' '{"schemaVersion":1,"workItems":{"directChanges":[{"id":"catalog-agreement","summary":"Register locales/en.json and locales/ja.json as shared resource translations. Features may update their own namespace; Direct maintenance may correct existing copy without changing keys or behavior. Preserve matching keys and interpolation variables across languages.","sharedContractChanges":["translations"]}]}}' | specbind milestone create --scope - >/dev/null
    test ! -e .specbind/specs/shared-contract.yaml
    specbind milestone status --json | grep -q 'shared_contract_plan'
else
    printf '%s' '{"schemaVersion":1,"workItems":{"directChanges":[{"id":"catalog-typo","summary":"Correct common.title in locales/en.json from Bookshp to Bookshop. Preserve the translation keys, Japanese catalog, and shared agreement."}]}}' | specbind milestone create --scope - >/dev/null
    specbind contract owners locales/en.json | grep -q 'shared-contract#resources/translations'
    specbind milestone review status | grep -q 'not_applicable'
fi
git add .specbind/steering/roadmap.md
git commit --quiet -m 'Scope shared-resource scenario'
specbind milestone scope
test -z "$(git status --porcelain)"
