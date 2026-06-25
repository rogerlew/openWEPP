# Pre-Implementation Evidence

Evidence mode: Static.

## Existing State

- SNOWFROST-FIDELITY-A left all pilot sites `UNRESOLVED` because modeled snow
  depth is absent.
- SNOWFROST-FIDELITY-B added no-migration heat-flow benchmarks and confirmed no
  production `crates/` `qwet`, `Qwet`, or `frzftp` implementation.
- Current `SC-SNOWFREEZE-001` production frost authority still binds fixed
  `kftill`/`kfutil` frozen surface-path constants and legacy lower-front
  unfrozen `kufz`, not an SFCC/frozen-K production model.

## Diagnostic Boundary

The C package may add a research/comparison tool only. It cannot feed WB12,
WB14, WB18, WB19, WAT/HBP/PASS, direct runtime, compatibility runtime, or field
classification decisions.

## Source Constraint

The diagnostic must be discoverable by explicit tool/test paths and absent from
production Rust crates. The closure source scan is:

`rg -n "frozen_k_diagnostics|sfcc_mualem|clapeyron_unfrozen|diagnostic_fixture" crates -S || true`
