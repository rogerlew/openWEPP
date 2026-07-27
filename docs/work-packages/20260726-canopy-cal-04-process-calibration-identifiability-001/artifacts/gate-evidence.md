# Gate Evidence

Evidence class: `Ran: intake and non-result-bearing closure; Static: scientific
execution blockers`

| Gate | Result | Evidence |
| --- | --- | --- |
| Authority source checksums | `PASS` | CMD-002: all 20 objects in the retained authority manifest passed. |
| Observation corpus checksums | `PASS` | CMD-003: all four corpus objects passed. |
| Protected bindings and runner identity | `PASS` | Exact hashes and runner size/mtime in `input-and-authority-manifest.csv`. |
| Exact timing-window rebuild | `PASS` | CMD-005/006: 1,251 rows; byte-identical SHA-256 `890a0f...1b61`. |
| Role immutability/disjointness | `PASS` | CMD-009: 932 Hubbard calibration, 319 Harvard holdout, disjoint, no Harvard fall 1992. |
| Prospective domain/grid freeze | `BLOCKED` | Both reviewers confirm that admissibility domains are not finite calibration bounds. |
| Candidate deterministic enumeration/rebuild | `BLOCKED` | Zero candidates authorized; no lawful complete grid. |
| Candidate/failure completeness | `PASS` | CMD-008: zero candidates and zero failures join consistently. |
| Objective reconstruction from candidate traces | `BLOCKED` | No lawful candidate trace exists. |
| Stage order/upstream freeze | `PASS` | CMD-008: every downstream plan remains blocked and all seven stage rows hold. |
| Holdout embargo/one-time open | `PASS` | CMD-008: Harvard ledger empty and opening record `SEALED`. |
| Holdout scoring | `BLOCKED` | No nonempty accepted calibration ensemble. |
| Downstream evaluation | `BLOCKED` | No accepted ensemble; no downstream result inspected for selection. |
| No probability-prior mislabeling | `PASS` | No probability prior or numeric domain was invented. |
| Prospective dual review | `PASS` | Both independent reviewers returned `HOLD`; accepted findings corrected. |
| Documentation lint | `PASS` | CMD-010: package-local Markdown passed with zero errors/warnings. |
| Diff hygiene | `PASS` | CMD-011: `git diff --check` passed. |
| Dual terminal scientific review | `PASS` | Both reviewers supported `HOLD`; TA-1 replay finding corrected. |
| Dual terminal verification | `PASS` | Both verifiers independently passed the corrected state. |
| Post-verification reconciliation | `PASS` | CMD-014..018 refreshed validators, roles/write set, lint, diff/prompt state, and terminal evidence identities. |

`BLOCKED` scientific gates prevent `COMPLETE` and are the direct basis for
`EXECUTED / HOLD`; they are not waived or represented as passed.

The first authority rebuild attempt (CMD-004) used summarized Harvard date
tables rather than raw daily rows and failed with `KeyError: date`. It changed
no repository artifact. The corrected raw-table invocation passed
byte-identically; the failed command remains retained.
