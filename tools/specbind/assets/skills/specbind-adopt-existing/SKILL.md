---
name: specbind-adopt-existing
description: Adopt a selected part of an existing implementation into new Specs by establishing boundaries, retaining implementation evidence, and reconciling observed behavior into confirmed Brief intent before the normal lifecycle begins.
argument-hint: "<area to adopt, or entire repository>"
---

# Adopt an existing implementation

Turn an explicitly selected part of a brownfield repository into new SpecBind
Specs. Existing code and tests are **evidence**, never automatic authority for
what the product ought to promise.

This workflow has two invocations separated by ordinary Discovery:

1. establish candidate Spec boundaries and checkpoint the adoption dossier;
2. after Discovery creates the confirmed Specs and Briefs, investigate each
   Spec deeply, reconcile intent with the user, update its Brief and Research,
   and retire the project-level dossier.

Requirements, Design, Tasks, implementation, and approval remain owned by their
normal skills. Do not author or approve those artifacts here.

## 1. Determine whether this is a new run or a resume

Resolve the configured `specDir` from `.specbind.json`. The one project-level
dossier path is:

```text
<specDir>/adoption/reverse-discovery.yaml
```

If it does not exist, begin at **Start**. If it exists, begin at **Resume**.
Never create a second dossier and never infer a dossier from an ordinary
Research artifact.

## 2. Start

The user must name the area being adopted. A broad description such as
"authentication and account management" is sufficient; an omitted scope is
not permission to adopt the whole repository. Ask whether the intended scope is
the entire repository or a named area before investigating.

Run the deterministic prerequisite check:

```sh
specbind adoption preflight
```

It must report `ADOPTION_PREFLIGHT_READY`. Any other result stops this run.
Do not perform the routed recovery during the same invocation. In particular:

- `ADOPTION_STEERING_REQUIRED` routes to a separate `specbind-steering`
  bootstrap invocation; report that next step and stop immediately;
- malformed Steering routes to Steering repair;
- existing persistent Specs or an active milestone are outside this initial
  adoption contract;
- a dirty repository is the user's work to reconcile. Never commit, stash, or
  move it merely to satisfy this guard.

Record the exact `source_revision` the command returned. Every claim about the
existing implementation is about that committed tree.

### Read the complete Steering baseline

```sh
specbind steering list
specbind steering read <selector> --for consume
```

Read every listed document. Confirm that the collection, regardless of file
names or decomposition, establishes all three areas needed for adoption:

- product purpose, audience, and non-goals;
- project-wide technology and verification constraints;
- structure, dependency direction, and responsibility placement.

If one is materially absent, stop and route to `specbind-steering`. A non-empty
collection is the CLI prerequisite; semantic coverage is your judgment.

Steering is the confirmed coordinate system for boundary decisions, not proof
of runtime behavior. If the implementation materially contradicts it, present
the contradiction and stop before proposing Specs. The user must decide whether
the code is divergent or Steering is stale, then synchronize Steering and start
again from a new clean revision.

### Map broadly, then investigate only the selected area

Dispatch fresh readers for independent evidence lines. Use the registered
`specbind-researcher` role when available, with ordinary fresh readers only when
the role is absent. A configured role that cannot start is an environment or
configuration failure, not permission to substitute another model.

The shallow repository map covers:

- public APIs, routes, commands, events, schemas, and other entry points;
- package and module ownership, dependency direction, and integration seams;
- test groupings and externally observable behavior;
- boundaries implied by Steering and evidence that contradicts them.

This pass prevents a narrow requested area from being split without seeing its
neighbors. It does not extract every behavior in the repository. Deep evidence
collection is limited to the selected adoption area.

Propose boundaries by durable responsibility, never directory size, task count,
or the convenience of the current layout. For each candidate state:

- a lowercase kebab-case identity;
- the responsibility it owns and what is outside it;
- public entry points and neighboring candidates;
- why Steering and implementation evidence support this boundary;
- any overlap, uncertainty, or dependency the user must decide.

### Confirm the boundary set

Present the complete proposal before writing or invoking Discovery:

```text
Adoption scope: <the selected area>
Candidate Specs: <identity, responsibility, and boundary reason for each>
Dependencies: <candidate-to-candidate edges, or None>
Unmanaged area: <what remains outside this adoption, or None>
Uncertainties: <boundary questions the user must resolve, or None>
```

The request to run this skill is **not** confirmation of the boundaries. Revise
the proposal on feedback and stop when the same substantive disagreement
survives one revision.

After explicit confirmation, create the UTF-8 dossier at the exact path above.
It is a transient, Git-tracked investigation ledger rather than lifecycle state.
Use this version-1 shape and keep candidate and observation IDs stable:

```yaml
schema_version: 1
source_revision: <full Git object ID from preflight>
adoption_scope: <confirmed selected area>
stage: boundaries_confirmed
boundary_candidates:
  - spec: <canonical-spec-id>
    responsibility: <owned responsibility>
    evidence_paths:
      - <project-relative path>
    depends_on: []
observations: []
```

Paths are project-root-relative POSIX paths. The dossier contains no secrets,
credentials, generated dumps, or source excerpts. It records claims and precise
locations, not copied implementation.

