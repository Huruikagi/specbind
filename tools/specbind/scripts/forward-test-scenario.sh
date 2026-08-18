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
#   c3     c2 plus a confirmed cart scope, so a run reaches the approval point
#   d7     cart driven to implementation with every gate approved
#   d10    a milestone whose Direct item is already completed
#   ds1    a new `order` Spec with its requirements authored and approved
#   ds2    r5's state: the cart cap approved, so design is the next phase
#   ds3    ds2 with the requirements edited afterwards, so that gate is stale
#   ds4    cart with the design gate approved and the cross-spec review accepted
#   ds5    ds2 plus a `checkout` Spec consuming the cart export
#   t1     ds4's state: design approved and the review accepted, no plan yet
#   t2     t1 without the accepted cross-spec review
#   t3     an approved three-task plan with the first two already completed
#   t4     d7's state: the tasks gate approved and cart in implementation

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

# The cart quantity cap, approved. Five recipes need the same starting contract,
# and building it five times is where they drift apart.
cart_cap_approved() {
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
}

# Decision 0061 wants the Front Matter set and the union of the body markers to
# match exactly, so both list the same four IDs.
cart_design_approved() {
    {
        echo "---"
        echo "type: SpecBind Design"
        echo "artifact_id: main"
        echo "requirement_ids:"
        echo '  - "1.1"'
        echo '  - "1.2"'
        echo '  - "1.3"'
        echo '  - "1.4"'
        echo "---"
        echo
        echo "# Design"
        echo
        echo "## Holding and capping quantities"
        echo
        echo "add_item reads the current held quantity, applies the floor and the"
        echo "cap, and leaves the cart unchanged when either bound is violated."
        echo
        echo "_Requirements: 1.1, 1.2, 1.3, 1.4_"
    } > .specbind/specs/cart/design.md
    specbind spec design approve cart --approval-mode explicit >/dev/null \
        || fail "could not approve the design gate"
}

# Decision 0078 requires this before Tasks authoring, and the acceptance itself
# refuses to run while a task plan exists. Every recipe that writes a plan
# accepts the review first.
cross_spec_review_accepted() {
    printf '%s' '{"schemaVersion":1,"assessment":"One Spec participates and its contract is unchanged.","deepInputs":[]}' \
        | specbind milestone review accept --candidate - >/dev/null \
        || fail "could not accept the cross-spec review"
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

r4 | r5 | ds2 | ds3 | ds5)
    milestone '{"schemaVersion":1,"workItems":{"specUpdates":[{"spec":"cart","summary":"Cap cart quantities at 99 per SKU."}]}}'
    brief cart "A cart has no upper bound per SKU."
    if [ "$scenario" != r4 ]; then
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
    if [ "$scenario" = ds3 ]; then
        # An edit after the approval, so the gate the design phase depends on is
        # stale rather than missing. The design skill must route this back
        # instead of repairing it.
        printf '4. Reading a cart states the largest accepted quantity per SKU.\n' \
            >> .specbind/specs/cart/requirements.md
        expect "the requirements gate is still fresh" \
            'specbind spec status cart | grep -q "requirements=stale"'
    fi
    if [ "$scenario" = ds5 ]; then
        # A second persistent Spec that consumes the cart export, so removing
        # that export has a consumer the graph can name.
        mkdir -p .specbind/specs/checkout
        printf 'schema_version: 1\nactive_change: null\n' \
            > .specbind/specs/checkout/spec.yaml
        {
            echo "---"
            echo "type: SpecBind Requirements"
            echo "heading_labels:"
            echo "  requirement: Requirement"
            echo "  acceptance_criteria: Acceptance Criteria"
            echo "---"
            echo
            echo "# Requirements"
            echo
            echo "## Context"
            echo
            echo "A customer turns an assembled cart into a placed order."
            echo
            echo "## Scope"
            echo
            echo "### In scope"
            echo
            echo "Placing an order from a cart."
            echo
            echo "### Out of scope"
            echo
            echo "Payment and fulfilment."
            echo
            echo "## Requirements"
            echo
            echo "### Requirement 1: Place an order"
            echo
            echo "**Objective:** A customer can commit to the purchase they assembled."
            echo
            echo "#### Acceptance Criteria"
            echo
            echo "1. Placing an order records every SKU the cart holds."
            echo "2. Placing an order from an empty cart is rejected."
        } > .specbind/specs/checkout/requirements.md
        {
            echo "---"
            echo "type: SpecBind Contract"
            echo "---"
            echo
            echo "# Contract"
            echo
            echo "## Owns"
            echo
            echo '- `placed-order` — the committed record of a purchase'
            echo
            echo "## Exports"
            echo
            echo "## Consumes"
            echo
            echo '- `cart-add` → `cart/exports/add-item` — replays a cart while placing an order'
            echo
            echo "## Invariants"
            echo
            echo "## File Ownership"
            echo
            echo '- `checkout-module` — `src/checkout.py`'
        } > .specbind/specs/checkout/contract.md
        expect "the seeded contract graph does not resolve" \
            'specbind check contracts'
        expect "checkout does not consume the cart export" \
            'specbind artifact read checkout contract | grep -q "cart/exports/add-item"'
    fi
    ;;

