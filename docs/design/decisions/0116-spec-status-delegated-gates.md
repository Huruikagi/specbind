# 0116: Report delegated gates in Spec status

Status: Accepted

## Context

[Decision 0012](./0012-delegated-approval.md) lets an intentionally started run
cross a gate without the user confirming that specific artifact, and
[Decision 0040](./0040-state-gate-evidence-invariants.md) records how: each
requirements, design, and tasks gate persists `approval_mode`, and the delegated
variant additionally requires `delegation_workflow`. The generated schema splits
these into distinct evidence types, so an explicit approval cannot carry a
workflow and a delegated one cannot omit it.

That record is the audit trail for a skipped confirmation.
[Decision 0100](./0100-requirements-skill-contract.md) says so directly: under
delegation "the selection is still stated in the report **so the delegation
remains auditable**."

The report is run-scoped. Once the run ends, the only durable trace is the
evidence in `spec.yaml`, and no command reads it back. `approval_mode` is
rendered in exactly one place in the CLI — `render_gate_approval`, the result of
the approving command itself. Neither `spec status` nor `milestone status`
mentions it.

So the audit trail has no reader, and answering "was this gate crossed without
me?" means opening `spec.yaml` by hand — the machine state every skill is
forbidden to parse. This is the same shape as the gap
[Decision 0107](./0107-spec-status-contract-review-barrier.md) closed: a fact
the CLI holds, needed by a reader, surfaced nowhere.

It has a deadline. `specbind-quick-plan` and `specbind-batch-plan` are, under
[Decision 0075](./0075-v1-skill-and-orchestration-scope.md), the workflows that
exist to use delegated gates. Their forward tests will need to observe that a
gate was crossed by delegation and under which workflow name, and today that is
unobservable through any supported surface.

## Decision

`spec status` reports a `Delegated gates:` field listing every gate whose
accepted evidence records delegated authority, with the workflow that carried
it:

```text
  Delegated gates: requirements (quick), tasks (quick)
```

### Absence and emptiness mean different things

The field is omitted entirely when the Spec has no gate evidence, and reads
`none` when it has evidence and every approval was explicit.

Collapsing those would lose the distinction that matters: "nothing has been
approved yet" and "everything was approved by a person" are different facts, and
a reader auditing delegation needs to tell them apart. An absent field says the
question does not arise yet; `none` is a positive statement that no confirmation
was skipped.

### It is a separate field, not part of `Gates:`

The `Gates:` line answers freshness. Authority is a different question about the
same objects, and annotating each entry with both would make one line carry two
unrelated axes and grow with every future gate property.

### It carries no health or diagnostic weight

Delegation is legitimate. A delegated gate is not a fault, does not make the
Spec inconsistent, and produces no diagnostic. The field is a fact about how the
gate was crossed, reported so it can be reviewed — not a finding.

### The completion gate is excluded

Completion has no approval mode. Decision 0037 fixes its evidence as exactly
`passed_at`, `implementation_revision`, and `mechanical_checks`, because it
records a validation rather than an approval. There is nothing to report.

## Consequences

- The audit trail Decision 0100 relies on has a reader that outlives the run
  that wrote it.
- `specbind-quick-plan` and `specbind-batch-plan` become testable in the dimension that
  distinguishes them from the deliberate flow, before either is authored.
- A reviewer can answer "which of these did I actually approve?" from the same
  command they already use, without opening machine state.
- One more line appears in `spec status` for every Spec with gate evidence. The
  `none` case is the cost of keeping absence and emptiness distinguishable.

## Implementation status

Implemented. `spec_status::resolve` collects delegated gates from the wire
evidence into a field no health computation reads, and `cli.rs` renders it after
`Contract review:`.

The regression test drives one Spec through a delegated requirements approval
and an explicit design approval, asserting the field is absent before any
approval, names the workflow after the delegated one, and does not grow when an
explicit approval follows.
