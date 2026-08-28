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
# expectations to check afterwards are routed from the
# docs/skill-forward-tests.md index. The scenario documents stay the contract.
#
# Usage: forward-test-scenario.sh <scenario> <target-directory> [en|ja] [--instrument-dispatch]
#
# Scenarios:
#   base   the fixture as built, nothing added
#   a1     an initial-adoption project with no Specs and no Steering
#   a2     an initial-adoption project with no Specs and complete Steering
#   d9     base plus an uncommitted edit to an owned file
#   d12    base plus a steering document that cannot be parsed
#   r1     milestone scoping a new `order` Spec, with its brief written
#   r6     r1 plus a Requirements template with one repeated Unicode variable
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
#   ds4    cart with the design gate approved and the contract review accepted
#   ds5    ds2 plus a `checkout` Spec consuming the cart export
#   ds7    a new dashboard Spec whose user-visible screen requires Design UI
#   ds8    a new parser Spec whose library-only behavior must omit Design UI
#   t1     ds4's state: design approved and the review accepted, no plan yet
#   t2     t1 without the accepted contract review
#   t3     an approved three-task plan with the first two already completed
#   t4     d7's state: the tasks gate approved and cart in implementation
#   x1     t2's state: one participant ready for review, contract unchanged
#   x2     ds5 with cart's approved design removing the export checkout consumes
#   x3     cart in tasks state with a plan already written and no review
#   x4     d10's state: a Direct-only milestone that needs no review
#   i3     a Direct item still pending, for the run under test to implement
#   i4     t4 plus an unrelated uncommitted edit the run must not touch
#   i6     cart in implementation with two sequential Tasks to checkpoint apart
#   rt1    t4 plus an uncommitted implementation that caps at the wrong bound
#   rt2    rt1 plus unrelated uncommitted work no task owns
#   db1    t4 whose approved design contradicts the requirements, gates fresh
#   vi1    t4 implemented correctly, task recorded, with a real test command
#   vi2    vi1 with the cap off by one, so the suite fails
#   vi3    vi1 with the canonical test command removed
#   vd1    an approved design that defers the bound to a research document
#   rl1    cart released-ready but with no version bound yet
#   rl2    rl3 plus a release adapter whose Verify step cannot succeed
#   rl3    a milestone ready for release with an explicitly empty adapter body
#   rl4    rl3 readiness with the installed Release scaffold and release docs

set -eu

scenario=${1:?usage: forward-test-scenario.sh <scenario> <target-directory> [en|ja] [--instrument-dispatch]}
target=${2:?usage: forward-test-scenario.sh <scenario> <target-directory> [en|ja] [--instrument-dispatch]}
language=${3:-en}
instrument_dispatch=${4:-}

if [ -n "$instrument_dispatch" ] && [ "$instrument_dispatch" != "--instrument-dispatch" ]; then
    echo "forward-test-scenario: unknown option: $instrument_dispatch" >&2
    exit 1
fi

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)

if [ "$instrument_dispatch" = "--instrument-dispatch" ]; then
    sh "$script_dir/forward-test-fixture.sh" "$target" "$language" "$instrument_dispatch" >/dev/null
else
    sh "$script_dir/forward-test-fixture.sh" "$target" "$language" >/dev/null
fi
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
    desired=$3
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
        echo "$desired"
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
#
# The mechanism sentence is a parameter because one scenario needs a design that
# contradicts the requirements. Writing that before approval keeps every gate
# fresh, so the run meets the contradiction itself rather than a stale-gate
# report, which is a louder and different signal.
cart_design_approved() {
    mechanism=${1:-"cap, and leaves the cart unchanged when either bound is violated."}
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
        echo "$mechanism"
        echo
        echo "_Requirements: 1.1, 1.2, 1.3, 1.4_"
    } > .specbind/specs/cart/design.md
    specbind spec design approve cart --approval-mode explicit >/dev/null \
        || fail "could not approve the design gate"
}

