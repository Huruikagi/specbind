# Contract review protocol

This protocol is the shared semantic baseline for the one accepted contract
review a Spec-backed milestone must pass between Design approval and Tasks
authoring. It applies to every supported agent and cannot be waived by a project
template or shared rule.

It owns the compatibility judgment. The CLI resolves and fingerprints every
input, enforces the lifecycle guards, and owns acceptance; the review skill owns
remediation, confirmation, and how many times it reruns. Neither replaces the
judgment described here.

## What is being judged

The question is whether the milestone's changes leave every persistent seam in
the project coherent, including seams belonging to Specs the milestone never
touched.

The review is required even when a single Spec participates. One changed
producer is enough to break consumers, and those consumers are frequently
outside the milestone. Reviewing only the Specs that were edited answers a
different, easier question than the one that matters.

## Contract-first

Start from the persistent Contracts and the complete current graph. The Contract
is the durable statement of what a Spec owns, exports, and consumes, and the
milestone baseline gives the before-state to compare against.

- Judge the change in the Contracts first. When the Contract difference and the
  current graph are sufficient to reach the conclusion, that is the complete
  review; nothing further is required.
- Compare the Roadmap's scoped behavior with the current Contract even when the
  Contract diff is empty. A new owned boundary, export, consumption, invariant,
  or file-ownership claim that exists only in the delivery scope is a Contract
  omission, not an unchanged-seam result.
- Go deeper into Requirements or Design only when the conclusion genuinely
  depends on content the Contracts do not carry. Deep reading is a declared
  input to the accepted record, so declare exactly what the judgment relied on
  and say why it was necessary.
- A file that was merely opened or consulted incidentally is not an input.
  Declaring it dilutes the record and makes later freshness noisier.
- Task plans are never inputs. The review happens before Tasks exist precisely
  so that plans are written against a settled seam.

## Compatibility

For each changed, added, or removed Contract entry, establish who depends on it
and what the change does to them.

Also establish whether the scoped behavior changed a persistent guarantee
without changing any Contract entry. Contract silence does not make that change
compatible. Read the relevant Requirements or Design as a declared deep input
when that is needed to distinguish an implementation detail from a missing
Contract boundary, and leave the review unaccepted while the omission remains.

- A removed or narrowed export breaks its consumers unless every consumer is
  also updated in this milestone. Absence of a compile-time link is not evidence
  of absence of dependency.
- A widened or added export is usually safe, but check that it does not create a
  second way to do something the project already has, or claim ownership another
  Spec already holds.
- A changed invariant is a behavioral change even when the entry's shape is
  identical. Consumers rely on the guarantee, not on the wording.
- Ownership overlap and dependency cycles are reported by the CLI as warnings
  because they are sometimes deliberate. They still require a judgment: state
  why the overlap is acceptable, or treat it as a finding.

## External and unmanaged consumers

Not every consumer is a Spec in this project.

Where a seam is depended on by something SpecBind does not manage — a published
interface, another repository, a stored data shape, an operational contract —
that impact is part of this review. There is no closed list of dispositions and
no mechanism that will detect it. Name the affected consumer, state the impact,
and bring it to the user when the change requires a decision they own.

The delivery request is evidence of a decision when it explicitly asks for the
exported behavior under review. In that case, record the requested disposition
and any possible unmanaged impact; do not stop merely to ask the user to confirm
the same behavior again. Stop only when repository or project evidence exposes
an additional consumer or compatibility choice the request did not settle.

Silence here is the most expensive failure mode in the review, because nothing
downstream will catch it.

## A seam with no consumer is a claim, not a fact

`check contracts` reports an exported entry that no managed spec consumes. The
graph cannot distinguish a seam whose only consumer is outside it from one cut
for a consumer that never arrived, which is why the reviewer decides and the
report is a warning.

- When the consumer is external or unmanaged, name it. That is the same duty the
  section above states, and it turns the warning into a recorded fact.
- When there is no consumer at all, the seam is a boundary the project is paying
  for in advance. Say so. Whether it is retired now, retired later, or kept for a
  stated reason is the user's decision, not a detail to leave unremarked. A
  delivery request that directly uses or changes that export is already a stated
  reason to keep it for the milestone; record that reason instead of asking for
  duplicate confirmation.
- An export added by the change under review deserves the question directly:
  what depends on it today?

Neither answer is a defect by itself. Leaving the warning unexamined is, because
an unconsumed seam is exactly what a boundary looks like before anyone discovers
it was unnecessary.

## Scope expansion is surfaced, not absorbed

Review frequently reveals that another persistent Spec needs work.

- A Spec that requires owned work must be added to the milestone scope and
  brought through Design before the review can be accepted. It cannot be noted
  as follow-up and left behind a passing review.
- Review does not itself change any Spec's state. Present what is affected,
  obtain confirmation where the scope changes materially, and let the explicit
  operations perform the change.
- Quietly narrowing the review to what already fits the current scope defeats
  its purpose.

## Unresolved findings block acceptance

Acceptance means the complete current semantic assessment passed. There is no
partial, conditional, or provisional acceptance, and no field in which to record
a caveat.

- A finding that is understood and accepted as safe is explained in the
  assessment. That is a resolved finding.
- A finding that requires work is resolved by doing the work, or by returning
  the affected Specs to Design.
- An unresolved finding means the review has not passed. Accepting anyway
  records a judgment that was not actually made, and every later boundary that
  rechecks freshness will trust it.

The assessment is the durable explanation of why the milestone's seams are
coherent. Write it so that a reader who did not participate can tell what was
examined and why the conclusion holds.
