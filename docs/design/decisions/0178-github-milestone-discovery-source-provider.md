# 0178: Add GitHub Milestones as a Discovery source provider

Status: Accepted

## Context

Decision 0164 established provider-neutral Source Collection semantics and the
project-local `local-files` provider. A release scope often already exists as a
GitHub Milestone, but treating its Issue list as an informal prompt loses closed
Issues, exclusions, provenance, and the normal Discovery confirmation boundary.

## Decision

`github-milestone` is the first remote Source Collection provider. It is a
progressive `sb-discovery` reference, not a Rust CLI capability.

The maintainer supplies either both a canonical `OWNER/REPO` and a Milestone
number, or the exact canonical URL
`https://github.com/OWNER/REPO/milestone/NUMBER`. Discovery parses that URL as
the same two explicit identities; it does not infer them. A URL with a different
host, path shape, query, fragment, empty component, or non-numeric number is
not a collection selector and stops for clarification.
Discovery reads the selected repository and numbered Milestone through an
available authenticated GitHub integration; authenticated `gh` is the fallback.
It must prove that the returned repository and Milestone identities match the
request, inventory every page of entries in that Milestone with `state=all`, and
take one logical acquisition snapshot before classification. A title alone is
never enough to choose between Milestones.

For every returned entry, Discovery records repository identity and URL,
Milestone number, title, and URL; and the entry number, title, state, URL, and
observed update time. Entries with `pull_request` metadata are visible
non-Issue items, not requirements. Issue bodies may inform classification.
Comments and timeline events are excluded: Discovery neither reads nor treats
them as source material. Labels, state, author, assignee, and Milestone metadata
are routing evidence only.

An inaccessible repository or Issue, ambiguous or missing identity, unsupported
entry, failed page, or incomplete pagination is a partial acquisition. Discovery
reports every known item and stops before classification, Gate invalidation,
Roadmap mutation, or Brief authoring. It never comments, edits, labels, assigns,
closes, or otherwise mutates GitHub.

The shared `source-material` semantics remain unchanged: every captured item has
a visible disposition; the Roadmap holds the complete collection mapping; each
Spec-backed Brief holds only its exact relevant Issues and why; a shared Issue
may inform several Specs; and Direct items have no Brief. GitHub metadata and
Briefs are request context, not Contract Review `deepInputs` or a second
specification authority.

Discovery records the observed remote provenance but creates neither a universal
remote revision nor a persisted source-set fingerprint. Later Requirements and
Design promote the approved Brief's captured meaning into canonical artifacts;
they do not silently re-query GitHub. A later GitHub change enters only through
an explicit Discovery rerun and ordinary confirmed scope/update and rewind flow.

## Carrier allocation

- `source-material` keeps provider-neutral coverage, authority, provenance, and
  promotion boundaries, including the remote no-requery rule.
- `sb-discovery/references/github-milestone.md` owns identity resolution,
  complete read-only acquisition, entry handling, and failure stops.
- `sb-discovery` selects that procedure only for explicit repository/Milestone
  identities, whether stated separately or supplied by the exact canonical URL.
- The Rust CLI packages the new embedded reference but adds no network client,
  OAuth, credentials, GitHub model, or source lifecycle state.

## Consequences

- A GitHub Milestone can be decomposed into existing Specs, new Specs, and
  Direct work without one Spec per Issue or hidden closed work.
- Remote changes cannot silently alter approved scope or canonical artifacts.
- Other remote providers still require their own acquisition decisions.

## Verification

Mechanical tests cover packaged resources, canonical-URL parsing and rejection,
provider selection, fallback
commands, complete pagination, partial-acquisition stops, non-Issue handling,
and the no-requery promotion boundary. Forward-test scenario D15 exercises a
fixture against an authenticated read-only GitHub Milestone where available.
