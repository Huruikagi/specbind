# 0103: Expose the structured artifact and command-input schemas

Status: Accepted

## Context

`spec.yaml` and `tasks.yaml` are the two artifacts SpecBind validates
structurally. [Decision 0014](./0014-structured-spec-metadata.md) and
[Decision 0020](./0020-positional-task-ids.md) fix their content, the versioned
Rust wire models under `tools/specbind/src/schema/` are the structural source of
truth, and the generated Draft 2020-12 schemas are committed and checked in CI.

`spec.yaml` is only ever written by guarded CLI operations, so nothing outside
the CLI needs its structure. `tasks.yaml` is different: an agent authors it, and
no command tells that agent what shape it must have. `template list spec`
returns Markdown artifacts only, the `task-planning` protocol carries planning
judgment rather than structure, and no schema file is installed into a project.

A forward test made the cost visible. An agent authoring a task plan ran
`strings` and byte-range greps over `specbind.exe`, then extracted the embedded
JSON Schema from the binary's printable regions to recover the field names. It
produced a valid plan. It should never have had to, and the next agent may guess
instead of digging.

## Decision

### Two read-only commands

```text
specbind schema list
specbind schema read <selector>
```

`list` reports every embedded schema with the artifact it governs and its wire
version. `read` writes one schema to stdout as raw JSON with no result wrapper,
in the same family as `artifact read`, `template read`, `protocol read`,
`steering read`, and `adapter read`.

### Selectors carry the version

The accepted selectors are `spec/v1`, `scope/v1`, and `tasks/v1`.

The version is part of the selector because the wire model is versioned. An
unversioned `tasks` would have to mean "whatever this binary considers current",
so adding a v2 would silently change what an existing skill reads, and a skill
that writes `schema_version: 1` would have no way to ask for the schema it is
actually targeting. The conformance tests already name these artifacts this way.

### Transient command input counts

`scope/v1` is the milestone scope candidate. Decision 0089 calls it transient
command data that SpecBind reads once and discards, so it has no persisted
artifact — but an agent must author it exactly as strictly, and the shape it must
hit is no more discoverable for being short-lived.

Two forward-test runs found this independently, each probing `milestone create
--scope -` about ten times with guessed JSON before landing on the accepted
document. One of them had already run `schema list` and found the answer absent.
`milestone scope` publishes the shape, but only once a milestone exists, so the
very first command in the workflow was the one with no way to ask.

The same reasoning covers the contract review candidate and the completion
evidence document, which are still unexposed. They are deferred rather than
excluded: neither has been reached by a forward test yet, and adding them is the
same mechanical step taken here.

### Project-independent, like protocols

Neither command resolves a project, reads `.specbind.json`, or requires a
milestone. The schemas are properties of the binary, exactly as
[Decision 0094](./0094-embedded-product-protocols.md) protocols are, and the
absence of a project argument is the structural guarantee of that rather than a
convenience.

### It cannot drift

The commands read the same `SPEC_V1_SCHEMA_JSON`, `SCOPE_V1_SCHEMA_JSON`, and
`TASKS_V1_SCHEMA_JSON` constants the runtime validator compiles and the conformance tests check against
the generated output of the wire model. There is no second copy to fall behind:
a structural change that skipped regeneration already fails
`cargo run --example generate_schemas -- --check`.

This is why the structure does not go into the `task-planning` protocol instead.
[Decision 0092](./0092-template-skill-authoring-boundary.md) gives protocols
semantic judgment and leaves structure to the CLI, and prose restating a schema
is a second copy that nothing validates. It is also why this is not a template:
[Decision 0059](./0059-okf-artifact-templates.md) templates are Markdown OKF
artifact prototypes, and `tasks.yaml` is neither Markdown nor OKF.

### What this does not add

- No schema for Markdown artifacts. Their structure is fixed by their profiles
  and taught by their templates; a JSON Schema of a Requirements document is not
  the shape an author works from.
- No validation command. `check` and the guarded operations already validate,
  and a schema read is for authoring, not for a second opinion on a written file.
- No installed schema files. The binary answers the question, so copying schemas
  into a project would create exactly the stale duplicate this decision avoids.

## Consequences

- An agent authoring a task plan has a supported way to learn its structure, and
  the one that reverse-engineered the binary was the last that needed to.
- The answer comes from the artifact CI already guards, so the documentation of
  the format and the enforcement of the format are the same bytes.
- `specbind-tasks`, when it is written, names `tasks/v1` rather than carrying a
  transcription of the plan shape that would drift on the next wire change.
- The read surface is now uniform: every kind of thing an agent must read —
  artifacts, templates, protocols, steering, adapters, schemas — is reachable
  through one command pattern.

## Implementation status

Implemented for `spec/v1`, `scope/v1`, and `tasks/v1`. The review-candidate and
completion-evidence documents remain unexposed.

`tools/specbind/src/schema/scope.rs` holds the scope candidate as a wire model
beside the persisted ones, and `tools/specbind/src/milestone/candidate.rs` now
decodes through it rather than through a private duplicate, so the schema and the
loader cannot disagree.

`tools/specbind/src/schema/mod.rs` registers the schemas beside
the constants the runtime validator already compiles, and `specbind schema
list/read` expose them. Neither command takes a project path, so
project-independence is structural rather than a documented promise; a test runs
both from a directory that is no SpecBind project at all.

The read is asserted to be byte-identical to `TASKS_V1_SCHEMA_JSON`, which
`cargo run --example generate_schemas -- --check` already holds equal to the
generated output of the wire model. An unversioned `tasks` selector is refused,
so a caller always names the version it is targeting.
