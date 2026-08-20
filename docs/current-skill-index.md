# Current generated skill index

This page indexes the product-managed skills embedded in the current SpecBind
CLI. `specbind install` renders the same agent-neutral body for each selected
agent while preserving the platform-specific front matter and invocation
syntax.

For the files installed or maintained by these skills, see the
[current generated artifact index](./current-artifact-index.md). For the design
history behind the set, see the
[target skill catalog](https://github.com/Huruikagi/specbind/blob/main/docs/design/target-skill-catalog.md) and
[Decision 0075](https://github.com/Huruikagi/specbind/blob/main/docs/design/decisions/0075-v1-skill-and-orchestration-scope.md).

Both supported agents receive the same 17 skills:

- Claude Code: `.claude/skills/<skill>/SKILL.md`; invoked as `/specbind-*`
- Codex: `.agents/skills/<skill>/SKILL.md`; invoked as `$specbind-*`

| Skill | Current role |
| --- | --- |
| `specbind-discovery` | Confirm milestone scope, classify Direct, existing-Spec, and new-Spec work, delegate state changes to the CLI, and author Briefs. |
| `specbind-requirements` | Maintain the complete current behavioral contract, select the milestone's active Requirement IDs, and approve the Requirements gate. |
| `specbind-gap-analysis` | Compare intended work with the repository and preserve useful milestone-local Research without becoming a gate. |
| `specbind-design` | Investigate the system, maintain the complete current Design and Contract, and approve the Design gate. |
| `specbind-validate-design` | Independently judge Design coverage, boundaries, buildability, self-containment, and architectural fit. |
| `specbind-contract-review` | Review the milestone's complete persistent Contract graph and accept the review required before Tasks authoring. |
| `specbind-tasks` | Author the executable structured task plan, verify schema and Requirement coverage, and approve the Tasks gate. |
| `specbind-implement` | Implement one Spec-backed or Direct Roadmap item using the required dispatched implementation and review cycle. |
| `specbind-review-task` | Judge one implemented Task from its actual diff and approved inputs without applying fixes. |
| `specbind-debug` | Establish and categorize the root cause of a stopped run and return a bounded next action without applying it. |
| `specbind-validate-implementation` | Judge one Spec's implementation against its active Requirement IDs and accept completion evidence only on `GO`. |
| `specbind-verify-completion` | Check an explicit completion claim against fresh evidence without changing lifecycle state. |
| `specbind-release` | Bind the release, execute project release guidance, verify the result, and finalize the complete milestone. |
| `specbind-status` | Explain current Spec, milestone, or task state and the next available action without judging completion. |
| `specbind-steering` | Bootstrap, synchronize, repair, or add durable project guidance. |
| `specbind-quick-plan` | Take one Spec-backed item from Brief through Tasks approval with one bounded delegated-gate authorization. |
| `specbind-batch-plan` | Take every Spec-backed milestone item through Tasks approval while respecting phase dependencies and the global Contract-review barrier. |

There are no `kiro-*` compatibility aliases. Milestone and Spec initialization
are deterministic CLI operations invoked by `specbind-discovery`, not separate
skills.

## Sources of truth

- Agent-neutral skill sources: `tools/specbind/assets/skills/`
- Registry and per-agent rendering: `tools/specbind/src/catalog/skill.rs`
- Installation planning and refresh: `tools/specbind/src/installation/install.rs`
- Mechanical conformance tests: `tools/specbind/tests/skill.rs` and `tools/specbind/tests/cli.rs`
- Behavioral verification ledger: `docs/skill-forward-tests.md`

When a product skill changes, update its one embedded source and the applicable
mechanical and forward tests. Both agent renderings are derived from that source.
