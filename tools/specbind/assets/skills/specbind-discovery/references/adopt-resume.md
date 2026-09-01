# Resume existing-implementation adoption through Discovery

Read this procedure only when selected by `specbind-discovery` after ordinary
Discovery created the dossier's exact candidate Specs and Briefs.

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

## Reverse each Spec deeply

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

## Materialize the per-Spec handoff

Before the first managed Markdown write, read:

```sh
specbind protocol read okf-authoring
specbind template resolve spec <spec> research
specbind template read spec research
```

Write or replace the Spec's Research from the resolved scaffold. Follow every
`create output=<name>` instruction once to produce its named output. An output
may be a short string or a Markdown fragment. Replace every reference to that
name with the same produced output, and omit the `create` instruction. Preserve its durable
scoped instructions. Include the observation claims, exact evidence,
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

## Retire the project dossier and checkpoint

Delete `<specDir>/adoption/reverse-discovery.yaml` only after every accepted
candidate has been materialized and read back. The file disappears from the
current tree but remains in Git history. Do not delete the per-Spec Research;
normal release finalization owns that lifecycle.

Read the Git adapter and checkpoint only the Brief, Research, and dossier
deletion paths this phase produced. A missing or inactive adapter means no
adapter-directed commit; push is never inferred.

Report each Spec as ready for its normal next command:

```text
specbind-plan <spec> requirements
```