Checkpoint only the dossier through the project's Git adapter:

```sh
specbind adapter read git
```

Absence or inactive guidance means no adapter-directed commit. Otherwise follow
the adapter narrowly, staging only the dossier. Never infer push authority.

Now hand the confirmed candidate set to `specbind-discovery` as one self-contained
request to create those new Specs. Discovery owns its own four-field scope
confirmation, milestone mutation, and initial Briefs. Stop this invocation; do
not collapse its confirmation into the adoption-boundary confirmation.

## 3. Resume after Discovery

Read the dossier and verify:

- its version is `1` and its `source_revision` is a full existing commit;
- the repository has the active milestone and exact new Specs recorded as
  accepted boundary candidates;
- each candidate has a Brief;
- Steering is readable and unchanged from the source revision;
- changes since `source_revision` are limited to the tracked dossier and the
  ordinary Discovery-created SpecBind paths. Any implementation, test,
  configuration, dependency, or unrelated documentation change makes the
  evidence baseline stale and stops the run.

Use `git diff --name-only <source_revision>...HEAD` plus the current worktree
status to establish that classification. Do not silently rebaseline. The user
must reconcile the unrelated work, synchronize Steering when needed, and start
a fresh adoption run.

Read the active scope and every candidate's Brief:

```sh
specbind milestone scope --include-body
specbind artifact read <spec> brief --for maintain
```

## 4. Reverse each Spec deeply

Process one confirmed Spec boundary at a time. Independent evidence collection
may run in parallel, but user reconciliation and Brief changes remain per Spec.

For the current Spec, inspect implementation, tests, schemas, public interfaces,
and runtime checks that lie inside or cross its boundary. Record observations in
the dossier using this shape:

```yaml
- id: OBS-001
  spec: <canonical-spec-id>
  kind: observed
  claim: <one externally meaningful behavior>
  evidence:
    - path: <project-relative path>
      locator: <symbol, test name, route, schema entry, or line range>
  disposition: pending
```

`kind` is `observed` only for a claim directly supported by the named evidence;
use `inferred` when several facts support a conclusion that none states alone.
Never label a claim `intended` merely because the code behaves that way.

### Reconcile intent

Present the observations with enough evidence for the user to decide. Every
observation receives exactly one disposition:

| Disposition | Meaning |
| --- | --- |
| `requirement` | Confirmed intended behavior to carry into the Brief |
| `design` | Technical or structural constraint for Research and later Design |
| `bug` | Current behavior is not intended and needs ordinary lifecycle work |
| `historical_constraint` | Kept for now but not promoted as a product promise |
| `implementation_detail` | Internal detail with no specification obligation |
| `unknown` | Evidence or intent remains insufficient |

Do not treat silence as acceptance. A `pending` observation blocks completion of
this Spec's adoption preparation. An `unknown` is an explicit accepted outcome;
state what Requirements or Design must decide rather than inventing the answer.

## 5. Materialize the per-Spec handoff

Before the first managed Markdown write, read:

```sh
specbind protocol read okf-authoring
specbind template resolve spec <spec> research
specbind template read spec research
```

Write or replace the Spec's Research from the resolved scaffold. Preserve its
durable scoped instructions. Include the observation claims, exact evidence,
dispositions, and destinations needed by Design. Research remains background,
not authority, and is deleted at release.

Revise the Brief only with user-confirmed intent:

- fold `requirement` dispositions into a concise Adoption intent section in the
  requester's terms;
- state decisions still required for accepted `unknown` dispositions;
- route a confirmed bug into the same Spec's adoption intent only when the user
  wants this milestone to correct it;
- keep evidence paths, code structure, and implementation mechanisms in
  Research, not the Brief.

This is the same request-mediated boundary used by gap analysis: Requirements
reads the confirmed request, not implementation evidence. Read the Brief back
after writing it. Do not author `requirements.md` here.

Repeat until every accepted boundary has a complete Brief and Research handoff
and no observation remains `pending`.

## 6. Retire the project dossier and checkpoint

Delete `<specDir>/adoption/reverse-discovery.yaml` only after every accepted
candidate has been materialized and read back. The file disappears from the
current tree but remains in Git history. Do not delete the per-Spec Research;
normal release finalization owns that lifecycle.

Read the Git adapter and checkpoint only the Brief, Research, and dossier
deletion paths this phase produced. A missing or inactive adapter means no
adapter-directed commit; push is never inferred.

Report each Spec as ready for its normal next command:

```text
specbind-requirements <spec>
```

## Boundaries

- Initial adoption supports a project with no persistent Specs and no active
  milestone. Later incremental reverse adoption is future work.
- Steering is mandatory for this workflow even though it remains optional for
  ordinary SpecBind work.
- Existing implementation and tests are evidence, not intended specification.
- This workflow owns the temporary dossier, per-Spec adoption Research, and the
  narrow user-confirmed Brief revision. It owns no lifecycle state or gate.
- Discovery owns Spec creation and initial Briefs. Requirements, Design, Tasks,
  implementation, and validation use their ordinary skills without a brownfield
  branch.
- Do not change implementation, tests, dependencies, configuration, or Steering
  while establishing the adoption baseline. Findings become later lifecycle
  work; they are not repaired during reverse discovery.
