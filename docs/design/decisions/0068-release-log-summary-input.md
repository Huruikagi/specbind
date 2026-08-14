# 0068: Pass strict JSON summaries into release finalization

Status: Accepted

## Context

Decision 0066 assigns semantic release-summary authoring to the agent and structural `log.md` insertion to the CLI. Passing Markdown through repeated shell arguments would be fragile across shells, quoting rules, Unicode content, and inline punctuation. The summaries are structured mutation input, so an explicit UTF-8 JSON document is appropriate even though Decision 0067 keeps normal CLI output text-first.

## Decision

### Command input

- The accepted finalization command shape is:

  ```text
  specbind release finalize --log-entries <path|-> [--json] [--force]
  ```

- `--log-entries` is required. A filesystem path loads one UTF-8 JSON document; `-` reads that document from standard input.
- The input is transient command data. It is not copied into the SpecBind root, fingerprinted, archived, or persisted as release evidence. The CLI does not delete an input file after reading it.
- The decoded JSON shape is strict and contains exactly:

  ```json
  {
    "log_entries": [
      {
        "spec": "checkout",
        "summary": "Added authenticated checkout and account-lock handling."
      },
      {
        "spec": "cart",
        "summary": "認証後もカート内容を維持するようにした。"
      }
    ]
  }
  ```

- The root permits only `log_entries`. Each array item permits only the string fields `spec` and `summary`; both are required.
- `spec` is the canonical spec identity used by the active roadmap. Values are unique, and array order has no semantic meaning.
- The entry set must equal the complete participating spec set exactly. Missing, duplicate, or extra specs return `LOG_ENTRY_SET_MISMATCH`. A direct-change-only milestone therefore supplies an explicit empty array.
- `summary` is trimmed before use and must then be non-empty and contain no carriage return or line feed. It may contain inline Markdown and either supported artifact-content language. V1 defines no character-count limit.
- The CLI does not translate, rewrite, summarize, or add punctuation to `summary`.

### Canonical log mutation

- The CLI wraps each summary in this exact single-line list-item form, using the exact Decision 0073 release label without normalization:

  ```markdown
  * **Release <version>** — <summary> ([roadmap](<relative-roadmap-path>), milestone `<milestone_id>`)
  ```

- `Release`, `roadmap`, and `milestone` are English CLI-authored tokens under Decision 0067. The summary remains in its source language.
- Before mutation, the CLI parses the generated Markdown and requires one top-level unordered-list item with one paragraph while retaining the generated release label, roadmap link, and milestone code span. A summary that escapes or corrupts that structure returns `LOG_INPUT_INVALID`.
- On the first successful finalization attempt, the CLI uses the host's local calendar date in `YYYY-MM-DD` form. If that date heading exists, the new entry is inserted first in its flat list; otherwise the CLI creates the heading in newest-first date order.
- If `log.md` does not exist, the CLI creates it with the canonical `# Spec Update Log` title. If it exists, its single current document title is preserved.
- The CLI searches the complete log for the canonical milestone ID before insertion:
  - an identical canonical entry is an idempotent match and retains its existing date
  - a matching milestone with different summary, release version, or roadmap reference returns non-forceable `LOG_ENTRY_CONFLICT`
  - no match inserts the new canonical entry
- All participating log mutations remain part of the atomic Decision 0066 finalization transaction. Input and log conflicts are non-forceable; Decision 0065 `--force` bypasses only target-path Git dirtiness.

### Results

- A successful new finalization uses `OK RELEASE_FINALIZED` and reports the release version and participating spec count.
- An identical retry uses `NO_CHANGE RELEASE_ALREADY_FINALIZED`.
- JSON input, entry-set, Markdown-safety, and existing-entry conflicts use the stable English Decision 0067 result contract and perform no mutation.

## Consequences

- Agents can safely submit multilingual inline Markdown without shell-specific escaping conventions.
- Direct users and CI can inspect or generate the same small input document.
- The CLI owns date ordering, canonical metadata, and retry behavior without taking ownership of semantic summary authoring.
- A JSON mutation input does not make verbose JSON the default CLI output.
