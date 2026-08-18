# 0106: Rename the cross-spec review to the contract review

Status: Accepted

## Context

The milestone-wide review accepted by
[Decision 0078](./0078-contract-first-review-between-design-and-tasks.md) has
been called the cross-spec review since
[Decision 0050](./0050-global-cross-spec-review.md). That name describes the
wrong thing, and the evidence is in the accepted record rather than in anyone's
opinion.

Three documents spend prose correcting an impression the name creates:

- Decision 0075: "It is a contract-first milestone review, **not a general
  Design review** or release gate."
- Decision 0078: one accepted review is required "**even when only one
  Spec-backed item participates**."
- The protocol: "The review is required **even when a single Spec
  participates**," and "Reviewing only the Specs that were edited answers a
  **different, easier question** than the one that matters."

A name that needs three corrections is doing damage in between them. The
misreading it produces is specific and predictable: *cross-spec* reads as
"across the Specs in this milestone", which makes a single-Spec milestone look
like a degenerate case where the review is ceremony. It is not. The review
compares the complete current Contract graph against the milestone baseline, and
the consumer it protects is frequently a Spec the milestone never touched. The
forward-test scenario DS5 is exactly that shape: one Spec in scope removes an
export, and the Spec that breaks is not in the milestone.

The name was also read that way by the person who wrote the decisions, which is
the strongest evidence available that it is not merely an unfamiliar term.

Decision 0078's own title already says **contract-first**, and the protocol's
opening section is titled *Contract-first*. The semantics have been stated in
contract vocabulary from the beginning; only the artifact's name kept the Spec
vocabulary.

## Decision

The review is the **contract review**. The skill accepted by Decision 0075 as
`specbind-cross-spec-review` is renamed `specbind-contract-review`.

### Why not `validate-contract`

`validate-*` is an established family under Decision 0075:
`specbind-validate-design` and `specbind-validate-implementation`. That family
means an independently invoked second opinion that produces a run-scoped verdict
and gates nothing — Decision 0104 fixes `specbind-validate-design` as a
precondition of no gate.

This review is the opposite on all three counts. It is mandatory for every
Spec-backed milestone, it persists a fingerprinted artifact, and it blocks Tasks
approval, implementation validation, and release preflight. Naming it
`validate-contract` would replace a misreading about scope with a worse one
about authority.

### What is renamed

| Before | After |
| --- | --- |
| `specbind-cross-spec-review` | `specbind-contract-review` |
| protocol selector `cross-spec-review` | `contract-review` |
| `state/cross-spec-review.md` | `state/contract-review.md` |
| `releases/<version>-cross-spec-review.md` | `releases/<version>-contract-review.md` |
| `type: SpecBind Cross-Spec Review` | `type: SpecBind Contract Review` |
| `CROSS_SPEC_REVIEW_*` diagnostics | `CONTRACT_REVIEW_*` |
| milestone stage `cross_spec_review` | `contract_review` |
| status label `Cross-spec review:` | `Contract review:` |

The public command surface is unaffected. Decision 0087 named it
`specbind milestone review status` and `specbind milestone review accept`, which
carries no Spec vocabulary and needs no change. That is the reason this rename is
cheap.

### What is not renamed

- **The adjective.** "Cross-spec seam", "cross-spec graph", "cross-spec
  dependency", and "cross-spec boundary" describe relationships between Specs
  and remain correct. Only the review's own name moves.
- **Decision filenames.** They are stable identifiers, like a commit hash.
  `0050-global-cross-spec-review.md` keeps its path while its title and body use
  the new term. A reader following a link is never sent to a file that does not
  exist.
- **Superseded decisions.** Decisions 0035 and 0053 are frozen history and are
  left exactly as they were. Rewriting the vocabulary of a decision that no
  longer governs anything would misrepresent what was decided at the time.
- **Internal Rust identifiers.** The `cross_spec_review` module, its types, and
  its struct fields keep their names. They are not a user-visible contract, and
  renaming them would enlarge the diff without changing what anyone reads. A
  later mechanical rename remains available.

### Migration

None. SpecBind is unreleased, no project has an installed `state/` artifact, and
`install` writes no review file. This decision is taken now precisely because
the rename will never be cheaper.

## Consequences

- The name states the scope: the review is about Contracts, project-wide, and a
  single participating Spec is an ordinary case rather than a degenerate one.
- The three corrective sentences remain true but stop being load-bearing. They
  now reinforce the name instead of fighting it.
- Vocabulary is consistent across the skill, the artifact a user reads, the type
  they see in Front Matter, the diagnostics they get, and the milestone stage,
  so no translation is needed between the product and its documentation.
- Two accepted decisions carry a filename that no longer matches their title.
  That is the accepted cost of keeping links stable.
- Rust module names disagree with product vocabulary until a later mechanical
  pass, which is visible only to this repository's own contributors.

## Implementation status

Implemented. The rename is applied across the CLI, embedded protocols, embedded
skills, default rules, forward-test recipes, the accepted decision record, and
the design documents. `specbind-contract-review` itself is not embedded yet; the
name is fixed here so the skill is authored under it rather than renamed after.
