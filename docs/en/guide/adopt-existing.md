# Establish Specs from an existing implementation

If your project has working code but no trusted specification, use `sb-adopt`
to establish Specs from the current implementation. It investigates code and
tests, confirms the behavior you intend to maintain, and captures it in
Requirements, Design, and Contracts that define responsibilities and interfaces.

The result is a set of Specs for a named existing product version, with a record
of their establishment called a **baseline**. This process creates no Tasks,
implementation changes, or product release. To start using SpecBind for your
next change, see [Start with an existing project](./start-existing-project.md).
Migration from another SDD product is also outside this procedure.

Observed behavior does not automatically become a requirement. The proposal
separates behavior to maintain from implementation details and suspected
defects, and asks you to resolve decisions that affect the specification.

## Steps at a glance

1. Install the temporary `sb-adopt` Skill and prepare Steering and shared configuration.
2. Request an investigation with a selected area and existing product version.
3. Confirm the proposed Spec boundaries and behavior to maintain.
4. Let the Agent create and validate the Specs, then finalize the baseline.

Finish configuration reviews before starting the investigation. After that,
you normally confirm once, at step 3. Questions that affect Spec meaning or
failed checks can still stop the run.

## 1. Complete installation and configuration

Starting a new establishment requires a project with no durable Specs and no
active Milestone. A Milestone groups the Specs being established in this run.

### Install the temporary Skill

[Install SpecBind](./install.md) with `--with-adoption`. If SpecBind is already
installed, commit your changes and start from a clean worktree:

```sh
specbind install --dry-run --with-adoption
specbind install --with-adoption
```

Review the installation plan from the first command before applying it with the
second. Commit the installed files, then reopen the Agent session.

### Prepare Steering

Steering holds project-wide guidance such as product purpose, technical
constraints, and responsibility placement. If it is incomplete, ask
`sb-configure` for an initial configuration review:

```text
$sb-configure Review this project's initial configuration for establishing
Specs from the existing implementation. Start with the Steering it needs.
```

`sb-configure` inspects the current configuration and delegates Steering creation
or updates to `sb-steering` when needed. Review and commit the proposed Steering.

### Review templates and shared Rules

Once Steering is ready, check that the Requirements and Design templates and
shared Rules fit the project:

```text
$sb-configure Use the confirmed Steering and repository facts to review the
Requirements and Design templates and shared Rules for this project.
```

Make separate requests for any further reviews of Agents, operational adapters,
or other configuration areas. `sb-configure` rechecks configuration after each
change and completes the required follow-up work. Add a Design template only
when a distinct responsibility needs to be documented repeatedly; a different
technology name alone does not justify one. Configuration changes do not
automatically reconcile existing Specs or lifecycle artifacts.

See [Customize SpecBind](./customization.md) for details. Commit all
configuration changes and ensure the worktree is clean before continuing.

## 2. Specify the area and existing version

Include both of the following in your request:

- **Selected area:** the whole repository or a concrete area.
- **Existing version:** the product version represented by the current code, rather than the next release version.

For example, to cover the whole repository, use the following request. Replace
`v2.4.0` with your project's existing version.

```text
$sb-adopt Establish Specs from the existing implementation across this
repository as existing version v2.4.0. Investigate the current code and
tests as evidence, and ask me to confirm the Spec boundaries and behavior
to maintain before creating artifacts.
```

`sb-adopt` runs preflight checks, fixes the Git revision used as evidence, and
investigates the code and tests. **Do not change implementation, tests,
dependencies, configuration, or Steering until the run finishes.** Source changes
during the investigation require restarting from a new clean revision.

## 3. Review the Spec proposal

The investigation produces one complete proposal. Check that:

- The selected area and existing version are correct.
- Each Spec's responsibilities and maintained behavior match your intent.
- The supporting code and tests and the dependencies between Specs are sound.
- Any shared resources have appropriate scope, guarantees, and consuming Specs.
- Blocking questions, deferred questions, suspected defects, and excluded areas are clear.

Differences between implementation and Steering are also addressed here. Resolve
questions needed to determine maintained Spec meaning before confirming. Ask
for any corrections at this stage. **No Specs or Milestone are created until
you explicitly confirm the proposal.**

## 4. Continue through creation, validation, and completion

Once you confirm, `sb-adopt` continues within the same request:

