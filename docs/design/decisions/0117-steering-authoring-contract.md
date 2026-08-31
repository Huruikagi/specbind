# 0117: Fix the steering authoring contract

Status: Accepted

## Context

[Decision 0098](./0098-steering-read-surface.md) fixed what a steering document
is and how one is read, and deliberately deferred everything else: the
`specbind-steering` skill, bootstrap, synchronization, drift detection, steering
templates, and any write command.

That deferral left the authoring side with no contract at all, while the
inherited `kiro-steering` and `kiro-steering-custom` skills remain as the only
description of the workflow.
[Decision 0075](./0075-v1-skill-and-orchestration-scope.md) merges those two into
one skill, so their combined behavior needs a decision before the merged skill
can be written.

The inherited pair also disagrees with an accepted SpecBind rule on the central
question of what synchronization does, which is settled below.

## Authoring has no state to transition

Every other v1 skill authors content and delegates state changes to the CLI.
Steering has no state to change. Under Decision 0098 it is never fingerprinted,
never gate evidence, and never a freshness input, so there is no approval to
cross and no transition to guard.

This decision therefore adds no steering mutation command. The skill writes the
files, exactly as `specbind-requirements` writes `requirements.md`.

What the absent command does remove is validation. Nothing checks that a written
document declares `type: SpecBind Steering`, carries a well-formed `artifact_id`,
or avoids colliding with an existing one — and a collision is a hard discovery
error under [Decision 0057](./0057-type-based-artifact-discovery.md) that drops
**both** documents from the inventory.

The read surface is the validator:

- Before choosing an `artifact_id`, the skill runs `specbind steering list` and
  treats every listed selector as taken.
- Every listed document carries both its SpecBind-root-relative logical `path`
  and its directly usable project-root-relative `project_path`. Authoring and
  repair use only `project_path`; they never infer `specDir` from `path`.
- After writing, it runs `specbind steering list` again and confirms its document
  appears with the expected selector. A document that does not appear was
  authored wrong, and the skill says so rather than reporting success.

This costs two command runs and needs no new surface. A dedicated validation
command would duplicate discovery for one caller.

## A broken collection cannot be read through the CLI

Decision 0098 makes a steering read fail whenever any inventory diagnostic
exists, including one attached to a different document. That is right for
consumers: acting on project guidance known to be incomplete is worse than
stopping.

For the authoring skill it is a deadlock. Repairing a malformed steering document
is this skill's work, and `steering read` refuses to hand it the input —
including the healthy documents it would need for context.

`steering list` still reports each fault with its path, so a malformed file is
identifiable. The skill therefore reads that file directly. A duplicate
identity is different: every colliding path is reported, but the diagnostic
cannot decide which document owns the identity. The skill may remove one only
when repository history proves it is the newly introduced duplicate in the
repair scope. Matching content or a copy-like filename is not proof; without
provenance, the paths and consequences go to the maintainer for a choice.

**The repair exception.** The steering skill may read steering files directly
when `steering list` reports diagnostics naming them, and only to repair them.
Every other read, by this skill and by every other skill, goes through
`steering list` and `steering read` as Decision 0098 requires.

The exception is narrow in the way that matters: it is available only for a
document the CLI has already declared broken, so it can never become a convenient
way to skip the read model for documents that are fine.

## Synchronization revises in place

The inherited skill states its update philosophy as "Add, don't replace. Preserve
user sections." The `steering-principles` rule accepted by
[Decision 0093](./0093-default-shared-rule-set.md) states the opposite: steering
is edited rather than accumulated, guidance is revised in place, and Git holds
the history.

The rule wins. A steering document is supposed to describe the project as it is
now, and an additive workflow produces documents where the current statement and
the one it replaced sit side by side, leaving readers to guess which is in force.

In-place revision is not a license to rewrite:

- Revise what the codebase demonstrably contradicts.
- Do not rewrite for style, and do not restructure a document merely because this
  run would have written it differently.
- When content's intent is unclear, propose the change and let the user decide
  rather than performing it. Unclear is not the same as stale.

Drift is reported in both directions, as the inherited skill had it: steering
claiming something the code no longer does is a warning, and a durable new
pattern the code has established is an update candidate.

## The familiar three are a default, not a privilege

Decision 0098 removed the core-versus-custom split and stated that nothing
privileges `product`, `tech`, and `structure`. Bootstrap still proposes exactly
those three, because they are a genuinely good first decomposition and a project
starting from an empty `steering/` benefits more from a concrete starting point
than from an open question.

The two are compatible because of where the default lives. The three exist as
**template assets**, which a project may override, and whose output a user may
rename, merge, split, or decline at bootstrap. Nothing in discovery, the read
model, or the CLI knows those names; type-based discovery is untouched.

A default a project can delete is not a privilege.

## Steering is recommended after a release, and only then

