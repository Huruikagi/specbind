# Bootstrap the Release adapter

Inspect only repository evidence that can define this project's real release
procedure: root agent instructions, package and version manifests, release
workflows, build or packaging scripts, and existing release documentation. Do
not edit any of them. Before concluding that release documentation is absent,
enumerate the repository-root files and inspect matching release documents such
as `RELEASE*`, `RELEASING*`, and `CHANGELOG*` (case-insensitively), in addition to
linked documentation. Do not rely on `README.md` being the only entry point.

Draft a complete replacement Release adapter that:

- preserves the exact `type: SpecBind Release Adapter` Front Matter;
- removes the scaffold marker;
- gives concrete Prepare, Publish, Verify, and After-finalize guidance;
- says `Nothing.` in a section that requires no action;
- names fresh success evidence, not merely a command to run; and
- never invents a version label, credential, destination, release channel, or
  external verification capability the repository does not establish.

When repository evidence cannot answer a material release question, ask the
user. Do not turn a guess into durable project policy.

Present the **entire proposed adapter** and state both boundaries before writing:

1. approval authorizes only replacing the adapter and its narrow local
   checkpoint — not binding, tagging, publishing, pushing, or finalizing; and
2. the settings write is an ordinary project change, so every participating
   Spec that already has accepted completion must run its completion handshake
   again before release preflight can pass.

After explicit approval, replace only the Release path reported by `adapter
list` below the configured SpecBind root. If the project explicitly chooses no
project-specific release work at all, preserve the Front Matter and remove the
entire body instead. Then read the result back and confirm that the scaffold
marker is absent.

Read the Git adapter and inspect `git status --short`. When it has active
guidance, follow it for one checkpoint containing only the Release adapter. The
configuration approval authorizes this narrow local checkpoint as the ordinary
final action of the bootstrap, but does not authorize push or history rewriting.
If the adapter file cannot be separated safely, leave it uncommitted and report
that fact.

**Stop after bootstrap.** Report which completion handshakes must be rerun. Do
not continue into any release step in this run, even when the user originally
asked to release the milestone.
