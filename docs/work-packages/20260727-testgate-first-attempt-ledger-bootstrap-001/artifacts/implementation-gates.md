# Implementation Gate Evidence

Exact corrected implementation commit:
`bd64bc1b8adcd9dd2db789e7770d2126f5f5bfc6`.

Evidence class: `Static + Ran`

## Static

- The implementation diff is confined to the five prospectively authorized
  Python/Rust files.
- Python creates and guards the selected ledger without following links, then
  passes exactly that descriptor through `pass_fds`.
- Rust retains ordinary no-follow path admission, matches device/inode, and
  uses one owned bound handle for all transition ledger reads and appends.
- The original lexical path remains the audit-hash and recovery authority.
- Existing non-transition pathname APIs and ledger record schema remain
  unchanged.
- No `unsafe` was introduced.

## Ran

| Gate | Result |
|---|---|
| `.venv/bin/python -m unittest tests.python.test_testgate` | PASS, 41/41 |
| `.venv/bin/python -m py_compile tools/local_ci/testgate.py tests/python/test_testgate.py` | PASS |
| `cargo nextest run -p openwepp-gate-planner bound_ledger_` | PASS, 7/7; run `c246f844-5200-40b3-9e6e-66f734b89758` |
| inherited-descriptor transition test | PASS, 1/1; run `0ec1ef9c-2a1e-4255-ab30-4b75dd98bd04` |
| `cargo nextest run -p openwepp-gate-planner` | PASS, 236/236; independent reviewer |
| `cargo nextest run --test auth11_required_suite_obligation_guards_contract` | PASS, 3/3; run `764e5467-9336-40ad-a7f5-9a3d84b5717e` |
| `bash tools/release/check_authority_suite_antievasion.sh` | PASS |
| `cargo clippy -p openwepp-gate-planner --all-targets -- -D warnings` | PASS |
| `cargo fmt --all -- --check` | PASS |
| package Markdown lint and `git diff --check` | PASS |

Both immutable failed-root evidence baselines remain byte-identical.

## Closure Successor

The package-owned canonical transaction reached the exact ledger consumer,
LIGHT PASS, and ten-check READY audit, then failed only on the unrelated
assurance integration-test Clippy warning recorded in `canonical-execution.md`.
That external defect and its stale-source follow-up are now closed.

Fresh successor campaign `ASSURANCE-V2-CLIPPY-LINE-01` at exact head
`ffe1dd71eec578a621f66fc2939304971653e92b` passes all 12 nodes,
2,387/2,387 inventory items, and full 2,361/2,361. Terminal and receipt
verifiers must explicitly verify the combined evidence: the package-owned
receipt proves the corrected ledger path, while the successor receipt proves
the unrelated failing/blocked workspace nodes now pass. Neither receipt may be
relabelled as the other campaign.

## Closure

Dual terminal and receipt verifiers pass the combined closure evidence with no
finding. The five ledger implementation/test paths remain byte-identical from
the corrected package subject through the passing successor subject.

Harvard remained sealed and CAL population remained prohibited.
