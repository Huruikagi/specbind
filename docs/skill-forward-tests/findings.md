# Forward-test findings worklist

[Back to the measurement dashboard](./results.md).

This is the mutable, triaged worklist. Measurements stay in the
[run archive](./runs/). A mechanical scenario pass does not by itself resolve a
usability finding; the exact affected branch must be confirmed in a fresh
fixture.

## Open

| ID | First seen | Scenario | Finding | Evidence | Next confirmation |
| --- | --- | --- | --- | --- | --- |
| FT-0020 | `8db72c0` | DS9 | The one-off supplement assessment appeared only in the later splitting guidance, so Design authoring could create `design/main` and Contract before considering an independently reviewable infrastructure responsibility. | `58e6155` makes the assessment mandatory before any Design or Contract write. | `58e6155`: fresh DS9 proposed `runtime-operations` and stopped with no Design or Contract artifact. |

## Fixed, behavioral confirmation pending

| ID | First seen | Scenario | Finding | Resolution | Next confirmation |
| --- | --- | --- | --- | --- | --- |
| FT-0019 | `086620a` | CF3 | A request to configure future API and infrastructure Design templates can be read as authority to add candidates even when no current Steering or repository fact establishes those independent responsibilities. | `ada3fa9` makes candidate addition fail closed on that missing responsibility and directs the existing-template or Rule path first. | `ada3fa9`: fresh CF3 updated only the main Design template, preserved the candidate set and Rule, and left existing artifacts unchanged. |
| FT-0018 | `163909b` | A1, A2 | After adoption moved under Discovery, ordinary-route reads competed with adoption preflight and the fresh-reader boundary was too easy for the orchestrator to replace with its own inspection. | `630f08e` scopes ordinary project-shape reads and makes both independent evidence lines non-negotiable; `8329421` forbids redundant milestone/Spec reads and permits capacity-limited sequential fresh readers. | Rerun A1 and A2 on `8329421` when the fresh driver can consume the installed Skill and dispatch instrumentation can retain every context line. |
| FT-0015 | `78ec888` | HP1 | A phase-owned deferred Design finding can make the bounded dirty handoff fail before independent validation. | `d68ae66` admits only the exact active deferred-adapter destination alongside Design and Contract, then checkpoints it after READY approval. | Rerun HP1 through Design validation when the host permits fresh phase execution. |

## Resolved after migration

