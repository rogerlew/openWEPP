# CAL-07C Terminal Verification B

Evidence class: `Static`

Read-only verification of the terminal CAL-07C record. No validation command was
rerun by this verifier.

## Verification result

`PASS / VERIFIED` for the final bounded-execution disposition. `Order 7 hold retained`.

## Required fact checks

| Check | Static verification |
| --- | --- |
| Full-period hourly metadata | Source/package/gate evidence bind the Alerce POWER hourly LST object for `2022-01-01` through `2026-07-24`, with 39,984 `T2M` and 39,984 `T2MDEW` hourly keys and 1,666 complete 24-hour days from `2022010100` through `2026072423`. |
| Negative hourly retained | Admission-table inspection sums `negative_hourly_count` to `349`; package, science summary, gate evidence, roadmap, and catalog all preserve the same no-clipping claim ceiling. |
| Negative admitted daily | Admission-table inspection found `0 negative admitted daily` rows, `0` failed daily admissions, and 1,666/1,666 finite nonnegative admitted daily means. |
| Rejected dates not consumed | `2022-07-22`, `2022-09-15`, and `2025-09-09` remain negative under the rejected daily-summary contract operand but are consumed only as positive admitted hourly-mean VPD rows in CAL-07C. |
| Executor VPD residual | Executor-path proof and gate evidence report exact VPD residual `0 Pa` (`0.000e+00 Pa`) between admitted forcing and package-local executor output. |
| No production OBL replacement | Package, executor-path proof, science summary, final disposition, roadmap, and catalog all state CAL-07C is package-local/bounded and does not replace `SC-PLANT-001` OBL-PLANT-P-013 or modify production Rust. |
| Roadmap/catalog status | Roadmap and catalog truthfully say CAL-07C lifts only the Alerce forcing blocker for bounded execution; Order 7 remains held for transition chronology, signed-latitude/seasonal-direction, and amplitude/floor/decomposition ceilings. |

Final disposition is internally consistent with the inspected evidence.
