# Forward-test run: 2026-10-17 / Codex / Drive default reads

[Back to the dashboard](../results.md).

- Build: `4c8bbd5` plus Decision 0228 and the FT-0069 invocation-boundary correction
- Driver: real CLI `gpt-5.6-terra` / `medium`, explicit compatibility comparison
- Fixture language: English, with explicit standalone instruction-language hygiene
- Binary SHA-256: `6ba917af4e45f79a37c40fab07af3188c94dceab6c46decc021cb88a4ed81c79`
- Session: `01a149a8-0b66-7d23-9f42-94ed29d3430e`

## Measurement

| Scenario | Verdict | Expectation that did not hold | State | Evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| DR13 | `fail` | Required validated configuration read was omitted | HEAD, status and all 139 tracked/unignored file hashes unchanged | All eleven decisions match, including case 9's new bounded selection with three retained cycles. Three read actions inspect raw files but never run `configuration show`. | FT-0069 behavior corrected; FT-0070 opened |

The read-only request prohibited tests, mutations and dispatch, not read commands.
The debrief nevertheless described it as prohibiting commands. Product wording
now requires `configuration show` even for an explanation of actual project
defaults, while preserving the read-only boundary. The next fresh run uses the
same scenario prompt; no expected answer or command hint was added.

## Debrief dispositions

The same-session debrief made zero tool calls and left the fixture unchanged.

| Observation | Impact | Disposition | Reason |
| --- | --- | --- | --- |
| Distinguishing four invocation/continuation paths took careful reading | ambiguity | discarded | All eleven decisions, including the previously failing branch, now match |
| Treated the request as forbidding validation commands | wrong-action-risk | retained | The observed omission violates the validated-read expectation; FT-0070 |
| Checked cart's Roadmap presence | extra-step | discarded | Read-only confirmation of current subject, no invented eligibility |
