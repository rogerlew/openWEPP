# Canonical Execution Evidence

Status: `COMBINED CLOSURE PASS`

Evidence class: `Ran`

Exact subject head:
`2e3d51eca5945a8fb324cf0e23e6371cd04b05d9`.

Authority base:
`4028709f6a18f50a5f3552215df5f3f17f79afe4`.

## Attempt 001

Root: `/home/workdir/gate-testgate-ledger-bootstrap-canonical-001`

The comparator runner interrupted terminal planning before transition. No
LIGHT, audit, receipt, or gate execution occurred. The empty ledger and five
planning evidence files are retained byte-identically. This was an
infrastructure/operator interruption, not a ledger-bootstrap failure.

## Attempt 002

Root: `/home/workdir/gate-testgate-ledger-bootstrap-canonical-002`

Ledger:
`/home/workdir/gate-testgate-ledger-bootstrap-history-002.jsonl`

- package authority chain: `READY`;
- intent plan:
  `e5901dca0e0986152d9a2ac7285cb15121f9f7ccf07f55898172d30930366222`;
- terminal plan:
  `2bf971bee4d6d862222f51e22a61beb2b4f519dd275703eacf6dbee0eaf98145`;
- selected nodes: 12;
- globally unique planned inventory: 2,387;
- LIGHT: `PASS`;
- pre-HEAVY audit:
  `ab6edd2ad7c07ce07097f9b77d8bec93f09b7c6bdabbd267f7d0307a71b3c81b`,
  `READY`, ten checks PASS;
- receipt:
  `30054b51863488b85d23c95a68b8d5ebc8f5d2d9be5b94959dfec4dab194b54f`;
- HEAVY: `FAIL`, 9 passed, 1 failed, 2 dependency-blocked.

The repaired consumer path is proven: Python securely created the fresh
ledger, Rust consumed the inherited descriptor, LIGHT appended a durable PASS
record, audit admitted the exact ledger, and HEAVY wrote balanced
STARTED/CLOSED records.

The sole failing node was workspace Clippy:

```text
cargo clippy --workspace --all-targets -- -D warnings
```

It reported `clippy::too_many_lines` on the pre-existing integration test
`report_source_adoption_is_read_only_deterministic_and_invalidates_review_authority`
in `tests/integration/assurance_v2_amendment_contract.rs`. Doc tests and the
full-workspace Nextest node were dependency-blocked. This defect is outside the
ledger package write set and requires a prospective successor correction
before canonical closure.

## Exact retained hashes

| Artifact | SHA-256 |
|---|---|
| `receipt.json` | `7d1bd119a12b598bec5f061eaaf1414dbcbb7db507e927a42138a50ac4fc9d26` |
| `pre-heavy-audit.json` | `064dbecac245eb76319c4a1714794dac0c5a28f4e4005f79936ee48206d9c579` |
| `light-receipts.json` | `81259b9a331e479f80061b38a201591b27d33777809d43f938141ce87cb5e716` |
| `terminal-plan.json` | `dde2623d3c6f0dddfa38e24d741e48248b0affae8b400b819834c1f175eb94d5` |
| `attempt-index.json` | `ed705dbd2bfe49757736f5bd46e96af861de9f536e487506e04dd8590e00127b` |
| durable ledger / `attempts.jsonl` | `dcf82f416fd3be95ef60cacc0370c90041462b53621d41b34f315bafe605dbaa` |

Successor `ASSURANCE-V2-CLIPPY-LINE-01` corrected the exact function-scoped
Clippy defect without changing test behavior. Its source-contract follow-up
then restored full 2,361/2,361, and its own fresh canonical transaction passed
receipt `29d71a54d2cf38680190885abaf2d2967d547cdedefc0c31af5e00de669aa5d4`,
12/12 nodes, 2,387/2,387 inventory items, ten-check READY audit, and dual
receipt verification. This artifact's Clippy `HEAVY HOLD` is `LIFTED`; dual
terminal and receipt verifiers confirmed all five ledger implementation/test
paths are byte-unchanged into the passing successor subject. The two campaign
identities and verdicts remain distinct; their combined evidence closes the
package without relabelling the original failure. No CAL population, Harvard
calibration workflow, or protected/sealed-state mutation occurred. Required
read-only Harvard fixture coverage in the successor regression is disclosed.
