# Forward-test run: 2026-10-21 / Codex / Bounded Drive owner isolation

[Back to the measurement dashboard](../results.md).

- Tested build: `2036dec` + final FT-0073 clarification tree
- Driver: Codex app `gpt-6.1-sol` / `medium`, `fork_turns: none`
- Scenario: fresh DR9w, unchanged request and expectations
- Fixture language: `en`; standalone instructions and dispatch instrumentation
- Release and fixture binary SHA256:
  `7AB84CC4943E07063ABA86849646054F4DEB254D4B79A5B9ACA86FCEC5F26609`

The [earlier build](./2026-10-21-codex-2036dec-owner-isolation.md) passed DR6
with three distinct no-history owners, but failed DR9w by executing withdrawal
in Drive. This final correction explicitly keeps a withdrawal-only Drive request
in the controller and requires the separate validation owner from both entry
points. It does not change the previously measured ordinary isolation or
receiver-lifetime rules. DR6 was not rerun against this binary; its earlier
measurement and the failed DR9w remain recorded separately. Canonical CLI
ENV-0007 remains an environment limitation; no Claude behavioral pass is claimed.

## Measurement

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| DR9w | pass | none | Clean cart-only withdrawal checkpoint `abfdf2e`; cart-input Task 1 still pending; no implementation or final validation | Actual Drive dispatch creates `withdraw_cart` with `fork_turns: none`; separate owner reads installed validation and uses the same fixed revisions before/after its narrow commit. Independent final check proves cart withdrawn and remaining empty. | FT-0073 withdrawal branch confirmed; together with prior DR6 resolves the finding |

## Independent evidence

- Driver session: `01a15d93-824e-7c60-bc6a-9cbb818fd3fb`.
- Withdrawal owner: `01a15d94-af25-7f72-add9-8e6d7b1a01ac`.
- Actual dispatch at `08:29:47.671Z` uses `fork_turns: none` and an ordinary
  complete Skill owner. No prior receiver is resumed. Two instrumentation
  entries corroborate the separate contexts; the raw call proves isolation.
- Fixed baseline: `faf3fc43fd25cc85a6f80fd2bc7b3c80f9f31f71`.
- Fixed recovery: `252f7d8c01419a0d9f011520e5cfa59e9e8deb0c`.
- Withdrawal checkpoint: `abfdf2e05735a9d4522cdb3b457c524afe4c1059`.

The owner independently ran the fixed-revision check at `08:30:20.435Z`, then
invalidated completion and checkpointed only cart's `spec.yaml`. It reran the
same check after that checkpoint. Drive also verified the terminal result.
Independent judgment against the final fixture returned
`REPLAN_COMPLETION_CHECKED`, `eligible: [cart]`, one matching `withdrawn`
checkpoint, `remaining: []`, and `requiresAuthorityAndDeltaReview: true`.

The exact diff is one file: release_ready becomes implementation and completion
evidence is removed, with earlier gates untouched. Milestone health is
consistent and Contract Review fresh; cart retains its completed Task while
cart-input's single Task remains pending. Git is clean. The bounded request
stops before implementation, tests, final acceptance and Release. This does not
claim full DR9 convergence or grouped final-validation coverage.

## Debrief dispositions

The fixture remained clean before and after the post-judgment read-only debrief.

| Observation | Impact | Disposition | Reason |
| --- | --- | --- | --- |
| Temporary Git snapshot needed escalated execution | extra-step | discarded | Identical fixed-revision check passed with temporary-directory permission; no state cleanup or eligibility bypass. |
| Validation owner requested the actual human instruction rather than only Drive's authority summary | extra-step | discarded | Intended independent authority check. Drive supplied the original wording to the same live owner, without asking the human again or changing workflows. It then completed normally. |

## Verification and cleanup

Final formatting, schema generation check, all-target/all-feature Clippy, all
569 Rust tests, and release build passed. Both rendered agent packages preserve
the isolation and bounded-owner instructions. The documentation and Decision
index checks passed after recording these results.

Removed the dedicated fixtures after judgment and debrief:
`C:\Users\hurui\AppData\Local\Temp\sb-ft0073-dr6-20261021`,
`C:\Users\hurui\AppData\Local\Temp\sb-ft0073-dr9w-20261021`, and
`C:\Users\hurui\AppData\Local\Temp\sb-ft0073-dr9w-final-20261021`.
Session IDs and the initial failed measurement remain in the run records.
