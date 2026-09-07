# 0198: Retire Requirements with inline markers and ordinary delivery coverage

Status: Accepted

## Context

Requirements can cease or be consolidated without retiring their owning Spec.
Deleting positional criteria reassigns later identities. Explicit named IDs and
a separate retirement registry would add authoring and migration cost. Keeping
the retired position in the document preserves identity and readable context.

## Decision

The exact `_Retired_` token at the start of an Acceptance Criteria item, or at
the start of a mapped Requirement heading's title, marks retirement. The token
must end there or be followed by whitespace. Other emphasis spellings, code,
escaped text, examples, and occurrences elsewhere have no retirement semantics.
Recognition remains scoped to the existing Markdown AST grammar.

An individual marker continues to occupy its original list position. A group
marker retires and reserves the complete numbered group, including all former
child IDs. A retired group may retain its body or consist only of its heading.
An individual marker may retain its original prose or stand alone. Prefer
retaining the original prose; place ordinary prose or nested-list explanations
of continuation, consolidation, partial cessation, or complete cessation beside
the marker. No fixed disposition DSL, manifest, alias, or new state field is
introduced. Nested explanations never create Requirement IDs.

The agent reconciles every affected baseline obligation against those
explanations and any named successor. A successor link alone does not prove
preservation. Update Context and Scope to the resulting live responsibility.
Retired text is historical context, never a current behavioral promise.

Requirements approval compares the immutable Roadmap baseline with the current
document. Missing positions without a marker remain rejected with
`SPEC_REQUIREMENTS_RETIREMENT_UNSUPPORTED`. Established retired positions and
groups cannot disappear or return to live status. Newly retired identities must
have live baseline obligations and must all be included in the approved active
Requirement ID set. A compact group expands to its live baseline child IDs at
this boundary. New Specs cannot fabricate retired identities. A Spec must retain
at least one live obligation; complete Spec retirement remains outside scope.

The existing nonempty active set now means obligations to deliver, re-verify,
or retire in this change. It can contain only retirement work, without a dummy
new behavioral requirement. IDs already retired at the baseline cannot be
selected again. The Requirements fingerprint covers markers and explanations;
changing either after approval requires the ordinary rewind and approval flow.
Delegation grants no additional invalidation or product-intent authority.

Design and Tasks cover each selected retirement ID using the existing reference
fields. Coverage means implementing and verifying the described continuation or
cessation, not implementing the historical promise. Design records the resulting
boundary and retirement treatment; completed retirement references can remain
as explanatory traceability after release, without becoming live obligations or
future active scope. Compact groups reserve their numeric child namespace; a
historical reference into that namespace identifies retirement, not live coverage.
Contract consumers and File Ownership changes use ordinary Contract review.
Completion checks verify actual preservation or cessation, affected interfaces,
data and consumers as applicable. No code change is invented for prose-only
consolidation, but its preservation and reference checks still require evidence.

Release clears active state as before, leaving the markers as durable identity
reservations. Requirements approval is a plan approval, not retirement completion.
Unapproved drafts can be corrected; after release a retired identity cannot be
reactivated. Historical retirement prose needs no ongoing promise maintenance.

This extends Decisions 0003, 0060, 0100, 0172 and 0173 where they required live-only
selection or prohibited explicit retirement. It does not introduce explicit IDs,
change existing ID derivation, or require a v2 migration. Existing documents
without markers retain their interpretation. Refresh customized template
instructions that still prohibit retirement. Older binaries do not understand
the new markers; do not downgrade a project after adopting them.

## Verification

Parser tests cover exact token positions, retained and compact groups, nested
explanations, Japanese headings, and non-marker examples. Lifecycle tests cover
mandatory retirement selection, pure-retirement approval, missing placeholders,
reuse prevention, and complete-Spec rejection. Skill forward tests measure
authoring and downstream treatment without teaching the marker in the prompt.
