# WB17 Legacy ET Physics Provenance Map

Status: `completed`
Evidence mode: `Static`

## Baseline Authority
- Legacy baseline root: `/workdir/wepp-forest_260430_baseline`
- Baseline commit: `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`

## Provenance Mapping

### ET component semantics (`Ep`, `Es`, `Er`) are canonical legacy outputs
- `src/outfil.for:623-624` defines output semantics:
  - `Ep=plant transpiration`
  - `Es=soil evaporation`
  - `Er=residue evaporation`
- `src/bigout.for:163` daily ET aggregate contains
  `(es + ep + eres)`.
- `src/bigout.for:176-177` records component outputs `ep` and `es`.

### Residue interception coupling into ET partition
- `src/evap.for:566-570` initializes residue interception as residue
  evaporation component:
  - `es = es + resint`
  - `eres = resint`
- `src/evap.for:595-614` constrains residue/soil partition behavior with
  explicit residue-first handling and soil pushdown when residue-intercepted
  water exceeds soil-evaporation remainder.

### LAI-driven plant transpiration partition behavior
- `src/evap.for:583-587` applies LAI-conditioned transpiration branch to
  compute `ep` from ET demand surrogate (`eo`) with canopy saturation branch.

## openWEPP WB17 Port Interpretation
- Canonical WB17 contract equations in `SC-EVAP-001` encode deterministic
  runtime partition authority as:
  - `Esp = Eu * exp(-0.4 * L)`
  - `Etp = Eu - Esp`
  - residue partition `Er = min(Esp, wb17_residue_interception)`
  - soil/plant closure with emitted `Ep`, `Es`, `Er`, `ET`, `Ws`
- This mapping preserves legacy ET component semantics and residue/LAI coupling
  posture while expressing kernel behavior through explicit typed runtime
  symbols and guardable equations.

## Traceability Notes
- Legacy source names (`ep`, `es`, `eres`, `lai`, interception-residue
  behavior) were preserved in canonical alias mapping and WB17 output symbols.
- Provenance is static line-level mapping; equation-shape adaptation and symbol
  lifting are documented in canonical contracts and corresponding WB17 test
  vectors.
