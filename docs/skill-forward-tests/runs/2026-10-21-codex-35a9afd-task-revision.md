# Forward-test run: 2026-10-21 / Codex / Task revision

[Back to the measurement dashboard](../results.md).

- Date: `2026-10-21`
- Driver: Codex
- Model: `gpt-6.1-sol`
- Driver profile: `medium`; canonical CLI attempt, then a fresh app subagent
  with `fork_turns: none` and standalone English fixture instructions
- Tested build: `35a9afd` + Decision 0230 working tree
- Fixture binary SHA256:
  `FC7A3DAF24A1606B28F5EB3A323BC4DFD001452166DAB312DB6E75CE93B98334`
- Fixture language: `en`
- Scenarios: T3

The final source adds detailed rejection diagnostics after this binary was built.
The successful mutation path and installed Skill measured here are unchanged;
final Rust checks cover those diagnostic additions. This is not a forward-test
claim about the final binary or Claude Code behavior. Installed Codex and Claude
Tasks references were byte-identical (SHA256
`894F6B5D06568609A66545100FD32A035A9C52DB958E7B9758C0F35C738717C4`).

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| T3 CLI attempt | environment_blocked | Driver could not start | Initial fixture unchanged; zero tool actions | Harness stderr: `'gpt-6.1-sol' model is not supported when using Codex with a ChatGPT account`; session `01a15cc8-e894-78e3-8970-c2a60b3418be` | ENV-0007 |
| T3 fresh app fixture | pass | none | Four Tasks, two completed and two pending; fresh Tasks gate; implementation state; clean checkpoint `2f51fb3` | Actual calls read installed `sb-plan`, invalidated Tasks, ran revision preflight and schema read, submitted external JSON to dry-run and apply, checked traceability and separately approved Tasks. Independent reads verified execution keys `2` and `3`, new `1` pending, remaining `4` pending, coverage 4/4, fresh review and earlier gates. Checkpoint changed only cart `spec.yaml` and `tasks.yaml`. | FT-0074 is a separate existing readiness observation |

The app driver session is `01a15cc9-a552-7530-8b7e-bcdcccf4073e`.
Its actual tool calls contain `tasks revision preflight cart`,
`schema read task-revision/v1`, and the same external candidate passed to
`tasks revision apply cart --input ... --dry-run` and then apply. The candidate
contains only the replacement plan, fingerprint and two preserve dispositions:
`1 -> 2` and `2 -> 3`. There was no manual Tasks execution write.

## Confirmation turns

1. The initial request was the unchanged T3 request: add a first task moving the
   cap into its own module. The driver asked for a named Spec. Confirmed cart,
   Tasks only.
2. Before mutation, the driver presented all four Tasks and the exact progress
   mapping. It disclosed that no cap implementation existed, so the new task
   would establish the module. Independent status was clean with the original
   three Tasks and two completed records. Approved the Tasks-only rewind and
   that exact mapping, and explicitly prohibited implementation.
3. After CLI apply, independently checked the revised file, execution map and
   traceability. Approved the presented revised Tasks plan and its gate, with
   another explicit stop after Tasks. The driver approved and checkpointed only
   the Tasks phase.

## Debrief dispositions

The fixture was clean before and after the read-only debrief. Observations do
not replace the mechanical T3 judgment.

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| T3 | Naming cart required an extra confirmation | extra-step | discarded | Intended named-Spec scope boundary; initial request omitted the name. |
| T3 | Mapping/rewind approval did not clearly include gate approval | ambiguity | discarded | Confirmation deliberately authorized application only. Separate gate approval is the contract; no duplicate permission for the same mutation occurred. |
| T3 | Tasks 1 and 4 both displayed actionable despite pending first work | wrong-action-risk | retained | FT-0074. Independently reproduced `tasks list cart` and `tasks show cart 4`: Task 4 has only effective prerequisite 3. Existing read-model code checks direct prerequisite completion, while the protocol says order carries all earlier work. |
| T3 | Request implied existing cap code, but source had none | ambiguity | discarded | Fixture/request wording; driver disclosed it and obtained approval for establishing the cap module. No product fact was invented. |

## Cleanup

- Fixture paths: `C:\Users\hurui\AppData\Local\Temp\sb-task-revision-t3-20261021`,
  its adjacent `.drive` directory, and
  `C:\Users\hurui\AppData\Local\Temp\sb-task-revision-t3-app-20261021`.
  Removed after recording judgment and debrief.
- The external candidate `sb-task-revision-t3-candidate.json` was absent from
  the host and sandbox temporary directories checked during cleanup.
- Main worktree contained only the intended Decision 0230 implementation,
  product assets, tests, bilingual documentation and this evidence record.
