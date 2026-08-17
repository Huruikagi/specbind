---
type: SpecBind Rule
---

# EARS format

This rule is the project's preferred style for writing acceptance criteria. It
is a `SpecBind Rule`: your project owns this file and may strengthen, relax,
replace, or remove it. Removing it does not change the SpecBind lifecycle; it
only removes this project's writing convention.

Requirement heading grammar, Requirement ID derivation, active-scope selection,
and approval are not defined here. They belong to the CLI contract and the
`requirements-review` protocol, which remain authoritative regardless of this
file.

## Why a pattern helps

EARS describes the logical shape of a criterion: a condition, a subject, and the
response that must follow. Writing to a small set of shapes makes it harder to
produce a criterion that reads well but cannot be decided, and easier for a
reviewer to notice a missing condition.

The value is the discipline, not the vocabulary. A criterion that is observable
and decidable satisfies the product baseline whether or not it matches a pattern
below.

## Preferred patterns

- **Ubiquitous** — The `<system>` shall `<response>`.
  For behavior that is always active.
- **Event-driven** — When `<event>`, the `<system>` shall `<response>`.
  For a response to something that happens.
- **State-driven** — While `<state>`, the `<system>` shall `<response>`.
  For behavior that holds during a condition.
- **Unwanted behavior** — If `<trigger>`, the `<system>` shall `<response>`.
  For errors, failures, and inputs the system must reject.
- **Optional feature** — Where `<feature is present>`, the `<system>` shall
  `<response>`.
  For behavior that exists only in some configurations.

Patterns combine when the behavior genuinely has two conditions, for example
"While `<state>`, when `<event>`, the `<system>` shall `<response>`". Nesting
more than two conditions usually means the criterion should be split.

## Choosing the subject

Name the component that is actually responsible rather than "the system",
whenever the Spec owns more than one. A criterion with a vague subject tends to
produce a design where nobody owns the behavior.

Keep the subject stable across criteria describing the same component so a
reader can group them without reconstructing your naming.

## Writing in the project language

Artifacts are authored in the project's configured language. The patterns above
describe logical roles, not fixed machine syntax. Translate trigger keywords,
obligation phrases, subjects, conditions, and responses together so the complete
criterion reads naturally in that language:

> `<状態>`の間に`<イベント>`が発生したとき、`<システム>`は`<応答>`する。

Preserve the condition, subject, and response roles after translation. Do not
mix fixed English phrases into otherwise localized prose merely to make the EARS
shape visible.

## Review questions

- Can a reviewer decide whether this criterion holds without reading the code?
- Is the condition that triggers the behavior actually stated, or assumed?
- Does the subject name something that will exist and own this?
- Would a failure of this criterion be visible to a user or an operator?
