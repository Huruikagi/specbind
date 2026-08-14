# 0036: Use timezone-qualified RFC 3339 gate timestamps

Status: Accepted

## Context

Every accepted gate record uses `passed_at` to identify when its current input revision passed. A timezone-free local timestamp is ambiguous across machines, agents, and release history, while a free-form string cannot be validated consistently by the CLI.

## Decision

- Every persisted `passed_at` value is an RFC 3339 `date-time` with an explicit timezone designator.
- Both UTC `Z` and numeric offsets such as `+09:00` are valid. Date-only values, timezone-free local times, timezone names, and other free-form strings are invalid.
- Fractional seconds are permitted by RFC 3339 but are not required.
- CLI-created values should use UTC `Z` for consistent output. Validation and chronological comparison parse the timestamp as an instant rather than comparing its source text lexicographically.
- The same definition is reused by requirements, design, tasks, and completion gate evidence.

## Consequences

- Gate evidence remains unambiguous when produced on different machines or displayed in release history.
- The runtime schema can reject missing timezone information before lifecycle validation.
- Offset-bearing user or migrated data remains valid even though CLI-generated output uses UTC.
