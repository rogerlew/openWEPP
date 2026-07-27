# Gate Evidence

Evidence class: `Ran + Static`

| Gate | Result | Evidence |
| --- | --- | --- |
| Daymet source checksums | `PASS` | README and nine source-native responses verified. |
| Source/request custody | `PASS` | Manifest, retrieval receipt, returned headers, DOI, timestamps, and checksums retained. |
| Native equation binding | `PASS` | Static Rust binding plus reference vectors and all-row numerical reconstruction. |
| VPD validity | `PASS` | All 118,260 values finite and non-negative; no clamp. |
| Exact observation join | `PASS` | 932 unique calibration IDs equal the admitted set; calendar, plot, interval, and forcing fields agree. |
| Deterministic rebuild | `PASS` | Ten generated artifacts rebuilt byte-identically. |
| Finite design | `PASS` | 21 ordered pairs per family; 9,261 complete vectors; no refinement. |
| Evidence-role truthfulness | `PASS` | Search bounds are execution assumptions, not physiological or probability bounds. |
| Harvard embargo | `PASS` | The combined admitted timing ledger was role-filtered, so holdout metadata was inspected; zero Harvard row was joined or used and no raw Harvard source, modeled trace, or score was opened. |
| Dual scientific review | `PASS` | Both corrected reviews pass; all blocking findings disposed. |
| Dual terminal verification | `PASS` | Both verifiers pass after role-wording, write-set, prompt, and terminal-state corrections. |
| Documentation lint | `PASS` | Package-local Markdown has zero errors/warnings. |
| Diff hygiene | `PASS` | `git diff --check` passes. |
| Production/protected writes | `PASS` | Dedicated write-set validator passes; ambient predecessor CAL-04 is classified separately. |
