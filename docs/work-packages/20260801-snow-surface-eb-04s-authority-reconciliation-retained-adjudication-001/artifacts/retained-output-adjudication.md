# Retained-Output Scientific Adjudication

Evidence mode: `Ran` (analysis only; no model subprocess).

## Physical And Provenance Gate

- Immutable cells: `48` (`12 lanes x 4 cells`).
- Retained provenance file identities rechecked: `288`.
- EB-04R package and retained-output hashes before/after: `PASS`.
- Simulation subprocesses launched: `0`.
- Authority-bound vapor-to-sublimation tolerance: `1e-6 kg m^-2`.
- Maximum retained vapor-to-sublimation residual: `8.1099832877074007e-08 kg m^-2`.
- EB-04R-frozen vapor-aggregation tolerance: `1e-12 kg m^-2`.
- Maximum retained vapor-aggregation residual: `7.9936057773011271e-15 kg m^-2` (`PASS`).
- Frozen tool/protocol/source/fixture/observation/role/filter/selector/decision dependencies: `PASS`.
- Population physical/provenance gate: `PASS`.
- Observations loaded only after that complete gate: `True`.

## Unchanged Empirical Rule

Baseline B score/failures: `177` / `16`.

Combined LS score/failures: `180` / `16`.

The combined mechanisms increase the robust ordinal score but do not reduce
the robust failure count. The prospectively frozen rule requires both.

Decision: `CLOSE_NONPROMOTION_EMPIRICAL_RULE`.

Stop-loss invoked: `True`. Another calibration or
factorial round authorized: `False`.

This successor decision does not rewrite EB-04R. It is a separate retrospective
adjudication under authority frozen without result access.
