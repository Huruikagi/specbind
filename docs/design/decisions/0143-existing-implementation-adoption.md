# 0143: Adopt existing implementations through Steering-first reverse discovery

Status: Superseded by 0175

## Context

[Issue #2](https://github.com/Huruikagi/specbind/issues/2) proposes a brownfield
adoption path for projects whose implementation and tests already exist but
whose intended behavior and responsibility boundaries are not described by
reliable Specs. This is not migration from cc-sdd or another SDD product.

The implementation cannot be treated as the intended contract. A duplicate
email check may be a product rule, a defect, an obsolete constraint, or an
incidental consequence of a data structure. Reverse engineering may establish
what is observed; only reconciliation with the user establishes what is
intended.

The existing workflow already has useful owners:

- [Decision 0117](./0117-steering-authoring-contract.md) bootstraps durable
  project-wide product, technology, and structure guidance from repository
  evidence.
- [Decision 0097](./0097-discovery-routing-and-read-models.md) confirms durable
  Spec boundaries and delegates milestone and Spec creation to guarded CLI
  operations.
- [Decision 0118](./0118-gap-analysis-skill-contract.md) keeps implementation
  evidence from flowing directly into Requirements and routes an accepted
  change in intent through the Brief.
- [Decision 0104](./0104-design-skill-contract.md) already investigates current
  reality and promotes Research conclusions into the complete current Design
  and Contract.

Adding reverse variants for Requirements and Design would duplicate those
owners. Adding a `reverse` state to `spec.yaml` would make a bootstrap method
part of the permanent lifecycle even though a newly established Spec has the
same obligations as any other Spec.

## Decision

### One explicit adoption skill

SpecBind adds one product-managed skill, `specbind-adopt-existing`. It is an
adoption workflow around the existing phase owners, not a parallel lifecycle.

The initial supported workflow is:

```text
committed Steering baseline
  -> adoption preflight
  -> shallow repository map
  -> confirmed candidate Spec boundaries
  -> ordinary Discovery creates Specs and Briefs
  -> deep per-Spec observation and reconciliation
  -> confirmed intent folded into each Brief
  -> ordinary Requirements, Design, Tasks, and validation
```

The skill is invoked explicitly with a selected area or an explicit whole-
repository scope. Existing code does not automatically trigger an expensive
repository scan. A shallow whole-repository map is still required to avoid
cutting the selected area without seeing neighboring seams; deep behavioral
investigation is limited to the selected area.

The adoption skill stops after it has prepared every accepted Spec's Brief and
Research for the normal Requirements phase. It neither authors nor approves
Requirements, Design, Contract, or Tasks.

### Steering is mandatory only for adoption

Ordinary SpecBind work continues to permit an empty Steering collection.
Existing-system adoption does not. Reverse discovery needs a confirmed
project-wide coordinate system before implementation structure can be judged as
a durable responsibility boundary.

The prerequisite is semantic rather than filename-based. The collection must
cover:

- product purpose, audience, and non-goals;
- project-wide technology and verification constraints;
- structure, dependency direction, and responsibility placement.

`product`, `tech`, and `structure` remain bootstrap defaults, not privileged
identities. The coverage may be merged, split, or renamed. The CLI proves that
the collection is non-empty and structurally valid; the skill judges whether it
covers the three subjects.

If Steering is missing or invalid, adoption reports the separately invoked
Steering bootstrap or repair route and stops. It does not enter Steering work in
the same invocation. Adoption may be rerun only after the user-reviewed
Steering baseline is committed.

When Steering and implementation evidence materially contradict each other,
adoption stops before proposing Specs. The user decides whether the code is
divergent or Steering is stale. Synchronizing Steering establishes a new source
revision; the adoption skill never changes guidance to make the code appear
conforming.

### A clean committed evidence revision

The deterministic command is:

```text
specbind adoption preflight
```

It succeeds only when:

- configuration and the configured SpecBind root resolve;
- the Steering collection is non-empty and has no structural diagnostics;
- no persistent Specs exist;
- no milestone is active;
- the complete Git worktree and index are clean; and
- `HEAD` resolves to a commit.

Success returns the full `source_revision` and Steering document count. The
revision identifies the tree about which every observation is made. The command
does not judge Steering content or inspect implementation semantics.

Preflight does not commit, stash, clean, or otherwise reconcile user work. A
dirty repository is a stop. Initial adoption only is supported; adding Specs to
an already adopted project through reverse discovery is later scope.

### A temporary tracked dossier

The skill records resumable project-level investigation at:

```text
{{SPEC_DIR}}/adoption/reverse-discovery.yaml
```

The dossier is a Git-tracked transient ledger, not canonical lifecycle state.
Its version-1 profile records:

- the immutable `source_revision`;
- the explicitly selected adoption scope;
- stable candidate Spec identities and responsibilities;
- project-relative evidence paths and candidate dependencies;
- stable Observation IDs, claims, evidence locators, and dispositions; and
- the current handoff stage.

The file contains no source excerpts or secrets. A path alone is insufficient
behavioral evidence: a deep observation also names a symbol, test, route,
schema entry, or line range against the source revision.

The dossier is checkpointed before ordinary Discovery so the clean-repository
milestone guard can run and another session can resume the investigation. Git
history retains it. It is deleted from the current tree only after every
accepted Spec has a complete Brief and Research handoff and no observation is
pending.

The first implementation keeps the dossier agent-authored and protocol-shaped.
It is not a new strict wire model or CLI-owned state machine. Behavioral use will
show whether deterministic schema validation and dedicated resume commands are
worth the additional surface.

### Observed, inferred, and intended remain distinct

An observation is `observed` when its named evidence directly supports the
claim. It is `inferred` when several facts support a conclusion that none states
alone. `intended` is not an observation kind. Intended behavior exists only
after the user gives a candidate the `requirement` disposition and the skill
folds that decision into the Brief.

Every observation receives one disposition:

| Disposition | Destination |
| --- | --- |
| `requirement` | Confirmed user intent in the Brief, then ordinary Requirements |
| `design` | Research, then ordinary Design or Contract |
| `bug` | Ordinary lifecycle work only when the user includes the correction |
| `historical_constraint` | Research context, not a product promise |
| `implementation_detail` | No authoritative promotion |
| `unknown` | An explicit decision still owed by Requirements or Design |

Silence is not acceptance and `pending` blocks completion of the adoption
handoff.

### Discovery and the phase skills remain authoritative

The adoption boundary confirmation does not replace Discovery confirmation.
The former chooses candidate responsibility boundaries from evidence; the
latter confirms the exact Roadmap scope, dependencies, invalidations, and new
Spec identities before guarded mutation. The adoption skill hands off and stops
at that boundary, then resumes after Discovery has created the confirmed Specs
and initial Briefs.

For each Spec, confirmed requirement intent is folded into its Brief without
implementation mechanisms or evidence paths. Research retains the evidence and
Design destinations. This preserves Decision 0118's request-mediated influence
path: `specbind-requirements` reads a confirmed request rather than trusting
Research as authority.

No `specbind-reverse-requirements` or `specbind-reverse-design` skill is added.
The ordinary Design skill already reads Research, investigates current reality,
authors the complete current Design and Contract, and identifies implementation
work where the approved Requirements and current code differ.

## Consequences

- Brownfield adoption gains an explicit, resumable path without changing the
  Spec state machine.
- Steering becomes mandatory for this one workflow while remaining optional for
  ordinary work.
- Implementation evidence stays traceable without automatically becoming a
  product obligation.
- One new skill contains adoption-specific complexity; ordinary phase skills do
  not acquire brownfield branches.
- The initial dossier format is deliberately practical rather than a premature
  permanent wire contract. A later Decision may add schema validation after
  forward use reveals what must be mechanical.
- Adoption can be expensive, so selected-area scope and two-depth investigation
  are the default rather than automatic whole-repository extraction.

## Implementation status

Implemented. `specbind adoption preflight` enforces the deterministic initial
guards and returns the committed evidence revision. The embedded
`specbind-adopt-existing` package owns the two-invocation dossier workflow. Its
entrypoint selects the directly linked Start or Resume reference from dossier
presence while retaining the shared boundaries. The package owns Steering and
source-drift stops, boundary confirmation, per-Spec reconciliation, and handoff
to the existing Discovery and phase owners.
