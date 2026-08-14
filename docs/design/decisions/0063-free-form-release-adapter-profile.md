# 0063: Keep the release adapter free-form and agent-interpreted

Status: Accepted

## Context

Decision 0002 separates the non-overridable SpecBind release lifecycle from project-specific release instructions, and Decision 0010 assigns execution of those natural-language instructions to the AI agent rather than the CLI. Earlier drafts still described Prepare, Publish, Verify, and After finalize as explicit adapter phases whose Markdown schema remained undecided.

If the agent owns semantic interpretation, requiring literal headings or a parsed section order would add a second command language without improving the CLI's safety boundary. Some projects also need no project-specific action beyond the core SpecBind release contract, so an empty adapter must be representable without placeholder commands.

## Decision

- `{{SPEC_DIR}}/settings/release.md` is an OKF concept whose only SpecBind-owned Front Matter field is:

  ```yaml
  ---
  type: SpecBind Release Adapter
  ---
  ```

- Unknown top-level Front Matter extensions are allowed under Decision 0045 but carry no SpecBind semantics.
- The Markdown body is free-form agent-readable project guidance. The CLI does not require or parse an H1, phase headings, heading order, lists, code blocks, or any other body structure.
- The body may be empty. Empty content means that the project defines no adapter-specific release actions; it does not weaken or skip any core readiness, evidence, verification, or finalization requirement.
- The default scaffold may suggest Prepare, Publish, Verify, and After finalize headings for readability. Those labels are template guidance, not machine syntax, and projects may rename, reorganize, combine, or omit them.
- The release skill reads the complete current adapter and semantically maps any applicable guidance into its core orchestration sequence:
  1. core preflight
  2. project preparation when instructed
  3. project publication when instructed
  4. project verification when instructed
  5. guarded core finalization after required evidence is satisfied
  6. project after-finalize work when instructed
- The CLI validates OKF syntax, the exact `type`, and required file presence. It neither decides which prose belongs to a phase nor executes Markdown as commands.
- If non-empty guidance is ambiguous, unsafe, contradictory, or insufficient for an action it appears to require, the agent stops for clarification rather than inventing project commands. Absence of adapter guidance by itself is not an error.

## Core boundary

- An empty or permissive adapter cannot waive a core gate, authorize an external write, provide credentials, or count as publication or verification evidence.
- The release skill remains responsible for obtaining any authority required by project instructions and for submitting the structured evidence required by the CLI finalization contract.
- Whether a no-publication project can satisfy the core immutable-reference and verification contract, and the exact evidence representation, belong to the release-evidence decision rather than the adapter Markdown profile.
- A missing `settings/release.md` remains an installation or configuration error because the project-owned customization surface is absent. A present empty adapter is the explicit no-project-actions representation.

## Consequences

- Projects can document release procedures in the structure natural to them without maintaining a CLI-specific Markdown grammar.
- The same adapter remains usable by every supported agent because interpretation belongs to the shared release workflow contract.
- Projects with no special preparation, publication, verification, or cleanup steps can keep an intentionally empty body.
- Safety continues to come from core preflight, structured evidence, authorization boundaries, and guarded finalization rather than from parsing prose headings.
