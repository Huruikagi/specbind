# 0072: Require an explicit operation for release rebinding

Status: Accepted

## Context

A milestone has a stable UUID v7 identity before its eventual release version is necessarily known. Binding an unset version is ordinary milestone progression, while replacing an existing version changes the future archive filenames and every canonical per-spec release-log label. The target version does not change requirements, design, implementation, or contract compatibility, so phase-dependent gate invalidation would add noise without protecting a semantic input.

## Decision

### Command

- The accepted CLI forms are:

  ```text
  specbind milestone bind-release <version> [--json]
  specbind milestone bind-release <version> --rebind [--json]
  ```

- `<version>` is validated and persisted exactly under the Decision 0073 portable release-label grammar. No normalization or implicit leading `v` is applied.
- When `target_release` is `null`, the normal form binds the requested version and returns `OK RELEASE_BOUND`.
- When the requested version already equals the binding, either form performs no mutation and returns `NO_CHANGE RELEASE_ALREADY_BOUND`.
- When a different non-null binding exists, the normal form returns `ERROR RELEASE_REBIND_REQUIRED` and reports the current and requested versions. It performs no mutation.
- `--rebind` permits only the deliberate replacement of that existing binding and returns `OK RELEASE_REBOUND` with the old and new versions.
- Both initial binding and rebinding resolve the corresponding roadmap and cross-spec-review archive destinations and reject a conflicting version before changing the roadmap.
- The command operates only on an active milestone. Released roadmap archives and per-spec log history are immutable through this operation.
- The operation changes only the roadmap-owned `target_release`. It does not rewrite briefs, `spec.yaml`, requirements, design, contracts, tasks, gate evidence, or cross-spec review state.

### Authorization and freshness

- In an agent-assisted workflow, replacing a non-null binding requires explicit user confirmation after the agent shows both the current and requested versions. The agent then invokes the `--rebind` form.
- A human directly invoking the CLI expresses the same deliberate intent by supplying `--rebind`.
- SpecBind provides no `-y` alias, `--force` bypass, or delegated gate-approval shortcut for rebinding. Gate delegation under Decision 0012 does not authorize milestone metadata changes.
- The rule is the same before and after implementation begins. SpecBind stores no separate phase flag for external release execution, so it does not invent a phase-dependent rebind state.
- `target_release` is excluded from the Decision 0055 cross-spec scope projection and is not a spec-local gate input. Binding or rebinding therefore does not invalidate gate evidence, completion evidence, or accepted cross-spec review state.
- If project-specific release work has already begun, the release agent must inspect and reconcile actual external state before requesting confirmation. The CLI does not infer or roll back external publication under Decisions 0066 and 0071.

## Consequences

- An initially unknown release version can be assigned without rewriting milestone-owned spec artifacts.
- Accidental replacement of a visible release identity is rejected, while an intentional correction remains straightforward.
- Approval behavior does not depend on an implementation phase that the binding itself does not semantically affect.
- Existing evidence remains fresh because none of its accepted fingerprint inputs changed.
