# Forward-test run: 2026-09-03 / Codex / d59eae8

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-03`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`, fresh context, `fork_turns: none`
- Tested build: `d59eae8`
- Fixture language: `en`
- Scenarios: `D16`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `D16` (first attempt) | `environment_invalid` | The driver resolved an older global `specbind`, so the new Contract projection was not measured. | No active milestone; `cart` idle; `src/cart.py` modified directly | Driver received `unrecognized subcommand 'contract'`; the fixture-local command separately returned `CONTRACT_OWNERS_REPORTED` for `specs/cart#contract/file-ownership/cart-module` | `ENV-0005` context; invalid PATH application |
| `D16` (fresh retry) | `product_failure` | The driver found the owned `cart` boundary but treated the imperative named-file request as implementation authority, bypassing Discovery confirmation. | No active milestone; `cart` idle with every Gate `not_reached`; only `src/cart.py` modified | Fixture-local CLI provenance was proved before launch; debrief recorded `specbind milestone status`, `specbind contract owners src/cart.py`, and the cart Requirements read; fixture diff added the quantity guard without a Milestone or Brief | `FT-0042` |

## Confirmation turns

None. Both attempts edited source before presenting an approvable Discovery
scope. The PATH-invalid attempt was rebuilt at a new path before retrying.

## Debrief dispositions

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `D16` (first attempt) | The driver selected an old global CLI that lacked both `contract` and `rule`. | ambiguity | discarded | Environment-invalid attempt; the fixture-local binary accepted both command families. |
| `D16` (fresh retry) | A precise source-edit request was treated as sufficient authority after ownership and Requirements were read. | wrong-action-risk | retained | `FT-0042` |
| `D16` (fresh retry) | The driver guessed nonexistent `contract show` before reading the artifact directly. | extra-step | discarded | One-off command recovery; `contract --help` exposed the supported surface. |

## Cleanup

- Fixture paths removed: `/tmp/sb-d16-d59eae8`, `/tmp/sb-d16-d59eae8-2`
- Main worktree after recording: checked separately before the record commit
