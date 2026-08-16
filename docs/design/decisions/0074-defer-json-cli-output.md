# 0074: Defer JSON CLI output until after v1

Status: Accepted

## Context

SpecBind v1 needs deterministic parsing, guarded mutations, stable result codes, and concise agent-readable diagnostics. A second JSON rendering for every command would require a common envelope, command-specific response schemas, compatibility policy, and duplicate test coverage before those core workflows are implemented. Agents can consume the accepted concise English text, and CI can branch on exit status and stable codes in v1.

Structured JSON remains appropriate for owned artifact schemas and for the Decision 0068 release-log mutation input. Those are data contracts, not alternate CLI result renderings.

## Decision

- SpecBind v1 exposes no general `--json` output option and publishes no CLI result-envelope or command-response JSON Schema.
- Non-raw commands emit only the concise English Decision 0067 text result with stable `OK`, `NO_CHANGE`, or `ERROR` codes and appropriate process exit status.
- Skills consume that text directly and translate or explain it to the user when useful. CI may branch on exit status and stable result codes without a structured output guarantee.
- Commands may still accept JSON as mutation input when a separate decision defines it. In particular, `specbind release finalize --log-entries <path|->` consumes the strict Decision 0068 document, `specbind spec completion accept <spec> --evidence <path|->` consumes the strict Decision 0086 completion candidate, and `specbind milestone review accept --candidate <path|->` consumes the strict Decision 0078 candidate through Decision 0087.
- Runtime JSON Schemas under `tools/specbind/schemas/` continue to validate structured YAML/JSON-compatible SpecBind artifacts under Decision 0015. They are not CLI response schemas.
- Raw single-artifact and single-template reads remain wrapper-free content output. V1 read commands accept exactly one selector per content invocation; provenance-preserving multi-content JSON output is deferred with the general JSON surface.
- A future JSON output design must make a new explicit decision covering its common envelope, command-specific typed payloads, schema versioning, stdout/stderr behavior, compatibility rules, and exit-category relationship. V1 commands reserve no partial response shape for that future design.

## Consequences

- V1 can ship one tested result surface while retaining stable machine-recognizable codes.
- Agent context remains small without requiring response-envelope schemas for every command.
- Multi-artifact consumers issue separate selector reads in v1 after using the compact text inventory.
- Adding JSON output later is an additive CLI capability but requires an explicit versioned contract rather than inheriting an accidental v1 shape.
