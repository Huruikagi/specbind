# Forward-test run: 2026-09-10 / Codex / fd2b28f + shared implementation

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-10`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`, no inherited conversation
- Tested build: `fd2b28f` + working tree implementing Decision 0209
- Binary SHA-256: `5574e8ae1ce429ee019c8e4a1c5c0ff0b8afbfa6c58bdeb8ebda98b44a11b34e`
- Fixture language: `en`
- Scenarios: `SH1`, `SH2`

This build precedes the final owners wording/independent shared-read refinement
and reverse-adoption integration. Those later changes are not attributed to
these measurements.

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| SH1 | pass | none for the artifact/lifecycle expectations | Shared agreement and review checkpointed; Direct pending; catalogs unchanged; only original idle cart Spec | `milestone status --json`: review fresh, Direct 0/1, implementation next; owners returned `shared-contract#resources/translations`; `git diff HEAD~2 HEAD --stat`: only shared Contract and review; clean tree at `0564178` | none |
| SH2 | pass | none for the artifact/lifecycle expectations | English typo corrected and Direct completed; Japanese catalog/shared agreement unchanged | status: review not_applicable, Direct 1/1, release_pending with RELEASE_VERSION_UNBOUND; diff only English catalog and Roadmap status; clean tree at `95a289c` | none |

## Confirmation turns

SH1 first stopped on the unchanged cart export's unconsumed warning. The
maintainer explicitly retained it as a forward-looking boundary. The driver
then presented its complete shared assessment and `deepInputs: []`; approval
authorized acceptance and its checkpoint only, retaining the stop before
catalog implementation or Direct completion. SH2 needed no additional product
scope or review-acceptance confirmation.

## Debrief dispositions

Both fixture worktrees were clean before and after the no-tools debrief.

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| SH1 | Extra non-selected Plan procedures and early review reads | extra-step | discarded | Installed scope instructions already select the shared procedure; no wrong artifacts or lifecycle mutation; exploratory driver observation only |
| SH1 | Existing unconsumed cart export needed a semantic disposition | ambiguity | none | Correct Contract Review boundary; the driver did not invent an external consumer |
| SH2 | Short SHA rejected at Direct completion | wrong-action-risk | none | CLI rejected it and the driver used preflight's exact revision; installed procedure already requires the full revision |
| SH2 | Broad installed-Skill directory listing | extra-step | discarded | No reproduced product ambiguity |
| Both | Git Bash signal-pipe failure in sandbox | extra-step | none | Scoped escalation allowed completion; no result claimed for a blocked operation |
| Both | Japanese narration inherited from the outer repository despite standalone instructions | ambiguity | discarded for language claims | Neither run measures language behavior. Checkpoint results were re-read against each fixture's installed procedures and active Git adapter; both drivers attributed commits solely to those local instructions |

SH2's recipe initially used an incorrect review command and then the internal
`not_required` token. Both preparation attempts failed before any driver ran.
The corrected recipe asserts the public `milestone review status` output
`not_applicable`, and the fixture was rebuilt before measurement.

## Cleanup

The two dedicated fixtures under `tools/specbind/target/forward-sh1-0209` and
`forward-sh2-0209` are removed after recording. Only the intended Decision 0209
implementation, documentation, and test changes remain in the main worktree.
