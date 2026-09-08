# Lifecycle mutation closure

- Date: 2026-09-09
- Driver: Codex, `gpt-5.6-terra`, `medium`, fresh context per scenario
- Tested build: `ad88786361fbf5bb5bc351dac482bd5df494ebfb` plus Decisions 0204-0206 working tree
- Host: Windows; native Git Bash at `C:\Program Files\Git\bin\bash.exe`
- Language: English fixture; maintainer conversation may use Japanese

## Results

| Scenario | Verdict | Failed expectation | Final state | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `VI5-A` | `pass` | none | `cart` is `release_ready`; Requirements, Design, Tasks, Contract Review, and completion are fresh; one Task remains completed; worktree clean | Setup HEAD `fc3518b`; invalidation commit `d1874e4` and acceptance commit `88af37d` each changed only `.specbind/specs/cart/spec.yaml`; `sh scripts/test.sh` passed 4 tests; diff from setup through all non-completion paths was empty | Decision 0205 clarification from debrief |
| `VI5-B` | `pass` | none | Same required final state on the clarified Skill; worktree clean | Setup HEAD `a1f31b5`; invalidation commit `a27b0f8` and acceptance commit `fd5bbdd` each changed only `.specbind/specs/cart/spec.yaml`; canonical `sh scripts/test.sh` passed 4 tests; no ad-hoc runtime command was used; all non-completion paths remained unchanged | none |
| `RR1-A` | `environment_invalid` | Driver resolved a non-fixture `specbind` despite the prompt's PATH instruction and falsely reported that `rule` was unavailable | Lifecycle artifacts unchanged; only dispatch instrumentation appended | Exact fixture binary subsequently returned `NO_CHANGE RULE_ABSENT` for `rule read language-style --for consume` | none |
| `RR1-B` | `product_failure` | Proposed retaining a completed Task whose only Requirement IDs were inactive, so the confirmed replacement could not pass traceability | Requirements and Design fresh; Contract Review fresh; retained Tasks draft dirty and unapproved; no implementation | `TRACEABILITY_TASK_SCOPE_INACTIVE`; the driver stopped without bypassing the check. Decision 0206 now requires an explicit remove/reset/retain disposition against active obligations | none |
| `RR1-C` | `scenario_invalid` | Even with Decision 0206 installed, the second maintainer turn said to preserve the existing plan and completion record without delimiting preservation to the review checkpoint | Requirements and Design fresh; Contract Review fresh; old Task plan still byte-preserved and no Tasks repair applied | Installed Tasks procedure contained the active-obligation disposition rule, but the driver reasonably treated the maintainer's stronger wording as final-retention authority. The continuation now says byte preservation ends at the renewed-review checkpoint and does not require inactive items in the replacement | none |
| `RR1-D` | `environment_blocked` | Host safety rejected the exact user-confirmed replacement that removes old Task 1 and its keyed completion entry | Clean at `49145d7`; Requirements and Design fresh for active `1.5`; Contract Review fresh; old completed plan still byte-preserved; Tasks not reached | Driver proposed the correct mapping: remove old completed Task 1 and execution entry, create new pending Task 1 for `1.5`, inherit no completion. Status reported Design 1/1 and retained Tasks 0/1 with `TRACEABILITY_TASK_SCOPE_INACTIVE`. No edit or workaround followed the host rejection | `ENV-0004` |
| `RR1-E` | `environment_blocked` | Host safety rejected the exact confirmed Requirements invalidation | Original Requirements, Design, Tasks, review, and completed Task remained fresh and unchanged; worktree clean | On the final clarified build, the driver correctly distinguished removed gate evidence/review from retained Design, Contract, Task plan, execution record, and source before asking for confirmation. The host rejected the exact operation after that confirmation, and no workaround ran | `ENV-0004` |

## Notes

VI5 reached the required confirmation boundary without mutation. After exact
confirmation, the same driver created a completion-withdrawal checkpoint before
fresh preflight and a separate accepted-evidence checkpoint. The source change
that made the original evidence stale was preserved unchanged.

RR1-B preserved the old plan through the renewed Contract Review as required,
but its proposed final mapping was not approvable. The dirty failed-attempt
fixture is retained only until the corrected fresh retry is judged.

RR1-D confirmed the corrected semantic route through renewed Contract Review and
the exact active-only Task mapping. The host blocked the final file replacement,
so the final approval/checkpoint remains unmeasured rather than inferred.

RR1-E confirmed the final rewind-cost clarification in a fresh context. Its
earlier host stop means RR1-D remains the deepest measured recovery state.

## Usability debrief

- CLI / extra-step: the driver first tried `specbind status cart`, then corrected
  itself from the installed Skill to `specbind spec status cart`. This was a
  driver slip; the product procedure already supplied the exact command.
- Protocol / ambiguity: the whole-implementation runtime-liveness dimension was
  read as requiring a separate command. With an inactive Validation adapter and
  only canonical `sh scripts/test.sh`, the driver invented a Python smoke
  invocation. Decision 0205 now states that evidence dimensions do not imply one
  command each, allows inspected canonical tests to prove a library's first
  usable operation, and forbids an ad-hoc command solely to fill that slot.

The first fixture remained clean before and after the read-only debrief. VI5-B
confirmed the clarified Skill with only the canonical project command.

- RR1-D Skill / ambiguity: “clears Design, Tasks, and completion evidence” was
  read alongside retained-plan guidance and required an inference that gate
  evidence is removed while artifact bytes and execution records remain. The
  Requirements procedure now states that distinction directly.
- RR1-D Adapter / wrong-action-risk: the driver was concerned about separating
  Requirements and Design checkpoints around the shared `spec.yaml`. Mechanical
  inspection showed the actual commits were correctly bounded: `ebd06ac`
  contained Requirements plus the CLI rewind/review removal, `52635c2` contained
  Design/Contract plus Design approval, and `49145d7` contained only renewed
  review.
- RR1-D CLI / extra-step: the driver first omitted required `deepInputs` from
  the review candidate despite the Skill's schema-read instruction. The retry
  used `deepInputs: []`; no product change was justified.
- RR1-D Other / wrong-action-risk: host safety rejected the exact confirmed Task
  replacement. The driver did not work around it; this is ENV-0004 rather than a
  SpecBind defect.

The RR1-D fixture remained clean before and after its read-only debrief.

- RR1-E CLI / ambiguity: `invalidate --help` summarizes cumulative evidence but
  does not enumerate every artifact effect. The installed owning Skill supplied
  the exact cost correctly, so no CLI contract change was justified.
- RR1-E Skill / wrong-action-risk: the driver noted that prospective permission
  to approve unseen replacement artifacts must still satisfy the named delegated
  workflow rules. The host blocked before this distinction affected a mutation;
  no new evidence justified changing the approval contract.
- RR1-E Other / extra-step: host safety rejected the already confirmed rewind
  and requested more explicit wording. This remains ENV-0004.

The RR1-E fixture remained clean before and after its read-only debrief.
