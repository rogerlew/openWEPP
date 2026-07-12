# Terminal Validation

Evidence class: **Ran + Static**

| Gate | Result | Evidence |
| --- | --- | --- |
| Red regression | PASS as pre-fix reproduction | Exit 100; per-OFE marker admitted `Q=3`, `QOFE=14`. |
| Summary package | PASS | `18/18` |
| Public WB13 output-surface integration | PASS | `3/3` |
| Final coverage/CRAP | PASS | 98.732% lines, 96.144% regions, 71/71 functions, zero eligible raw floor below 75% |
| Target | PASS | CC 29, coverage 100%, CRAP 29 |
| Format/Clippy/diff | PASS | Focused Clippy used `-D warnings` |
| Runner production adoption | **HOLD** | No evidence proves the runner constructs production WB13 rows through this crate. |

The red log is archived as `qofe-red.log`, SHA-256
`fff22728d0a9dcf68baabe6354ae62e99e3d98add74f650761f088ebae60424e`.
Final coverage artifacts are archived under `coverage/`.

Disposition: bounded `DC-CQR-HB06-001` is `TERMINAL-PASS`; broader consumer
adoption is pending and cannot be promoted by the public output-surface test.
