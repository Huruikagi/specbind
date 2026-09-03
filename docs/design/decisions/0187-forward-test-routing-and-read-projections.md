# 0187: Close routing and read-projection gaps from forward testing

Status: Accepted

## Context

Two Claude Code forward-test batches against `9492a83` and `4635a0b` exposed
three product gaps. The installed project-instruction block can route an
explicit claim check to the completion-accepting validator, omits dedicated
routes for several installed Skills, and still refers to the retired
`specbind-*` Skill namespace. Design validation can prove active-scope coverage
only as a count, so the artifact under review can become the reviewer's source
for the IDs it is meant to cover. At the Contract Review barrier, milestone
status can call the lifecycle consistent while the mechanically required
project-wide Contract graph is unresolvable.

The same runs also raised two CLI expansion proposals that do not belong in the
product contract. Gate invalidation has a fixed, deterministic downstream loss
under Decisions 0088, 0104, and 0108, so a second preview operation would
duplicate that contract. Completion preflight deliberately establishes only
lifecycle and checkout readiness; Decision 0182 places project Validation
adapter evaluation after that preflight and before a verdict.

Two Skill procedures need smaller clarifications. Reverse establishment should
not ask for proposal values before its read-only preflight proves the route can
start, and a Direct item's semantic suitability remains the implementing
Skill's judgment over the Roadmap summary rather than a new CLI classification.

## Decision

### Explicit intent reaches the dedicated Skill

The installed project-instruction block uses the current `sb-*` namespace and
routes explicit Contract Review, gap analysis, release, independent Design
validation, consequence-free claim verification, and completion-accepting
implementation validation to their distinct owners.

An explicit request to check whether a claim is true without changing state
uses `sb-verify-completion`, including a claim that names a Spec. A request to
validate a named Spec for lifecycle completion uses
`sb-validate-implementation` only when the user is asking to record completion
on `GO`. When both readings remain possible, consequence-free verification
takes precedence until the user explicitly authorizes recording completion.
Phrases such as “done”, “complete”, and “ready” do not by themselves grant
mutation authority.

The block remains a routing surface rather than a workflow copy. Each route
names the intent and owning Skill; the selected Skill retains all commands,
guards, and result semantics.

### Traceability enumerates the active scope

`specbind check traceability <spec>` keeps the existing `Requirements` and
`Active requirement IDs` counts and adds:

```text
  Active requirement set: 1.1, 1.2, 3.1
```

The value is the deterministic Decision 0003 set already held by the
traceability read model. An idle Spec omits this field and continues to report
`Active requirement IDs: none`.

Design validation fixes its semantic review scope from this enumerated set
before reading Design prose. It may compare the Design mappings with that set,
but never derive the set from the artifact under review.

### Milestone health includes a required Contract graph

Once every Spec-backed milestone participant has fresh Design approval, the
complete Contract graph is no longer expected future work: it is the mechanical
precondition for the global Contract Review. At that point `milestone status`
folds project discovery failures, Contract discovery failures, and error-level
`CONTRACT_GRAPH_*` issues into its diagnostics and reports inconsistent health.

Before all participating Designs are approved, graph incompleteness remains
phase-expected and is not added to milestone health. This preserves the
dependency-ordered reverse Design exception in Decision 0186. Graph warnings
remain review judgment and do not make status inconsistent.

### Existing ownership boundaries stay fixed

- Gate invalidation gains no preview command or flag. The owning Skill states
  the fixed downstream loss and obtains explicit confirmation before invoking
  the existing mutation.
- `spec completion preflight` does not inspect the Validation adapter. A
  `READY` result means the lifecycle and checkout are ready to validate, not
  that required validation procedures exist or passed.
- A pending Direct item remains mechanically actionable. `sb-drive` dispatches
  its exact summary to `sb-implement`; if that summary requires canonical
  artifacts or an unsettled product or architecture decision, the owner
  returns `REROUTABLE` and `HUMAN_DECISION`, which Drive parks while continuing
  safe independent work. The CLI does not infer semantics from Roadmap prose.
- Reverse establishment runs `adoption preflight` before requiring the selected
  area or `baseline_version`. If Steering is missing, the run stops and reports
  that `sb-steering` is a separate maintainer-gated workflow, never inline
  repair.

## Consequences

- A consequence-free verification request cannot silently advance a Spec.
- Independent Design validation receives the exact active scope from CLI-owned
  lifecycle state.
- Drive cannot schedule Contract Review from a status snapshot that hides a
  mechanically broken graph.
- Read-only preflight results keep narrow meanings instead of becoming claims
  about later agent-judged policy.
- Semantic Direct reclassification remains with the workflow that reads the
  obligation, without teaching the Rust CLI to interpret prose.

## Verification

Focused asset tests cover every dedicated route, claim-versus-acceptance
wording, reverse preflight ordering, Validation preflight meaning, and Direct
reroute handoff. CLI tests cover active-ID enumeration and phase-relative
Contract graph diagnostics. Forward-test findings FT-0030, FT-0032,
FT-0034..FT-0039 require fresh behavioral confirmation; FT-0031 and FT-0033 are
closed as accepted boundaries rather than implementation defects.
