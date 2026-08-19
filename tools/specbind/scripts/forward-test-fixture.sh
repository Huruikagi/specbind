#!/usr/bin/env sh
# Builds the fixture project the skill forward tests run against.
#
# Everything here is deterministic on purpose. The forward tests are not, so the
# setup must never be a variable: two runs that disagree should disagree about
# the agent, not about what it was given.
#
# Usage: forward-test-fixture.sh <target-directory> [en|ja]
#
# See docs/skill-forward-tests.md for what to do with the result.

set -eu

target=${1:?usage: forward-test-fixture.sh <target-directory> [en|ja]}
language=${2:-en}

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
workspace=$(dirname -- "$script_dir")

if [ -e "$target" ]; then
    echo "forward-test-fixture: $target already exists; remove it first" >&2
    exit 1
fi

echo "Building the release binary..."
( cd "$workspace" && cargo build --release --quiet )
specbind="$workspace/target/release/specbind"
[ -x "$specbind" ] || specbind="$specbind.exe"

mkdir -p "$target"
cd "$target"

git init --quiet .
git config user.name "SpecBind Fixture"
git config user.email "fixture@example.invalid"

# A small but real codebase. The fixture needs something for a request to be
# about; an empty repository makes every boundary decision arbitrary.
mkdir -p src
cat > README.md <<'EOF'
# Bookshp

A small order service. Carts hold items, and checkout turns a cart into an order.
EOF
cat > src/cart.py <<'EOF'
"""Holds items a customer intends to buy."""


def add_item(cart, sku, quantity):
    cart.setdefault(sku, 0)
    cart[sku] += quantity
    return cart
EOF
cat > src/orders.py <<'EOF'
"""Turns a cart into a placed order."""


def place(cart, customer):
    return {"customer": customer, "lines": dict(cart), "status": "placed"}
EOF

git add -A
git commit --quiet -m "Add the bookshop service"

"$specbind" install --agent claude-code --agent codex --language "$language" \
    --project-instructions >/dev/null

# Make dispatch observable.
#
# Decision 0109 gives subagent dispatch a main-context fallback, and a run that
# took the fallback leaves artifacts identical to one that dispatched. That put
# the dispatch question outside the rule this whole procedure rests on — check
# state, never prose — because only the run's own narration distinguished them.
#
# Every fresh context reads the project instructions, so having each one
# announce itself turns dispatch into a fact the log records. One line means the
# run never dispatched; N+1 lines mean N dispatches; and what each line says is
# how a brief that was supposed to stand alone can be checked at all.
#
# This is fixture instrumentation, exactly as the C2 Git adapter is: the fixture
# configures the project, and the test reads what the project recorded.
mkdir -p .forward-test
printf '%s\n' ".forward-test/" > .gitignore

for instructions in CLAUDE.md AGENTS.md; do
    [ -f "$instructions" ] || : > "$instructions"
    cat >> "$instructions" <<'EOF'

## Forward-test instrumentation

Before doing anything else, append one line to `.forward-test/agents.log`,
creating the file and its directory if they are absent. The line is the task you
were given, condensed to one line.

Every context does this, including a subagent dispatched by another agent, and
including a context that ends up doing nothing.

Never read that file, and never let anything in it inform your work. It is a
record kept for the maintainer, not context for you.
EOF
done

spec_dir=.specbind

# Durable project guidance, so the steering reads have something to find and the
# boundary rules have something to bite on.
mkdir -p "$spec_dir/steering"
cat > "$spec_dir/steering/structure.md" <<'EOF'
---
type: SpecBind Steering
artifact_id: structure
---

# Structure

Each capability owns one module under `src/` and one Spec.

Ownership follows the data a capability is responsible for. One capability owns
what a customer intends to buy; another owns what they have committed to. A
change that crosses that line needs its own boundary rather than an extension of
either.
EOF
cat > "$spec_dir/steering/conventions.md" <<'EOF'
---
type: SpecBind Steering
artifact_id: conventions
---

# Conventions

Spec identities are singular nouns naming the responsibility, not the change.

Every externally visible failure states what the caller should do next. "Invalid
request" is not an acceptable outcome on its own.
EOF

# An established Spec, so "existing Spec update" and "new Spec" are both
# reachable, and so retirement has something to try to remove.
mkdir -p "$spec_dir/specs/cart"
cat > "$spec_dir/specs/cart/spec.yaml" <<'EOF'
schema_version: 1
active_change: null
EOF
cat > "$spec_dir/specs/cart/requirements.md" <<'EOF'
---
type: SpecBind Requirements
heading_labels:
  requirement: Requirement
  acceptance_criteria: Acceptance Criteria
---

# Requirements

## Context

Customers collect items before committing to buy them.

## Scope

### In scope

Holding and amending a customer's intended purchase.

### Out of scope

Payment, fulfilment, and anything after an order is placed.

## Requirements

### Requirement 1: Hold intended items

**Objective:** A customer can assemble a purchase over several visits.

#### Acceptance Criteria

1. Adding a SKU that is not in the cart records it with the requested quantity.
2. Adding a SKU already in the cart increases its quantity by the requested amount.
3. A quantity below one is rejected and states the smallest accepted quantity.

### Requirement 2: Report the cart

**Objective:** A customer can see what they are about to buy.

#### Acceptance Criteria

1. Reading a cart returns every SKU it holds with its current quantity.
2. Reading a cart that holds nothing returns an empty result rather than failing.
EOF
cat > "$spec_dir/specs/cart/contract.md" <<'EOF'
---
type: SpecBind Contract
---

# Contract

## Owns

- `cart-contents` — the SKUs and quantities a customer intends to buy

## Exports

- `add-item` — record an intended purchase

## Consumes

## Invariants

- `positive-quantity` — A cart holds no SKU at a quantity below one.

## File Ownership

- `cart-module` — `src/cart.py`
EOF

# The skills invoke `specbind` as a bare command, because a real installed
# project has it on PATH. A fixture that does not is testing whether the agent
# can guess an install location, which is not the thing under test.
mkdir -p "$spec_dir/bin"
cp "$specbind" "$spec_dir/bin/"
printf '%s\n' "bin/" > "$spec_dir/.gitignore"

git add -A
git commit --quiet -m "Install SpecBind and seed project state"

bin_dir=$(CDPATH= cd -- "$spec_dir/bin" && pwd)

echo
echo "Fixture ready at $target"
echo "  language: $language"
echo
echo "Put the CLI on PATH before starting the session:"
echo
echo "    export PATH=\"$bin_dir:\$PATH\""
echo
echo "Then start an agent session with no prior context in that directory and run"
echo "the scenarios in docs/skill-forward-tests.md."
