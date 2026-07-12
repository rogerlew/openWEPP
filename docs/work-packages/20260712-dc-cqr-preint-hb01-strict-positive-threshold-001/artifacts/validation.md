# Validation

Evidence class: **Ran**

| Gate | Result | Evidence |
| --- | --- | --- |
| Contract-derived HB-01 tests | PASS | `11/11` |
| Existing zero-upstream real consumer | PASS | `1/1` |
| Shared orchestrator profile | PASS with attributed noise | 380 pass; one source-unchanged R3C parallel counter failure; no retry |
| Focused LCOV/JSON/CRAP | PASS | Expanded slice 95.673% lines / 94.649% regions; minimum floor 85.714%; maximum CRAP 15.136 |
| Erosion workspace profile | PASS | `344/344`, 1,501 skipped, three slow; executed once |
| Focused Clippy | PASS | Library/tests with `-D warnings` |
| Format and diff | PASS | `cargo fmt --check`; `git diff --check` |
| Line governance | PASS | Production 1,251 lines; focused test 398 lines |

Final primary artifacts and hashes are recorded in the HB-01 module record.
The shared-profile failure is the known R3C parallel audit-counter family; its
unchanged source SHA-256 is
`9117d2ff4e0a0d9ecc5f30ae1fe1dfd2aecee28574fbe3dea5aed034a9ddaf7c`.
No failure was rerun and no target-related failure remains.
