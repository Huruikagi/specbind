# Forward-test run: 2026-10-21 / Codex / Contract baseline diff

[Back to the measurement dashboard](../results.md).

- Tested build: `4fb2077` + Decision 0234 working tree
- Driver: Codex app `gpt-6.1-sol` / `medium`, `fork_turns: none`
- Fixture language: `en`; separate standalone projects with their own instructions
- Release and both fixture binary SHA256:
  `68C3284126430068BCE89BF8EF20F45CD93A032C66B174E3727703F7CE4198A2`
- Product Contract Review Skill source SHA256:
  `BFAF848B47DD8C23522478C0130CE5082985882466702923F3475A98986D30EF`

The drivers received the scenario request without naming the Skill or comparison
command. Native PowerShell and Git Bash PATH facts selected the installed fixture
binary, and the prompts isolated the fixture from host repository instructions.
These runs stop at findings before a confirmation or dispatch. They use the
fresh-context app driver allowed for that boundary; they do not establish
canonical headless CLI or Claude Code coverage.

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| X1 | pass | none | Clean original HEAD `ee9966dbb72947e85cbc7456ad0ac3574bcf565d`; cart at tasks, fresh Requirements/Design, review absent, no task plan | Driver reads installed Skill and invokes `contract diff --baseline 38ce59947220c76d2af7b4fe3c47b4153592f617 --json`; unchanged Contract does not suppress the scoped 99-per-SKU omission. Independent status and Git checks confirm no mutation. | none |
| X2 | pass | none | Clean original HEAD `ffe92e5a34e707b64dbef80ab7162359b9f93b32`; review absent; checkout Contract unchanged | Driver reads installed Skill and invokes `contract diff --baseline 0362f3da78e7f20f4f0edebfcac86e42b397a88d --json`; names checkout's `consumes/cart-add` reference to removed `exports/add-item`, and routes any checkout work to a Discovery scope decision. Independent `check contracts` confirms `CONTRACT_GRAPH_TARGET_ENTRY_MISSING`. | none |

Driver sessions are `01a15dd3-54a6-76f3-b03e-2efd94dd1319` (X1) and
`01a15dd3-6d06-75f1-aefc-4d3f307c2363` (X2). Their actual tool traces show
installed-Skill reads, the fixed-baseline diff calls and no historical `git show`
substitution. Both independently read current Contracts and governing protocol,
check the graph and present the Design-owner rewind cost before requesting
confirmation. X1 additionally reads Requirements and Design to establish the
omitted guarantee; the successful structural check is not treated as approval.

## Confirmation turns

No confirmation was supplied. The expected finding boundary is the end of each
measurement. Neither driver invalidated a gate, accepted review, changed scope,
authored Tasks or edited the outside consumer.

## Debrief dispositions

Both drivers answered `none` to the prescribed post-judgment read-only debrief.
Independent before/after `git status --short` remained empty in both fixtures.
No new product finding is inferred.

## Verification and cleanup

Focused CLI tests cover normalized order/formatting, semantic edits, added and
removed Specs, current-only and baseline-only consumers, malformed/missing
required inputs, custom and moved roots, shared path compatibility, empty and
schema-only document changes, and committed link rejection. Full Rust checks,
Decision validation and strict bilingual documentation build accompany the run.

Fixture roots under `tools/specbind/target/ft-contract-diff-x1` and
`tools/specbind/target/ft-contract-diff-x2` were removed after evidence and debrief
checks. Main checkout changes are the Decision 0234 implementation, documentation,
tests and this measurement record.
