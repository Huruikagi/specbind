# 0115: Fix the release skill contract

Status: Accepted

[Decision 0138](./0138-dedicated-adapter-scaffold-marker.md) supersedes the use
of `specbind:instruction` to identify an installed adapter scaffold. Release now
uses the exact dedicated `<!-- specbind:adapter-scaffold -->` marker.

## Context

`specbind-release` is the first v1 skill with no cc-sdd predecessor — the
inherited tree has no release workflow at all — and simultaneously the most
heavily pre-decided. [Decision 0010](./0010-release-execution-boundary.md) fixes
an eight-step execution sequence and the failure semantics,
[Decision 0066](./0066-agent-judged-release-and-cli-log-insertion.md) fixes who
judges success, [Decision 0068](./0068-release-log-summary-input.md) fixes the
summary transport and the canonical log mutation,
[Decision 0071](./0071-no-partial-milestone-release.md) fixes what happens when
publication succeeds and verification does not,
[Decision 0072](./0072-explicit-release-rebinding.md) fixes binding, and
[Decision 0073](./0073-portable-release-version.md) fixes the label grammar.

So the orchestration is largely written. What is not written is an ordering
constraint that only appears when the commands are run in sequence, and the
handling of the one irreversible outward-facing action in the entire workflow.

## Binding late invalidates completion

`milestone bind-release` writes `target_release` into the Roadmap. Under
[Decision 0080](./0080-v1-task-contract-and-completion-details.md), completion
evidence is project-revision-scoped and any later non-metadata project change
stales it — and a Roadmap edit is not one of the completion-metadata mutations
the freshness evaluator tolerates.

Running the commands in the obvious order therefore fails:

```text
spec completion accept cart   → OK
milestone bind-release v1.4.0 → OK
release preflight             → ERROR
    FRESHNESS_COMPLETION_PROJECT_CHANGED: commit history since
    implementation_revision contains a non-metadata project change
    RELEASE_SPEC_GATE_NOT_FRESH: Spec cart completion gate is not fresh
```

The binding is a one-line metadata change that a reader would never expect to
touch an implementation revision, and the only exit is re-running the completion
handshake for every affected Spec at the new revision. Binding first and then
accepting completion reaches `OK RELEASE_READY` with the same inputs.

Nothing states this from the direction anyone travels. Decision 0072 describes
binding as changing "only the roadmap-owned `target_release`" and explicitly
lists what it does not rewrite, which reads as reassurance that it is cheap. It
is cheap; it is just not free once completion evidence exists.

This decision states it, and puts the cost where the skill can act on it.

## Decision

### The version comes from the user, and early

The skill never invents a release label. Decision 0073 makes the value opaque
and case-sensitive, with no normalization: `v1.4.0` and `1.4.0` are distinct
release identities. Choosing one, or helpfully adding or removing a leading `v`,
silently picks an identity the project did not.

When no version is bound, the skill asks. When one is bound and the user names a
different one, `--rebind` is a deliberate replacement and is run only after
explicit confirmation.

Because of the ordering above, the skill states the cost when binding late: if
any participating Spec already holds completion evidence, binding stales it and
those Specs must be revalidated through the Decision 0086 handshake before
release preflight can pass. Where the version is known earlier, binding earlier
avoids the round trip entirely.

### Publication is confirmed with the user

Publishing is the only action in the SpecBind workflow that is outward-facing
and cannot be undone by SpecBind. A tag, a deployment, a package upload, or a
store submission leaves the repository and becomes visible to people the run
cannot reach.

The skill therefore confirms with the user before executing Publish guidance,
stating what the adapter will do and to which version. This holds even when the
run was started with broad instructions, because the authority to release is not
implied by the authority to prepare a release.

Prepare and Verify need no such pause; they are repeatable and local.

### Verification is a completion claim

Adapter Verify guidance asks whether the intended version really was published
and is usable. That is a completion claim, so the skill applies the
`completion-verification` protocol to it — the third consumer of that protocol,
for the same reason as the first two.

The adapter scaffold already states the rule this enforces: re-reading what the
publish step reported is not verification. A publish command's own success
output is a claim about itself, and Decision 0066 is explicit that neither the
CLI nor SpecBind verifies external publication.