Historical resolved findings through 2026-08-30 remain in the
[legacy ledger](./runs/legacy-through-2026-08-30.md#resolved-usability-findings).

| ID | First seen | Scenario | Finding | Resolution | Behavioral confirmation |
| --- | --- | --- | --- | --- | --- |
| FT-0001 | `1736d0c` | S5 | A Steering read failure did not name the project path searched. | `6df80fc` adds `searched_project_path=.specbind/steering` to unknown and ambiguous selector diagnostics. | `7e6fd42`: fresh unknown and ambiguous branches both named `.specbind/steering`. |
| FT-0002 | `1736d0c` | X1 | Contract Review did not name Design as the owner of `contract.yaml` or provide the exact rewind command. | `7e6fd42` requires the Design owner, full rewind state, and exact invalidation command in the terminal report. | `7e6fd42`: fresh review reported the boundary, then invalidated only after confirmation. |
| FT-0003 | `1736d0c` | X1 | Acceptance used a prefixed deep-input selector without explaining its relation to the selector returned by `artifact list`. | `ae6c562` maps an exact listed selector to its persisted deep-input identity. | `7e6fd42`: accepted review recorded `specs/cart#design/main` and its fingerprint. |
| FT-0004 | `1736d0c` | RT1 | Review's read-only boundary and deferred adapter write had no stated ordering. | Decision 0159 fixes the verdict first under a byte-identical worktree, then permits only the adapter-directed deferred record. | `1a843d9`: fresh APPROVED review wrote only `.specbind/deferred.md` after the verdict. |
| FT-0005 | `1736d0c` | CLI recovery | Unknown nested commands could suggest an unrelated top-level command. | Decision 0159 disables token-only similarity suggestions while retaining help and usage. | `d993293`: unknown `milestone stats` emitted no unrelated suggestion and the driver recovered through supported routing. |
| FT-0006 | `4738ca2` | T1 | The default task rule did not decide whether one behavior needed a separate test task. | `cc37049` defaults tests into the behavior task unless verification is separately reviewable. | `d993293`: fresh T1 authored and approved one Task covering behavior plus automated tests. |
| FT-0007 | `7e6fd42` | S5 | Duplicate Steering recovery could choose a survivor from matching content or a copy-like filename without provenance. | `8aaa198` permits deletion only when Git history proves the path is the newly introduced duplicate. | `8aaa198`: fresh recovery inspected both histories and removed only the later duplicate. |
| FT-0008 | `d993293` | S5 | Steering authoring said not to read milestone state while requiring a completion-safety status preflight. | `7e6fd42` limits status to write-safety and excludes it from content evidence. | `1a843d9`: fresh authoring used status only for safety; independent readers supplied content evidence. |
| FT-0009 | `8aaa198` | RT1 | Review prose required initial status capture, but its command example listed the diff first. | `1a843d9` puts `git status --short` before `git diff` and tests that order. | `1a843d9`: fresh deferred review followed the aligned order and reported no ambiguity. |
| FT-0010 | `8aaa198` | HP1 | After every Task completed, `spec status` still reported implementation while milestone status routed to validation. | `20ca375` derives per-Spec validation when no Task is pending or blocked and tests text plus JSON output. | `20ca375`: at the same completed-Task fixture revision, fresh CLI reads agreed on validation and left the worktree clean. |
| FT-0011 | `388b8bc` | R8 | Requirements authoring can mistake an intentionally abstract before/after boundary for missing information and stop instead of preserving its stated abstraction. | `dfd858e` requires preserving an abstract but observable boundary without invention. | `78ec888`: fresh R8 authored and approved the intended Japanese before/after contract. |
| FT-0012 | `c012d50` | R6, R8 | The new-Spec path can issue a Contract read before Design creates one. | `dfd858e` forbids that read and uses artifact inventory only on the existing-Spec branch. | `78ec888`: fresh R8 completed new-Spec Requirements without the Contract probe. |
| FT-0013 | `dfd858e` | R8 | Requirements can infer the active Roadmap as an unlisted Steering selector. | `42724f6` makes `steering list` the closed set and explicitly excludes the Roadmap. | `78ec888`: fresh R8 read only the listed Steering documents and completed. |
| FT-0014 | `42724f6` | R8 | Requirements can infer a writable path from artifact inventory instead of the configured template target. | `78ec888` requires `template resolve` and the exact `Project path`. | `78ec888`: only `.specbind/specs/order/requirements.md` was written. |
| FT-0016 | `d68ae66` | HP1 | Fresh Plan phases can select another `specbind` when project-local execution context is omitted. | `57ec8f6` carries working directory, executable, version, PATH facts, and forbids fallback. | `3b294db`: the fresh Design receiver named the exact fixture CLI and stopped on environment denial instead of selecting another binary. |
| FT-0017 | `57ec8f6` | HP1 | Existing Requirements preservation can be detected only after approval, after baseline IDs were lost. | `3b294db` combines a pre-approval ledger with CLI baseline-ID rejection and one bounded repair. | `3b294db`: fresh Requirements retained all five baseline IDs, added `1.4`, kept tests out of Requirements, and reached approval with no loss. |

## Active environment limitations

| ID | Limitation | Effect |
| --- | --- | --- |
| ENV-0001 | A Claude Code Agent-tool subagent does not see fixture-installed Skills in its registry. | A fallback that reads the packaged `SKILL.md` can measure the body, but not platform selection or dispatch. A run that instead infers commands is environment-invalid. |
| ENV-0002 | The Claude Code driver appends its own status line after the agent report. | Exact terminal result blocks must be judged before the harness-owned line; the extra line is not Skill output. |
| ENV-0003 | A Claude Code Agent-tool subagent refuses approval relayed by the driving session. | Authoring scenarios that cross approval are environment-blocked at the draft boundary unless consent comes from the user through a valid channel. |
| ENV-0004 | A Codex subagent's host safety layer can reject fixture-required instrumentation, an exact approved lifecycle mutation, or a project-local CLI check relayed by the driving session. | Judge work before the boundary; execute only an explicitly approved exact mutation separately when permitted, and classify a blocked instrumentation or fresh-phase CLI check as environment-blocked without binary fallback. |
| ENV-0005 | A fresh Codex subagent does not receive fixture-installed Skills in its platform registry. | The driver must discover the conventional `.agents/skills/` tree from project instructions; a run that guesses another root or proceeds without reading the installed Skill is environment-invalid. |

Remove an environment row when it no longer affects interpretation. Do not move
it into the product finding lifecycle merely because it recurs.
