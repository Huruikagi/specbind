# Establish Specs from a fixed existing implementation

Read this procedure completely only when the maintainer explicitly asks to
establish Specs from existing code and tests or to resume an active reverse
establishment. This is reverse establishment, not ordinary change Discovery
and not a product release.

## Fix the evidence and version

Run the read-only route preflight first:

```sh
specbind adoption preflight
```

It reports exactly one ready result:

- `ADOPTION_PREFLIGHT_READY` starts a new reverse establishment. Record its
  exact `source_revision` and continue with this section.
- `ADOPTION_RESUME_READY` proves a clean, internally consistent active reverse
  milestone, matching temporary record, and unchanged implementation source.
  Go directly to [Resume an active reverse milestone](#resume-an-active-reverse-milestone).

If it stops on missing Steering, report that `sb-steering` is a separate
maintainer-gated workflow and stop this reverse run. Do not select or perform
Steering repair inline.

Only after preflight is ready, require the maintainer to name the selected area
and the product version already represented by the code. If either is missing,
ask only for the missing value before mutation. `baseline_version` names an
existing version; never invent the next version or bind `target_release`.

Then run:

```sh
specbind steering list
specbind steering read <every-selector> --for consume
```

Steering must establish product purpose, technology and verification constraints,
and responsibility placement. Internally contradictory Steering stops and routes
to the same separate maintainer-gated `sb-steering` workflow; do not repair it
in this run. A difference between valid Steering and observed implementation is
not itself a reason to bypass the proposal: classify it below as a suspected
defect or an unknown.

The revision is fixed. From now until finalization, do not change code, tests,
dependencies, configuration, or Steering. A changed source makes the reverse
result stale and requires a new run. Do not rebaseline it.

## Collect independent evidence

Map the repository broadly, then inspect the selected area deeply. When the
host exposes agent delegation, you MUST dispatch at least two fresh readers in
parallel before synthesizing the boundary proposal: one for observable behavior
and tests, and one for structure, dependencies, and seams. If delegation is not
available, collect those evidence lines yourself and say so in the proposal.
All claims are about the fixed revision.

For each candidate Spec collect precise project-relative locators for:

- externally observable behavior and acceptance evidence;
- owned responsibility, public entry points, and neighboring boundaries;
- structural constraints relevant to Design; and
- unknowns, contradictions, and suspected defects.

Code is evidence, never specification authority. Classify an observation as
maintained requirement, Design constraint, historical constraint,
implementation detail, suspected defect, blocking unknown, or deferred unknown.
A deferred unknown is allowed only when every later answer leaves current Spec
meaning unchanged. Otherwise it is blocking.

After the fresh readers return, build one closed contradiction ledger before
synthesizing the proposal. Compare every direct claim they touched in valid
Steering with the corresponding selected implementation, test, or product
documentation evidence. Give every difference exactly one visible disposition:
blocking unknown, deferred unknown, pending suspected-defect record, or excluded
historical detail with its reason. Do not silently drop a naming, behavior, or
boundary mismatch merely because it does not change a Spec's meaning. The
proposal fields below are the projection of this complete ledger.

Discover the Deferred Findings Adapter rather than deriving a selector from its
type name:

```sh
specbind adapter read deferred --for consume
```

`NO_CHANGE ADAPTER_ABSENT` or `NO_CHANGE ADAPTER_SCAFFOLD` means the project has
no destination. Any `ERROR` stops adapter use; do not guess `deferred-findings`
or another selector. Otherwise follow the returned active guidance.

When the active adapter accepts an exact local destination, prepare a suspected
defect with source revision, locator, and claim, and inspect the destination only
far enough to deduplicate on those three fields. Do not call it a confirmed bug,
correct it, or write the destination before the reverse milestone exists.
External posting needs separate authority. Findings never change reverse scope.

## Present one complete proposal

This is one complete proposal and the only confirmation boundary for the
reverse run.

Do not stop before this proposal merely because implementation evidence differs
from valid Steering. Put the exact semantic question under `Blocking unknowns`
when its answer changes maintained meaning; otherwise classify it as a
suspected defect or deferred unknown.

Present exactly one proposal containing:

```text
Mode: reverse
Source revision: <full object ID>
Baseline version: <existing product version>
Selected area: <area>
Reverse Specs: <id, responsibility, maintained intent, evidence basis>
Dependencies: <Spec edges or None>
Blocking unknowns: <questions that prevent meaningful Specs or None>
Deferred unknowns: <recorded non-semantic choices or None>
Suspected defects: <pending adapter records or None>
Excluded area: <outside the reverse scope or None>
After confirmation: create the reverse milestone and continue through
Requirements, Design, Design validation, Contract Review, and adoption finalize.
No Tasks, implementation change, or product release will be created.
```

The invocation is not confirmation. Revise the proposal on feedback. Do not
create partial scope while any candidate has a blocking semantic unknown.
After the maintainer explicitly confirms this visible proposal, do not ask for
a second boundary, scope, or phase confirmation.

## Create the reverse milestone

Create one external candidate JSON document with `schemaVersion: 1`, the
confirmed `baselineVersion`, and only `workItems.reverseSpecs`. Each item has
`spec`, `summary`, and optional `dependsOn`. Then run:

```sh
specbind milestone create --scope <external-candidate-or->
```

Verify that every created `spec.yaml` contains matching `establishment.kind:
reverse`, `source_revision`, `baseline_version`, and `milestone_id`. Write each
confirmed Brief and evidence-oriented Research handoff. Read `specDir` from
`.specbind.json` and use that value literally for the temporary adoption record
at `<specDir>/adoption/reverse-discovery.yaml` until finalization. Do not insert
a `specs/` segment: for `"specDir": ".specbind"`, write
`.specbind/adoption/reverse-discovery.yaml`, never
`.specbind/specs/adoption/reverse-discovery.yaml`. The `specs/` child contains
durable Specs; keeping the temporary record outside it prevents the record from
entering Spec discovery or the Contract graph. Record the fixed revision,
selected area, proposal, observation classifications, and exact evidence
locators without copying source text. For every pending suspected defect that
is written, also record its exact project-relative `destination`; finalization
uses that checkpointed field to distinguish the adapter output from source
drift.

Only after milestone creation and provenance verification, rerun
`specbind adapter read deferred --for consume`. If the same exact
local destination is still active, append each pending suspected defect that is
not already present. If it is absent or changed, record nothing and report the
adapter mismatch; never invent or recover a destination. This post-creation
write preserves the fixed clean baseline.

Checkpoint the reverse Roadmap and Spec state, Briefs, Research, temporary
adoption record, and the verified deferred destination when written as one
Discovery unit according to the active Git Adapter. Admit no other dirty path.
Never infer push authority.

## Resume an active reverse milestone

Enter here only after `ADOPTION_RESUME_READY` and only when the maintainer
explicitly asked to resume this reverse establishment. A generic request to
drive an active milestone does not grant the remaining Requirements or Design
Gate authority; return `HUMAN_DECISION` to `sb-drive` until the maintainer names
reverse continuation.

Do not rerun repository discovery, synthesize another proposal, change the
confirmed reverse scope, or create another temporary record. The active
Roadmap, Specs, Briefs, Research, and record are the checkpointed continuation
inputs. Run:

```sh
specbind milestone status --json
```

The result must report `milestoneKind=reverse`, and every actionable entry must
name `handler.kind=skill`, `handler.target=sb-discovery`, and
`handler.mode=reverse_resume`. Any other handler, inconsistent health, source
drift, or dirty checkout stops the run without repair.

An explicit maintainer request to resume authorizes the remaining reverse
orchestration under delegated workflow `sb-discovery`; it does not authorize a
scope change, implementation change, release, external write, or destructive
recovery. Continue only the phase-relative actions the fresh status exposes.
Already approved phases remain accepted and are not repeated.

## Continue without routine pauses

The confirmation above authorizes this establishment orchestration, including
Requirements and Design gate acceptance under delegated workflow `sb-discovery`.
For every actionable reverse Spec, follow the installed `sb-plan` Requirements
and Design procedures and their full checks. Run Design validation after each
new Design draft, then the milestone Contract Review when status exposes that
global action. Do not select or author the Tasks procedure. Design approval
must report `adoption_ready`.

Keep progressing every independent Spec when one is blocked. Stop the affected
Spec only for a question whose answer changes maintained meaning; global
Contract Review and finalization wait until all such questions are resolved.
Stop the whole run on source drift, invalid evidence, a failed mechanical
guard, or an implementation/configuration/Steering change.

An ordinary change request arriving during this run may be explained as a
future proposal, but it must not update the reverse Roadmap. Finalize reverse
first, then use a new ordinary milestone. For an emergency, show the active
milestone ID and ask for explicit abandonment. Only after confirmation run
`specbind milestone reverse abandon --milestone-id <id>`; never delete lifecycle
state manually. After the urgent change, reverse starts again from the new
clean revision.

## Finalize as an adopted baseline

When every reverse Spec is `adoption_ready` and the Contract Review is fresh,
prepare one strict JSON document with exactly one entry per participating Spec:

```json
{"log_entries":[{"spec":"<spec-id>","summary":"<one-line summary>"}]}
```

`spec` is the exact participating Spec ID, `summary` is a non-empty single
line, and extra or duplicate entries are invalid. Pass its external path, or
pass `-` and write that JSON to stdin:

```sh
specbind milestone reverse finalize --log-entries <path-or->
```

Finalization writes Baseline entries to each `log.md`, retains establishment
provenance, archives Roadmap and Contract Review under `baselines/`, removes the
temporary Brief and Research artifacts and the adoption record, and closes the
milestone. It must not run a Release Adapter, bind a target release, tag,
publish, or claim that the product was released.
