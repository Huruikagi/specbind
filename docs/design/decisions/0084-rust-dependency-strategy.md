# 0084: Prefer focused Rust dependencies behind SpecBind boundaries

Status: Accepted

## Context

The Rust CLI must implement argument parsing, structured artifact validation, embedded product assets, portable paths, dependency graphs, fingerprints, timestamps, and testable filesystem behavior. Reimplementing these general-purpose capabilities would increase defect risk and maintenance cost without strengthening the SpecBind product contract.

At the same time, some library defaults are broader than the accepted v1 interface. SpecBind has exact YAML restrictions, stable text outcomes and stream routing, an opaque release-label grammar, a deliberately small boundary-path grammar, and Git behavior that must agree with the user's installed Git. External library APIs must not accidentally become public artifact or CLI contracts.

## Decision

- The Rust implementation prefers maintained, focused external crates for general-purpose capabilities when they materially reduce custom parsing, platform handling, or test infrastructure.
- Each dependency is wrapped at the relevant SpecBind module boundary. Lifecycle and semantic-validation code consumes SpecBind-owned types and results rather than exposing third-party types across the core.
- `Cargo.lock` is committed for the distributed CLI. Direct dependencies use only the features needed by the accepted interface, especially where a crate can enable networking, color, asynchronous execution, or additional format support.
- The initial implementation direction is:
  - `clap` with derive support for command and argument parsing;
  - `serde` and `serde_json` for typed models and `.specbind.json`, plus `serde_json_canonicalizer` for the RFC 8785 task-plan projection;
  - `saphyr-parser` for the YAML event layer and `serde-saphyr` where its serialization and deserialization behavior passes the shared conformance fixtures;
  - `schemars` for explicit Draft 2020-12 generation from versioned wire models and `jsonschema` for evaluating the embedded generated schemas;
  - `thiserror` for typed internal errors, with a SpecBind-owned renderer for stable outcomes, codes, stream routing, sanitization, and exit mapping;
  - `camino` for validated UTF-8 managed paths, while native repository and process paths remain ordinary platform paths where necessary;
  - `walkdir` for non-symlink-following discovery;
  - `include_dir` for embedded product-managed asset trees, with explicit embedded schema lookup where version selection benefits from a closed mapping;
  - `petgraph` for contract graphs, cycle analysis, and milestone dependency projections;
  - `pulldown-cmark` for offset-aware Markdown structure and template-instruction handling;
  - `uuid`, `time`, `sha2`, and `hex` for UUID v7, RFC 3339 values, and tagged lowercase SHA-256 fingerprints;
  - `dialoguer` only for the initial installer's allowed TTY interaction.
- Initial CLI and conformance testing use focused development dependencies such as `assert_cmd`, `predicates`, `tempfile`, `insta`, and `proptest`. Snapshot approval does not replace assertions tied to accepted decisions.
- YAML is the first dependency spike. Fixtures must prove rejection of prohibited aliases, anchors, tags, duplicate keys, unsupported document shapes, and parser-layer invalid input before the YAML stack becomes a stable implementation dependency. Schema-valid values must then deserialize into the versioned wire model as required by Decision 0085.
- Minimum supported Rust version is decided before locking versions whose current releases impose a newer compiler requirement.

## Deliberate exceptions

- Git repository discovery, cleanliness, ignored-path checks, submodule state, revision identity, and object-format queries use the installed `git` executable through stable machine-readable commands. V1 already requires Git, and the CLI must agree with that Git installation's configuration and semantics. A Rust Git implementation is not added merely to avoid subprocesses.
- V1 does not add a general template engine. Decision 0059 keeps managed Markdown templates in final artifact form and does not define `{{...}}` as a deterministic rendering language.
- V1 does not add a SemVer parser for `target_release`. Decision 0073 defines it as an opaque portable label and explicitly rejects SemVer interpretation.
- V1 generates checked-in runtime schemas only from dedicated versioned Rust wire models under Decision 0085. It does not generate public schemas from lifecycle or domain models.
- V1 does not use a general glob language for Task boundaries or Contract ownership. The accepted exact-path and terminal-`/**` grammar remains a small SpecBind-owned matcher.
- Third-party diagnostic renderers do not control public output. Rich internal errors are mapped into the exact text, sanitization, stdout/stderr, and exit behavior accepted by Decision 0081.

## Consequences

- The implementation can concentrate custom code on SpecBind lifecycle semantics instead of common infrastructure.
- Dependency replacement remains localized, and a crate's permissive syntax or presentation defaults cannot silently broaden the product contract.
- YAML selection carries an explicit early validation cost because parser-layer restrictions are more important than committing prematurely to one Serde facade.
- Git subprocess parsing becomes an intentional adapter with fixtures rather than incidental shell snippets spread through lifecycle code.
