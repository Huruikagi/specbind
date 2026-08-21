# 0131: Make the default deferred destination explicit and OKF-conformant

Status: Accepted

[Decision 0138](./0138-dedicated-adapter-scaffold-marker.md) supersedes the
shared instruction-marker rule and removes the deferred compatibility exception.
All adapter selectors now use the same exact dedicated scaffold marker.

## Context

[Decision 0122](./0122-finding-disposition-and-deferred-destination.md) gives
review findings a non-blocking destination, but describes its working default
only as `deferred.md` at the "specification-directory root." That phrase can be
read as the configured SpecBind root, the `specs/` collection, or one Spec's
directory. Those interpretations produce different files.

The same decision calls the destination outside the artifact system and gives
it no OKF profile. The configured SpecBind root is nevertheless an OKF v0.2
Knowledge Bundle. Every non-reserved Markdown file in that bundle is an OKF
concept and therefore requires Front Matter with a non-empty `type`.

There is a second ambiguity in the adapter itself. Decision 0101 makes a
remaining `specbind:instruction` comment the general signal that an adapter is
an inactive scaffold, while Decision 0122 makes the installed deferred scaffold
an active working default despite carrying the same signal. Raw adapter bodies
therefore require a selector-specific exception that is visible only in skill
prose.

## Decision

The file destination in the installed deferred adapter is exactly
`<specDir>/deferred.md`, where `specDir` is the configured SpecBind root. With
the default configuration its project-relative path is `.specbind/deferred.md`.
It is one project-wide file; entries name the Spec that produced the finding.

When the file does not exist, the adapter creates it as this minimal OKF
concept before appending the first entry:

```markdown
---
type: Deferred Findings
---

# Deferred findings
```

The file remains outside SpecBind lifecycle management. Its type is valid OKF
identity, not a SpecBind artifact profile: the CLI does not discover it as a
Spec artifact, fingerprint it, gate it, archive it, or treat its contents as
work. Decision 0122's rejection of a managed deferred artifact still stands;
only its claim that the Markdown destination has no OKF profile is replaced.

The embedded deferred adapter expresses its working default as ordinary active
policy and carries no `specbind:instruction` marker. `adapter list` reports a
deterministic state for every known selector:

- `absent` when the project file does not exist;
- `scaffold` when the document has no body guidance, or when a non-deferred
  adapter still carries a `specbind:instruction` marker;
- `active` otherwise.

For compatibility with already-installed projects, a deferred adapter with body
guidance is `active`, including an older copy that still carries the instruction
marker. Removing it yields `absent`; emptying its body yields `scaffold`. Both
mean that no destination may be followed.

## Consequences

- The default route names one unambiguous project-relative file.
- A default deferred file preserves OKF bundle conformance without becoming
  lifecycle state or a source of scope.
- Skills no longer need to infer whether a deferred adapter is actionable from
  a comment marker that means the opposite for Git and release adapters.
- Existing project-owned adapters remain untouched by installation refreshes.
