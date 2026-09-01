# Installation and Agent configuration

Use this procedure for selected Agents, artifact language, root project
instructions, or role capability overrides.

## Inspect

Read `.specbind.json` only after `specbind configuration show` has validated it.
Treat these as the supported configuration fields:

- `specDir`: fixed after initial installation in v1;
- `language`: `en` or `ja`;
- `agents`: `claude-code`, `codex`, or `generic`;
- optional `projectInstructions: true`; and
- optional `agentRoles` for Codex or Claude Code.

`generic` has no product role definitions and cannot own `agentRoles`.

## Add or refresh

Agent selection through install is additive. Preview the exact current plan,
then apply the same inputs:

```sh
specbind install --dry-run [--agent <agent>] [--language <language>] [--project-instructions]
specbind install [--agent <agent>] [--language <language>] [--project-instructions]
```

Do not omit a persisted true `projectInstructions` choice when constructing an
explicit invocation. Never edit installed Skill files, generated role files,
or managed root-instruction blocks directly.

For `agentRoles`, edit only `.specbind.json`, preserve unrelated fields, run the
dry run, and reinstall. A configured model that the host cannot start is a
configuration or environment failure; do not silently substitute another.

## Remove

One-Agent removal is plan first and keeps shared surfaces required by remaining
Agents:

```sh
specbind remove-agent <agent>
specbind remove-agent <agent> --apply
```

Present the exact plan and obtain confirmation before `--apply`. Removing the
last Agent routes to guarded project uninstall and its explicit durable-
knowledge choice; configuration approval alone never authorizes uninstall.

## Language

Changing the configured language refreshes product-managed renderings but never
overwrites existing project-owned templates, Rules, adapters, Steering, or live
artifacts. Enumerate those retained files as aftercare. Offer translation as a
separate previewed content change; never describe a mixed-language project as
automatically migrated.

## Verify

Re-run:

```sh
specbind configuration show
specbind install --dry-run
```

The final dry run must contain no unexpected create or replace action. Report
host model availability separately when it cannot be proved mechanically.
