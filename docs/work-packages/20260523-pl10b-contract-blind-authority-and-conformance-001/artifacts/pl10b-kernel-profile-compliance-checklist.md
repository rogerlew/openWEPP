# PL10b Kernel Profile Compliance Checklist

Status: `complete`
Evidence mode: `Static`

Checklist authority:
`docs/specifications/science-contracts/kernel-process-contract-profile.md`

| Requirement | Evidence anchor | Status | Notes |
|---|---|---|---|
| Canonical `SC-*` file updated | `docs/specifications/science-contracts/contracts/SC-PLANT-001.md` (`contract_version: 4`) | `pass` | PL10b amendment applied in canonical location. |
| Purpose + scientific scope present | `SC-PLANT-001` `## Purpose`, `## Scientific Scope` | `pass` | Includes transition-control projection scope. |
| Authority anchors present | `SC-PLANT-001` `## Authority Anchors` | `pass` | Includes top-down refs plus baseline provenance anchors. |
| Variables/units table includes changed behavior surfaces | `SC-PLANT-001` `## Variables and Units` | `pass` | Added annual/perennial transition-control symbols and payload arrays. |
| Algorithm state surfaces documented (inputs/outputs/mutations) | `SC-PLANT-001` `## Algorithm State Surfaces (PL Transition-Control Runtime Projection)` | `pass` | Projection purity + required symbol families documented. |
| Numbered algorithm specification with branches | `SC-PLANT-001` `## Algorithm Specification (PL10b Transition-Control Authority)` | `pass` | Branch partition/cardinality/date-domain/failure posture included. |
| Branch/guard table present | `SC-PLANT-001` `## Branch and Guard Table (Transition Controls)` | `pass` | Annual/perennial/unsupported-landuse branches mapped to guard posture. |
| Invariants and guard map updated | `SC-PLANT-001` `INV-PLANT-011..015`, `## Guard Map` | `pass` | Transition-control invariants and guard mappings added. |
| Symbol alias map updated | `SC-PLANT-001` `## Symbol Alias Map` | `pass-with-notes` | Alias continuity defined; one parser-field naming ambiguity remains tracked as `PL10B-GAP-006`. |
| Constants/parameters table present | `SC-PLANT-001` `## Constants and Parameters Table` | `pass` | Day-domain and index-origin constants documented. |
| Tolerance/numeric notes present | `SC-PLANT-001` `## Tolerance and Numeric Notes` | `pass` | Added transition-control integer-domain tolerance policy. |
| Test-vector obligations present | `SC-PLANT-001` `## Test-Vector Obligations` | `pass` | Directly mapped to PL10b contract-conformance tests. |
| Gap register with promotability labels | `SC-PLANT-001` `## Gap Register` | `pass` | Implementation closure gaps remain explicitly non-promotable/promotable-with-risk. |

## Compliance Verdict

`PASS-WITH-NOTES`: schema/profile obligations are satisfied for PL10b contract
authority. Remaining implementation conformance deficits are explicitly tracked
for PL11.
