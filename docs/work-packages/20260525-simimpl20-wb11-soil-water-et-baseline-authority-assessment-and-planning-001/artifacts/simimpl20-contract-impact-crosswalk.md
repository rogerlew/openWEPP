# SIMIMPL20 Contract Impact Crosswalk

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
| contract | current authority signals | required SIMIMPL20 follow-on amendment scope | follow-on queue dependency |
|---|---|---|---|
| `SC-EVAP-001` | WB17 deterministic partition is documented; `GAP-EVAP-005` records missing stage-memory/runtime-state projection. | Add canonical first-class surfaces and invariants for `s1/s2/tu/tv`, `dx/ds`, `UPi/Ui`, and explicit baseline ordering/guard semantics tied to `evap` + `swu` authority. | queue item 1 |
| `SC-WATBAL-001` | WB17/WB18/WB19 execution invariants and ET outputs exist (`INV-WATBAL-009..011`), but companion-gap and full ET lineage closure remain provisional. | Amend algorithm/alias sections to encode baseline ET/perc/lateral/drain/root-uptake sequencing and `wb11_soil_water` aggregate lineage from per-layer state. | queue item 1 |
| `SC-PLANT-001` | `INV-PLANT-007` and `INV-PLANT-017` enforce plant->watbal coupling and ordering. | Add cross-contract obligations for migrated `Ws`/root-depth-dependent uptake surfaces consumed by growth/stress coupling paths. | queue item 1 |
| `SC-SOIL-001` | `GAP-SOIL-002` notes unresolved runtime alias closure. | Finalize soil-layer alias mappings required for ET layer extraction and publication continuity. | queue item 1 |
| `SC-SYSTEM-001` | `INV-SYSTEM-011`, `INV-SYSTEM-018..020` enforce execution-owned publication and coupled-lane gating; `GAP-SYSTEM-001/002` persist. | Add explicit ET lineage publication provenance and output alias obligations for migrated `Ep/Es/Er` and `Total-Soil` semantics in replay surfaces. | queue items 1 and 4 |

## Contract-Derived Test Impact
| test family | purpose | source contract obligations | planned queue item |
|---|---|---|---|
| `wb17_stage_memory_transition_vectors` | Validate `s1/s2/tu/tv` transitions and `Es` stage behavior against baseline vectors. | `SC-EVAP-001` amendments, `SC-WATBAL-001 INV-WATBAL-009` | queue item 2 |
| `wb17_root_uptake_layer_distribution_vectors` | Validate `UPi/Ui` layer extraction and stress (`Ws`) with root-depth coupling. | `SC-EVAP-001`, `SC-PLANT-001 INV-PLANT-007` | queue item 2 |
| `wb11_hydrology_ordering_guard_vectors` | Assert baseline-authoritative ET/perc/lateral/drain/root-uptake ordering and typed failure posture. | `SC-WATBAL-001 INV-WATBAL-010/011`, `SC-SYSTEM-001 INV-SYSTEM-011` | queue item 2 |
| `wb13_publication_alias_lineage_vectors` | Validate `Ep/Es/Er` and `Total-Soil` publication semantics from simulation-owned runtime surfaces. | `SC-WATBAL-001 INV-WATBAL-026/027`, `SC-SYSTEM-001 INV-SYSTEM-020` | queue item 4 |

## Crosswalk Conclusion
- Contract amendments are prerequisite work for code-authoring ET/soil-water
  migration packages; no production edits are promotable before these
  contract/test closures land.

## Ran
- `rg -n "INV-WATBAL-009|INV-WATBAL-010|INV-WATBAL-011|GAP-WATBAL-002" docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `rg -n "GAP-EVAP-005|INV-EVAP-011|INV-EVAP-012" docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `rg -n "INV-PLANT-007|INV-PLANT-017|GAP-PLANT-004" docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `rg -n "GAP-SOIL-002" docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `rg -n "INV-SYSTEM-011|INV-SYSTEM-018|INV-SYSTEM-019|INV-SYSTEM-020|GAP-SYSTEM-001|GAP-SYSTEM-002" docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
