# Terminal Validation

Evidence class: **Ran + Static**

| Gate | Result | Evidence |
| --- | --- | --- |
| Full watershed-output | PASS | `8/8` |
| Real watershed CLI consumer | PASS | Sediment-active P102/jobs identity `1/1`; reads production Parquet output. |
| Fixed targets | PASS | Writer CC `15`, coverage `84.483%`, CRAP `15.841`; Float64 CC `7`, coverage `100%`, CRAP `7`. |
| Function floor | PASS | Zero `writers.rs` cargo-crap rows below 75%; `write_single_output` is `77.778%`/CRAP `9.889`. |
| Format/Clippy/diff | PASS | Workspace format; all-target watershed-output/runner Clippy with `-D warnings`; diff clean. |

Whole-run LLVM: lines `2449/2510` (`97.570%`), regions `3136/3252`
(`96.433%`), functions `194/206` (`94.175%`).

| Item | SHA-256 |
| --- | --- |
| `writers.rs` | `b122b8254aa5328269161253cb328528cc5d5b9eda68ce78e305d81f10ab9772` |
| JSON | `a63f19c7b6b1772de7cdbcb2a11acd6761ef81d748ab8f6f94c9468ce4a32695` |
| LCOV | `cf84590dae2a52453fa419e300c871161e47da207aec881b1eb9803cdae1adee` |
| CRAP | `f9c6478e7c5b92c5ccfd8c0a1c221b68dc53046d6deada73fde22d3f38167bef` |

Line governance: `2,706 -> 2,865`, WARN above 2,000 but below the 3,000-line
blocker. The added lines are private coherent helpers and characterization;
no source split is required for this bounded tranche.
