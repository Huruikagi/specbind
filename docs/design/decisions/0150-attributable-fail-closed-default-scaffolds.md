# 0150: Keep default artifact scaffolds attributable and fail closed

Status: Accepted

## Context

Decision 0149 makes the owning Spec visible in rendered artifact titles, but a
collection artifact can still become ambiguous when separated from its path.
Design and Implementation Notes are identified by `artifact_id`, so their
titles need that second stable identity as well.

Some default scaffolds also look more complete than they are. The Requirements
template contains a dummy Requirement that satisfies the live parser if copied
unchanged. Brief, Research, and Implementation Notes accept a heading without
any authored body. In both cases an agent can accidentally persist a
structurally valid placeholder instead of real content.

## Decision

- Spec collection templates may use the built-in body variable
  `{{artifact_id}}` in addition to `{{spec}}`. Its value is the validated
  `artifact_id` from the selected template's literal Front Matter.
- Every use follows the Decision 0149 binding rule and therefore requires
  exactly one `specbind:instruction create bind=artifact_id` comment. Using the
  variable in a singleton template is a template diagnostic because no value
  exists there.
- The official Design and Implementation Notes titles render both the canonical
  Spec identity and their `artifact_id`. Front Matter, target paths, and all
  other rendering boundaries remain unchanged.
- The official Requirements scaffold contains no dummy live Requirement. Its
  `create` instruction explains the required shape, and the empty Requirements
  section deliberately fails live validation until the author supplies at
  least one real Requirement and Acceptance Criterion.
- A live Brief, Research, or Implementation Notes artifact must contain
  substantive Markdown outside headings, complete HTML comments, and managed
  instruction comments. Ordinary prose, list content, code, and non-comment
  HTML count; headings and comments alone do not.
- This is a narrow structural completeness check, not a prescribed outline or
  semantic quality judgment. Brief and Research remain free-form, and optional
  Research or Implementation Notes should remain absent when there is no real
  content to preserve.
- An unfilled template is an authoring scaffold, not a valid live artifact. The
  owning workflow renders it, follows its `create` guidance, removes `create`
  comments, and validates the completed artifact before writing it.
- Contract and Roadmap retain their existing fixed boundaries. Broader
  project-defined values and rendering behavior remain deferred to Issue #10.

## Consequences

- Collection artifacts identify both their Spec and stable selector when read
  away from their directory.
- Default scaffolds cannot pass validation merely because an example or heading
  was copied unchanged.
- Projects may still customize the prose and headings that the artifact profile
  does not reserve, but cannot weaken the minimum live-content checks.
- Template validation can mechanically prove that each CLI-owned value is both
  available for the selected artifact kind and paired with creation guidance.
