# Installation and Agent configuration

Use this procedure for selected Agents, artifact language, root project
instructions, Drive defaults, or role capability overrides.

## Inspect

Read `.specbind.json` only after `specbind configuration show` has validated it.
Treat these as the supported configuration fields:

- `specDir`: fixed after initial installation in v1;
- `language`: `en` or `ja`;
- `agents`: `claude-code`, `codex`, or `generic`;
- optional `projectInstructions: true`;
- optional `adoption: true` while the temporary `sb-adopt` capability is installed; and
- optional `agentRoles` for Codex or Claude Code; and
- optional `drive` with optional boolean `replan` and `recovery: bounded|progress`.

`generic` has no product role definitions and cannot own `agentRoles`.

## Drive defaults

Edit only the requested fields under `.specbind.json` `drive`, preserving all
unrelated configuration. Read `specbind schema read drive/v1` for this member's
shape. Explicit null, wrong types, unknown member keys and unknown recovery
values are invalid. Validate with `specbind configuration show`; a change only
to Drive defaults needs no reinstall or generated-asset refresh.

A fresh install without root configuration saves `replan: true` and
`recovery: progress`. Existing omitted fields retain `false` and `bounded`;
refresh, Agent addition/removal and language changes do not opt them in.
Explain these effective defaults and their per-field sources.

They apply when the user next invokes Drive. Explicit `--no-replan` or
`--recovery bounded` overrides them for that invocation; `--replan` and
`--recovery progress` can opt in on a conservative project. Configured replan
plus an actual Drive execution request authorizes the same in-scope
Design/Contract/Tasks recovery as the option, never Requirements, scope or
Release changes. Progress removes only the Task cycle cap while independent
assessment proves all prior findings resolved. Read-only requests stay read-only.
Drive passes its resolved settings and provenance to implementation and Plan.
Standalone implementation retains its own default. Changing config does not
change an ongoing run, renew recovery counters or authorize lifecycle work.
Continue through ordinary configuration aftercare and its local checkpoint.

## Add or refresh

Agent selection through install is additive. Preview the exact current plan,
then apply the same inputs:

```sh
specbind install --dry-run [--agent <agent>] [--language <language>] [--project-instructions] [--with-adoption|--without-adoption]
specbind install [--agent <agent>] [--language <language>] [--project-instructions] [--with-adoption|--without-adoption]
```

Do not omit a persisted true `projectInstructions` choice when constructing an
explicit invocation. Omit the adoption flags to preserve the persisted choice;
use `--with-adoption` only for explicit initial reverse establishment and
`--without-adoption` only for its guarded retirement or retry. Never edit
installed Skill files, generated role files, or managed root-instruction blocks
directly.

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
