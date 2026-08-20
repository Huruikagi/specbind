# Implementation architecture

This document defines the dependency direction of the canonical Rust CLI. It is
an implementation constraint, not a new product contract. Accepted Decisions
remain authoritative for observable behavior, persisted artifacts, and
lifecycle policy.

## Dependency direction

```text
args / main
    |
    v
cli facade and command families
    |
    v
application lifecycle services
    |
    +--> current-state read models
    +--> validated domain and wire models
    +--> repository and guarded filesystem adapters
```

Dependencies point down this diagram. A lower layer must not call a CLI command
or depend on its rendering. Command families may compose application services,
but must not call one another to reuse policy.

## Boundaries

### Transport

`args.rs`, `main.rs`, `cli.rs`, and `cli/` translate arguments and external
input into application calls and render the stable text-first result contract.
They do not decide whether a lifecycle transition is allowed.

### Application lifecycle services

Approval, completion, milestone, contract-review, and release modules own
guarded use cases. A mutating use case must rediscover and revalidate its
authoritative current inputs immediately before persistence. Earlier inventory,
status, or agent-authored evidence is never mutation authority.

### Current-state read models

`artifacts.rs` is the stable facade for spec-local artifact reads:

- `artifacts/discovery.rs` owns filesystem enumeration, logical identity, OKF
  profile validation, and partial inventories.
- `artifacts/resolution.rs` owns typed Spec and Task loads, gate inputs,
  fingerprints, and traceability projections.

`contract_graph.rs`, freshness/status projections, Roadmap scope reads, and
release-readiness projections are also read models. They report current state
and diagnostics; they do not authorize transitions.

### Domain and wire models

`schema/` and `yaml.rs` own structural loading. `domain/` and the focused
Requirements, Design, Contract, Roadmap, and traceability modules own semantic
validation and normalized values. Third-party parser types stop at these
boundaries and are not public CLI or persisted contracts.

### Adapters

`repository.rs` owns Git process interaction. `guarded_fs.rs` owns guarded
regular-file reads and atomic replacement. Lifecycle services may depend on
these adapters; adapters do not know lifecycle policy.

## Facade rule

Large capabilities expose a small stable facade from their top-level module.
Implementation files may be reorganized behind it, while callers continue to
depend on SpecBind-owned request, result, and diagnostic models. Add a new
public path only when it represents a new product capability, not merely a new
source-file boundary.

## Change checklist

When changing a core Rust boundary:

1. Identify the accepted Decisions that define observable behavior.
2. Keep transport, application policy, read models, domain validation, and
   adapters separated according to the direction above.
3. Add focused tests at the boundary being changed.
4. Inspect the public re-exports and final diff for accidental contract drift.
5. Run the repository validation gates documented in `AGENTS.md`.