Nothing invokes this skill. Steering describes durable facts, so no gate,
freshness rule, or command produces the moment where it should be revisited, and
the observed result in cc-sdd is that steering falls behind the codebase it
describes. Under Decision 0098 discovery reads the whole collection and routes on
it, so steering that has quietly gone stale is worse than steering that was never
written: routing trusts it either way.

**The window is fixed by completion freshness, not by preference.** Completion
evidence is project-revision-scoped under
[Decision 0080](./0080-v1-task-contract-and-completion-details.md), and the only
tolerated change is a Spec's own completion transition in `spec.yaml`. A steering
edit is an ordinary project change, so editing steering costs:

| Interval | Cost |
| --- | --- |
| Milestone start until the first accepted completion | Free. No gate fingerprints steering |
| First accepted completion until `release finalize` | The completion handshake must be re-run for every affected Spec |
| After `release finalize` | Free, and the next milestone has no completion evidence yet |

This is the trap [Decision 0115](./0115-release-skill-contract.md) found for
`bind-release`, in a second place. The recommendation therefore belongs **after
finalization succeeds**, never before it, and the post-release moment is the
widest safe window in the cycle rather than merely a convenient one.

**The recommendation is conditional.** Offered after every release it becomes
ceremony, and a prompt that is usually noise is one users learn to skip. The
release skill already holds what decides it, so it recommends steering work when
any of these holds:

- the milestone's Roadmap work items included a new Spec, which means the project
  took on a durable responsibility it did not have before
- Contracts changed against the milestone baseline revision, which means a
  boundary moved
- the project has no steering documents at all and has now shipped a release

**It is a recommendation and nothing more.** A release never fails, waits, or
warns because steering is old. Decision 0098 keeps steering out of gate evidence
and freshness inputs, and no last-reviewed timestamp, fingerprint, or staleness
flag is introduced here — that is the state that decision refused, and a
recommendation does not need it.

