# Current generated skill index

This page indexes the product-managed skills embedded in the current SpecBind
CLI. `specbind install` renders the same agent-neutral body for each selected
agent while preserving the platform-specific front matter and invocation
syntax.

For the files installed or maintained by these skills, see the
[current generated artifact index](./current-artifact-index.md). For the design
history behind the set, see the
[target skill catalog](https://github.com/Huruikagi/specbind/blob/main/docs/design/target-skill-catalog.md) and
[Decision 0176](https://github.com/Huruikagi/specbind/blob/main/docs/design/decisions/0176-skill-namespace-separation.md).

Every supported Agent profile receives the same 15 skills:

- Claude Code: `.claude/skills/<skill>/SKILL.md`; invoked as `/sb-*`
- Codex: `.agents/skills/<skill>/SKILL.md`; invoked as `$sb-*`
- Generic: `.agents/skills/<skill>/SKILL.md`; invocation is defined by the
  compatible host rather than by SpecBind

Selecting both Codex and generic installs each shared `.agents/skills/` target
once.

Codex installations also receive
`.agents/skills/<skill>/agents/openai.yaml`. It presents branded names such as
`SpecBind Plan`, a compact UI description, and an example prompt that names the
exact `$sb-*` identifier. This OpenAI-specific metadata is not installed for
Claude Code or the generic profile. It does not change implicit invocation or
declare tool dependencies. See
[Decision 0183](https://github.com/Huruikagi/specbind/blob/main/docs/design/decisions/0183-codex-skill-interface-metadata.md).

| Skill | Current role |
| --- | --- |
| `sb-configure` | Review and change supported SpecBind project configuration, coordinate the owning workflow, verify the result, and complete authorized aftercare. |
| `sb-discovery` | Confirm milestone scope from a request, explicit local Source Collection, or selected existing implementation; classify durable boundaries, delegate state changes to the CLI, and author provenance-bearing Roadmaps, Briefs, and adoption Research handoffs. |
| `sb-plan` | The only planning entry point: take one named Spec or every Spec-backed milestone item through Tasks approval, or run one explicitly requested Requirements, Design, or Tasks phase for one named Spec. |
| `sb-drive` | Drive the active milestone through safe reachable planning, implementation, and validation work, park branch-local attention, and stop before release execution. |
| `sb-gap-analysis` | Compare intended work with the repository and preserve useful milestone-local Research without becoming a gate. |
| `sb-validate-design` | Independently judge Design coverage, boundaries, buildability, self-containment, and architectural fit. |
| `sb-contract-review` | Review the milestone's complete persistent Contract graph and accept the review required before Tasks authoring. |
| `sb-implement` | Implement one Spec-backed or Direct Roadmap item using the required dispatched implementation and review cycle. |
| `sb-review-task` | Judge one implemented Task from its actual diff and approved inputs without applying fixes. |
| `sb-debug` | Establish and categorize the root cause of a stopped run and return a bounded next action without applying it. |
| `sb-validate-implementation` | Judge one Spec's implementation against its active Requirement IDs and accept completion evidence only on `GO`. |
| `sb-verify-completion` | Check an explicit completion claim against fresh evidence without changing lifecycle state. |
| `sb-release` | Bind the release, execute project release guidance, verify the result, and finalize the complete milestone. |
| `sb-status` | Explain current Spec, milestone, or task state and the next available action without judging completion. |
| `sb-steering` | Bootstrap, synchronize, repair, or add durable project guidance. |

There are no compatibility aliases for earlier `kiro-*`, any former
`specbind-*` product Skill, removed phase-specific `specbind-plan-*`, or
`specbind-adopt-existing` Skill names. Milestone and Spec initialization and
existing-implementation adoption are routed through `sb-discovery`, not
separate skills.

## Sources of truth

- Agent-neutral skill sources: `tools/specbind/assets/skills/`
- Registry and per-agent rendering: `tools/specbind/src/catalog/skill.rs`
- Installation planning and refresh: `tools/specbind/src/installation/install.rs`
- Mechanical conformance tests: `tools/specbind/tests/skill.rs` and `tools/specbind/tests/cli.rs`
- Behavioral verification index: `docs/skill-forward-tests.md`
- Measurement dashboard: `docs/skill-forward-tests/results.md`
- Historical run records: `docs/skill-forward-tests/runs/`
- Findings worklist: `docs/skill-forward-tests/findings.md`

When a product skill changes, update its one embedded package and the applicable
mechanical and forward tests. Each package has one `SKILL.md` entrypoint and may
have directly linked `references/` files for conditional detail. Both agent
renderings are derived from that package.