# Decision 0078 requires this before Tasks authoring, and the acceptance itself
# refuses to run while a task plan exists. Every recipe that writes a plan
# accepts the review first.
contract_review_accepted() {
    printf '%s' '{"schemaVersion":1,"assessment":"One Spec participates and its contract is unchanged.","deepInputs":[]}' \
        | specbind milestone review accept --candidate - >/dev/null \
        || fail "could not accept the contract review"
}

# The base fixture has no verification command at all, which would make every
# validation run correctly return MANUAL_VERIFY_REQUIRED and make the GO and the
# cannot-verify scenarios indistinguishable. These recipes add a real one.
# Probe by running, not by looking. On Windows `python3` resolves to a Microsoft
# Store stub that is on PATH, exits successfully, and prints an advertisement
# instead of interpreting anything — so `command -v` finds an interpreter that
# cannot run a test.
python_runner() {
    for candidate in python py python3; do
        if "$candidate" -c "import sys; sys.exit(0)" >/dev/null 2>&1; then
            echo "$candidate"
            return 0
        fi
    done
    fail "no working python interpreter found; the validation scenarios need one to run their tests"
}

cart_tests() {
    mkdir -p tests
    # `unittest discover` requires an importable start directory.
    : > tests/__init__.py
    cat > tests/test_cart.py <<'PYEOF'
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "src"))

from cart import add_item


class AddItemTest(unittest.TestCase):
    def test_records_a_new_sku(self):
        self.assertEqual(add_item({}, "a", 2), {"a": 2})

    def test_increases_an_existing_sku(self):
        self.assertEqual(add_item({"a": 1}, "a", 2), {"a": 3})

    def test_rejects_below_one(self):
        with self.assertRaises(ValueError):
            add_item({}, "a", 0)

    def test_rejects_above_the_cap(self):
        cart = {}
        with self.assertRaises(ValueError) as raised:
            add_item(cart, "a", 100)
        self.assertIn("99", str(raised.exception))
        self.assertEqual(cart, {})


if __name__ == "__main__":
    unittest.main()
PYEOF
    {
        echo "#!/usr/bin/env sh"
        echo "# The project's canonical test command."
        echo "set -eu"
        echo "exec $1 -m unittest discover -s tests -t . \"\$@\""
    } > scripts-test.sh
    mkdir -p scripts
    mv scripts-test.sh scripts/test.sh
    chmod +x scripts/test.sh
    {
        echo
        echo "## Verification"
        echo
        echo 'The canonical test command is `sh scripts/test.sh`. It must pass before any'
        echo "change is considered complete."
    } >> .specbind/steering/conventions.md
}

cart_cap_implemented() {
    {
        echo '"""Holds items a customer intends to buy."""'
        echo
        echo
        echo "MAX_PER_SKU = 99"
        echo
        echo
        echo "def add_item(cart, sku, quantity):"
        echo "    if quantity < 1:"
        echo '        raise ValueError("quantity must be at least 1")'
        echo "    current = cart.get(sku, 0)"
        echo "    if current + quantity > MAX_PER_SKU:"
        echo '        raise ValueError(f"at most {MAX_PER_SKU} per SKU")'
        echo "    cart[sku] = current + quantity"
        echo "    return cart"
    } > src/cart.py
}

leave_dirty=no

case "$scenario" in
base)
    ;;

a1 | a2)
    rm -rf .specbind/specs
    if [ "$scenario" = a1 ]; then
        rm -rf .specbind/steering
        mkdir -p .specbind/steering
    else
        cat > .specbind/steering/product.md <<'EOF'
---
type: SpecBind Steering
artifact_id: product
---

# Product

The bookshop lets a customer collect intended purchases and place an order.
Adoption must preserve the distinction between a mutable cart and a committed
order. Payment and fulfilment are outside the current product.
EOF
        cat > .specbind/steering/technology.md <<'EOF'
---
type: SpecBind Steering
artifact_id: technology
---

# Technology

