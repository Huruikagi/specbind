# 0196: Install reverse adoption as a temporary optional Skill

Status: Accepted

Supersedes the `sb-discovery` ownership and routing portions of Decisions
[0181](./0181-reverse-spec-establishment.md),
[0188](./0188-retire-legacy-staged-adoption.md),
[0191](./0191-resumable-reverse-establishment.md), and
[0192](./0192-typed-milestone-action-handlers.md), and
[0193](./0193-progressive-discovery-procedures.md). Their fixed-evidence,
confirmation, lifecycle, resume, and non-release finalization contracts remain
authoritative.

## Context

Reverse establishment is an initial-adoption workflow. A project uses it to
establish durable Specs from one existing implementation, while ordinary
Discovery remains the recurring entry point for later change requests.

Decision 0193 moved the large reverse procedure behind a progressive reference,
but `sb-discovery` still carried reverse selection in its entrypoint and
description. Every project therefore advertised and routed a one-time workflow
for its entire lifetime. Once a baseline is established, those tokens and that
responsibility no longer serve a reachable use case.

The installed Skill files are product-managed assets. A Skill deleting its own
files directly would bypass the installer’s multi-Agent target calculation,
configuration marker, guarded replacement policy, and retry behavior.

## Decision

SpecBind adds `sb-adopt` as one optional product-managed Skill. It owns the
complete current reverse-establishment procedure previously held by
`sb-discovery/references/reverse.md`, including start, verified resume,
Requirements and Design delegation, Contract Review, and non-release baseline
finalization. This is not the retired staged `specbind-adopt-existing` workflow.

Initial installation and later refresh omit `sb-adopt` by default. An explicit
installation option enables it:

```sh
specbind install --with-adoption
```

The enabled choice is persisted as `adoption: true` in `.specbind.json`, so an
ordinary refresh neither loses an active adoption capability nor adds one that
was not selected. `specbind install --without-adoption` performs the guarded,
retry-safe manual retirement path.

`sb-discovery` owns only ordinary change Discovery and its explicit Source
Collection providers. Its entrypoint, metadata, resources, and the installed
project-instruction block contain no reverse-adoption route. Reverse Milestone
status actions identify `sb-adopt` in `reverse_resume` mode. The Plan procedures
record `sb-adopt` as the delegated workflow for reverse Requirements and Design
approvals.

Before reverse finalization mutates lifecycle state, the CLI prepares the exact
retirement plan for every configured Agent and the configuration marker. A
planning failure blocks finalization while the repository is still unchanged.
After core finalization succeeds, the CLI removes only the exact managed
`sb-adopt` files, removes the former Discovery reverse reference when present,
and writes `.specbind.json` last with adoption disabled. Extra project files in
either package directory are preserved.

If retirement application fails after core finalization, the adopted baseline
remains final. The command reports the pending cleanup and the maintainer may
checkpoint the finalization changes and retry with
`specbind install --without-adoption`. Finalization is never rerun to obtain
cleanup. Failed or interrupted reverse work retains the Skill. Explicit reverse
abandonment also retains it so a later clean-revision adoption can start.

## Consequences

- Ordinary projects install 15 durable Skills and carry no reverse-adoption
  content in Discovery.
- A project pays for the `sb-adopt` description and procedure only while the
  maintainer has explicitly enabled initial adoption.
- Successful finalization retires the one-time capability without making Skill
  files self-owned.
- Installation remains the sole owner of Agent-specific assets and persisted
  capability selection.
- Core baseline finalization and post-finalization Skill cleanup have explicit,
  safe failure semantics.

## Verification

Catalog and installation tests cover the 15 default Skills, opt-in rendering for
every selected Agent, persisted refresh, explicit retirement, old Discovery
reference removal, and preservation of extra package files. Lifecycle tests
cover preplanned automatic retirement, failed-finalization retention, and the
final-success output. Skill tests cover the new ownership, delegated workflow,
and absence of reverse language from Discovery. Fresh forward tests exercise a
new adoption run and a resumed finalization from fixtures installed with the
optional capability.
