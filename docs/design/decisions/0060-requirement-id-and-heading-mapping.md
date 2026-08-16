# 0060: Derive Requirement IDs from mapped Markdown headings and list position

Status: Accepted

## Context

SpecBind needs deterministic Requirement IDs for active-scope storage, design traceability, task coverage, and CLI diagnostics. The inherited cc-sdd convention derives IDs such as `3.2` from an explicitly numbered Requirement heading and an Acceptance Criteria list position. That convention has been usable in practice and avoids adding hidden stable IDs to every criterion.

Fixed English headings would make parsing simple but would unnecessarily constrain Japanese and project-customized requirements templates. Arbitrary regular-expression configuration would make capture semantics, ambiguity, diagnostics, and cross-implementation compatibility unsafe. The document should instead declare a small mapping from semantic heading role to literal label while retaining one fixed Markdown grammar.

## Decision

- The singleton `SpecBind Requirements` artifact requires this SpecBind-owned OKF Front Matter mapping:

  ```yaml
  heading_labels:
    requirement: Requirement
    acceptance_criteria: Acceptance Criteria
  ```

- `heading_labels` contains exactly the two required string fields `requirement` and `acceptance_criteria`. Each value is a non-empty, single-line string with no leading or trailing whitespace.
- Labels are literal Unicode text. They are not regular expressions, locale keys, aliases, or renderer variables.
- The artifact is self-describing. Parsing does not select labels implicitly from `.specbind.json.language`, the current process locale, or a template filename.
- A Requirement begins at a level-three heading with this fixed grammar:

  ```text
  ### <requirement label> <N>: <title>
  ```

- `<requirement label>` exactly matches `heading_labels.requirement`.
- `<N>` is an ASCII base-10 positive integer with no leading zero. It is the explicit Requirement group identity.
- The separator after `<N>` is the literal ASCII colon followed by one space.
- `<title>` is non-empty after trimming. Prose and inline Markdown after the fixed prefix may use either supported product language.
- Requirement group numbers are unique within the artifact. Their document order is presentation only, gaps are allowed, and deleting or reordering a Requirement does not renumber another group.
- Each Requirement contains exactly one level-four heading whose extracted plain text exactly matches `heading_labels.acceptance_criteria`:

  ```text
  #### <acceptance_criteria label>
  ```

- That heading is followed by exactly one non-empty top-level ordered list before the next heading of level four or higher. Its items, in document order, are the Requirement's Acceptance Criteria.
- Nested list items, ordered lists elsewhere in the Requirement, examples inside code fences, and lists under another heading do not create Requirement IDs.
- The ordered-list item at one-based position `<M>` has canonical Requirement ID `<N>.<M>`. Source marker spelling is presentation only; the list must begin at one, while ID derivation uses AST item order rather than trusting repeated or manually edited marker text.
- Document-title and surrounding section headings are not part of ID extraction.
- An Objective or equivalent rationale block is optional free-form Markdown for human and agent context. SpecBind does not require a particular label, heading, user-story shape, or EARS notation, and the CLI does not parse or validate it. An authoring agent should include such context when the Acceptance Criteria alone would not communicate the Requirement's intent clearly.
- Criterion position is intentionally identity-bearing. Inserting, deleting, or reordering an Acceptance Criterion may change subsequent IDs in that Requirement group. The responsible requirements workflow must update active Requirement IDs and downstream design/task references; existing gate freshness and rewind rules then apply.
- No separate per-criterion stable ID, UUID, anchor, or hidden HTML metadata is introduced in v1.

## Examples

English:

```markdown
---
type: SpecBind Requirements
heading_labels:
  requirement: Requirement
  acceptance_criteria: Acceptance Criteria
---

### Requirement 3: Account Locking

**Objective:** As an account owner, I want repeated failed access to be limited.

#### Acceptance Criteria

1. When five consecutive sign-in attempts fail, the system shall lock the account.
2. When thirty minutes have elapsed, the system shall unlock the account.
```

The canonical IDs are `3.1` and `3.2`.

Japanese:

```markdown
---
type: SpecBind Requirements
heading_labels:
  requirement: 要件
  acceptance_criteria: 受入条件
---

### 要件 3: アカウントロック

#### 受入条件

1. ログインに5回連続で失敗したとき、システムはアカウントをロックする。
2. 30分経過したとき、システムはアカウントのロックを解除する。
```

The canonical IDs are again `3.1` and `3.2`.

## Template and CLI behavior

- A requirements template contains literal `heading_labels` values and matching scaffold headings under Decision 0059. AI instructions may explain content generation but never redefine the parser grammar.
- The CLI parses the Markdown syntax tree rather than using a document-wide regular expression. It reports source locations for duplicate group numbers, malformed headings, missing or duplicate Acceptance Criteria headings, empty criteria lists, and invalid mapping values where available.
- Mapping changes are ordinary requirements-content changes. Because the complete OKF file is fingerprinted, they invalidate requirements approval even when the derived ID set happens to remain identical.
- CLI-authored summary framing and diagnostics are English-only under Decision 0067. A summary may echo artifact-defined labels and titles in their source language without translating them; machine-facing IDs and diagnostic codes remain language-neutral.

## Consequences

- Requirements remain readable and locally customizable without sacrificing deterministic ID extraction.
- English and Japanese specs can coexist in one project without parser behavior depending on ambient locale.
- Requirement group identities survive document reordering and deletion gaps, while criterion editing retains the familiar positional behavior.
- Requirement rationale can remain natural in either supported product language without expanding the deterministic parser contract.
- Template authors can rename the two structural labels but cannot redefine heading levels, numeric capture, punctuation, or list semantics.

## Implementation status

The Rust Requirements parser now walks the Markdown syntax tree with source ranges, recognizes the literal Front Matter labels, validates the fixed level-three Requirement and level-four Acceptance Criteria structure, and derives canonical `N.M` IDs from direct ordered-list item positions. It ignores nested and out-of-section lists, requires the criteria list to begin at one, returns groups in numeric identity order, and reports stable diagnostics with source lines. Artifact discovery converts body-relative locations to complete document line numbers.
