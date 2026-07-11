# Target Selection — CQR Nightly Batch 01

Ran: delegated fresh baseline on 2026-07-11.

- LCOV: `/tmp/openwepp-cqr-nightly-20260711.lcov`, `4342427` bytes,
  SHA-256 `2bdf21b533fd22c82130d806b9e216e2c386b36ab4745f4b63b5ae1678859e6f`.
- CRAP JSON: `/tmp/openwepp-cqr-nightly-20260711-crap.json`, `2844560`
  bytes, SHA-256
  `2bfc23ac1c6c759c281555670645e9fba1dbcb5995e73b56d89ff87c17ccd4e1`.
- Ranking: `/tmp/openwepp-cqr-nightly-20260711-module-rank.tsv`, SHA-256
  `b22fb04d947af3b889cb319c027e55f9333de887bcec82f02e73fe59536de883`.

Rows de-duplicate `(file,function,line)` and rank by summed excess above `30`,
unique high-function count, then maximum CRAP.

| Rank | Module | Excess | Functions >30 | Max |
|---:|---|---:|---:|---:|
| 1 | `crates/openwepp-runner/src/totalwatsed3.rs` | 237.469 | 4 | 110.000 |
| 2 | `crates/openwepp-runner/src/watershed_wat.rs` | 148.000 | 3 | 110.000 |
| 3 | `crates/openwepp-input-contract/src/parsers/watershed_channel.rs` | 146.671 | 3 | 132.000 |
| 4 | `crates/openwepp-kernel-contract/src/lib_mod/writeback.rs` | 140.000 | 4 | 90.000 |
| 5 | `crates/openwepp-input-contract/src/parsers/pmetpara.rs` | 126.000 | 1 | 156.000 |
| 6 | `crates/openwepp-input-contract/src/parsers/irrigation_fixeddate.rs` | 125.909 | 2 | 132.000 |
| 7 | `crates/openwepp-input-contract/src/parsers/chaninp.rs` | 111.254 | 2 | 132.000 |
| 8 | `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs` | 106.668 | 2 | 132.000 |

Excluded above rank 1: `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
(267.974 excess, six functions, max 191.186) is the unresolved exact target of
`20260709-cqr-nightly-b02-04-runner-watershed-cli-001`, which held for a
dedicated ADR-0021 testability/characterization architecture. Re-selecting it
before that prerequisite would repeat the same legitimate local hold.

Baseline warning: the instrumented `laned_shadow_h2637` target had its known
four fail-closed failures; `--ignore-run-fail` allowed LCOV emission and the
top-level coverage command exited `0`.
