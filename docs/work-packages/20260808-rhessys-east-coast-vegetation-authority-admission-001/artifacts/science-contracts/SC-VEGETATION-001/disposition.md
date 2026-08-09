# Disposition: SC-VEGETATION-001

Status: `complete / dual verification PASS`

Date: 2026-08-08 UTC

Canonical contract:
`docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md`

Reviewed commit SHA:
`ea1df89d78fa7a79d7b1d0aac4f81899b90c68f0`

Post-fix contract SHA-256:
`7e62cf907eb328ad1b1aaf535ab1556896f686c7d0a8e01ed22a6ce81d635f7a`

| finding_id | source | severity | decision | action_taken | artifact_ref | rationale |
| --- | --- | --- | --- | --- | --- | --- |
| `A-01` | `agent_a` | `critical` | `accepted` | Followed and recorded ORNL/White/Reich/Hwang/Ford/Coweeta/Harvard routes; narrowed the blocker to incomplete selected mapping and compatible state. | `artifacts/authority-route-attempts.md:7-38`; `SC-VEGETATION-001.md:88-90` | Exhausted concrete leads are required before a terminal scientific hold. |
| `A-02` | `agent_a` | `critical` | `accepted` | Evaluated every Gate 2/3 symbolic family independently of selected values and retained exact equation/domain/guard/vector gaps. | `artifacts/canopy-water-energy-gate.md:7-29`; `artifacts/carbon-phenology-root-gate.md:7-26` | Gate 1 failure cannot defer separable authority work. |
| `A-03` | `agent_a` | `high` | `accepted` | Separated acquisition, schema-form, selected-value, and dated-state dispositions. | `artifacts/schema-profile-initial-state-gate.md:38-59`; `:78-92` | The lifecycle must credit admitted sub-boundaries without overstating readiness. |
| `A-04` | `agent_a` | `high` | `accepted` | Added refs 015-017, invariants 053-056, guard rows, strict acquisition/schema requirements, BEI-003, and updated gaps; added a focused contract test. | `SC-VEGETATION-001.md:88-90`; `:271-274`; `:299-303`; `:355-391`; `:470`; `:495-497`; `tests/integration/vegetation_boundary_authority_contract.rs:110-136` | Canonical contract and direct assertions, not package prose, bind the admitted boundary. |
| `A-05` | `agent_a` | `medium` | `accepted` | Split partial typed/schema progress from blocked execution, sensitivity, identifiability, and selected-value readiness. | `artifacts/calibration-readiness-matrix.md:11-27` | Type authority is distinct from data/calibration readiness under ADR-0042. |
| `A-06` | `agent_a` | `medium` | `accepted` | Revised disposition and gap records to name all three gates and schema-form-only admission. | `artifacts/disposition.md:7-16`; `artifacts/authority-gap-disposition.md:7-20` | Prevents an authority-only partial result from implying implementation readiness. |
| `B-01` | `agent_b` | `critical` | `accepted` | Added exact source-route attempts and primary-source comparisons. | `artifacts/authority-route-attempts.md:7-38`; `artifacts/schema-profile-initial-state-gate.md:19-25` | Primary-source inspection is necessary to distinguish recovered authority from counterevidence. |
| `B-02` | `agent_b` | `high` | `accepted` | Recorded partial Ford/Day/Monk observations and the missing selected stand/date/topology plus incomplete C/N/root state. | `artifacts/schema-profile-initial-state-gate.md:61-76`; `SC-VEGETATION-001.md:495` | The precise blocker is compatibility/completeness, not global observation absence. |
| `B-03` | `agent_b` | `high` | `accepted` | Added the exact 71-row-by-two-profile ledger with field role, units/basis, alias, source route, and terminal disposition. | `artifacts/selected-field-ledger.md:7-101` | Field-complete evidence prevents aggregate admission or omission errors. |
| `B-04` | `agent_b` | `high` | `accepted` | Required source/contract authority for aliases before implementation proof. | `artifacts/schema-profile-initial-state-gate.md:38-47`; `SC-VEGETATION-001.md:371-379` | Parser correctness cannot establish scientific semantic equivalence. |
| `B-05` | `agent_b` | `high` | `accepted` | Reclassified `gsurf_intercept/slope` as wet-canopy consumed inputs; rejected the source law and held replacement authority. | `artifacts/selected-field-ledger.md:89-90`; `artifacts/canopy-water-energy-gate.md:13-16` | A consumed source input cannot evade an authority disposition. |
| `B-06` | `agent_b` | `high` | `accepted` | Narrowed all canonical, test, and lifecycle claims to schema-form partial `AUTH-RHEC-001`; selected declarations/aliases remain missing. | `SC-VEGETATION-001.md:355-360`; `artifacts/authority-gap-disposition.md:7-12`; `tests/integration/vegetation_boundary_authority_contract.rs:110-135` | Prevents false release of selected parameters or implementation work. |
| `B-07` | `agent_b` | `low` | `accepted` | Completed the constants/parameters prohibition sentence. | `SC-VEGETATION-001.md:342-350` | Restores an unambiguous no-value-admission rule. |
| `N-01` | `agent_b` | `medium` | `accepted` | Extended the canopy-snow, fixed-point/fallback, and tolerance prohibitions explicitly through contract version 3. | `SC-VEGETATION-001.md:115`; `:221-223`; `:244`; `:264`; `:410-413`; `:454` | Removes ambiguity about whether the current revision preserves fail-closed guards. |
| `N-02` | `agent_b` | `low` | `accepted` | Corrected the B-05 evidence locators to the wet-canopy ledger rows. | `artifacts/selected-field-ledger.md:89-90`; `review_agent_b.md` B-05 | Restores directly replayable finding evidence. |

All findings were accepted; none was rejected, deferred, waived, or silently
closed. Canonical contract references above are relative to
`docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md`; package
artifact references are relative to the owning work-package directory.
