# Independent V6 Test-Vector Ledger

Evidence: `Static + Ran`

| Vector | Expected result | Guard |
|---|---|---|
| observed CPython/Rust `step_norm` | pass | finite same-field rejected trajectory inside bound |
| exact largest representable boundary | pass | inclusive `<=` rule |
| one representable value inside | pass | interior behavior |
| first representable value outside | fail | boundary cannot expand silently |
| reversed observed operands | pass | comparison symmetry |
| lower-side boundary | pass | both directions around reference covered |
| negative `step_norm` pair | fail | max-absolute norm is nonnegative by definition |
| `+0.0` versus `-0.0` | pass | signed zeros are one exact zero class |
| `0.0` versus minimum positive subnormal | fail | zero/nonzero cannot be laundered |
| positive versus negative | fail | sign class exact |
| NaN metadata | fail | nonfinite evidence prohibited |
| positive/negative infinity metadata | fail | nonfinite evidence prohibited |
| null versus present | fail | optional shape exact |
| wrong field/solve/acceptance posture | fail | scope identity exact |

The independent generator finds the largest passing binary64 value and uses
`nextafter` to bind the first representable outside value. Rust output is not
used to generate expected data.
