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
- `--non-interactive` controls prompting only. It grants no approval authority, chooses no semantic defaults, and fails when required authorization or user judgment is unavailable.
- Approval evidence records the mode, authorization scope, originating workflow, time, and approved input revision. It must not claim an authenticated human identity when the environment cannot verify one.
- The state remains the authoritative lifecycle value. Approval evidence explains and validates a gate crossing; it is not a second set of independently writable phase booleans.

## Approval modes

| Mode | Authorization timing | Gate behavior | Evidence expectation |
| --- | --- | --- | --- |
| `explicit` | After the current gate output exists. | Present or identify the current revision, obtain approval, then emit the gate event. | Record explicit mode, time, gate, scope, and input revision; include an actor reference only when reliably available. |
| `delegated` | Before future gate outputs exist, through intentional accelerated or batch workflow invocation. | Run the complete gate; on success emit the gate event without another confirmation; on ambiguity or failure stop. | Record delegated mode, time, named workflow, authorization scope, covered gates, and input revision accepted at the event. |

The first contract does not add a repository policy that permanently auto-approves gates. Such a mode would need a separate decision because it broadens authorization beyond one intentional workflow run.

## Delegation contract

An accelerated workflow must establish delegation before it crosses a future gate. The authorization identifies at least:

- active milestone and, for a single-spec workflow, active Change ID
- originating workflow or orchestration mode
- gates covered by the delegation
- time the delegation was recorded
- whether the run may continue across every covered passing gate or must stop at a named boundary

Delegation is consumed only after each gate independently passes. It cannot:

- convert a failed or ambiguous review into approval
- approve an artifact outside its recorded scope
- survive a state rewind into a new approval attempt unless the original authorization explicitly covers reruns in the same workflow run
- authorize external publication or other actions outside the workflow's normal user authority
- be inferred merely because an agent or CLI process is running without a terminal

If the workflow encounters a material choice that was not covered by the initial delegation, it pauses for user input even in an accelerated run.

## Interaction flags and inherited `-y`

The target interface does not use `-y` as a general “approve everything” switch.

- New interfaces use an explicit accelerated workflow or approval-mode contract for delegation.
- `--non-interactive` remains orthogonal and never implies `delegated`.
- Quick and batch orchestration carry one run-scoped delegation reference into the individual gate events instead of asking phase skills to mutate all approval booleans.
- An inherited `-y` entry point may remain temporarily as a deprecated compatibility alias for command-scoped delegated approval, but only when invoked from that command's immediately preceding valid state.
- The compatibility alias never skips multiple missing prerequisite gates. A design command may start in `requirements` and approve that gate before generating design; a tasks command must start in `design` and cannot use `-y` to manufacture both requirements and design approval.
- A command may accept its own generated output only where that command's compatibility contract already included output auto-approval, and only after the normal output gate passes. If the current state is earlier than the adjacent prerequisite, the command stops and points to explicit approval or an intentional accelerated workflow.

This is an intentional product change from cc-sdd behavior rather than porting parity.

## Approval evidence

Every accepted gate records enough structured evidence to answer:

- Which gate crossed?
- Was approval explicit or delegated?
- What active change or milestone was authorized?
- Which workflow recorded delegated authorization?
- Which exact artifact inputs and active Requirement ID set were accepted?
- When did approval occur?
- Do the current inputs still match the approved revision?

Artifact fingerprints are required to detect out-of-band edits, but their canonicalization and storage schema remain Draft. The implementation must avoid recording unverifiable identity claims or sensitive conversational content merely to prove authorization.

## Consequences

- Accelerated workflows retain phase quality gates while avoiding repeated confirmation prompts.
- Status and audit output can distinguish direct approval from delegated continuation.
- Non-interactive CI or agent execution fails safely when approval authority is absent.
- Phase commands no longer gain authority to retroactively approve prerequisites.
- Quick and batch workflows need a run-scoped delegation handoff rather than repeated `-y` flags.
- Migration from inherited boolean approvals can preserve that a gate was approved, but must not invent whether it was explicit or delegated when historical evidence is absent.

## Open schema details

- Exact authorization-reference and gate-evidence JSON fields.
- Fingerprint algorithm, normalization rules, and artifact-input sets for each gate.
- How long a run-scoped delegation remains valid across retries or process restarts.
- Human-readable and JSON diagnostics for missing, expired, or out-of-scope delegation.
