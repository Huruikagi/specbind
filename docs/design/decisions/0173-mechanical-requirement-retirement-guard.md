# 0173: Reject established Requirement ID removal mechanically

Status: Accepted

## Context

Decisions 0100 and 0172 make Requirements a complete current behavioral
contract and require a preservation proof before approval. Requirement
retirement remains unsupported because there is no durable retired-ID registry.

Repeated forward tests showed that an authoring agent can still approve a
narrowed document and discover its preservation error only afterward. The
milestone baseline already identifies the established Requirements artifact,
and canonical Requirement IDs are deterministic, so this invariant does not
need to depend only on semantic self-review.

## Decision

For a Roadmap `spec_updates` participant, Requirements approval reads the
Requirements artifact at the Roadmap's `baseline_revision` from Git and parses
its canonical Requirement IDs with the same profile and instruction masking as
the live artifact.

Every baseline ID must remain present in the live Requirements artifact. If any
is absent, approval fails with
`SPEC_REQUIREMENTS_RETIREMENT_UNSUPPORTED`, names the missing IDs, and writes no
gate evidence. In-place changes to the obligation at an existing ID and newly
added IDs remain supported.

Failure to read or parse the established baseline artifact fails closed with
`SPEC_REQUIREMENTS_BASELINE_READ_FAILED`. A `new_specs` participant has no
baseline Requirements artifact and is not subject to this comparison.

When the retirement diagnostic rejects an authoring phase's first approval
attempt, no invalidation is needed because no gate transition occurred. The
Requirements Skill restores the named IDs and unaffected behavior, repeats its
semantic and traceability checks, and retries approval once. A repeated
diagnostic stops the phase. A baseline-read failure is not recoverable by
rewriting the live artifact and stops immediately.

The mechanical guard supplements rather than replaces the Skill's semantic
preservation ledger. Canonical IDs can prove that an obligation slot survived;
the authoring review must still prove that unaffected behavior was not narrowed
inside a retained ID, Context, Scope, or Objective.

## Consequences

- An agent error cannot record approval after deleting an established ID.
- The guard uses the milestone's immutable baseline rather than mutable working
  memory or a newly invented registry.
- New Specs and supported in-place revisions retain their existing workflow.
- Semantic narrowing under a retained ID remains an authoring-review concern.

## Verification

CLI integration coverage removes a baseline ID, asserts the stable diagnostic,
and confirms that `spec.yaml` remains in Requirements state without gate
evidence. HP1 exercises the same guard through delegated planning.
