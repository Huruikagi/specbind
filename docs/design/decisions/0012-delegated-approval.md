# 0012: Separate delegated approval from non-interactive execution

Status: Accepted

## Context

The inherited cc-sdd skills use `-y` as an auto-approval flag. Its effect varies by phase: design can approve requirements, tasks can approve requirements, design, and newly generated tasks, and quick or batch workflows can set every approval to true without preserving why each gate was accepted.

This combines several different concerns:

- suppressing interactive prompts
- authorizing an agent to continue after a gate passes
- approving the exact content produced at a gate
- orchestrating several phases in one workflow
- persisting approval state

A stored boolean cannot distinguish direct user approval from previously delegated continuation. It also cannot show which artifact revision was approved. SpecBind needs intentional accelerated workflows without treating lack of interaction as approval or weakening phase gates.

## Decision

- Gate approval has two initial modes: `explicit` and `delegated`.
- `explicit` means the user approves the current artifact revision after it is presented or otherwise made available for review.
- `delegated` means the user intentionally authorizes a named accelerated or batch workflow to accept specified future gates after their normal semantic and mechanical checks pass.
- Delegation is bounded to an identified active change or active milestone and to named gate types. It is not a standing permission for later unrelated work.
- Both modes emit the same state-machine approval events and must satisfy the same gate guards.
- Delegation changes whether the workflow pauses after a passing gate. It does not skip generation, review, traceability, contract, or validation checks.
- Delegation exists only in the accelerated workflow's run context. It is not persisted as a project artifact or authorization object.
- `--non-interactive` controls prompting only. It grants no approval authority, chooses no semantic defaults, and fails when required authorization or user judgment is unavailable.
- Approval evidence records only the mode, `passed_at`, approved input revision, and conditionally `delegation_workflow`. `delegation_workflow` is required when `approval_mode` is `delegated` and omitted when it is `explicit`; it identifies the accelerated or batch workflow whose run context allowed the post-gate confirmation pause to be skipped. `passed_at` is the time that the current revision passed the gate, recorded by the same guarded mutation; the target schema does not add separate `approved_at` or `recorded_at` fields. Its active change and milestone scope come from the surrounding lifecycle state rather than duplicated authorization metadata.
- The state remains the authoritative lifecycle value. Approval evidence explains and validates a gate crossing; it is not a second set of independently writable phase booleans.

## Approval modes

| Mode | Authorization timing | Gate behavior | Evidence expectation |
| --- | --- | --- | --- |
| `explicit` | After the current gate output exists. | Present or identify the current revision, obtain approval, then emit the gate event. | Record explicit mode, `passed_at`, and input revision; omit `delegation_workflow`. |
| `delegated` | Before future gate outputs exist, through intentional accelerated or batch workflow invocation. | Run the complete gate; on success emit the gate event without another confirmation; on ambiguity or failure stop. | Record delegated mode, `passed_at`, required `delegation_workflow`, and input revision accepted at the event. |

The first contract does not add a repository policy that permanently auto-approves gates. Such a mode would need a separate decision because it broadens authorization beyond one intentional workflow run.

## Delegation contract

An accelerated workflow must establish delegation in its run context before it crosses a future gate. The run context identifies at least:

- active milestone and, for a single-spec workflow, active Change ID
- originating workflow or orchestration mode
- gates covered by the delegation
- time the delegation was recorded
- whether the run may continue across every covered passing gate or must stop at a named boundary

This run context is orchestration state, not a user-facing SpecBind artifact. It is not written to `spec.yaml`, `roadmap.md`, or a separate authorization file. If the workflow run ends or restarts, its delegation ends; continuing acceleration requires a new intentional workflow invocation.

Delegation is consumed only after each gate independently passes. It cannot:

- convert a failed or ambiguous review into approval
- approve an artifact outside its recorded scope
- survive a state rewind into a new approval attempt unless the original authorization explicitly covers reruns in the same workflow run
- authorize external publication or other actions outside the workflow's normal user authority
- be inferred merely because an agent or CLI process is running without a terminal

If the workflow encounters a material choice that was not covered by the initial delegation, it pauses for user input even in an accelerated run.

## Interaction flags and inherited `-y`

The target interface does not support `-y`.

- New interfaces use an explicit accelerated workflow or approval-mode contract for delegation.
- `--non-interactive` remains orthogonal and never implies `delegated`.
- Quick and batch orchestration keep delegation in their run context instead of asking phase skills to mutate all approval booleans.
- SpecBind skills and CLI commands do not expose a compatibility alias for the inherited flag.
- Supplying `-y` to a target SpecBind interface stops with a stable unsupported-option diagnostic and points to an intentional accelerated workflow.
- Migration guidance explains the replacement rather than silently reinterpreting an inherited invocation as delegated approval.

This is an intentional product change from cc-sdd behavior rather than porting parity.

## Approval evidence

Every accepted gate records enough structured evidence to answer:

- Which gate crossed?
- Was approval explicit or delegated?
- For delegated approval, which `delegation_workflow` crossed the gate?
- Which exact artifact inputs and active Requirement ID set were accepted?
- When did the current revision pass the gate?
- Do the current inputs still match the approved revision?

Artifact fingerprints are required to detect out-of-band edits. Their stored value uses the `sha256:<64 lowercase hex characters>` format accepted in [Decision 0016](./0016-fingerprint-value-format.md). Requirements inputs are defined by Decisions 0017 and 0018, and the task-plan projection and canonicalization by Decision 0028; design- and completion-gate details remain Draft. Approval evidence does not preserve conversation transcripts or other fields beyond this contract merely to prove authorization.

## Consequences

- Accelerated workflows retain phase quality gates while avoiding repeated confirmation prompts.
- Status and audit output can distinguish direct approval from delegated continuation.
- Non-interactive CI or agent execution fails safely when approval authority is absent.
- Phase commands no longer gain authority to retroactively approve prerequisites.
- Quick and batch workflows need a run-scoped delegation handoff rather than repeated `-y` flags.
- Delegation adds no project file or persistent authorization object for users to understand or maintain.
- Target SpecBind interfaces make inherited `-y` invocations fail visibly instead of preserving ambiguous auto-approval semantics.
- Migration from inherited boolean approvals can preserve that a gate was approved, but must not invent whether it was explicit or delegated when historical evidence is absent.

## Open schema details

- Remaining gate-evidence YAML fields.
- Fingerprint normalization rules and artifact-input sets for each gate.
- Representation of migrated gate state when the original `passed_at` is unavailable.
- Human-readable and JSON diagnostics for missing or out-of-scope run-context delegation.
