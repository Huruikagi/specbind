# Forward-test run: 2026-09-03 / Claude Code / 9492a83

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-03`
- Driver: `Claude Code`
- Model: `claude-opus-5`
- Driver profile: Agent-tool subagent, fresh context, no inherited turns
- Tested build: `9492a83`
- Fixture language: `en`
- Scenarios: `A1`, `X2`, `VD1`, `VI4`, `RT2`

Scenarios were selected to avoid a guarded approval, because ENV-0003 blocks a
Claude Code Agent-tool subagent at every authoring boundary. Each driver was
given only the standalone fixture path, the `export PATH=` fact, the statement
that instructions from any other repository do not apply — naming the response
language and the commit/push rules — and the scenario's verbatim request. No
prompt named a Skill, command, verdict, or expectation.

ENV-0001 recurred in all five runs: `Skill(sb-*)` returned `Unknown skill` while
the fixture's `.claude/skills/` tree was present on disk, and every driver fell
back to reading `SKILL.md` directly. These five measure the Skill body. They do
**not** measure platform Skill selection or dispatch.

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `A1` | `pass` | none | `HEAD` `c3753f5` unchanged, worktree clean, no `.specbind/adoption/`, `steering list` 0, `spec list` 0, no milestone | `adoption preflight` was the first product command and returned `ERROR ADOPTION_STEERING_REQUIRED`; the run stopped and routed to Steering bootstrap without a deep scan, dossier, milestone, or Spec; `milestone status` → `NO_ACTIVE_MILESTONE` | FT-0034, FT-0035 |
| `X2` | `pass` | none | `HEAD` `3d4b348` unchanged, worktree clean, no `.specbind/state/`, `checkout/contract.yaml` md5 `522e19af…` unchanged | `milestone review status` still `absent` with `CONTRACT_REVIEW_MISSING`; the run named `checkout` as the affected consumer of the removed `cart/exports/add-item` and returned the removal to the maintainer as a scope question instead of editing the non-participant's contract | FT-0031, FT-0032 |
| `VD1` | `pass` | none | `HEAD` `cbfd5fc` unchanged, `design.md` md5 `1912bf47…` and `research.md` md5 `cdb56647…` unchanged, `design=fresh` retained, contract review still `absent`; only `.specbind/deferred.md` created | Verdict `NOT_READY` whose leading blocking finding names the deferral itself — the cap value and the reject-rather-than-trim decision exist only in `research.md`, which is excluded from gate fingerprints and deleted at finalization — not the wording; the only write is the active deferred-findings adapter's destination, after the verdict | FT-0030 |
| `VI4` | `pass` | none | `HEAD` `decc95d` unchanged, worktree clean, `cart` still `implementation`, `completion=not_reached`, no completion evidence file | `adapter read validation` was read and `sh scripts/validation-audit.sh` entered the required set; the command failed because `scripts/` holds only `test.sh`; verdict `MANUAL_VERIFY_REQUIRED`, not `GO` and not `NO-GO`; the passing canonical suite was not substituted and no `mechanical_checks` entry claims the unavailable command | FT-0033 |
| `RT2` | `pass` | none | `src/cart.py` md5 `643c46ca…` and `src/orders.py` md5 `2bddf7d0…` unchanged, `HEAD` `2874c75`, no `.specbind/deferred.md` | The run reviewed the task's own `src/cart.py` change to `REJECTED` and stated explicitly that the `src/orders.py` edit was excluded, citing contract `file_ownership` and the steering capability boundary; the verdict does not silently cover both changes, and no gate or task-state command was run | none |

## Confirmation turns

None. All five scenarios stop before a guarded transition by design, which is
why they were selected for this driver. No approval was relayed, so ENV-0003 was
not reached.

## Debrief dispositions

Each fixture was read before and after the debrief and was unchanged in all five
cases, including `VD1`, whose `.specbind/deferred.md` kept md5 `0b4056e4…`
across the debrief.

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| all | `Skill(sb-*)` returned `Unknown skill` while the fixture Skill tree was on disk, so every driver read `SKILL.md` by hand although project instructions say to select Skills through the platform. | wrong-action-risk | discarded | ENV-0001, already an active environment limitation. Not a product defect. |
| `A1` | `reverse.md` says missing Steering "stops and routes to `sb-steering`; do not repair it in this run" without saying that `sb-steering` itself stops for maintainer decisions, so the safe answer was reachable only by reading a second Skill body. | wrong-action-risk | retained | FT-0034 |
| `A1` | `reverse.md` requires the maintainer's selected area and `baseline_version` above the read-only `adoption preflight` that gates the whole run, so a literal reading asks a question the maintainer cannot yet act on. | extra-step | retained | FT-0035; same prose/command ordering shape as FT-0009. |
| `A1` | The legacy adoption-record recovery block surrounds the reverse route, so a first reverse run reads past compatibility material. | cosmetic | discarded | The disambiguating sentence is present and the driver selected `reverse.md` correctly. |
| `A1` | `CLAUDE.md` and `AGENTS.md` are byte-identical and both were read. | cosmetic | discarded | Fixture installs for both agents by design. |
| `X2` | `spec status` and `milestone status` report `State health: consistent` and `Diagnostics: none` while `check contracts` fails on an unresolvable graph. | wrong-action-risk | retained | FT-0032 |
| `X2` | The Skill requires presenting the exact state a rewind removes before confirmation, but no command reports it and `spec design invalidate` has no preview. | wrong-action-risk | retained | FT-0031 |
| `X2` | `contract consumers` and `contract dependencies` hard-fail on an incomplete graph, so the tool that answers "who depends on this export" is unusable in the case the review exists for. | extra-step | retained in this record | Refusing to report a partial graph may be deliberate; check the owning decision before giving this a finding identity. |
| `X2` | The protocol covers a removed export and an added export separately, never both ends of one seam moving since the baseline. | wrong-action-risk | retained in this record | One observation, not reproduced against the owning decision. Watch for recurrence. |
| `X2` | A non-participant Spec whose own artifacts moved outside any milestone has no stated disposition. | ambiguity | retained in this record | Needs reproduction before a finding identity. |
| `X2` | "Report in the project's language" does not name `.specbind.json` as where the language is recorded. | cosmetic | discarded | The driver resolved it on the first read. |
| `VD1` | The installed routing block never names `sb-validate-design`, while naming `sb-plan` as the default entry point when the user asks about Design for one named Spec — and `sb-plan` owns gate invalidation, which this request must not trigger. | wrong-action-risk | retained | FT-0030 |
| `VD1` | `check traceability` prints `Requirements: 6` against two `### Requirement N` headings, and it is the count the Skill fixes review scope from. | ambiguity | retained in this record | Concrete, but the unit may be an accepted contract; check the owning decision before a finding identity. |
| `VD1` | `contract-principles` assigns dispositions to ownership overlap and dependency cycles but not to `CONTRACT_GRAPH_EXPORT_UNCONSUMED`, while stating that silence never authorizes inventing one. | ambiguity | retained in this record | Needs reproduction against the rule's owning decision. |
| `VD1` | No surface reports whether the `specbind-researcher` role is registered, while the fallback rule makes guessing wrong a configuration failure. | wrong-action-risk | retained in this record | Latent: avoided only because the fixture codebase is trivially small. Reproduce on a fixture large enough to require dispatch. |
| `VD1` | The deferred adapter specifies exact bytes for file creation but only prose for entry format. | cosmetic | discarded | Two reviews formatting differently is not yet a demonstrated defect. |
| `VI4` | `spec completion preflight` reports `READY` with `Diagnostics: none` although the active Validation adapter names a command the project does not contain. | wrong-action-risk | retained | FT-0033 |
| `VI4` | The clean-invocation rule is written against `git status --short`, which cannot see gitignored generated bytecode. | ambiguity | discarded | `running.md` already states that the fixture's ordinary Python bytecode ignores are not a product finding. |
| `VI4` | `specbind tasks status` reported an unrecognized subcommand without enumerating valid ones. | cosmetic | discarded | Decision 0159 deliberately disabled similarity suggestions; help and usage were emitted and the driver recovered. |
| `RT2` | The Skill reads as near-mandatory `CANNOT_REVIEW` for unrelated working-tree changes, while contract `file_ownership` made the stray hunk objectively separable. | wrong-action-risk | discarded | The scenario deliberately admits both outcomes, so the two readings are contract-accepted rather than a defect. |
| `RT2` | An out-of-scope but real edit fits none of the three dispositions: the deferred adapter excludes it, `RESOLVED` means "needs no work", and the protocol states there is no fourth state. | ambiguity | retained in this record | Concrete, single occurrence. Reproduce against the task-review protocol before a finding identity. |
| `RT2` | The task record carried requirement IDs only, so the completion standard was assembled from three other artifacts. | extra-step | retained in this record | May be the intended minimal task record; check the owning decision. |

## Cleanup

- Fixture paths removed: `/tmp/sb-a1`, `/tmp/sb-x2`, `/tmp/sb-vd1`, `/tmp/sb-vi4`, `/tmp/sb-rt2`
- Main worktree after recording: only this run record, the dashboard projection, and the findings worklist were modified
