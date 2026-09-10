# 0210: Project durable artifact responsibilities into inventories

Status: Accepted

## Context

Issues #53, #55, and #56 need concise routing metadata without a duplicate index
or summaries inferred from Markdown bodies.

## Decision

Requirements `description` names the Spec's complete current responsibility.
Design and Steering descriptions name their lasting technical and project-wide
decision boundaries respectively, not the current Milestone delta. They are OKF
Front Matter strings, non-empty after trimming, on one line, without control
characters or Unicode line/paragraph separators. Quotes are legal and escaped
by the existing text-result renderer.

Missing descriptions in existing major-one live documents and project templates
remain valid. Present invalid values receive a profile diagnostic. New authoring
and materialization require descriptions. Requirements authors replace the
template's generic description with a Spec-specific sentence. Fixed Design and
Steering templates carry literal recurring responsibilities into live documents;
custom Steering scaffolds and one-off Designs receive the assessed responsibility.
New custom templates describe actual recurring decisions rather than technology
labels. Update descriptions with the body only when responsibility boundaries
change. Template edits never implicitly rewrite live documents.

`spec list` appends `description="..."`, `description=missing` (a valid
Requirements artifact without the field), `description=invalid` (recognized
Requirements with profile/content diagnostics), or `description=unavailable`
(no unambiguous Requirements artifact). This is current authored metadata, not
approval evidence; existing lifecycle state remains independent.

`artifact list` adds the field only to Design entries. `steering list` adds it to
each Steering entry. Missing and invalid values are explicit. Existing identity,
sort order, path semantics, and partial-inventory error behavior remain in force.
Raw reads retain their exact-byte success contract and existing diagnostic guards.
`artifact check <spec> <selector> --template <selector>` verifies new Markdown
materialization against an explicitly selected scaffold. It checks profile,
durable instructions, placeholders, required descriptions, and literal inheritance
for a fixed identity. A one-off Design may use `design/main` as its shape while
replacing identity and responsibility description. Like `steering check`, this is
a read-only creation check, not inferred provenance for an established artifact.
Discovery continues reading every Steering document.

## Consequences

There is no new index, manifest, lifecycle field, or metadata on other profiles.
Description reconciliation is recommended, never a same-major read prerequisite.
Decision 0211 provides its version-range plan. Editing descriptions follows the
owning authoring workflow and ordinary fingerprint/approval rules.
