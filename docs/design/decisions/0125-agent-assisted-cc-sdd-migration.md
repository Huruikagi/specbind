# 0125: Make agent-assisted cc-sdd migration a supported path

Status: Accepted

The requirement to preserve `.kiro` after final cutover is superseded by
[Decision 0127](./0127-retire-cc-sdd-source-at-final-cutover.md). Planning and
guided authoring remain non-destructive toward the source.

## Context

[Decision 0077](./0077-v1-installation-distribution-and-migration.md) makes
`specbind migrate cc-sdd` a read-only plan by default and permits `--apply`
only for known, unambiguous conversion. That fail-closed boundary is necessary,
but a diagnostic that says only "repair this manually" leaves adopters without
a supported route for the semantic decisions that made automatic conversion
unsafe.

Those decisions include grouping legacy Specs into one active milestone,
normalizing a project-global artifact language, reconstructing Requirement
traceability, creating missing Contracts, and reviewing customized cc-sdd
rules. An agent can investigate and present those choices, but the CLI must not
make them implicitly or accept invented approval and completion evidence.

The repository now publishes its user documentation through MkDocs Material on
GitHub Pages. That gives the product a maintained place for a longer migration
playbook without making the binary fetch or execute remote content.

## Decision

### Two supported migration paths

cc-sdd migration has two supported paths:

- **automatic migration** converts and applies only inputs whose meaning and
  target representation are uniquely established by the CLI;
- **agent-assisted migration** begins when the read-only plan reports a
  semantic decision or unsupported legacy shape, follows the official guide,
  and returns to deterministic CLI validation before legacy workflows are
  retired.

Agent assistance is not permission for best-effort conversion. The migration
remains fail-closed, reviewable, and non-destructive toward the original
`.kiro` tree.

### CLI handoff

When automatic migration cannot proceed, the CLI:

- returns the stable top-level code `MANUAL_MIGRATION_REQUIRED`;
- reports one or more focused migration finding codes with relevant relative
  paths and concise reasons;
- prints the official guide URL selected from the established project
  language, or the language-neutral entry URL when language is mixed or
  unknown;
- states that it changed no files and preserved the original `.kiro` tree; and
- returns a nonzero exit status.

The stable documentation entry points are:

- `https://huruikagi.github.io/specbind/guide/migration/cc-sdd/`
- `https://huruikagi.github.io/specbind/guide/ja/migrate-from-cc-sdd/`
- `https://huruikagi.github.io/specbind/guide/en/migrate-from-cc-sdd/`

The CLI may deep-link to an explicitly assigned heading anchor for a finding.
Finding codes remain the primary lookup key and must still be printed when a
deep link is present.

GitHub Pages is guidance, not a runtime dependency. The CLI never downloads,
parses, trusts, or executes the page. Automatic planning, safety diagnostics,
and `--apply` remain fully offline after the binary is installed.

### Guide contract

The published guide provides both user-facing context and a self-contained
agent playbook. It must:

1. start by running the read-only migration plan and preserving its findings;
2. inspect the named legacy and target files rather than infer their content
   from filenames alone;
3. ask the user only for semantic choices that cannot be established from the
   repository;
4. preserve the cc-sdd source during guided work and use Git as the recovery boundary for its final retirement;
5. use normal SpecBind CLI operations and owning skills for target lifecycle
   state wherever those operations exist;
6. refuse to translate legacy approval flags into SpecBind gate evidence;
7. refuse to invent milestones, release history, Contracts, traceability, or
   completion evidence;
8. rerun deterministic SpecBind validation after guided authoring; and
9. remove only exact known legacy agent assets, and only after the converted
   project is valid and the user confirms the cutover.

The guide may help an agent author semantic artifacts or normalize legacy
source material. It does not waive schema, traceability, lifecycle, Git, path,
or approval guards.

### Rejoining deterministic migration

The migration implementation must define a retry-safe convergence contract.
After guided work, rerunning `specbind migrate cc-sdd` recognizes valid target
artifacts and reports remaining findings without overwriting those artifacts.
`--apply` performs only the remaining planned deterministic writes and exact
legacy-agent cleanup.

Until that recognition exists for a guided action, the guide must stop before
claiming that the migration is complete. A partially converted `.specbind`
tree is not a successful migration merely because an agent authored it.

### Documentation compatibility

The pre-1.0 documentation site publishes the current guide at stable URLs. The
guide states the CLI versions it covers, and migration finding codes and their
meaning remain backward-compatible within the accepted v1 migration contract.
If a later release needs incompatible instructions, SpecBind introduces a
versioned documentation path before changing the existing interpretation.

The Pages workflow builds every pull request that changes the migration guide
and deploys it from `main`. A strict MkDocs build remains the minimum
documentation verification.

## Consequences

- Automatic migration remains small and trustworthy instead of guessing at
  semantic state.
- Users have a supported recovery path for real cc-sdd projects that do not
  match the automatic subset.
- Agent judgment is bounded by published stop conditions and deterministic CLI
  validation.
- The public guide and migration diagnostic codes become a coordinated product
  interface and must change together.
- The migration implementation needs fixtures for both automatic conversion
  and guided-work convergence.

## Implementation status

The GitHub Pages entry pages and bilingual playbook are published. The Preview
CLI provides the read-only inventory, stable semantic findings, language-aware
guide selection, guarded deterministic `--apply`, and the Decision 0126
candidate-acceptance handshake for retry-safe recognition of agent-authored
work. Exact legacy-instruction recognition and mechanical legacy Spec
conversion remain outside the automatic subset.
