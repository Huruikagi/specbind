# Release a milestone

This page explains how to close a fully implemented and verified Milestone as
one release. SpecBind releases the whole Milestone or none of it; it does not
release only a subset of participating Specs.

## When to release

Before starting:

- every participating Spec has completed implementation and
  `sb-validate-implementation`, and the CLI has accepted its completion
  evidence; and
- you have decided to publish this Milestone.

For an initial trial, you may stop after implementation validation and begin a
new Milestone later instead of exercising release immediately.

## 1. Establish the release policy

Project-specific Prepare, Publish, Verify, and cleanup instructions live as
natural language in `.specbind/settings/adapters/release.md`.

- If the adapter is still a scaffold, `sb-release` investigates release
  workflows, version manifests, build scripts, and release documentation and
  proposes concrete instructions. After approval it saves and locally commits
  only `release.md`, then stops without binding or publishing a version.
- Changing `release.md` is an ordinary project change and makes accepted
  completion evidence stale. Revalidate completion before running release
  again.
- If no project-specific work is required, retain the Front Matter and leave
  the body empty to state that explicitly.

See [Customize SpecBind](./customization.md) for adapter ownership and editing.

## 2. Run sb-release

```text
$sb-release 1.0.0
```

The Skill orchestrates the flow from release binding through finalization,
while all lifecycle state changes go through the CLI. You confirm each external
or otherwise significant boundary.

### Bind the release

Release labels are opaque and case-sensitive: `v1.4.0` and `1.4.0` are
different values. The Skill never invents the value. Binding only
`target_release`, including an explicit rebind, preserves completion freshness.
Changing Roadmap scope, body text, or project version files at the same time
does not receive that exception.

The bound Roadmap is a normal Git change. The Skill follows the Git adapter and
checkpoints that file before preflight. You may also bind ahead of time:

```sh
specbind milestone bind-release 1.0.0
```

### Preflight, Prepare, Publish, and Verify

After preflight succeeds, the Skill follows the adapter in order:

- **Prepare** is repeatable and local. A failure stops before anything is
  published.
- **Publish** fixes the release identity or crosses an external boundary. The
  Skill states the exact action and version and asks for confirmation even
  when the original request was broad.
- **Verify** obtains fresh evidence that the intended version is actually
  published and usable. Re-reading the publish command output is insufficient.
  If no independent verification is possible, the result is unverified, not
  successful.

If publishing succeeds but verification fails, the Milestone remains active
and SpecBind artifacts remain intact. The workflow reports the state and asks
how to proceed rather than rolling back or retrying blindly.

### Finalize

After verification, the Skill summarizes what each participating Spec actually
delivered and asks the CLI to finalize the whole Milestone. The CLI owns the
structured `log.md` update; do not edit it in advance. Finalization is retry-safe
and does not duplicate history.

## 3. After finalization

- The Roadmap moves to the release archive, while durable Specs remain and
  return to idle state.
- Each Spec's `log.md` receives the Milestone record.
- The Milestone closes and the next `sb-discovery` may start another.
- When configured, the Git adapter checkpoints finalization metadata separately
  from the published product revision.

If the Milestone added a Spec, changed a Contract, or released before any
Steering existed, start a post-release configuration review through
`sb-configure`. It will determine whether Steering, Rules, Templates, Adapters,
or another supported surface needs updating. Steering is freely editable again
after finalization.

## Inspect readiness directly

Both commands are read-only:

```sh
specbind milestone status
specbind release preflight
```

## Next

- [Core concepts](./concepts.md)
- [Customize SpecBind](./customization.md)
- [Current generated skill index](../reference/current-skill-index.md)
- [Current generated artifact index](../reference/current-artifact-index.md)

---

[User guide](../index.md) | [Core concepts](./concepts.md) | [Customize SpecBind](./customization.md)
