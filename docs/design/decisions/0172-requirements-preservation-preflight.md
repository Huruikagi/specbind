# 0172: Make existing Requirements preservation a pre-approval proof

Status: Accepted

## Context

The Requirements Skill says an established Spec is a complete current contract,
forbids unsupported requirement retirement, and requires a preservation audit
before approval. A forward test still rewrote an existing document, replaced an
unaffected reporting obligation with a test-coverage obligation, approved the
narrowed contract, and only then noticed the loss.

Two ambiguities made that outcome possible. The preservation audit had no
required baseline representation before drafting, and a request for automated
coverage could be mistaken for new observable product behavior rather than a
delivery and verification constraint.

## Decision

For an existing Spec, the Requirements phase builds a private preservation
ledger immediately after reading the maintain projection and before authoring.
The ledger enumerates every existing Requirement group and acceptance criterion
by canonical ID and summarizes the owned behavior expressed by Context, Scope,
and Objective. It is working memory only and is never persisted as a product
artifact.

After drafting and before presenting or approving, the phase reconciles the
live file against every ledger entry. Every original ID and unaffected behavior
must remain, while a requested change may revise the obligation at an existing
ID. The phase restores an accidental omission in the same draft. If the request
truly requires an obligation to disappear, it follows the existing unsupported
retirement stop before approval. Approval is prohibited until the phase can
state that the reconciliation found zero lost obligations.

A request for automated tests, coverage, or a canonical verification command is
delivery evidence, not by itself observable product behavior. Requirements
records the behavior those tests must verify. Design and Tasks own the testing
mechanism and coverage work. A test-related Requirement is permitted only when
the test capability is itself part of the user-visible or system-visible product
contract.

Delegated approval changes no part of this proof. The phase performs the same
ledger reconciliation and traceability check before invoking the approval
command, even though it does not pause for the user to inspect the draft.

## Consequences

- An established obligation is represented before drafting can overwrite it.
- Preservation failure is corrected or stopped before gate evidence exists.
- Requests for automated coverage reach Design and Tasks without inflating the
  behavioral contract.
- Delegated and explicit approval retain identical semantic checks.

## Verification

Focused Skill tests require the private ledger, zero-loss approval precondition,
and verification-request separation. HP1 verifies that the existing cart-report
behavior survives while quantity bounds and automated coverage are delivered.
