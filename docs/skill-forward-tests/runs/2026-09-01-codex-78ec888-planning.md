# Forward-test run: 2026-09-01 / Codex / 78ec888

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-01`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`, fresh contexts with no prior turns
- Tested build: `78ec888`
- Fixture language: `ja` for R8, `en` for HP1
- Scenarios: `R8`, `HP1`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `R8` | `pass` | `none` | `order` reached Design with a fresh Requirements gate and clean checkpoint `710fe4f`. | Only `.specbind/specs/order/requirements.md` was authored; IDs `1.1` and `1.2`, natural Japanese, fresh gate, and no scaffold leakage. | `FT-0011`, `FT-0012`, `FT-0013`, `FT-0014` |
| `HP1` | `product_failure` | A legitimate deferred Design finding made `.specbind/deferred.md` dirty, but Plan admitted only Design and Contract paths before validation. | Unapproved Design, Contract, and deferred destination remained dirty; planning stopped before validation. | Dirty-set inspection matched the Design phase outputs and active deferred adapter. | `FT-0015` |

## Confirmation turns

R8 crossed its explicit Requirements approval. HP1 crossed Discovery and delegated planning approval before stopping at the Design handoff.

## Debrief dispositions

R8 reported `none`. HP1's deferred-path stop was retained as `FT-0015` from fixture evidence.

## Cleanup

- Fixture paths removed after the release batch was recorded.
- Main worktree retained only product and forward-test record changes.