The service is a Python codebase. Repository-local tests are the verification
surface; adoption documents current behavior but does not change source code.
EOF
    fi
    expect "the adoption fixture still has a persistent Spec" \
        'specbind spec list | grep -q "Found 0 spec(s)"'
    if [ "$scenario" = a1 ]; then
        expect "the no-Steering fixture still lists guidance" \
            'specbind steering list | grep -q "Found 0 steering document(s)"'
    else
        expect "the adoption Steering baseline is incomplete" \
            'specbind steering list | grep -q "Found 4 steering document(s)"'
    fi
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

r1 | r6)
    milestone '{"schemaVersion":1,"workItems":{"newSpecs":[{"spec":"order","summary":"Let a customer cancel an order they placed."}]}}'
    brief order \
        "Customers cannot cancel an order once placed." \
        "Customers can cancel an order they placed before its cancellation window closes; a later cancellation is rejected."
    if [ "$scenario" = r6 ]; then
        cat > .specbind/settings/templates/specs/requirements.md <<'EOF'
---
type: SpecBind Requirements
heading_labels:
  requirement: Requirement
  acceptance_criteria: Acceptance Criteria
---

<!-- specbind:instruction create bind=作成日
Resolve `作成日` to the exact literal value `fixture-day`.
-->

# Requirements prepared on {{作成日}}

Prepared on {{作成日}}.

<!-- specbind:instruction maintain
Keep this as the Spec's complete current behavioral contract.
-->

## Requirements
EOF
        expect "the repeated Unicode variable template was not accepted" \
            'specbind template read spec requirements | grep -q "{{作成日}}"'
        expect "the Unicode variable does not have two references" \
            'test "$(grep -o "{{作成日}}" .specbind/settings/templates/specs/requirements.md | wc -l | tr -d " ")" = 2'
    fi
    expect "order did not reach the requirements state" \
        'specbind spec status order | grep -q "State: requirements"'
    ;;

r3)
    milestone '{"schemaVersion":1,"workItems":{"specUpdates":[{"spec":"cart","summary":"Remove cart reporting."}]}}'
    brief cart \
        "The established cart contract still includes reporting behavior that the product no longer offers." \
        "Cart reporting is removed from the supported cart behavior."
    expect "cart did not reach the requirements state" \
        'specbind spec status cart | grep -q "State: requirements"'
    ;;

r4 | r5 | ds2 | ds3 | ds5 | x2)
    milestone '{"schemaVersion":1,"workItems":{"specUpdates":[{"spec":"cart","summary":"Cap cart quantities at 99 per SKU."}]}}'
    brief cart \
        "A cart has no upper bound per SKU." \
        "A cart rejects an addition that would raise one SKU above 99."
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
    if [ "$scenario" = ds2 ]; then
        # An established Spec can still be authoring its first Design. Relocate
        # the project-owned template so the run must resolve that new set's
        # target instead of treating "existing Spec" as "existing Design" or
        # guessing design.md from the Requirements path.
        mkdir -p .specbind/settings/templates/specs/technical-design
        mv .specbind/settings/templates/specs/design.md \
            .specbind/settings/templates/specs/technical-design/main.md
        expect "the relocated Design template did not resolve to its custom target" \
            'specbind template resolve spec cart design/main | grep -q "Target path: specs/cart/technical-design/main.md"'
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
    if [ "$scenario" = ds5 ] || [ "$scenario" = x2 ]; then
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
    if [ "$scenario" = x2 ]; then
        # The approved design removes the export checkout consumes. Design
        # approval does not run the project-wide graph check, so this state is
        # reachable exactly as a real milestone reaches it — which is the whole
        # reason the contract review exists.
        # The contract is reduced before the gate is approved, so the approval
        # covers the removal. Approving first and editing after would leave a
        # stale gate and measure freshness instead of the seam.
        awk '!/^- `add-item`/' .specbind/specs/cart/contract.md > contract.tmp
        mv contract.tmp .specbind/specs/cart/contract.md
        cart_design_approved
        expect "the cart export was not removed" \
            '! specbind artifact read cart contract | grep -q "add-item"'
        expect "the dangling consumer did not appear" \
            '! specbind check contracts'
    fi
    ;;

