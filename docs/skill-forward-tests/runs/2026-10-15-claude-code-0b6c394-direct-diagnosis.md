# Forward-test run: 2026-10-15 / Claude Code / 0b6c394 Direct diagnosis

[Back to the measurement dashboard](../results.md).

- Date: `2026-10-15`
- Driver: Claude Code, Windows headless real-session harness
- Model: `claude-opus-5-5` (host default; no model override)
- Tested build: final audit working tree committed as `0b6c394`; only two
  unrelated reference-file trailing blank lines were removed after measurement
- Release binary SHA-256: `67682c77c459645050a409ae670ebcc8a921ff1f2d30d78ea84ec89069281c69`
- Fixture language: `en`
- Scenario: `DB2`, fresh fixture and session after debrief-driven clarification

The [initial audit run](./2026-10-15-claude-code-ed3a1e3-skill-audit.md)
identified ambiguous Direct input sourcing and an unsupported shared-ownership
claim. The revised branch names the exact shared Contract read, treats an
absent manifest as a complete answer, separates Spec-only reads, and explains
that done conditions/checks are evidence rather than new Roadmap schema fields.

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| DB2 | `pass` | none | HEAD `523257a` unchanged, worktree clean, Direct item pending, no Spec artifacts created | Natural request selected `sb-debug`; shared Contract read returned `OK SHARED_CONTRACT_ABSENT`; both Steering documents were read. Guide check failed with exit 1 while the documented Python test command ran one passing test. Terminal `IMPLEMENTATION` diagnosis returned to the Direct owner without mutation or invented Spec reads | FT-0066 confirmed |

No approval turn was needed. A child-process-only fixture PATH/executable
binding and UTF-8 harness output used the same isolation as the initial run.

## Debrief dispositions

The same driver returned a read-only debrief with zero tool actions. HEAD and
worktree remained unchanged after it.

| Observation | Impact | Disposition | Reason |
| --- | --- | --- | --- |
| Origin of the failing guide could not be tied to an implementation handoff | ambiguity | discarded | The driver named that uncertainty and independently reproduced the explicitly requested current failure; no invented attempt history or workflow state |
| Raw Roadmap read to locate baseline revision | extra-step | retained in this record | Read-only and unnecessary for the decisive mismatch; not a new lifecycle requirement |
| Python reproduction can create ignored bytecode | wrong-action-risk | discarded | Fixture explicitly ignores bytecode; no tracked or unignored mutation occurred. This does not authorize changing unrelated files |

## Mechanical verification

- Full Rust workspace suite: 533 tests passed on the initial audit tree.
- Final Direct clarification: all 113 Skill, protocol and project-instruction
  tests passed; release build succeeded.
- Rustfmt, generated-schema check and Clippy with warnings denied passed.
- All rendered Agent packages' local Markdown links resolve to distributed
  targets; new references install through the ordinary catalog.
- Both extracted recovery procedures match their original text apart from
  headings/placement and trailing blank lines.
- Codex rendered Skill validation passed for all seven affected packages.
- Decision index and strict MkDocs build passed; English/Japanese guide pages
  link to their respective language counterparts.

## Cleanup

Removed `sb-audit-20261015-db2-2`, its `.drive` directory and `.env.sh` file from
the Windows user temp directory after judgment/debrief. No host settings,
credentials or user profile were changed.
