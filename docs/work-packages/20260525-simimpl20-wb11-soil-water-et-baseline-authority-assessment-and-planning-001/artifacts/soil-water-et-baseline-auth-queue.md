# Soil-Water-ET Baseline Authority Queue

Status: phase-d-complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- Queue is dependency-ordered from SIMIMPL20 baseline authority assessment.
- All code-authoring packages below must enforce internal contract-first order:
  1. contract amendments,
  2. contract-derived tests,
  3. pre-implementation contract gate,
  4. production edits.
- Baseline authority reference for all queue items remains
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.

## Ran
- Inputs used:
  - `simimpl20-wb11-soil-water-baseline-authority-path-assessment.md`
  - `simimpl20-ep-es-er-full-fidelity-migration-risk-register.md`
  - `simimpl20-contract-impact-crosswalk.md`
  - canonical `SC-WATBAL-001`, `SC-EVAP-001`, `SC-SOIL-001`, `SC-PLANT-001`, `SC-SYSTEM-001`

## Proposed Queue
| order | wp_id | objective | depends_on | exit signal |
|---|---|---|---|---|
| 1 | `20260525-simimpl21-wb11-et-soil-water-contract-authority-closure-001` | Amend canonical `SC-*` authority for baseline ET stage-memory, root-uptake, sequencing, and soil-water alias lineage closure required by SIMIMPL20. | SIMIMPL20 | Contract amendments dispositioned with dual review + dual verification and no unresolved authority contradictions. |
| 2 | `20260525-simimpl22-wb11-et-soil-water-contract-derived-tests-and-gate-001` | Add contract-derived tests for stage-memory transitions, layer uptake/stress behavior, ordering guards, and publication-lineage checks; capture pre-implementation gate evidence. | SIMIMPL21 | New tests fail on pre-migration behavior and pre-implementation gate is recorded before code edits. |
| 3 | `20260525-simimpl23-wb11-et-full-fidelity-kernel-migration-001` | Implement baseline-authoritative ET migration (`evap` + `swu` semantics) and required runtime state surfaces with typed guard posture and no heuristic substitutions. | SIMIMPL22 | Kernel emits baseline-authoritative `Ep/Es/Er/Ws` dynamics with stage-memory + layer-uptake evidence and contract tests pass. |
| 4 | `20260525-simimpl24-wb11-soil-water-lineage-and-publication-closure-001` | Close `wb11_soil_water` aggregate lineage and WB13 publication semantics (`Total-Soil`, `SoilWaterTotal`, ET components) using simulation-owned runtime surfaces only. | SIMIMPL23 | Publication/replay surfaces show authoritative lineage closure with passing contract-derived output tests. |
| 5 | `20260525-simimpl25-tier-a-rerun-and-soil-water-et-disposition-001` | Run Tier-A strict/semantic replay reruns and publish hold-lift disposition for ET/soil-water baseline-authority closure. | SIMIMPL24 | Closure criteria pass and disposition transitions from `HOLD` to `GO`, or retains `HOLD` with explicitly owned residual blockers. |

## Sequencing Constraints
1. `SIMIMPL21` must complete before any ET/soil-water production code edits.
2. `SIMIMPL22` must complete before `SIMIMPL23` starts production edits.
3. `SIMIMPL24` can begin only after `SIMIMPL23` runtime behavior is merged.
4. `SIMIMPL25` is final rerun/disposition and depends on `SIMIMPL24`.

## Queue Conclusion
- SIMIMPL20 closes as planning-complete with `HOLD` retained.
- Queue above is the minimum baseline-authoritative path for ET/soil-water
  closure without surrogate physics.