x3)
    milestone '{"schemaVersion":1,"workItems":{"specUpdates":[{"spec":"cart","summary":"Cap cart quantities at 99 per SKU."}]}}'
    brief cart \
        "A cart has no upper bound per SKU." \
        "A cart rejects an addition that would raise one SKU above 99."
    cart_cap_approved
    cart_design_approved
    # The trap state: a plan authored before the review was accepted. Acceptance
    # now refuses, and the only exit is a decision about the plan that belongs to
    # the user rather than to the review.
    {
        echo "schema_version: 1"
        echo "plan:"
        echo "  items:"
        echo "    - id: '1'"
        echo "      kind: task"
        echo "      title: Enforce the quantity bounds"
        echo "      requirement_ids: ['1.1', '1.2', '1.3', '1.4']"
    } > .specbind/specs/cart/tasks.yaml
    expect "cart is not waiting in the tasks state" \
        'specbind spec status cart | grep -q "State: tasks"'
    expect "the contract review is already accepted" \
        'specbind milestone review status | grep -q "Status: absent"'
    expect "no task plan is present" \
        'test -e .specbind/specs/cart/tasks.yaml'
    ;;

ds1)
    milestone '{"schemaVersion":1,"workItems":{"newSpecs":[{"spec":"order","summary":"Let a customer cancel an order they placed."}]}}'
    brief order \
        "Customers cannot cancel an order once placed." \
        "Customers can cancel an order they placed before its cancellation window closes; a later cancellation is rejected."
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
    # Put the project-owned Design scaffold somewhere its artifact_id does not
    # reveal. A passing run must resolve the target instead of guessing
    # `design.md` from earlier conventions.
    mkdir -p .specbind/settings/templates/specs/technical-design
    mv .specbind/settings/templates/specs/design.md \
        .specbind/settings/templates/specs/technical-design/main.md
    expect "the relocated Design template did not resolve to its custom target" \
        'specbind template resolve spec order design/main | grep -q "Target path: specs/order/technical-design/main.md"'
    expect "order did not reach the design state" \
        'specbind spec status order | grep -q "State: design"'
    expect "unstarted Design is still reported as inconsistent" \
        'specbind spec status order | grep -q "Health: consistent"'
    expect "status does not route the next workflow to Design" \
        'specbind spec status order | grep -q "Next action: design"'
    expect "status does not aggregate the expected Design coverage" \
        'specbind spec status order | grep -q "Expected work: cover 3 active requirement(s) in Design"'
    expect "order already has a contract" \
        '! test -e .specbind/specs/order/contract.md'
    ;;

ds7 | ds8)
    if [ "$scenario" = ds7 ]; then
        spec=dashboard
        summary="Add a customer account overview screen."
        problem="Customers cannot see their account status in one place."
        desired="A responsive account overview screen shows status, recent activity, loading, empty, and error states with accessible navigation."
        context="A signed-in customer needs a visual overview of their account."
        title="Show the account overview screen"
        objective="A customer can understand their account status from one accessible screen."
        criterion="The account overview screen presents account status and recent activity with defined loading, empty, error, responsive, and keyboard-navigation behavior."
    else
        spec=parser
        summary="Expose a library function that parses catalog identifiers."
        problem="Callers parse catalog identifiers inconsistently."
        desired="A library-only parser returns one normalized identifier or a typed invalid-input error without changing any user interface."
        context="Internal callers need one stable parsing boundary."
        title="Parse a catalog identifier"
        objective="A caller can normalize a catalog identifier through one library API."
        criterion="The parser returns the normalized identifier for valid input and a typed invalid-input error otherwise; no screen, interaction, or user-visible UI behavior changes."
    fi
    milestone "{\"schemaVersion\":1,\"workItems\":{\"newSpecs\":[{\"spec\":\"$spec\",\"summary\":\"$summary\"}]}}"
    brief "$spec" "$problem" "$desired"
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
        echo "$context"
        echo
        echo "## Scope"
        echo
        echo "### In scope"
        echo
        echo "$desired"
        echo
        echo "### Out of scope"
        echo
        echo "Unrelated product behavior."
        echo
        echo "## Requirements"
        echo
        echo "### Requirement 1: $title"
        echo
        echo "**Objective:** $objective"
        echo
        echo "#### Acceptance Criteria"
        echo
        echo "1. $criterion"
    } > ".specbind/specs/$spec/requirements.md"
    specbind spec requirements approve "$spec" \
        --approval-mode explicit --requirement-ids 1.1 >/dev/null \
        || fail "could not approve the requirements gate"
    expect "$spec did not reach the design state" \
        "specbind spec status $spec | grep -q 'State: design'"
    expect "the Design selection rule does not classify both standard candidates" \
        'specbind rule read design-template-selection --for consume | grep -q "design/ui"'
    ;;

