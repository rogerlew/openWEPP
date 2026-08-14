# Numerical Solver Contract

The potential and capped passes use one nested-equivalent joint solve. The
outer tile vector is:

```text
[all V8 occupancy hydraulic potentials and beta values,
 all sun/shade/wet/stem temperatures and ci values,
 shared tile canopy-air T and q,
 ground/litter surface T,
 all configured soil temperatures]
```

Brent `ci` subsolves retain V7 brackets and limits. At each outer residual
evaluation the complete shortwave/longwave column, component gas/energy,
canopy-air heat/vapor, ground vapor, soil/litter storage and hydraulic laws are
recomputed from the current iterate. The capped pass adds fixed source/layer
water complementarity and otherwise uses the same residual system.

The equilibrium-zero surface has no physical beginning temperature. Its
current algebraic trial temperature supplies the surface-side operand at both
Crank--Nicolson top-interface endpoints; the soil-side operands remain the
beginning and trial-ending soil temperatures. The caller surface-temperature
warm start affects iteration initialization only.

The nonlinear outer solve uses V7 damped Newton with centered finite
differences, pivoted LU, strict infinity-norm decrease, at most 50 Newton
iterations and 20 halvings. Perturbations are
`sqrt(epsilon)*max(abs(x),unit_scale)`, with unit scales `1 K`, `0.001 kg
kg^-1`, `1 Pa`, `1000 mm`, and `1` for beta. A pivot below
`64*epsilon*matrix_inf_norm` is singular.

Each energy residual threshold is
`1e-6 W m^-2 + 1e-10*max(1,sum(abs(component operands)))`; each water/vapor
residual uses `1e-12 kg m^-2 s^-1 + 1e-9*scale`; thermal-column residuals use
the energy rule. Acceptance additionally requires maximum temperature step
`<=1e-8 K`, hydraulic step `<=1e-7 mm`, exact request identities, and every
owner closure. Representation comparison never makes a rejected iterate
accepted.

Failures carry model/configuration/state/transaction/OFE/tile/occupancy/pass/
solve identity, iteration counts, ordered normalized residuals, step norm,
backtracking count, active bounds/caps, bracket or pivot evidence and complete
rollback hashes. No last iterate is a candidate.
