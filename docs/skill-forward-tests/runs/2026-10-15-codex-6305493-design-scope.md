# Forward-test run: 2026-10-15 / Codex / initial Design scope

[Back to the measurement dashboard](../results.md).

- Build: `6305493` plus initial Decision 0223 working tree
- Binary SHA-256: `f1498712ee8c6e4721afaccf9a1574ddd883e2356076c5e23a57498eb333a2af`
- Driver: Windows Codex real-session harness, `en` fixtures
- Model: `gpt-5.6-terra`, `medium` (host default, confirmed from turn contexts)
- Requests: `Check the cart design before we build on it.`

| Scenario | Verdict | Expectation that did not hold | Fixture state | Mechanical evidence |
| --- | --- | --- | --- | --- |
| VD4 | `scenario_invalid` | The supposedly complete fixture lacked the required main-Design verification strategy. The final report also omitted the local/global qualification; researcher startup was environment-blocked | Clean at `64b47d2`; no artifact, approval or review changed | Installed `sb-validate-design` read; scoped checks passed; semantic review reached and `READY` reported, but no complete-scenario pass is credited |
| VD5 | `pass` | none for the structural-stop/read-only boundary | Clean at `3b0ac77`; Design remained approved/fresh, review absent, wishlist Contract absent | Installed Skill selected; `DESIGN_CONTRACTS_FAILED` with `CONTRACT_GRAPH_TARGET_CONTRACT_UNAVAILABLE`; final `NOT_READY` names the required provider and makes no mutation |

VD5 unnecessarily attempted architecture research after the decisive structural
failure. The configured `gpt-6-luna` researcher could not start (ENV-0007).
This does not invalidate the directly observed structural rejection, but it is
not evidence that independent architecture review completed.

The harness action digest lists shell actions but omits collaboration calls.
The source session logs for threads `01a13fd6-2429-7603-b578-b3b4152d5315`
and `01a13fd6-23c0-7671-af8e-37bb0fc21714` confirm the named researcher
dispatch and provider rejection. Do not infer successful dispatch from the
reported five or fifteen shell actions.

## Debrief dispositions

Both same-session debriefs used zero tools; HEAD and worktree stayed unchanged.

| Observation | Disposition |
| --- | --- |
| VD4 missing verification strategy | Confirmed against the required `design/main` template rule. Corrected the recipe to own the future unittest path, command and boundary cases; retained this invalid attempt and rebuilt at a new path |
| VD4 local `READY` lacked the global-review qualification | Added an explicit final-report requirement; checked in the fresh follow-up |
| VD4 manually inspected code after configured researcher failed | Retained as FT-0067, with fresh recurrence in the follow-up; no full independent-review pass credited |
| VD5 extra milestone inventory and architectural investigation after a structural failure | Retained here as unnecessary read-only work; it did not change the blocking verdict or state |

No product or fixture files were repaired during either measured run. The
updated recipe and Skill were measured separately in the
[follow-up](./2026-10-15-codex-4022a4f-design-scope.md). Removed the two isolated
fixtures and their harness-state directories after judgment and debrief.