It belongs in the release skill's closing summary rather than in `release
finalize` output, which stays a concise guarded mutation result under
[Decision 0067](./0067-text-first-english-cli-results.md).

**This is late, and knowingly so.** A durable pattern is established during
implementation, and by the release the agent that saw it is long gone, so the
release skill recommends from what was delivered rather than from what was
learned. Observing earlier and acting after finalization is the better shape, but
it needs somewhere for the observation to survive the milestone and is deferred
until this trigger has been used enough to show whether it is needed.

## The mode is confirmed, not inferred

The inherited skill selected bootstrap or sync by checking whether the core files
existed. With no core files, that test is gone.

The skill confirms intent with the user instead — bootstrap, synchronize, or add
one document — using the current inventory as input rather than as the decision.
It confirms even when the inventory is empty: an empty `steering/` is a valid
steady state for a project that has decided it does not want steering, and
silently bootstrapping one is the failure mode that teaches people to avoid the
skill.

## Adding documents records policy; it does not create it

A request to "write down how we do" a subject authorizes documenting an
existing durable convention. It does not authorize the authoring skill to turn
an absent practice, an implementation example, or its own preference into new
project policy.

For Add, the skill inspects subject-specific project evidence before writing. If
that evidence establishes no settled convention, it reports the absence and
asks the maintainer for the actual convention or for an explicit decision to
establish one. It stops with no new document until that answer arrives. Once the
maintainer supplies the policy, the same run materializes it without inventing
additional obligations. Recording that no tool or command currently exists is
not permission to add a normative replacement beside the absence.

## A steering template scope

`template list` and `template read` gain a second scope:

```text
specbind template list steering
specbind template read steering <selector>
```

The embedded set is `product`, `tech`, `structure`, and `document`. The first
three are the bootstrap defaults; `document` is the scaffold for any other
steering document.

**Embedded, not installed.** The installed customization surface fixed by
[Decision 0091](./0091-installed-template-surface.md) is unchanged. Steering is
optional, so installing four files into every project — including those that
never author steering — is exactly the stranding risk that decision warns about.
Widening later is safe; narrowing later is not.

A project owns any of them by creating
`settings/templates/steering/<selector>.md`. Per-selector resolution already
prefers a project copy.

The inherited `steering-custom/` set of seven domain templates is not carried
over. Those prescribe content for domains SpecBind knows nothing about, and they
are artifacts of the core-versus-custom split Decision 0098 removed.

## The steering scope supplies its own `artifact_id`

[Decision 0059](./0059-okf-artifact-templates.md) requires a collection template
to carry a literal stable `artifact_id`, and states that AI does not choose or
rewrite that value during ordinary materialization.

That rule cannot hold for steering. Decision 0098 makes `artifact_id` a free
identity — it notes that `main` is valid — so every steering document outside the
three named defaults has an ID only its author can choose. The constraint is
unavoidable whether or not templates exist.

For the `steering` template scope only:

- `product`, `tech`, and `structure` carry their literal `artifact_id`, and their
  output path is `steering/<artifact_id>.md`.
- `document` omits `artifact_id`. The authoring skill supplies it and derives the
  output path from it.
- Template listings retain that SpecBind-root-relative `output_path` and also
  expose its project-root-relative `project_path`. For `document`, the latter is
  the explicit pattern `<specDir>/steering/<artifact_id>.md`. The author writes
  only to `project_path` after substituting the chosen identity.
- A supplied ID follows the same kebab-case rule discovery enforces, and must not
  collide with a listed selector. The `document` template is never used to
  recreate one of the three named IDs.

This is the whole exception. Identity in every other template scope stays
CLI-owned.

## Hints belong in instruction comments

Decision 0139 later scopes Decision 0059's `specbind:instruction` comments.
`create` guidance is removed during materialization, while `maintain` and
`consume` guidance is copied into the Steering artifact and projected by its
purpose-specific CLI reads. They are available in the `steering` template scope
on the same terms as Spec templates.

They carry what a template is genuinely better at than prose stored elsewhere:
what this document is for, what a first version looks like, and the mistakes this
artifact invites — copying a directory listing, recording work in flight.

The `document` scaffold additionally names example subjects, such as API
conventions, testing approach, security posture, or deployment. That recovers the
only durable value the inherited seven templates had — telling a newcomer what
may go in steering at all — at the cost of one comment rather than seven files.

Templates do not restate the project's conventions. Those live in
`settings/rules/steering-principles.md`, which the project owns and may relax; a
template repeating them would keep whispering guidance the project had already
chosen to change.

## Codebase analysis is dispatched

Bootstrap and synchronization both analyze the codebase, and the inherited skill
already ran product, technology, and structure research in parallel.

Those runs are dispatched as fresh-context subagents under
[Decision 0109](./0109-subagent-dispatch-contract.md), which fits them well: each
is independent, each reads widely, and none of what they read needs to remain in
the authoring context once the pattern is extracted.

## Secrets and tooling are never written

The skill never writes credentials, keys, tokens, or other secrets into steering,
and does not document SpecBind's own `settings/` tree or agent tool directories
such as `.claude/` and `.agents/`. Those are project metadata rather than project
knowledge, and a steering document describing them ages against a tree the
project does not maintain.

Both are product-owned skill behavior, not rule content. A project may relax its
rules; it may not relax these by doing so.

## Boundary

- No gate, no approval, and no milestone requirement. Steering outlives
  milestones, and the skill runs whenever guidance changes.
- The skill reads the codebase and the steering collection. It does not use Spec
  or milestone state as evidence for Steering content, which puts the transient
  content the `steering-principles` rule excludes out of reach rather than
  merely discouraging it. The lifecycle status read required by Decision 0119
  remains a write-safety preflight only; it never becomes document content.
- Reasoning that changed a routing or scoping decision still lands in a Brief or
  the Roadmap body under Decision 0098. This skill does not become the place
  where such reasoning is recorded.

## Consequences

- The last deferral in Decision 0098 is closed, and `specbind-steering` can be
  written against a contract rather than against the inherited skill.
- Steering identity is checked by the read surface the product already has,
  without a validation command whose only caller would be one skill.
- A malformed steering document is repairable, and the fail-closed read every
  consumer relies on is unchanged.
- Synchronization produces documents that state the project's current conventions
  once, rather than an accumulating record of every convention it ever had.
- A new project gets the familiar three documents, and a project that wants
  something else is not arguing with the product to get it.
- One template scope carries an identity exception no other scope needs, because
  steering is the only collection whose identities are the author's to choose.

## Implementation status

Implemented. `template list steering` and `template read steering <selector>`
resolve a project-owned copy below `settings/templates/steering/` ahead of the
embedded `product`, `tech`, `structure`, and `document` scaffolds in both
artifact languages. The scope keeps its own narrow profile rather than joining
the spec-local `ArtifactKind` set, exactly as `steering.rs` does for discovery:
a declared `artifact_id` locates the output at `steering/<id>.md`, an omitted one
marks the scaffold whose identity the author supplies, and two templates claiming
one identity are reported rather than left to collapse the materialized
collection. The set is embedded and not installed, so `INSTALLED_SELECTORS` is
unchanged.

The embedded `specbind-steering` skill confirms its mode before writing, reads
the collection through the CLI, takes the repair exception only for a file the
listing has already faulted, revises in place, and verifies every document it
wrote by listing again. `specbind-release` carries the conditional post-release
recommendation in its After-finalize step.

Tests cover the embedded set per language, materialization into a discoverable
collection including an author-identified document, project override, the
duplicate and malformed identity diagnostics, silent skipping of another type,
and the CLI listing and read for both the fixed and authored output paths.
Forward-test scenarios S1 through S7 remain outstanding, pending a run against
the fixture project.
