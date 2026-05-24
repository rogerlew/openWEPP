# WB17 ET Partition Trajectory Evidence

Status: `completed`
Evidence mode: `Static + Ran`

## Nominal Trajectory (Soil-Limited)
Vector: `Eu=0.5`, `L=0.3`, `soil=0.2`, `residue_interception=0.05`

1. Potential partition:
- `Esp = Eu * exp(-0.4 * L) = 0.443460218358579`
- `Etp = Eu - Esp = 0.056539781641421`

2. Residue partition:
- `Er = min(Esp, residue_interception) = 0.05`
- `Es_potential = Esp - Er = 0.393460218358579`

3. Soil-water consumption:
- `Es_actual = min(soil, Es_potential) = 0.2`
- `soil_after_evaporation = 0.0`

4. Plant transpiration:
- `Ep = min(Etp, soil_after_evaporation) = 0.0`
- `soil_after = 0.0`

5. Closure outputs:
- `ET = Er + Es_actual + Ep = 0.25`
- `Ws = Ep / Etp = 0.0`

## Guard Trajectory Coverage
Contract-derived vectors verify typed fail-fast branches:
- Missing `wb17_residue_interception` -> `HKERNEL-WB11-ET-E-001`
- Non-finite `lai` -> `HKERNEL-WB11-ET-E-002`
- Negative `wb17_residue_interception` -> `HKERNEL-WB11-ET-E-003`

## Executed Evidence
Command:
```bash
cargo test --test wb17_et_physics_kernel_contract
```
Result:
- pass (`4 passed; 0 failed`)

Interpretation:
- Deterministic WB17 ET partition trajectory and guard branches execute as
  specified by canonical contract authority.