Where verification cannot be performed at all — no way to reach the published
artifact, no credentials for the check — that is the protocol's cannot-verify
outcome, and it is not a pass. The skill reports it and does not finalize.

### The summary describes what was delivered

For each participating Spec the skill writes one delivered-change summary.
Decision 0066 has it agree with the final Requirements, active Requirement IDs,
Design, completed tasks, Roadmap scope, and accepted records, and allows the
Brief only as drafting context.

That distinction is the substance of the authoring judgment. The Brief states
what was **asked for** at the start of the milestone, and the two diverge
routinely: scope was cut, an approach changed, a requirement was added during
design. A summary derived from the Brief describes an intention; the log entry
has to describe the delivery, because it is what the Spec's history will say
happened.

The skill does not pre-edit `log.md`. Decision 0068 gives the CLI the complete
structural update, including date headings, ordering, the canonical wrapper, and
idempotent retry matching by milestone ID.

### Failure before finalization stops, and does not tidy up

Under Decision 0071, failed or uncertain Prepare, Publish, or Verify work means
finalization is not invoked and every SpecBind artifact stays active.

Two things the skill must not do when publication succeeded but verification did
not:

- **Roll back the publication.** SpecBind has no authority over the external
  system, an unpublish is often impossible or itself destructive, and the
  decision to attempt one belongs to the user with knowledge the run does not
  have.
- **Retry blindly.** Decision 0071 requires a retry to interpret the adapter
  against current external state. A publish step that already partly succeeded
  may not be idempotent, and repeating it can produce a second tag, a duplicate
  artifact, or a failed upload that masks the first success.

The skill reports what was observed, states that the milestone remains active,
and works with the user on reconciling, retrying, or abandoning under Decision
0005.

### An empty adapter means no project work is needed

Decision 0063 makes the adapter free-form prose, and its scaffold says an empty
adapter is the explicit statement that releasing needs no project-specific
action. The skill treats it that way and proceeds to finalization.

This inverts the Git adapter's default under Decision 0101, where absent
guidance means do not commit, and the inversion is correct in both cases. A
missing Git policy leaves an action the project never asked for; a missing
release policy leaves a milestone that can be closed with no external step. As
elsewhere, an adapter still carrying its `specbind:instruction` comments is the
scaffold as installed rather than policy, and reads as no guidance.

### After finalize is separate

Decision 0010 has After-finalize guidance run only after core finalization
succeeds, and its failure reported separately. The skill does not treat such a
failure as a failed release, and never re-runs finalization because of it.

### Steering is recommended after finalization

[Decision 0117](./0117-steering-authoring-contract.md) adds one conditional
recommendation to the closing summary, because finalization is the point where a
steering edit stops costing a completion revalidation cycle. It is advisory: the
release neither waits on it nor fails because of it.

### Boundary

- The skill orchestrates; the CLI owns every mutation of SpecBind state.
- It authors no Spec artifact, edits no `log.md`, and approves no gate.
- It never claims SpecBind verified an external publication.
- It finalizes the complete milestone or nothing. Decision 0071 provides no
  subset option and no partially released state.

## Consequences

- The binding ordering is stated before it costs a full revalidation cycle,
  from the direction the skill travels.
- The one irreversible outward-facing action has an explicit confirmation, so
  broad initial instructions do not silently carry release authority.
- Verify guidance is held to the same evidence standard as every other
  completion claim in the workflow, using the protocol that already exists.
- The log entry describes delivery rather than intent, which is what a reader
  consulting a Spec's history a year later actually needs.
- A partially successful external release ends in a reported, human-owned
  situation rather than an automated rollback or a blind retry.

## Implementation status

Implemented. `tools/specbind/assets/skills/specbind-release/SKILL.md` is
embedded and installed.
`tools/specbind/assets/skills/specbind-validate-implementation/SKILL.md` gains
a pointer to bind the release version first where it is known.

Its forward tests are specified as scenarios RL1 through RL3 in
[Skill forward tests](../../skill-forward-tests.md) and are run manually.
