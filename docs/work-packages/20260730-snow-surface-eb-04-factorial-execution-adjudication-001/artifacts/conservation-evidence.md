# Conservation Evidence

Status: `partial pass / package hold / candidate rejected`

Evidence mode: `Ran`

For every retained successful or pre-failure trace row, the package consumer
reconstructed:

- `SWE_before + accumulation + rain_retained - sublimation
  - snowpack_SWE_loss - SWE_after`;
- Stage 3 cold-content closure using the producer-carried surface total,
  conduction, refreeze, and cold-content-export operands; and
- producer-carried latent/mass residual magnitude.

All completed cells and retained partial rows remained within the prospective
`1e-9 m`, `1e-6 J m^-2`, and `1e-6 J m^-2` tolerances before the typed rejected
step. Only the mass check is independently reconstructable. The cold-content
check confirms internal consistency downstream of the published surface total,
but it is not an independent reconstruction of that surface total. Likewise,
the latent/mass check audits the producer-carried residual; the trace does not
publish the signed per-step vapor mass and latent conversion operands required
for an anti-tautological reconstruction.

The trace does not publish shortwave. Therefore the preregistered full
surface-component reconstruction cannot be performed from the frozen retained
round. Independent latent/mass reconstruction also cannot be performed from
the retained operands. Those evidence gaps place the package on HOLD. This
does not rehabilitate failed cells: their full-run physical gate is failed,
and no downstream scientific score is accepted.