ds4 | t1 | t2 | x1 | vd1)
    milestone '{"schemaVersion":1,"workItems":{"specUpdates":[{"spec":"cart","summary":"Cap cart quantities at 99 per SKU."}]}}'
    brief cart \
        "A cart has no upper bound per SKU." \
        "A cart rejects an addition that would raise one SKU above 99."
    cart_cap_approved
    if [ "$scenario" = vd1 ]; then
        # The design points at Research for the bound instead of stating it.
        # Research is excluded from every gate fingerprint and is deleted at
        # release, so this design becomes incomplete the moment the milestone
        # closes — and nothing mechanical reports that.
        {
            echo "---"
            echo "type: SpecBind Research"
            echo "---"
            echo
            echo "# Research"
            echo
            echo "## Chosen bound"
            echo
            echo "Ninety-nine per SKU, matching the warehouse pick limit. Additions"
            echo "that would exceed it are rejected rather than trimmed."
        } > .specbind/specs/cart/research.md
        expect "the research artifact is not readable" \
            'specbind artifact read cart research | grep -q "Ninety-nine"'
        cart_design_approved "cap recorded in the research document, in the manner decided there."
    else
        cart_design_approved
    fi
    expect "the design gate is not approved" \
        'specbind spec status cart | grep -q "design=fresh"'
    expect "a task plan already exists" \
        '! test -e .specbind/specs/cart/tasks.yaml'
    if [ "$scenario" = t2 ] || [ "$scenario" = x1 ] || [ "$scenario" = vd1 ]; then
        # t2 measures what the tasks phase does when the review has not been
        # accepted, so this is the one recipe that deliberately leaves it out.
        expect "the contract review is already accepted" \
            'specbind milestone review status | grep -q "Status: absent"'
    else
        contract_review_accepted
        expect "no accepted contract review was written" \
            'test -e .specbind/state/contract-review.md'
    fi
    ;;

