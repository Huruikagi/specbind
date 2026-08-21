# 0140: Bootstrap release policy separately and checkpoint finalization metadata

Status: Accepted

## Context

The installed Release adapter is intentionally a scaffold because SpecBind
cannot infer a project's publication channel, credentials, versioning commands,
or external success evidence. Decision 0115 currently treats that untouched
scaffold as if the project had explicitly chosen no project-specific release
work. That lets a first release close without ever deciding how the project is
actually published.

A separate customization skill would mostly run once, and its only natural
entry point is the release workflow that consumes the adapter. The bootstrap
cannot silently become part of the same release authorization: writing a
project-owned adapter is an ordinary project change, stales accepted completion,
and says nothing about permission to tag, deploy, upload, or push.

Decision 0081 also accepts a post-publication lifecycle-metadata commit. The
published tag or package may identify the verified implementation revision and
exclude the later SpecBind log, archive, idle-state, Brief, Research, and Tasks
cleanup. The Release skill finalizes those files today but does not checkpoint
them, leaving the accepted lifecycle transition uncommitted despite the active
default Git policy.

## Decision

### Bootstrap is a one-time Release-skill branch

`specbind-release` reads the Release adapter before binding or publishing. An
absent adapter or one carrying the exact
`<!-- specbind:adapter-scaffold -->` marker is unconfigured. The skill:

1. inspects only repository evidence relevant to release, such as root agent
   instructions, package/version manifests, release workflows, build scripts,
   and existing release documentation;
2. drafts a complete project-owned Release adapter with concrete Prepare,
   Publish, Verify, and After-finalize guidance, using an explicit `Nothing.`
   where no action is required and never inventing credentials, destinations,
   release labels, or success evidence;
3. presents the full adapter and states that writing it stales any accepted
   completion evidence;
4. obtains confirmation that authorizes only the settings write and its narrow
   local Git checkpoint;
5. replaces the configured SpecBind root's `settings/adapters/release.md`,
   removes the scaffold marker, follows the Git adapter for a checkpoint
   containing only that settings file, and stops.

The run does not bind a version, execute Prepare, Publish, or Verify, or finalize
the milestone after bootstrapping. A later release run begins only after any
affected completion handshake has been rerun.

A Release adapter whose body is empty after Front Matter is different: it is an
explicit project decision that no project-specific Prepare, Publish, Verify, or
After-finalize action is required. It proceeds to core finalization. An active
body is followed as project policy.

### Finalization creates a separate local checkpoint

Immediately before `specbind release finalize`, the skill records
`git status --short`. After successful finalization it records status again and
identifies only the paths newly changed by the CLI's finalization transaction.
It then reads the Git adapter and creates one local checkpoint containing only
those SpecBind lifecycle paths when active policy directs it to commit.

The checkpoint occurs before project-specific After-finalize guidance. It does
not change the already published revision and does not authorize a push, tag,
deployment, upload, branch operation, or history rewrite. Publication approval
does not imply permission to push this metadata commit.

If the finalization paths cannot be separated safely, the Git adapter is absent,
empty, or marked as a scaffold, or the commit fails, the release remains
finalized. The skill reports the uncommitted lifecycle metadata separately and
never reruns finalization merely to obtain a checkpoint.

Release-specific version commits, tags, publication branches, and pushes remain
Release-adapter policy under Decision 0101. Any ordinary project commit made
after accepted completion and before finalization stales that evidence; the
adapter cannot bypass the required completion handshake and fresh preflight.

## Consequences

- A first release establishes visible project policy instead of interpreting an
  untouched scaffold as a deliberate no-op.
- Configuration approval cannot accidentally authorize publication.
- Projects that genuinely need no external release work can express that with
  an intentionally empty adapter body.
- Published artifacts can continue to identify the verified implementation
  revision, while SpecBind's later lifecycle transition is retained in a
  separate local commit.
- After-finalize work remains outside the core metadata checkpoint and follows
  its own project-specific commit or publication instructions.

## Implementation status

Implemented. The embedded Release skill owns the bootstrap and finalization
checkpoint branches, the default Git adapter names both eligible units, and the
release forward-test scenarios cover the first-run stop and the clean finalized
checkpoint.
