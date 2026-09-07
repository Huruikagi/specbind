# Forward-test run: 2026-09-07 / Codex / e1cd8a4

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-07`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`
- Tested build: `e1cd8a4` + final Issue #46 working tree, including the explicit
  requirement to preserve stronger approved Design verification
- Binary SHA256: `18015f8b051efeebea70bfb3a0f0d43c78302628f4298acb16fed11755fb28b5`
- Fixture language: `en`
- Scenarios: `T6`, `T7`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| T6 | `pass` | none | One draft Task; cart and cart-input remain in tasks; no gate or upstream changes. | The driver explicitly reported cart's required real JSON proof versus cart-input's later Roadmap position and routed the mismatch to Design. `spec status` showed Tasks not reached and fresh upstream gates; review remained fresh; Git showed only untracked cart Tasks. Both JSON implementation and cart-input Tasks remained absent. | FT-0049 resolved |
| T7 | `pass` | none | One approved Task, cart in implementation, cart-input still in tasks without a plan, clean worktree. | `tasks list cart`: 1 pending/actionable; `check traceability cart`: 4/4. The plan uses direct cart tests with 98 plus 2 and unchanged-state rejection. Checkpoint `adbcec4` changed only cart Tasks and its explicit approval metadata from setup `7909d28`; no upstream or implementation diff. | none |

## Confirmation turns

T6 stopped before approval and named the Design correction; no permission to
change upstream artifacts was supplied. T7 first presented a validated draft,
then received approval of that exact cart plan for Tasks only, with explicit
instructions to stop before implementation or other planning phases. Its local
checkpoint followed the fixture's active Git adapter.

## Debrief dispositions

Before/after read-only checks preserved T6's sole untracked Tasks draft and T7's
clean worktree.

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| T6 | Approved Design required a real connection scheduled after cart completion. | wrong-action-risk | discarded | Deliberately seeded prerequisite conflict; the final workflow reported it and stopped correctly. |
| T6 | The driver first invoked bare `sh` despite the supplied native Git Bash path. | extra-step | discarded | Driver environment usage; retrying the supplied executable passed without fixture mutation. |
| T7 | The driver read the raw Roadmap before loading the Tasks procedure. | extra-step | discarded | It read execution order, not product Steering; no scope or authority changed. The procedure already names the targeted scope projection. |
| T7 | The driver first used bare `sh` before retrying the supplied executable. | extra-step | discarded | Same environment usage as T6; the canonical test ran successfully with the provided path. |

The driver summaries used Japanese despite the English fixture; artifacts
remained English. Git checkpoint authority and scope were independently checked
against the fixture's adapter, not inferred from the host repository's policy.
These scenarios make no language-conformance claim.

## Other verification

- Rust formatting, generated-schema check, Clippy with warnings denied,
  all 443 Rust tests, and workspace release build passed.
- Inherited TypeScript: all 164 tests and build passed.
- Decision index check, strict bilingual MkDocs build, and shell syntax check
  passed; both generated guide pages link to their language counterpart.
- The installed Codex and Claude Code Tasks resources contain the same audit.

## Cleanup

- Fixtures removed: `tools/specbind/target/forward-tests/issue46-t6-final` and
  `issue46-t7-final`, together with the initial run's three fixtures.
- Main worktree after recording: only Issue #46 implementation and verification
  files, subsequently committed together.