d7 | t4 | i4 | i6 | rt1 | rt2 | db1 | vi1 | vi2 | vi3 | rl1 | rl2 | rl3 | rl4)
    milestone '{"schemaVersion":1,"workItems":{"specUpdates":[{"spec":"cart","summary":"Cap cart quantities at 99 per SKU."}]}}'
    brief cart \
        "A cart has no upper bound per SKU." \
        "A cart rejects an addition that would raise one SKU above 99."
    cart_cap_approved
    if [ "$scenario" = db1 ]; then
        cart_design_approved "cap, silently trimming the addition to the cap instead of rejecting it."
    else
        cart_design_approved
    fi
    contract_review_accepted
    if [ "$scenario" = i6 ]; then
        {
            echo "schema_version: 1"
            echo "plan:"
            echo "  items:"
            echo "    - id: '1'"
            echo "      kind: task"
            echo "      title: Establish canonical coverage for current cart behavior"
            echo "      details:"
            echo "        - Add the project test command and cover recording, increasing, and the lower bound"
            echo "      requirement_ids: ['1.1', '1.2', '1.3']"
            echo "    - id: '2'"
            echo "      kind: task"
            echo "      title: Enforce and verify the upper quantity bound"
            echo "      requirement_ids: ['1.4']"
        } > .specbind/specs/cart/tasks.yaml
    else
        {
            echo "schema_version: 1"
            echo "plan:"
            echo "  items:"
            echo "    - id: '1'"
            echo "      kind: task"
            echo "      title: Enforce the quantity bounds"
            echo "      requirement_ids: ['1.1', '1.2', '1.3', '1.4']"
        } > .specbind/specs/cart/tasks.yaml
    fi
    specbind spec tasks approve cart --approval-mode explicit >/dev/null \
        || fail "could not approve the tasks gate"
    expect "cart did not reach implementation with every gate fresh" \
        'specbind spec status cart | grep -q "requirements=fresh, design=fresh, tasks=fresh"'
    if [ "$scenario" = i6 ]; then
        expect "the two-task checkpoint fixture does not have two pending Tasks" \
            'specbind tasks list cart | grep -q "0 completed, 2 pending, 0 blocked"'
        expect "Task 2 is not waiting behind Task 1" \
            'specbind tasks list cart | grep -q "\[pending waiting\] 2 "'
    fi
    case "$scenario" in
    rl1 | rl2 | rl3 | rl4)
        runner=$(python_runner)
        cart_tests "$runner"
        cart_cap_implemented
        specbind tasks complete cart 1 >/dev/null \
            || fail "could not record the task complete"
        expect "the canonical test command does not pass" \
            'sh scripts/test.sh'
        git add -A
        git -c user.name=Fixture -c user.email=fixture@example.invalid \
            commit --quiet -m "Implement the cap"
        if [ "$scenario" != rl1 ]; then
            # Bind before accepting completion. The reverse order stales the
            # evidence, because the roadmap write is a non-metadata project
            # change — which is exactly what rl1 is built to leave unbound.
            # Binding also refuses a dirty roadmap, hence the commit above.
            specbind milestone bind-release v1.4.0 >/dev/null \
                || fail "could not bind the release version"
            git add -A
            git -c user.name=Fixture -c user.email=fixture@example.invalid \
                commit --quiet -m "Bind the release version"
        fi
        if [ "$scenario" = rl2 ]; then
            # Real release policy whose Verify step cannot succeed. Commit it
            # before completion acceptance so release policy setup does not
            # itself stale the evidence the scenario is meant to start with.
            {
                echo "---"
                echo "type: SpecBind Release Adapter"
                echo "---"
                echo
                echo "# Release adapter"
                echo
                echo "## Prepare"
                echo
                echo "Run \`sh scripts/test.sh\` and confirm it passes."
                echo
                echo "## Publish"
                echo
                echo 'Create an annotated tag named after the release version.'
                echo
                echo "## Verify"
                echo
                echo 'Confirm the tag is present on the `origin` remote.'
                echo
                echo "## After finalize"
                echo
                echo "Nothing."
            } > .specbind/settings/adapters/release.md
            git add -A
            git -c user.name=Fixture -c user.email=fixture@example.invalid \
                commit --quiet -m "State the project release policy"
        elif [ "$scenario" = rl1 ] || [ "$scenario" = rl3 ]; then
            # Front Matter only is the project's explicit no-project-action
            # policy. It is intentionally different from the installed marker.
            {
                echo "---"
                echo "type: SpecBind Release Adapter"
                echo "---"
            } > .specbind/settings/adapters/release.md
            git add -A
            git -c user.name=Fixture -c user.email=fixture@example.invalid \
                commit --quiet -m "Choose core-only release finalization"
        elif [ "$scenario" = rl4 ]; then
            # Repository evidence for the bootstrap driver to translate into
            # the project-owned adapter without inventing an external remote.
            {
                echo "# Releasing"
                echo
                echo "1. Run \`sh scripts/test.sh\` and require a passing suite."
                echo "2. Before SpecBind finalization, record the current HEAD and create an annotated local tag whose name is the bound release version and whose target is that exact HEAD."
                echo "3. Verify the tag resolves to the recorded pre-finalization HEAD and rerun the suite from that tagged tree."
                echo "4. No project-specific work is required after SpecBind finalization."
            } > RELEASING.md
            git add -A
            git -c user.name=Fixture -c user.email=fixture@example.invalid \
                commit --quiet -m "Document the local release procedure"
        fi
        printf '%s' '{"schemaVersion":1,"implementationRevision":"'"$(git rev-parse HEAD)"'","mechanicalChecks":[{"kind":"test","command":"sh scripts/test.sh","exitCode":0}]}' \
            | specbind spec completion accept cart --evidence - >/dev/null \
            || fail "could not accept completion"
        git add -A
        git -c user.name=Fixture -c user.email=fixture@example.invalid \
            commit --quiet -m "Accept completion"
        if [ "$scenario" = rl1 ]; then
            expect "the release is already bound" \
                'specbind release preflight 2>&1 | grep -q RELEASE_VERSION_UNBOUND'
        else
            expect "the milestone is not ready for release" \
                'specbind release preflight | grep -q "OK RELEASE_READY"'
        fi
        if [ "$scenario" = rl2 ]; then
            expect "the adapter still carries its scaffold comments" \
                '! specbind adapter read release | grep -q "specbind:instruction"'
            expect "the fixture unexpectedly has a remote" \
                '! git remote | grep -q .'
        fi
        if [ "$scenario" = rl4 ]; then
            expect "the release scaffold was already configured" \
                'specbind adapter read release | grep -q "specbind:adapter-scaffold"'
            expect "the release documentation is missing" \
                'test -e RELEASING.md'
        fi
        ;;
    vi1 | vi2 | vi3)
        runner=$(python_runner)
        cart_tests "$runner"
        if [ "$scenario" = vi2 ]; then
            # Caps at the wrong bound, so the suite fails and the correct
            # verdict is NO-GO with a stated cause.
            cart_cap_implemented
            sed -i.bak 's/MAX_PER_SKU = 99/MAX_PER_SKU = 100/' src/cart.py
            rm -f src/cart.py.bak
        else
            cart_cap_implemented
        fi
        specbind tasks complete cart 1 >/dev/null \
            || fail "could not record the task complete"
        if [ "$scenario" = vi3 ]; then
            # The documented command exists in the conventions but cannot run.
            # Nothing is known to be wrong and nothing is known to be right,
            # which is the distinction MANUAL_VERIFY_REQUIRED carries.
            rm -f scripts/test.sh
            expect "the canonical command is still runnable" \
                '! test -e scripts/test.sh'
        elif [ "$scenario" = vi1 ]; then
            expect "the canonical test command does not pass" \
                'sh scripts/test.sh'
        else
            # Specifically the cap test, not merely "something failed" — a bare
            # negation would also be satisfied by a broken harness, which is how
            # a recipe passes while building the wrong thing.
            expect "the cap test does not fail as the scenario needs" \
                'sh scripts/test.sh 2>&1 | grep -q "test_rejects_above_the_cap"'
            expect "the suite reports no failure at all" \
                'sh scripts/test.sh 2>&1 | grep -qE "FAILED \(failures=1\)"'
        fi
        expect "cart is not ready for completion validation" \
            'specbind tasks list cart | grep -q "1 completed"'
        ;;
    esac
    if [ "$scenario" = rt1 ] || [ "$scenario" = rt2 ]; then
        # An implementation that caps at the wrong bound and never states the
        # largest accepted quantity. Left uncommitted so the diff is the change
        # under review. A correct review rejects it and repairs nothing.
        leave_dirty=yes
        git add -A
        git -c user.name=Fixture -c user.email=fixture@example.invalid \
            commit --quiet -m "Set up the $scenario scenario"
        {
            echo '"""Holds items a customer intends to buy."""'
            echo
            echo
            echo "def add_item(cart, sku, quantity):"
            echo "    if quantity < 1:"
            echo '        raise ValueError("quantity must be at least 1")'
            echo "    cart.setdefault(sku, 0)"
            echo "    if cart[sku] + quantity > 100:"
            echo '        raise ValueError("too many")'
            echo "    cart[sku] += quantity"
            echo "    return cart"
        } > src/cart.py
        expect "the wrong implementation did not apply" \
            'grep -q "> 100" src/cart.py'
        if [ "$scenario" = rt2 ]; then
            # Unrelated work no task owns, so the reviewer has to decide what it
            # is reviewing instead of taking the whole tree.
            printf '\n# unrelated experiment\n' >> src/orders.py
            expect "the unrelated edit did not apply" \
                'test -n "$(git status --porcelain src/orders.py)"'
        fi
    fi
    if [ "$scenario" = i4 ]; then
        # An unrelated uncommitted edit. The run must leave it exactly as it is;
        # rescuing the worktree destroys work the user has not seen.
        leave_dirty=yes
        git add -A
        git -c user.name=Fixture -c user.email=fixture@example.invalid \
            commit --quiet -m "Set up the i4 scenario"
        printf '\n# pending experiment\n' >> src/orders.py
        expect "the unrelated edit did not apply" \
            'test -n "$(git status --porcelain src/orders.py)"'
    fi
    ;;

