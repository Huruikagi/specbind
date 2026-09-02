# Start existing-implementation adoption through Discovery

Read this procedure only when selected by `sb-discovery` for an explicit
initial adoption and no temporary adoption record exists.

The user must name the area being adopted. A broad description such as
"authentication and account management" is sufficient; an omitted scope is
not permission to adopt the whole repository. Ask whether the intended scope is
the entire repository or a named area before investigating.

Run the deterministic prerequisite check:

```sh
specbind adoption preflight
```

It must report `ADOPTION_PREFLIGHT_READY`. Any other result stops this run.
This command already checks for existing Specs, an active milestone, the
Steering prerequisite, repository cleanliness, and the committed source
revision. Do not precede or follow it with the ordinary Discovery route's
`specbind milestone status` or `specbind spec list`; those reads duplicate the
preflight and blur which route owns the start.
Do not perform the routed recovery during the same invocation. In particular:

- `ADOPTION_STEERING_REQUIRED` routes to a separate `sb-steering`
  bootstrap invocation; report that next step and stop immediately;
- malformed Steering routes to Steering repair;
- existing persistent Specs or an active milestone are outside this initial
  adoption contract;
- a dirty repository is the user's work to reconcile. Never commit, stash, or
  move it merely to satisfy this guard.

Record the exact `source_revision` the command returned. Every claim about the
existing implementation is about that committed tree.

## Read the complete Steering baseline

```sh
specbind steering list
specbind steering read <selector> --for consume
```

Read every listed document. Confirm that the collection, regardless of file
names or decomposition, establishes all three areas needed for adoption:

- product purpose, audience, and non-goals;
- project-wide technology and verification constraints;
- structure, dependency direction, and responsibility placement.

If one is materially absent, stop and route to `sb-steering`. A non-empty
collection is the CLI prerequisite; semantic coverage is your judgment.

Steering is the confirmed coordinate system for boundary decisions, not proof
of runtime behavior. If the implementation materially contradicts it, present
the contradiction and stop before proposing Specs. The user must decide whether
the code is divergent or Steering is stale, then synchronize Steering and start
again from a new clean revision.

## Map broadly, then investigate only the selected area

This is a non-negotiable fresh-reader boundary. Before the orchestrator reads
implementation or test files or synthesizes candidate boundaries, dispatch at
least two fresh readers for independent evidence lines. Run them concurrently
when capacity permits; sequential fresh readers are valid when capacity is
limited, provided neither inherits the other's findings:

- one for public behavior, tests, schemas, and observable entry points;
- one for structure, dependency direction, ownership, and neighboring seams.

Use the registered `specbind-researcher` role when available, with ordinary
fresh readers only when the role is absent. The orchestrator synthesizes their
reported evidence and does not replace either line with its own repository
inspection. A configured role that cannot start is an environment or
configuration failure, not permission to substitute another model or continue
the map in the orchestrator.

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

## Confirm the boundary set

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

After explicit confirmation, create the UTF-8 temporary adoption record at the
exact path named by the entrypoint. It is a transient, Git-tracked investigation
ledger rather than lifecycle state. Use this version-1 shape and keep candidate
and observation IDs stable:

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

Paths are project-root-relative POSIX paths. The record contains no secrets,
credentials, generated dumps, or source excerpts. It records claims and precise
locations, not copied implementation.

Checkpoint only the temporary adoption record through the project's Git adapter:

```sh
specbind adapter read git
```

Absence or inactive guidance means no adapter-directed commit. Otherwise follow
the adapter narrowly, staging only the record. Never infer push authority.

Now hand the confirmed candidate set back to the ordinary route in the
`sb-discovery` entrypoint as one self-contained later request to create
those new Specs. Ordinary Discovery still owns its four-field scope
confirmation, milestone mutation, and initial Briefs. Stop this invocation; do
not collapse its confirmation into the adoption-boundary confirmation.