```text
Create Roadmap, Specs, and temporary Briefs and Research
  → Create and approve Requirements
  → Create Design and Contracts, independently validate Design, and approve
  → Review Contracts across the Milestone
  → Finalize the baseline
```

It does not stop for routine phase confirmations. Designs proceed in dependency
order when Specs depend on each other. Once all Designs and Contracts exist,
the milestone-wide Contract Review checks their consistency.

Completion leaves these artifacts:

| Artifact | Retained content |
| --- | --- |
| Each Spec | Requirements, Design, Contracts, and source revision and existing version provenance |
| Each Spec's `log.md` | A `Baseline <version>` entry |
| `baselines/` | Archived Roadmap and Contract Review |

Temporary Briefs, Research, and the investigation record are removed, and the
active Milestone closes. The one-time `sb-adopt` Skill is also removed from all
configured Agents and disabled in `.specbind.json`.

The established Specs can now be used as ordinary existing Specs. For your next
change, follow [Start with an existing project](./start-existing-project.md).

## If the run stops

### The session ended

Explicitly ask `sb-adopt` to resume the active establishment:

```text
$sb-adopt Resume the active establishment of Specs from the existing implementation.
```

Resuming requires consistent saved state, a clean worktree, and unchanged fixed
source. If preflight passes, the run continues from the remaining work without
recreating the proposal or repeating approved phases.

### A question or suspected defect was found

A question whose answer changes Spec meaning blocks that Spec. Independent Specs
can continue, but the milestone-wide Contract Review and finalization wait for
an answer. A question can be deferred only if every answer leaves current Spec
meaning unchanged.

Suspected defects carry a source revision and evidence locator. If an active
deferred findings adapter (`deferred.md`) defines where to save deferred findings, the run
follows its instructions and records them at the verified local destination
after Milestone creation. They are not automatically confirmed as bugs or
requirements and are not fixed in this procedure. Posting outside the project
requires separate authorization.

### The implementation needs to change

Handle ordinary changes in a new Milestone after establishment finishes. You
cannot change the selected scope or replace the evidence revision (rebaseline)
during the run. If an urgent change requires abandoning establishment first, use:

```sh
specbind milestone reverse abandon --milestone-id <id>
```

Commit the abandonment changes before using ordinary Discovery for the change.
Afterward, restart establishment from the new clean revision. Do not manually
delete lifecycle state.

### Only Skill cleanup remains pending after completion

The baseline is already final. Commit the finalization changes, then run:

```sh
specbind install --without-adoption
```

Do not rerun baseline finalization to remove the Skill.

## Inspecting state and records

These details help you interpret Agent reports and saved artifacts. Normal use
does not require setting these states manually.

### Reverse establishment state

Establishing Specs from existing implementation is internally called *reverse
establishment*. The `source_revision` returned by `specbind adoption preflight`
fixes the evidence. The Roadmap uses `reverseSpecs`, with no `newSpecs`,
`specUpdates`, or `target_release`. Each Spec retains this provenance:

```yaml
establishment:
  kind: reverse
  source_revision: <fixed Git revision>
  baseline_version: <existing product version>
  milestone_id: <reverse milestone>
```

The ordinary owning Skills handle Requirements, Design, Design validation, and
Contract Review. Design approval moves the Spec to `adoption_ready`. No
`tasks.yaml` is created, and no implementation, implementation validation,
Release Adapter execution, tagging, or publication takes place.

### Contract checks and finalization

A Design check may treat the complete Contract graph as provisional only while
a downstream Spec is waiting for an earlier Design and therefore has no
Contract yet. No other graph errors are waived. The milestone-wide Contract
Review requires the complete graph.

When every Spec is `adoption_ready` and Contract Review is valid for the latest
artifacts (fresh), `sb-adopt` runs:

```sh
specbind milestone reverse finalize --log-entries <path-or->
```

Finalization closes each Spec's active change state while retaining provenance.
The resulting history records Spec establishment, not a product release.

## Next

- [Core concepts](./concepts.md)
- [Start with an existing project](./start-existing-project.md)
- [Customize SpecBind](./customization.md)
- [Current generated skill index](../reference/current-skill-index.md)

---

[User guide](../index.md) | [Start with an existing project](./start-existing-project.md) | [Core concepts](./concepts.md)
