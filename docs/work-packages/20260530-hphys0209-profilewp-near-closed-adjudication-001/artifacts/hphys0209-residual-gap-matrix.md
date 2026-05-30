# HPHYS0209 Residual Gap Matrix

Status: completed  
Evidence mode: Static + Ran

## Gap register
| Gap ID | Description | Evidence | Status |
| --- | --- | --- | --- |
| `HP209-GAP-001` | Near-closed `ProfileWPStore` lane lacked explicit canonical adjudication authority distinguishing defect lineage from bounded expected process-correct residuals. | Static: HPHYS0209 addenda in `SC-WATBAL-001`, `SC-SOIL-001`, `SC-SYSTEM-001`, and `science-contracts/index.md`. | closed |
| `HP209-GAP-002` | No lane-specific contract tests asserted both WP perturbation responsiveness and non-regressing profile-geometry families (`ProfileDepth`, `ProfilePorosityCap`). | Static + Ran: `tests/integration/hphys0209_profilewp_adjudication_contract.rs` plus targeted test execution logs under `/tmp/hphys0209_20260530T171007Z/tests/`. | closed |
| `HP209-GAP-003` | Residual lane remained at `1/39` fail hillslopes and required explicit bounded-classification evidence before integrated hold-lift adjudication. | Ran: `/tmp/hphys0209_20260530T171007Z/parity/reports/hphys0209_profilewp_focus_summary.json` (`ProfileWPStore` fail lane `H7` only; `ProfileDepth`/`ProfilePorosityCap` both `0/39` fails). | bounded (carry to HPHYS0210) |

## Residual risk after execution
- Ran: `ProfileWPStore` residual is isolated to `H7` (`1/39`) with stable
  non-regressing geometry families (`ProfileDepth`, `ProfilePorosityCap`).
- Static + Ran: HPHYS0209 classifies this lane as expected process-correct
  diagnostic evidence under preserved fail-closed guard posture.
- Static: integrated `HOLD`/`GO` decision for all active residual families is
  intentionally deferred to HPHYS0210.
