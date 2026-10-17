# 0228: Resolve Drive options from project configuration

Status: Accepted

## Context

The maintainer wants ordinary Drive requests to use project defaults for replan
and Task recovery, and new installations to enable replan with progress recovery.
Decision 0227's explicit per-invocation selection remains available as an override.

## Decision

The project-root `.specbind.json` accepts optional `drive` settings:

```json
"drive": { "replan": true, "recovery": "progress" }
```

`replan` is an optional boolean; `recovery` is optionally `bounded` or `progress`.
Omitted fields resolve independently to false and bounded. Nulls, wrong types,
unknown keys inside `drive` and unknown recovery values are invalid. The root
configuration version remains 1. The new member has a versioned wire model and
generated `drive/v1` schema, governing the member object rather than the entire
root configuration. Installation and configuration reads validate it.

Only an initial install with no existing root configuration writes both fields
as true/progress. Existing configuration, including absent or partial `drive`,
keeps its choices during refresh, Agent addition/removal, language changes and
adoption retirement. Do not silently opt existing projects into broader authority.
Installation output shows effective Drive defaults, and `configuration show`
shows each effective value and whether it was configured or a legacy fallback.

At each new Drive invocation or human-authorized resumption, validate/read the
root configuration using `configuration show`. Resolve each option separately:
explicit invocation flags or unambiguous human instructions override project
values; project values override false/bounded fallbacks. Add `--no-replan` as
the explicit false override. Conflicting explicit choices need clarification;
invalid configuration stops before scheduling rather than silently falling back.
Natural-language limits such as no planning changes or read-only inspection
still constrain the run. Merely configuring defaults never starts Drive.
Historical mode fields in a prior handoff are not current-request overrides.
No new allowance preserves counts, and no scope expansion preserves the existing
scope boundary; neither disables the newly resolved progress or in-scope replan.
Explaining actual project defaults read-only still uses `configuration show`.

An execution request to Drive grants the resolved replan authority within the
existing approved Requirements and Milestone, whether it came from `--replan`
or `drive.replan`. Report effective settings and their source at the start; no
extra delegation confirmation is required. A separate explicitly requested
readiness check does not execute, regardless of defaults.

Drive passes resolved replan authority and recovery mode with configuration or
user-selection provenance and scope to implementation and planning owners. A
direct `sb-implement` invocation ignores `drive` defaults and retains bounded
unless explicitly selected otherwise. A receiver must preserve a delegated
progress mode rather than applying its standalone default. Plan's own phase
bounds, Direct bounds, all independent finding checks and other authority
boundaries remain unchanged.

Snapshot settings for the invocation and preserve them through automatic
redispatch, compaction, capacity restart and replan. A configuration change does
not retroactively renew or change a running allowance. A new human-authorized
invocation rereads configuration but retains the recovery ledger: resolving
defaults alone cannot reset counts or unblock repeated findings. Decision 0226's
qualifying resumption after a reported limit stop can create a new allowance
using the resolved mode. Existing explicit in-run changes preserve counts.

`sb-configure` can edit these project-owned fields on request, preserving
unrelated configuration. Validate with `configuration show`; changing only
Drive defaults requires no asset refresh or lifecycle mutation. Existing
aftercare and eligible Git checkpoints apply. Default changes are visible in
the install guide, configuration guide and Drive invocation guide in both languages.

This refines Decisions 0077, 0154, 0194, 0226 and 0227. It adds no binary CLI
Drive command, new recovery file, Requirements/scope authority or release permission.

## Verification

Test fresh install defaults, old/partial configurations, invalid objects,
preservation through refresh and rewriting, generated-schema/wire conformance,
and CLI summaries. Fresh forward tests cover omitted flags, explicit disable
and bounded overrides, delegated mode transport, standalone implementation,
read-only limits and unchanged continuation history. Keep older scenario
fixtures explicit where they are intended to measure no-replan defaults.
