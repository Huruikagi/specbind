# 0067: Make CLI results concise, text-first, and English-only

Status: Accepted

Decision 0076 narrows the English-only rule to terminal results and machine tokens. CLI-authored display prose inserted into managed Markdown is localized to the project language.

## Context

SpecBind's CLI exists primarily to save agent context by performing deterministic discovery, parsing, summaries, validation, and guarded mutations. An exit code alone forces the agent or human to investigate again, while always returning verbose JSON spends tokens on repeated keys and structure when a short result would be sufficient.

The generated skills already mediate most user interaction and can translate concise CLI results when needed. Maintaining localized CLI messages would enlarge the diagnostic and testing surface without improving the machine contract.

## Decision

### Default text result

- Every non-raw result command emits a concise explicit outcome line in this grammar:

  ```text
  OK <STABLE_CODE>: <English message>
  NO_CHANGE <STABLE_CODE>: <English message>
  ERROR <STABLE_CODE>: <English message>
  ```

- `OK` means the requested operation or check completed. `NO_CHANGE` means the requested idempotent result already held. Both exit with code zero.
- `ERROR` means the command did not complete the requested result and exits nonzero according to the command's stable exit category.
- `<STABLE_CODE>` is uppercase ASCII snake case and language-neutral. Success and no-change outcomes have stable codes as well as errors.
- The message states the outcome or blocking reason in one short English sentence. It includes only immediately useful identifiers, counts, and resulting state.
- When details are necessary, compact indented lines follow the outcome line. A deterministic recovery action may add one `Hint:` line. The CLI does not emit speculative advice.
- Data-oriented list, status, and check commands may follow the outcome line with their compact primary output. Commands whose contract is raw byte/content output, especially single-selector `artifact read`, emit no success wrapper because that would corrupt the payload; their errors still follow the English result contract.

Examples:

```text
OK RELEASE_FINALIZED: Finalized v1.4.0 for 3 specs.
```

```text
NO_CHANGE RELEASE_ALREADY_FINALIZED: Release v1.4.0 is already finalized.
```

```text
ERROR FINALIZE_TARGET_DIRTY: 2 finalization targets contain uncommitted changes.
  specs/checkout/tasks.yaml (delete, unstaged)
  steering/roadmap.md (move_source, staged)
Hint: Commit, stash, or otherwise resolve the affected paths before retrying.
```

### Output language

- All CLI-authored terminal text is English-only in v1, including outcome messages, diagnostics, hints, help, usage errors, headings, column labels, and progress/status labels.
- The CLI has no message-locale setting and does not select output language from the spec, process locale, operating system, or document template.
- Stable codes, option names, structured artifact fields, and enum values are English machine tokens.
- User-authored content is not translated. Paths, spec names, task text, Requirement titles, artifact bodies, and release-log summaries remain in their source language when echoed or returned.
- Agent skills may translate or explain CLI results in the user's language. Translation belongs to the interaction layer and never changes the stable code or underlying result.

### V1 output boundary

- Human-readable text is the sole non-raw v1 result surface for both direct users and agent skills.
- Decision 0074 defers `--json`, the common JSON envelope, and command-response schemas until after v1.
- Raw-content operations remain exceptions only where their accepted contract requires unmodified file content on standard output.

## Consequences

- Agents can usually understand success or failure from one short line without another filesystem inspection or JSON parsing step.
- Direct users always receive an explicit result instead of silent success.
- Stable codes support reliable skill branching while English prose remains readable and easy for the agent to translate.
- A future JSON surface can be added deliberately without complicating the v1 result contract.
