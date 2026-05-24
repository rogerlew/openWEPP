# WB17 ET Physics Authority And Guard Map

Status: `completed`
Evidence mode: `Static`

## Canonical Authority Surfaces
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
  - `contract_version: 7`
  - WB17 ET production equations and invariants (`INV-EVAP-011`,
    `INV-EVAP-012`).
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - `contract_version: 18`
  - Scheduler-level WB17 ET + WB11 percolation/lateral/drain execution
    authority (`INV-WATBAL-009`).

## Runtime Equation Mapping
- Input symbols:
  - `wb11_soil_water`
  - `wb11_et_demand`
  - `lai`
  - `wb17_residue_interception`
- Partition equations implemented in runtime:
  - `Esp = wb11_et_demand * exp(-0.4 * lai)`
  - `Etp = wb11_et_demand - Esp`
  - `Er = min(Esp, wb17_residue_interception)`
  - `Es = Esp - Er`
  - `Es_actual = min(wb11_soil_water, Es)`
  - `Ep = min(Etp, wb11_soil_water - Es_actual)`
  - `ET = Er + Es_actual + Ep`
  - `Ws = 1` when `Etp <= 1e-12`, else `Ep / Etp`
- Output surfaces:
  - flux: `ET`, `Ws`, `Ep`, `Es`, `Er`
  - state update: `wb11_soil_water`

## Production Implementation Anchors
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
  - constant authority:
    - `WB17_LAI_PARTITION_COEFFICIENT = 0.4`
    - `WB17_SYMBOL_RESIDUE_INTERCEPTION`
    - `WB17_SYMBOL_EP`, `WB17_SYMBOL_ES`, `WB17_SYMBOL_ER`
  - ET runtime path: `run_evapotranspiration(...)`
- `crates/openwepp-kernel-contract/src/lib.rs`
  - state symbol: `Wb17ResidueInterception -> "wb17_residue_interception"`
  - flux symbols:
    - `Wb17PlantTranspirationEp -> "Ep"`
    - `Wb17SoilEvaporationEs -> "Es"`
    - `Wb17ResidueEvaporationEr -> "Er"`

## Guard Map
- Missing required input:
  - `HKERNEL-WB11-ET-E-001`
  - boundary class: `MissingRequiredInput`
- Non-finite input:
  - `HKERNEL-WB11-ET-E-002`
  - boundary class: `NonFinite`
- Domain-invalid input:
  - `HKERNEL-WB11-ET-E-003`
  - boundary class: `DomainViolation`

## Guard Enforcement Posture
- WB17 ET input surfaces are required and validated for finite/non-negative
  domain before partition calculations.
- Intermediate and output flux/state surfaces are range-checked before
  writeback.
- No silent defaults or clamping were introduced for missing/non-finite/
  domain-invalid WB17 ET inputs.
