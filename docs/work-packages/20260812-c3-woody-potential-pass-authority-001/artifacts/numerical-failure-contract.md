# V3 Numerical Failure Contract

Status: `approved authority complete`

Evidence mode: `Static`

The canonical payload includes:

```text
model_definition_sha256
transaction_id
occupancy_id
pass = potential | capped
solve = sun_ci | shade_ci | canopy_energy | hydraulic_system |
        outer_gas_energy_hydraulic
iterations
residual_norms[]
step_norm?
backtracking_count
active_bounds[]
active_water_caps[]
bracket?
pivot_magnitude?
matrix_norm?
```

Identity and schema validation precede solver arithmetic. Within a solve,
domain/nonfinite failure precedes bracket failure, which precedes singular-pivot
failure, which precedes iteration-limit failure. Optional fields are null only
when the corresponding operation was not reached. Present numeric values are
finite. Failures never expose a usable last iterate or partial candidate.

Each failure residual entry retains its component identity and signed
normalized value. The accepted-solution fixture additionally exposes signed
raw values, physical scales, exact owning tolerances, and signed normalized
values so the normalization is independently reconstructable. V3 fixes
centered finite-difference unit scales to `1 Pa` for `ci`, `1 K` for
temperatures, `0.001 kg kg-1` for canopy-air humidity, `1000 mm H2O` for
potentials, and one for class beta. Energy and water residual scales are the
explicit operand-sum and maximum-flux definitions in the canonical contract;
no hidden scale is admissible.