ds1)
    milestone '{"schemaVersion":1,"workItems":{"newSpecs":[{"spec":"order","summary":"Let a customer cancel an order they placed."}]}}'
    brief order "Customers cannot cancel an order once placed."
    # Requirements authored and approved deterministically. Only the design
    # phase is under test, and three runs starting from the same contract differ
    # in the request rather than in what the previous phase happened to write.
    {
        echo "---"
        echo "type: SpecBind Requirements"
        echo "heading_labels:"
        echo "  requirement: Requirement"
        echo "  acceptance_criteria: Acceptance Criteria"
        echo "---"
        echo
        echo "# Requirements"
        echo
        echo "## Context"
        echo
        echo "A customer sometimes changes their mind after committing to a purchase."
        echo
        echo "## Scope"
        echo
        echo "### In scope"
        echo
        echo "Cancelling a placed order while cancellation is still allowed."
        echo
        echo "### Out of scope"
        echo
        echo "Refund settlement and fulfilment reversal."
        echo
        echo "## Requirements"
        echo
        echo "### Requirement 1: Cancel a placed order"
        echo
        echo "**Objective:** A customer can withdraw an order they no longer want."
        echo
        echo "#### Acceptance Criteria"
        echo
        echo "1. Cancelling an order within the cancellation window marks it cancelled and states when it was cancelled."
        echo "2. Cancelling an order after the cancellation window is rejected and states when the window closed."
        echo "3. Cancelling an order that is already cancelled leaves it cancelled and reports no further change."
    } > .specbind/specs/order/requirements.md
    specbind spec requirements approve order \
        --approval-mode explicit --requirement-ids 1.1,1.2,1.3 >/dev/null \
        || fail "could not approve the requirements gate"
    expect "order did not reach the design state" \
        'specbind spec status order | grep -q "State: design"'
    expect "order already has a contract" \
        '! test -e .specbind/specs/order/contract.md'
    ;;

ds4 | t1 | t2)
    milestone '{"schemaVersion":1,"workItems":{"specUpdates":[{"spec":"cart","summary":"Cap cart quantities at 99 per SKU."}]}}'
    brief cart "A cart has no upper bound per SKU."
    cart_cap_approved
    cart_design_approved
    expect "the design gate is not approved" \
        'specbind spec status cart | grep -q "design=fresh"'
    expect "a task plan already exists" \
        '! test -e .specbind/specs/cart/tasks.yaml'
    if [ "$scenario" = t2 ]; then
        # t2 measures what the tasks phase does when the review has not been
        # accepted, so this is the one recipe that deliberately leaves it out.
        expect "the cross-spec review is already accepted" \
            'specbind milestone review status | grep -q "Status: absent"'
    else
        cross_spec_review_accepted
        expect "no accepted cross-spec review was written" \
            'test -e .specbind/state/cross-spec-review.md'
    fi
    ;;

