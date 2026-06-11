# disposition

Status: executed-hold

Evidence mode: Static + Ran

Date: 2026-06-11

## Outcome

FDHP01 is executed but held. Post-review cohort validation showed that the
current implementation does not close the single-OFE executable frost-depth
heat-flow boundary under `SC-SNOWFREEZE-001#INV-SNOWFREEZE-006`.

Static:

- The retired freeze-index frost-depth proxy and `0.20 m` model cap were
  removed from production frost-depth progression.
- Frost depth now advances/thaws from an hourly signed heat-flow balance with
  separate surface heat loss (`Qsrf`) and lower-front unfrozen-soil heat input
  (`Quf`), bounded by physical profile depth.
- Frozen-water exchange fails closed on liquid overdraw and credits thawed
  storage back to liquid `wb11_soil_water`.
- Runtime `frost.runtime_frdp_m` is now required and published to WAT as
  `frdp` in `mm`; WAT dataset version is `1.4`.
- WAT `SoilWaterTotal` was corrected back to the hydout-equivalent
  `Total-Soil` alias; `frozwt` remains separately published and is not folded
  into `SoilWaterTotal`.
- `SC-WATBAL-001` v150 ratifies the pinned-baseline
  `frwatc.for`/`watbalprint.for` provenance: `Total-Soil` and
  `SoilWaterTotal` are unfrozen `watcon` aliases, while frost-active storage
  audits use `Total-Soil + frozwt`.
- Active frost exchange now publishes in-process `frwatc` diagnostics for
  liquid before/after, frozen before/after, freeze debit, thaw credit, and
  signed liquid delta.
- `SC-SNOWFREEZE-001`, `SC-WATBAL-001`, and the runner CLI specification were
  updated to carry the new authority/versioning.

Ran:

- `cargo fmt --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `cargo deny check` passed.
- `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` passed.
- D2 focused diagnostics passed:
  `cargo test --test clim06_frost_frozen_soil_kernel_contract --
  --nocapture` (17 tests), plus the HPHYS0319/HPHYS0320
  contract-version guards after `SC-WATBAL-001` v150.
- Fresh 43-prefix `algebraic-radium` frost-on cohort run failed closure:
  `42/43` clean exits, `p2` failed before WAT publication at
  `HKERNEL-WB11-PERC-E-003` on `1990-308`.

## Hold Evidence

- Run root:
  `/tmp/fdhp01_closure_after_d1_restored_20260611T053545Z`.
- Persisted compact reports:
  - `fdhp01_closure_summary_20260611.json`
  - `fdhp01_run_status_20260611.tsv`
  - `fdhp01_activation_summary_20260611.csv`
  - `fdhp01_annual_closure_residuals_20260611.csv`
  - `fdhp01_depth_metrics_20260611.csv`
- Emitted-prefix annual closure max abs residual:
  `2.4798612273409617 mm` versus the FROSTVAL01 baseline
  `3.2173375075217336e-11 mm`; the pre-D1 post-review residual was
  `75.43917280313423 mm`.
- Emitted-prefix median depth correlation: `-0.10301692862035305` versus
  FDMC01 pre-fix proxy median `0.13332765680932177`.
- Emitted-prefix mean max depth: `1782.2670980346527 mm` versus matched
  legacy mean `417.4166666666667 mm`.
- `SC-SNOWFREEZE-001` v55 reopens `GAP-SNOWFREEZE-002` as an active defect.

## D2 Disposition

D2 exchange wiring is now observable in process. Focused freeze-onset and
warm-thaw vectors prove the exchange algebra is symmetric at the WB14/WB11
seam. The remaining WAT-level inconsistency must be audited against those
diagnostics before changing production hydrology or WB13 publication semantics
again; the `SWT`-only annual near-closure remains non-authoritative as a
completion claim.

## Review Closure

Claude's post-closeout review findings are accepted. `complete` was incorrect;
the package remains in defect closure. The next actionable item is to close
`FDHP01-FROST-DEPTH-HEATFLOW-001` by fixing the cohort failure, restoring
annual closure from the remaining `~2.48 mm` residual to numerical noise, and
materially closing the FDMC01 depth/duration gap. D3 depth-runaway and the
independent `p2` fail-closed defect remain downstream of the now-instrumented
D2 gate.
