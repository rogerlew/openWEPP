# Active Input Preflight

Status: EXECUTED-HOLD-ROUTE-COEFFICIENT-AUTHORITY. Evidence mode: Static + Ran.

## External OpenWEPP Runfile Availability

Ran:

```text
find /wc1/runs/al/algebraic-radium /wc1/runs/un/unpalatable-rind /wc1/runs/ar/arboreal-dendrite -name '*.run.toml' | wc -l
0
```

The external roots are WEPPcloud/legacy input roots, not ready-to-run
openWEPP active `*.run.toml` roots.

## Focused Active Fail-Closed Guard

Ran:

```text
cargo test -q --test laned_shadow_h2637 h2637_active_fails_closed_without_routing_coefficients

running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.24s
```

This confirms the active selector still fails closed when native
`routing_coefficients` are missing.

## H2637 Scope Note

H2637 can be made active-runnable in package/test scratch by applying the
existing native-cropland patch recipe used by earlier Lane-D timing packages:
datver `ow-lanuse-1`, `landuse=4`, and `routing_coefficients` blocks
`500.0 0.0 0.0 0.0 0.0`. That vector remains useful for H2637 regression and
timing evidence, but it is not broad cohort authority and cannot ratify default
promotion tolerances by itself.

## Result

No selected external cohort member is currently active-runnable under
source-authorized route-coefficient inputs.