d7 | t4)
    milestone '{"schemaVersion":1,"workItems":{"specUpdates":[{"spec":"cart","summary":"Cap cart quantities at 99 per SKU."}]}}'
    brief cart "A cart has no upper bound per SKU."
    cart_cap_approved
    cart_design_approved
    cross_spec_review_accepted
    {
        echo "schema_version: 1"
        echo "plan:"
        echo "  items:"
        echo "    - id: '1'"
        echo "      kind: task"
        echo "      title: Enforce the quantity bounds"
        echo "      requirement_ids: ['1.1', '1.2', '1.3', '1.4']"
    } > .specbind/specs/cart/tasks.yaml
    specbind spec tasks approve cart --approval-mode explicit >/dev/null \
        || fail "could not approve the tasks gate"
    expect "cart did not reach implementation with every gate fresh" \
        'specbind spec status cart | grep -q "requirements=fresh, design=fresh, tasks=fresh"'
    ;;

t3)
    milestone '{"schemaVersion":1,"workItems":{"specUpdates":[{"spec":"cart","summary":"Cap cart quantities at 99 per SKU."}]}}'
    brief cart "A cart has no upper bound per SKU."
    cart_cap_approved
    cart_design_approved
    cross_spec_review_accepted
    # Three tasks, two of them finished. A revision that inserts work ahead of
    # them renumbers the completed entries, which is the case the mapping rule
    # exists for. One task would not renumber anything.
    {
        echo "schema_version: 1"
        echo "plan:"
        echo "  items:"
        echo "    - id: '1'"
        echo "      kind: task"
        echo "      title: Reject a quantity below one"
        echo "      requirement_ids: ['1.3']"
        echo "    - id: '2'"
        echo "      kind: task"
        echo "      title: Record and increase held quantities"
        echo "      requirement_ids: ['1.1', '1.2']"
        echo "    - id: '3'"
        echo "      kind: task"
        echo "      title: Reject a quantity above the cap"
        echo "      requirement_ids: ['1.4']"
    } > .specbind/specs/cart/tasks.yaml
    specbind spec tasks approve cart --approval-mode explicit >/dev/null \
        || fail "could not approve the tasks gate"
    specbind tasks complete cart 1 >/dev/null || fail "could not complete task 1"
    specbind tasks complete cart 2 >/dev/null || fail "could not complete task 2"
    expect "the recorded progress did not take" \
        'specbind tasks list cart | grep -q "2 completed"'
    expect "task 3 is not the remaining pending one" \
        'specbind tasks list cart | grep -q "\[pending actionable\] 3 "'
    ;;

d10)
    milestone '{"schemaVersion":1,"workItems":{"directChanges":[{"id":"contributing-guide","summary":"Add a CONTRIBUTING guide."}]}}'
    printf '%s\n' "# Contributing" "" "Open an issue before a large change." > CONTRIBUTING.md
    git add -A
    git -c user.name=Fixture -c user.email=fixture@example.invalid \
        commit --quiet -m "Add the contributing guide"
    specbind milestone direct complete contributing-guide \
        --implementation-revision "$(git rev-parse HEAD)" >/dev/null \
        || fail "could not complete the Direct item"
    expect "the Direct item is not recorded as completed" \
        'specbind milestone status | grep -q "1/1 completed"'
    ;;

c2 | c3)
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
    if [ "$scenario" = c3 ]; then
        # Scope already confirmed, so the run reaches the requirements phase and
        # the adapter's "commit after each approved gate" has something to tempt
        # it with when the approval does not happen.
        milestone '{"schemaVersion":1,"workItems":{"specUpdates":[{"spec":"cart","summary":"Cap cart quantities at 99 per SKU."}]}}'
        brief cart "A cart has no upper bound per SKU."
        expect "cart did not reach the requirements state" \
            'specbind spec status cart | grep -q "State: requirements"'
    fi
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
