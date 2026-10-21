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

`lifecycle/` groups approval, completion, milestone, contract-review, release,
and task-progress services. `lifecycle.rs` is the crate-root compatibility
facade. A mutating use case must rediscover and revalidate its authoritative
current inputs immediately before persistence. Earlier inventory, status, or
agent-authored evidence is never mutation authority.

`lifecycle/adoption.rs` owns the read-only prerequisites for initial and resumed
reverse establishment. It returns typed readiness and diagnostic details; the CLI
owns text rendering and stream routing. The service grants no later mutation
authority, and retains the existing prerequisite evaluation order.

`lifecycle/release_finalize.rs` retains the public finalization API and the
normal readiness/revalidation sequence. Its private `plan`, `recovery`, and
`persistence` modules own output planning, recognized partial/completed retries,
and ordered guarded writes respectively. Both normal and resumed finalization
use the same persistence path, with the Roadmap archive written last as the
completion marker.

`lifecycle/adoption_finalize.rs` retains reverse-establishment readiness and
source-drift guards plus the public finalization and abandonment API. Its private
`plan` module builds every Spec and log output before mutation; `persistence`
applies logs, cleanup and Spec updates, Contract Review archival, Roadmap archival,
and temporary adoption-record removal in that order. These modules preserve the
existing failure boundaries and do not introduce release retry semantics.

### Current-state read models

`artifacts.rs` is the stable facade for spec-local artifact reads:

- `artifacts/discovery.rs` owns filesystem enumeration, logical identity, OKF
  profile validation, and partial inventories.
- `artifacts/resolution.rs` owns typed Spec and Task loads, gate inputs,
  fingerprints, and traceability projections.

`read_model/` groups configuration inventory, Contract graph, freshness,
status/list, Roadmap scope, task, and release-readiness projections. `read_model.rs` re-exports their
existing crate-root paths. They report current state and diagnostics; they do
not authorize transitions.

`read_model/contract_graph.rs` uses a private evaluation-scope type to distinguish
whole-project checks from Design-local checks. The scope owns consumer selection,
ownership-warning relevance, and eligibility for global cycle and unconsumed-export
diagnostics; the existing public read APIs and discovery boundaries stay separate.

`read_model/freshness.rs` evaluates gate-local freshness from resolved inputs and
preserves the public freshness API. Its private `completion_revision` module
assesses Git history and current project files, returning diagnostics through
`CompletionRevisionAssessment` for the gate evaluator and completion guards.

### Installation and catalogs

`installation/` owns installation planning, exact guarded agent removal and
project uninstall, agent-role rendering, and the project-instruction block.
Derived Agent assets are materialized as the union of the selected Agent
profiles. Removal recomputes that union for the remaining profiles so shared
generic and Codex paths are retained without persistent reference counts.
`catalog/` owns the closed product/project
registries for adapters, protocols, rules, skills, templates, and steering.
Their root facade modules preserve the existing public crate paths.

### Domain and wire models

`schema/` and `foundation/yaml.rs` own structural loading. `domain/` owns
validated structured-artifact wrappers, including the strict Contract wire and
domain models. `documents/` groups scoped managed-Markdown instructions and the
focused Requirements, Design, Roadmap, and traceability semantics behind the
`documents.rs` crate-root compatibility facade. Third-party parser types stop
at these boundaries and are not public CLI or persisted contracts.

`documents/roadmap/source.rs` owns source slicing for both strict Roadmap parsing
and scope-body preservation. Scope reads and lifecycle updates use the document
layer directly; the legacy delimiter behavior of each projection is preserved.

### Foundation

`foundation/` groups project configuration resolution, canonical
fingerprints, and the restricted YAML boundary. These are cross-cutting
mechanisms without lifecycle authority. `foundation.rs` preserves the existing
`crate::config`, `crate::fingerprint`, and `crate::yaml` paths.

### Adapters

`infrastructure/repository.rs` owns Git process interaction.
`infrastructure/guarded_fs.rs` owns guarded regular-file reads and atomic
replacement. `infrastructure.rs` keeps both adapters crate-private.
Lifecycle services may depend on these adapters; adapters do not know
lifecycle policy.

## Facade rule

Large capabilities expose a small stable facade from their top-level module.
The `catalog.rs`, `documents.rs`, `foundation.rs`, `installation.rs`,
`lifecycle.rs`, and `read_model.rs` facades re-export the established
crate-root module paths; callers do not need to know the physical directory
layout. Implementation files may be reorganized behind a facade while callers
continue to depend on SpecBind-owned request, result, and diagnostic models.
Add a new public path only when it represents a new product capability, not
merely a new source-file boundary.

## Change checklist

When changing a core Rust boundary:

1. Identify the accepted Decisions that define observable behavior.
2. Keep transport, application policy, read models, domain validation, and
   adapters separated according to the direction above.
3. Add focused tests at the boundary being changed.
4. Inspect the public re-exports and final diff for accidental contract drift.
5. Run the repository validation gates documented in `AGENTS.md`.