t3)
    milestone '{"schemaVersion":1,"workItems":{"specUpdates":[{"spec":"cart","summary":"Cap cart quantities at 99 per SKU."}]}}'
    brief cart \
        "A cart has no upper bound per SKU." \
        "A cart rejects an addition that would raise one SKU above 99."
    cart_cap_approved
    cart_design_approved
    contract_review_accepted
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

i3)
    # The same Direct item as d10, left pending: the run under test is what
    # implements and completes it. Unlike the base fixture, this scenario needs
    # real checkpoint policy: the Direct handshake requires a clean committed
    # implementation revision, while the installed adapter scaffold means
    # "commit nothing".
    milestone '{"schemaVersion":1,"workItems":{"directChanges":[{"id":"contributing-guide","summary":"Add a CONTRIBUTING guide."}]}}'
    {
        echo "---"
        echo "type: SpecBind Git Adapter"
        echo "---"
        echo
        echo "# Git adapter"
        echo
        echo "## When to checkpoint"
        echo
        echo "Commit reviewed Direct implementation immediately before its completion handshake."
        echo
        echo "## What to include"
        echo
        echo "Include only the Direct implementation paths produced by the run."
        echo
        echo "## Commit messages"
        echo
        echo 'Prefix the message with `direct:`.'
        echo
        echo "## Branches and pushing"
        echo
        echo "Stay on the current branch and never push."
    } > .specbind/settings/adapters/git.md
    git add .specbind/settings/adapters/git.md
    git -c user.name=Fixture -c user.email=fixture@example.invalid \
        commit --quiet -m "Configure Direct checkpoints"
    expect "the Direct item is not pending" \
        'specbind milestone status | grep -q "0/1 completed"'
    expect "the guide already exists" \
        '! test -e CONTRIBUTING.md'
    expect "the Git adapter still carries its scaffold comments" \
        '! specbind adapter read git | grep -q "specbind:instruction"'
    expect "the Direct checkpoint policy is missing" \
        'specbind adapter read git | grep -q "immediately before its completion handshake"'
    ;;

d10 | x4)
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
        brief cart \
            "A cart has no upper bound per SKU." \
            "A cart rejects an addition that would raise one SKU above 99."
        expect "the c3 brief contradicts its confirmed scope" \
            'specbind artifact read cart brief | grep -q "raise one SKU above 99"'
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
echo "The docs/skill-forward-tests.md index routes to the request and expectations."
