# WB17 ET Equation Vector Parity Evidence

Status: `completed`
Evidence mode: `Static + Ran`

## Vector Basis
WB17 nominal vector from
`tests/integration/wb17_et_physics_kernel_contract.rs` uses:
- `wb11_soil_water = 0.2`
- `wb11_et_demand = 0.5`
- `lai = 0.3`
- `wb17_residue_interception = 0.05`

## Expected Equation Results
Using `Esp = Eu * exp(-0.4 * L)` and `Etp = Eu - Esp`:
- `Esp = 0.443460218358579`
- `Etp = 0.056539781641421`
- `Er = min(Esp, 0.05) = 0.05`
- `Es_potential = Esp - Er = 0.393460218358579`
- `Es_actual = min(0.2, Es_potential) = 0.2`
- `Ep = min(Etp, 0.0) = 0.0`
- `ET = Er + Es_actual + Ep = 0.25`
- `Ws = Ep / Etp = 0.0`

## Test Assertions
The WB17 nominal conformance test asserts:
- `ET = 0.25`
- `Ws = 0.0`
- `Ep = 0.0`
- `Es = 0.2`
- `Er = 0.05`
- `wb11_soil_water(after) = 0.0`

## Executed Evidence
Command:
```bash
cargo test --test wb17_et_physics_kernel_contract
```
Result:
- pass (`4 passed; 0 failed`)

Interpretation:
- Runtime outputs match WB17 equation-vector expectations for the nominal
  partition trajectory.
