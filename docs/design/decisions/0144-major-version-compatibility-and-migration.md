# 0144: Bound compatibility by executable major version and provide migrations

Status: Accepted

## Context

[Decision 0124](./0124-pre-1.0-binary-release-line.md) separates the
executable's public SemVer from the v1 product contract and version-one wire
schemas. It also makes `1.0.0` an explicit compatibility milestone, but does
not define the promise made by that milestone or how a later major release may
break it.

Treating every rendered sentence, Skill instruction, or implementation detail
as a stable API would prevent normal product improvement. At the other extreme,
allowing a stable release to strand project configuration or durable lifecycle
state would make a maintained specification system unsafe to adopt.

## Decision

### Compatibility follows the executable major version

Starting with executable version `1.0.0`, a newer stable release in the same
major version accepts project configuration and durable SpecBind state created
by an earlier stable release in that major version. Users may upgrade forward
within the major without first rewriting those files by hand.

The same-major compatibility surface comprises:

- persisted SpecBind configuration and CLI-owned lifecycle state;
- versioned structured artifacts and their established interpretation;
- existing public command names, accepted arguments, exit categories, and
  stable result or diagnostic codes;
- supported project paths and selectors used by installed workflows; and
- the documented preservation boundary between product-managed assets and
  project-owned customization.

Additive commands, options, fields, result codes, and diagnostics are allowed
within a major version. Human-readable explanation text, Skill wording,
reasoning strategy, internal module layout, and undocumented implementation
details are not byte-for-byte compatibility surfaces. Product-managed assets
may improve within the major as long as their documented workflow and
persisted-state contracts remain compatible.

Compatibility is forward-upgrade compatibility. An older executable is not
required to read state first written or upgraded by a newer executable unless
a release explicitly documents that guarantee.

### Breaking changes require a new executable major version

Removing or repurposing an established compatibility surface, or requiring
existing durable state to be rewritten before the new executable can use it,
is a breaking change. Such a change requires a new executable major version
and an accepted Decision describing the new contract.

Every new major version provides a documented route from the latest stable
release of the immediately preceding major version. Users on an older release
may first upgrade within their current major before crossing the major-version
boundary. The route may be:

- a deterministic CLI migration when the transformation is mechanical;
- an agent-assisted migration when product judgment or reconciliation is
  required; or
- a manual migration guide when safe automation would be misleading.

A mutating migration follows SpecBind's existing safety model: preview before
apply where applicable, explicit invocation, Git and filesystem guards,
fail-closed ambiguity handling, and a documented recovery path. Release notes
identify the breaking surfaces and link the migration route. A new major is
not published if supported durable state would otherwise have no route forward.

### Artifact schema versions remain independent

Executable major versions, the v1 product contract, and artifact
`schema_version` values remain separate identities. A new executable major does
not automatically increment every artifact schema version, and a new artifact
schema version does not rename the executable release line.

Within one executable major, introducing a newer artifact representation does
not remove the forward-upgrade promise: the newer executable either reads the
earlier representation or provides the compatible guarded transition needed
to use it. An incompatible representation change is recorded through its own
schema Decision and migration contract rather than inferred from SemVer alone.

### The pre-1.0 line remains a stabilization line

The `0.x` releases retain Decision 0124's narrower promise: incompatible
changes are possible when documented and decided deliberately. The `1.0.0`
release notes nevertheless state how projects on the latest stable `0.x`
release move to `1.0.0`, so the compatibility milestone begins with an
exercised adoption path rather than an unexplained reset.

## Consequences

- Stable users receive a practical forward-upgrade guarantee without freezing
  prose, agent reasoning, or internal implementation.
- Breaking product changes remain possible at a visible major-version boundary.
- A major release carries migration work as part of the release, not as an
  unspecified follow-up.
- Artifact schemas evolve according to their own structural contracts while
  remaining usable through the executable's supported upgrade path.
- Downgrade support is explicit when offered and is never inferred from the
  forward-upgrade guarantee.
