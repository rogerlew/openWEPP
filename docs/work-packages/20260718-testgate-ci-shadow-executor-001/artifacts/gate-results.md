# Gate Results

Evidence class: `Ran`

| Gate | Result | Evidence |
| --- | --- | --- |
| Planner crate Nextest before review | PASS | 26/26 in 238.088 seconds. |
| Planner warnings-denied Clippy | PASS | Exact current crate/all-targets command passed. |
| TESTGATE-CI integration guard | PASS | 1/1. |
| Adjudicated-CRAP Python unit suite | PASS | 17/17. |
| Combined focused benchmark | HOLD | 26/26 and exact inventory; 48.8% saving misses 50% threshold. |
| Dirty terminal plan characterization | HOLD | 9 nodes, 2,134 inventory items, planner 91.876 seconds (over 30 seconds). |
| Shell syntax / Python compile / diff check | PASS | Focused checks passed. |
| Dual review | HOLD | Four high implementation blockers remain. |
| Terminal full Nextest / deny / global CRAP | NOT RUN | Correctly not started after current acceptance entered HOLD. |

No successful broad gate was repeated. No focused result is represented as
terminal closure.
