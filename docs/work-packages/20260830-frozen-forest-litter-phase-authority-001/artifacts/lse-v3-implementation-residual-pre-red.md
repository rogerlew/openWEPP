# LSE V3 phase-residual pre-red

Status: `CONFIRMED AND CORRECTED`

Evidence mode: `Static + Ran`

Static pre-red inspection confirmed two independent defects in the first V3
slice:

1. `publish_phase_free_litter_vapor` evaluated phase-specific vapor only after
   the covered V2 Newton solve, whose ground law and surface residual remained
   liquid-only.
2. `reconstruct_litter_phase_closure` checked separate vapor enthalpies and
   post-vapor/phase state, but had no independently supplied complete surface-
   energy operands and therefore could not reconstruct the inherited V2
   balance into `U*`.

The contract does not authorize the narrower proposed identity
`U*=U0-Q_v,l-Q_v,i`: absorbed shortwave, net longwave, sensible heat, and
ground conduction also own changes in sensible energy. SC-LSE v14 instead
inherits the complete V2 balance, implemented as

```text
U*-U0 = dt*(SW_abs + LW_net - H - Q_v,l - Q_v,i - G).
```

Focused positive/anti-mutant run:

```text
nix develop --command cargo nextest run \
  -p openwepp-land-surface-energy litter_phase
```

Result: `PASS`, run `3df50958-80bf-43cc-bfdd-5c06cc67a6b3`, 14/14. The run
includes phase-distinguishing ice-enthalpy alias, vapor-only storage, producer-
residual substitution, receipt replay, and failed-candidate nonmutation cases.
