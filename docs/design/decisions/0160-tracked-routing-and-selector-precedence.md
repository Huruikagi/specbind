# 0160: Resolve tracked delivery and declared selectors before surface inference

Status: Accepted

## Context

Fresh Codex forward tests against `dc6c022` passed X1 and I3 mechanically but
exposed two avoidable inference paths. In I3, a request for a CONTRIBUTING guide
looked like durable project guidance, so the driver selected Steering before it
read the active milestone and discovered the matching pending Direct item. In
X1, the driver used the milestone action `implementation` as though it were a
Design artifact identity, tried `design/implementation`, and recovered only
after `artifact list` reported `design/main`.

Both runs eventually followed the intended workflow. The recovery does not make
the contracts unambiguous: one route lets a tracked delivery lose precedence to
its file's apparent subject, and the other lets a lifecycle action label leak
into an unrelated artifact namespace.

## Decision

For a change request, the installed project-instruction block reads the active
milestone before choosing among discovery, Steering, ordinary work, and
implementation. A matching pending Spec-backed or Direct item is tracked
delivery work and routes to `specbind-implement`; that match takes precedence
even when the requested output also resembles durable project-wide guidance.
Explicit review, diagnosis, validation, release, and configuration intents keep
their dedicated routes.

When contract review needs a deep Design read, it runs `artifact list` before
constructing any Design selector and uses only the exact `design/<artifact-id>`
values reported there. Lifecycle states and action labels such as `tasks` and
`implementation` are never artifact identities.

## Consequences

- A tracked Direct documentation item cannot be captured by Steering merely
  because the resulting document is durable guidance.
- The milestone remains the authority for whether a change request is already
  part of an active delivery.
- Contract review cannot derive a Design selector from lifecycle vocabulary;
  split and project-selected Design identities remain discoverable only through
  the CLI listing.
- These are routing and discovery rules only. They do not change milestone
  membership, artifact identity, or semantic review authority.

## Implementation status

Implemented by the embedded project-instruction block and
`specbind-contract-review`, with focused asset-contract tests.
