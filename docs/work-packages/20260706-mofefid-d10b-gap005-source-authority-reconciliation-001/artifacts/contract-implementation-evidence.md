# Contract Implementation Evidence (D10B-S2)

Status: executed (rev 24 landed; final gap disposition lands with S4/S5)
Evidence mode: Static (contract authoring) grounded in the Ran S0/S1 evidence

`SC-OFEROUTE-001` rev 24 (2026-07-06, Claude Code). Amended surfaces:

| Surface | Change | Grounding |
|---|---|---|
| `REF-OFEROUTE-TVD-MACCORMACK` | Davis 1984 (R-102) + Tseng 2010 (R-103) added primary-in-hand; printed-(11c) transcription-error adjudication; two-sided face-form binding; alpha-timing and boundary-ownership adjudication recorded | `limiter-adjudication-evidence.md` A1-A5 |
| `REF-OFEROUTE-SHOCK-IWAGAKI` | Experiment-(B) configuration recorded verbatim (incl. the 0.800/0.0800 OCR artifact note); Manning `n=0.009` m-s; characteristics-oracle role; sidewall/laminar-switch fidelity caveats | `source-acquisition-record.md` |
| Algorithm Specification items 3-6 | Source-corrected limiter + face-based two-sided dissipation with zero boundary faces; frozen-alpha celerity note; NEW item 5 (boundary fluxes and mass ledger: booked-equals-actual, clamp stage weighting); handoff flux-integral injection | A1/A2/A5 + S0 ledger |
| Branch/guard "KWE/TVD conservation" row | booked-equals-actual + exact face-form telescoping | S0 ledger |
| `INV-OFEROUTE-006` | Rev-24 strengthenings (a) booked-equals-actual ledger, (b) exactly-telescoping face dissipation, (c) resolution-convergent monotone-bounded 19-OFE-class acceptance | S0 ledger (exact decomposition) |
| `INV-OFEROUTE-011` | Case-4 acceptance RE-ANCHORED to the Iwagaki-primary oracle; digitized enhanced-WEPP Figure-4 trace DEMOTED to ADR-0017 comparator flag; `k_o` removed from Case-4 acceptance; tolerances proposed-then-ratified with S3/S4 evidence | `oracle-reanchoring-evidence.md` |
| Guard-map D10 note | Superseded by the D10B note (hold lifted by reconciliation; D10 state kept for provenance) | — |
| Tolerance and Numeric Notes | Oracle-convergence acceptance shape + oracle self-evidence requirement + 19-OFE conservation acceptance shape | — |
| Test-Vector Obligations | NEW "D10B shock-numerics reconciliation" row (vectors a-f) | — |
| BEI `OFEROUTE-KWE-TVD-SOLVER` | Note updated to the rev-24 binding | — |
| `GAP-OFEROUTE-001` | CLOSED (all four family primaries in hand; scheme form bound) | `source-acquisition-record.md` |
| `GAP-OFEROUTE-005` | Moved to RECONCILED-CORRECTION-IN-FLIGHT with the exact defect decomposition recorded; closes with S4 validated evidence | S0 + S1 artifacts |
| Revision History | Rev-24 row appended | — |

Contract-first sequencing: this amendment precedes all production
solver/cascade edits. The S3 contract-derived tests are authored against the
rev-24 text; the pre-implementation gate is recorded in
`pre-implementation-contract-gate.md` before any S4 production edit.

Diagnostics-class code landed BEFORE this amendment (authorized by the
package envelope as measurement, not physics): `CascadeResult.per_ofe_solver_mass`,
the three `MassBalance` scheme-actual diagnostic fields, and
`examples/cascade_seam_ledger.rs`. None of these alter scheme behavior;
`cargo build` clean; behavior-affecting edits remain gated behind S3.
