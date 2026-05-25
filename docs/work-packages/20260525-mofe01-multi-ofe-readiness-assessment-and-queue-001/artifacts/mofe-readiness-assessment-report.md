# MOFE Readiness Assessment Report

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
### Objective coverage
MOFE01 objective is closed for planning: current-state readiness was assessed across
routing plus slope/landuse(=management)/soil OFE cardinality parity, and a
dependency-ordered follow-on queue is published.

### Canonical invariant baseline
Required invariant: slope OFE count == management/landuse OFE count == soil OFE
count for MOFE execution contexts.

Authority evidence:
- `SC-INFILE-SLOPE-001` declares cross-file OFE-count mismatch (`SLP-E-007`) and
  explicit slope-to-management/soil parity constraint.
  - `docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001.md:130`
  - `docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001.md:136-141`
- `SC-INFILE-SOIL-001` declares cross-file OFE-count mismatch (`SOL-E-007`) and
  explicit hillslope topology rule (`ntemp == nofe`).
  - `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md:160`
  - `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md:166-173`
- `SC-INFILE-MANAGEMENT-001` declares cross-file topology mismatch (`MAN-E-007`)
  and `nofe/nchan` parity obligations.
  - `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md:341`
  - `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md:348-353`

### Readiness matrix
1. Slope parser/runtime surface readiness: partial
- `build_hillslope_runtime_surface_from_slope` emits OFE-count symbols (`nelem`,
  `nwsofe`) and enforces internal slope-shape consistency.
  - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:1858-1876`
  - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:3003-3013`
- Gap: no slope-vs-management/soil parity check is executed in this path.

2. Management (landuse) parser/runtime surface readiness: partial
- Runtime seam enforces internal management topology/schedule consistency and
  emits `pl_schedule_nofe`.
  - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:960-999`
- Gap: no management-vs-slope/soil parity check is executed in this path.

3. Soil parser/runtime surface readiness: partial-plus
- Soil parser supports explicit cross-file topology validation via
  `expected_topology_count` and emits `SOL-E-007` on mismatch.
  - `crates/openwepp-input-contract/src/parsers/soil.rs:190-194`
  - `crates/openwepp-input-contract/src/parsers/soil.rs:333-347`
- Existing test confirms this behavior.
  - `tests/integration/infile_soil_parser_contract.rs:120-130`
- Gap: hillslope runner disables this check by passing
  `expected_topology_count: None`.
  - `crates/openwepp-runner/src/hillslope/mod.rs:427-431`

4. Cross-file parity at runner/orchestration boundary: not ready
- Hillslope runner independently builds soil/slope/management surfaces then
  merges maps; no pre-merge or post-merge cross-file OFE-count parity guard is
  present.
  - `crates/openwepp-runner/src/hillslope/mod.rs:693-731`
  - `crates/openwepp-runner/src/hillslope/mod.rs:1303-1309`

5. Routing runtime readiness for MOFE pathways: partial
- EROD14 Wave-2 runtime pathway exists and has typed domain guards for MOFE case
  symbols (`Qj-1`, `Vj`, `Qj`, `Fh`, `Fp`, `case`).
  - `crates/openwepp-hillslope-orchestrator/src/hydrology.rs:5085-5224`
- Gap: production hillslope runner does not seed `erod14_wave2_enabled` or
  `erod14_*` source symbols; symbol appears only in tests, so Wave-2 path is
  effectively inactive unless externally injected.
  - `tests/integration/erod14_wave2_multiofe_enrichment_kernel_contract.rs:257-347`
  - `crates/openwepp-hillslope-orchestrator/src/hydrology.rs:1256-1267`

6. Publication/output MOFE readiness: limited
- WB13 publication path is primary-OFE-oriented (`primary_ofe` geometry and
  `Wb13DailyWaterBalanceRow::from_surface(1, ...)`).
  - `crates/openwepp-runner/src/hillslope/mod.rs:2159-2187`
  - `crates/openwepp-runner/src/hillslope/mod.rs:2395-2397`

### Gap classification and risk register
- `F-001` severity `blocking`: missing enforced cross-file OFE-count parity gate
  in hillslope runner path.
- `F-002` severity `high`: existing soil cross-file validator is present but not
  wired from runner (`expected_topology_count: None`).
- `F-003` severity `high`: EROD14 Wave-2 MOFE routing path is guarded but not
  production-seeded/activated from parsed run inputs.
- `F-004` severity `medium`: WB13 publication remains primary-OFE-centric,
  limiting MOFE output fidelity semantics.

### Overall readiness verdict
- Contract authority readiness: `READY` (invariants are explicit in canonical
  contracts).
- Production MOFE execution readiness (routing + slope/landuse/soil parity):
  `HOLD` pending follow-on queue closure.

## Ran
- `rg -n "OfeCount|ofe_count|topology_count|ntemp|nwsofe|pl_schedule_nofe" crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `rg -n "expected_topology_count|topology_scope" crates/openwepp-input-contract/src/parsers/soil.rs crates/openwepp-runner/src/hillslope/mod.rs`
- `rg -n "erod14_wave2_enabled|erod14_Qj_minus_1|erod14_Vj|erod14_Qj|erod14_case" crates/openwepp-hillslope-orchestrator/src tests/integration`
- `sed`/`nl` evidence extraction across:
  - `crates/openwepp-runner/src/hillslope/mod.rs`
  - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
  - `crates/openwepp-hillslope-orchestrator/src/hydrology.rs`
  - `crates/openwepp-input-contract/src/parsers/soil.rs`
  - `tests/integration/infile_soil_parser_contract.rs`
  - `tests/integration/parser_runtime_seam_integration.rs`
  - `tests/integration/erod14_wave2_multiofe_enrichment_kernel_contract.rs`
  - `docs/specifications/science-contracts/contracts/SC-INFILE-{SLOPE,SOIL,MANAGEMENT}-001.md`
