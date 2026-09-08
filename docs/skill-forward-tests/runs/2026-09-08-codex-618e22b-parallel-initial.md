# Forward-test run: 2026-09-08 / Codex / 618e22b

[Back to the measurement dashboard](../results.md).

- Driver: Codex subagents, `gpt-5.6-terra` / `medium`, fresh contexts.
- Tested source: `618e22b` product assets (built before the source checkpoint).
- Binary SHA256: `53627B86E270D4E72A660F8C057A9C2306A3AE351F349E203013AF75FEDBDE52`.
- Fixture language: English. DP2's final Japanese report exposed residual host
  language contamination; its checkpoint and review evidence were re-inspected.

## Measurements

| Scenario | Verdict | Evidence | Finding |
| --- | --- | --- | --- |
| DP1 | environment_blocked | Base `59ee0ed`; subtotal and shipping worktrees both at that base, no implementation changes, all three Tasks pending, total waiting for both predecessors. Nested owner dispatch failed with `agent thread limit reached` while this harness also ran DP2. No concurrency or acceptance claim is established. | none |
| DP2 | product_failure | Only one `.forward-test/agents.log` context; no fresh implementer/reviewer/validator dispatch. Selective commits `b35ca65`, `78a2acd`, `94077d5` each include only their Task/module/test; six tests pass. Completion commit `df1dfdb` accepted all three Specs against `94077d5`, but independent review was skipped despite ordinary dispatch remaining available. | FT-0052 |
| Claude Code | environment_blocked | No `claude` executable on this session's PATH. No native Claude scenario ran. | none |

DP2's controlled fixture disables isolated session APIs, explicitly retaining
ordinary sequential role/review capability. The driver interpreted the former
as disabling all agent dispatch. Passing tests and valid CLI progress therefore
do not make this a behavioral pass. The correction explicitly distinguishes
missing worktree APIs from missing ordinary subagents and preserves the existing
main-context compatibility route only for hosts genuinely lacking dispatch.

## Setup and confirmation

The first DP2 fixture at `sb-dp2-0201` failed during setup because this new recipe
used `ref` instead of structured `target` in Contract consumes entries. No driver
ran against it. The recipe was corrected and a fresh `sb-dp2-0201-b` prepared.
A DP1 status check was first invoked from the source repository by mistake;
rerunning it from the fixture confirmed exactly subtotal/shipping actionable,
with total waiting on both. Neither was a product failure.

No extra approval turn, version binding or release invocation occurred.
DP1's wrapper spawned an additional driver and combined with concurrent DP2
exhausted this four-slot harness; do not infer a host-wide absence of worktree
support from that measurement.

## Debrief dispositions

Debriefs followed mechanical judgment. Fixture Git status was clean before and
after the read-only reflections.

| Observation | Disposition |
| --- | --- |
| DP2 conflated isolated workers and ordinary role dispatch, then self-validated. | FT-0052; strengthen the fallback boundary and rerun on a fresh fixture. |
| DP2 reported in Japanese and appended instrumentation after initial inspection. | Recorded measurement limitations; not separate product findings. |
| DP2 wondered whether total was in scope. | Discarded: the explicit milestone-wide continuation and authoritative dependencies include total. |
| DP1 first tried a host repository Skill path, then used the fixture-local package. | Recorded environment limitation; no implementation outcome measured. |

## Cleanup

Fixtures were created below `C:/Users/hurui/AppData/Local/Temp/` with prefixes
`sb-dp1-0201`, `sb-dp2-0201`, and `sb-dp2-0201-b`; remove only these attributed
measurement directories after recording evidence. DP1's two empty linked
worktrees must be removed through Git before removing its fixture root.

Cleanup completed: all attributed fixture directories were removed after evidence collection; the two empty DP1 linked worktrees were removed through Git first.
