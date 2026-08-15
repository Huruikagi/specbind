# 0033: Persist concise mechanical completion checks

Status: Accepted

## Context

Accepted completion evidence should identify which project commands grounded the `GO` decision. A category-only `tests: passed` assertion is too vague to reproduce or interpret later, while storing stdout, stderr, timing, environment values, or full CI logs would bloat `spec.yaml` and risk retaining sensitive data.

Projects may run several commands of the same type and may have important checks outside a fixed test/build/smoke trio. The core schema needs recognizable categories without making every project-specific check a new SpecBind field.

## Decision

- Accepted completion evidence contains `mechanical_checks` as a non-empty ordered array.
- Each entry requires:
  - `kind`: one of `test`, `build`, `smoke`, `lint`, `typecheck`, or `custom`
  - `command`: the non-empty, display-safe command invocation that was run
  - `exit_code: 0`
- An entry may include `working_directory`, a portable project-root-relative POSIX path. Omission means the SpecBind project root.
- Multiple entries may use the same `kind`; array order records execution order.
- `custom` covers a project-specific mechanical check without expanding the core enum. Its command remains the explanation of what ran; v1 adds no separate label field.
- Because only accepted `GO` evidence is persisted under Decision 0030, a nonzero exit code is candidate failure output and cannot appear in persisted completion evidence.
- The schema does not prescribe that every project has all categories. The validation skill derives the required set from project automation and rules, while the CLI requires a non-empty, structurally successful submitted set.
- `command` contains no inline secret value. Credentials and sensitive values must be supplied outside the recorded command, such as through the execution environment; environment variable names may remain visible.
- Entries do not store stdout, stderr, duration, per-command timestamps, environment values, agent identity, or retry history.
- The completion-level `passed_at` supplies the accepted timestamp in the Decision 0036 format. Detailed logs remain with CI, agent-run output, or other project tooling.

Example:

```yaml
mechanical_checks:
  - kind: test
    command: npm test
    exit_code: 0
  - kind: build
    command: npm run build
    exit_code: 0
  - kind: smoke
    command: npm run smoke
    exit_code: 0
```

## Consequences

- A reader can see the concrete successful commands without opening an external transcript.
- Evidence remains compact and avoids retaining raw logs or secrets.
- Projects can add unusual validation through `custom` while common checks remain consistently classified.
- The CLI validates evidence shape and accepted success status but does not claim that command text alone proves execution; the Decision 0029 handshake and validation workflow establish that provenance.
