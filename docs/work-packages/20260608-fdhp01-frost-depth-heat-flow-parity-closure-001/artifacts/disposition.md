# disposition

Status: executed-hold

Evidence mode: Static + Ran

Date: 2026-06-11

## Outcome

FDHP01 is executed but held. The D2 storage/publication defect is closed by the
layered-state continuation, while the D3 frost-depth parity defect remains
open under `SC-SNOWFREEZE-001#INV-SNOWFREEZE-006`.

Static:

- The retired freeze-index frost-depth proxy and `0.20 m` model cap were
  removed from production frost-depth progression.
- Frost depth advances/thaws from an hourly signed heat-flow balance with
  separate surface heat loss (`Qsrf`) and lower-front unfrozen-soil heat input
  (`Quf`), bounded by physical profile depth.
- Active frost now carries per-layer `wb18_perc_frozen_depth_####` and
  `wb18_perc_frzw_####` state. WAT `frozwt` is bound to the legacy
  `Σ soilf(i)` lineage:
  `Σ(wb18_perc_frzw_#### + thetdr_#### * wb18_perc_frozen_depth_####)`.
- Freezing transfers active liquid from the affected layer into `frzw`; thawing
  returns active ice to liquid storage. Layer capacity/domain violations fail
  closed.
- WAT `SoilWaterTotal` remains the hydout-equivalent unfrozen `Total-Soil`
  alias; frost-active storage audits use `Total-Soil + frozwt` under
  `SC-WATBAL-001` v152.
- `SC-SNOWFREEZE-001` v57 rejects scalar `frdp * theta` frozen-water stores,
  prohibits post-hoc scalar depth projection into layer stores, and records
  that full depth parity still requires the fine-sublayer frost port.

Ran:

- `cargo fmt --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `cargo deny check` passed.
- `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` passed.
- `cargo test --test clim06_frost_frozen_soil_kernel_contract --
  --nocapture` passed, `19` tests.
- `cargo test --test hphys0319_fixed_baseline_stmtim_observe_contract --
  --nocapture` passed after `SC-SNOWFREEZE-001` v57 /
  `SC-WATBAL-001` v152 updates.
- `cargo test --test hphys0320_stmtim_start_time_source_line_contract --
  --nocapture` passed after the same version updates.
- `cargo test -p openwepp-runner --lib fdhp01_wb13 -- --nocapture` passed.
- Fresh 43-prefix `algebraic-radium` frost-on cohort run:
  `/tmp/fdhp01_layered_store_20260611T080722Z`, `43/43` clean exits.

## Closure Evidence

- Persisted compact reports:
  - `fdhp01_layered_closure_summary_20260611.json`
  - `fdhp01_layered_execution_summary_20260611.json`
  - `fdhp01_layered_run_status_20260611.tsv`
  - `fdhp01_layered_activation_summary_20260611.csv`
  - `fdhp01_layered_annual_closure_residuals_20260611.csv`
  - `fdhp01_layered_depth_metrics_20260611.csv`
  - `fdhp01_layered_frozwt_frdp_ratio_20260611.csv`
- Clean frost-on prefixes: `43/43`.
- `p2` no longer reproduces the prior `HKERNEL-WB11-PERC-E-003` fail-closed
  event.
- Annual `Total-Soil + frozwt` identity max abs residual:
  `1.2683574368566042e-07 mm`.
- Mean abs annual `Total-Soil + frozwt` residual:
  `2.1277404919798806e-09 mm`.
- Soil-only annual identity now fails as expected under the ratified additive
  storage term: max abs residual `119.04111532237937 mm`.
- `frozwt/frdp` is no longer an exact scalar audit: `36064` active rows,
  minimum per-prefix correlation `0.8210678396408895`, median correlation
  `0.963536279373424`, maximum ratio standard deviation
  `0.0700106996666242`.

## Hold Evidence

- OpenWEPP maximum frost depth remains near the physical profile bound:
  mean `1782.0379909380451 mm`, range
  `1780.3226093850215..1783.3684719591117 mm`.
- Matched legacy mean maximum frost depth is `414.22093023255815 mm`; mean
  open-minus-legacy max-depth delta is `1367.8170607054872 mm`.
- Median depth correlation is `-0.27756218032931956`.
- All `43/43` prefixes exceed `200 mm`; this proves the retired proxy cap is
  gone, but the depth progression is still not legacy-envelope faithful.
- OpenWEPP frozen-day count is `518.5348837209302` days lower than legacy on
  average.

## D3 Attempt Evidence

- A coarse continuous per-layer energy-front attempt was run from dirty commit
  `8d13ba898a111aeed97375cc997d0ca65d6b85b7` into
  `/tmp/fdhp01_d3_layered_energy_20260611T085142Z`.
- The attempt ran `43/43` clean and improved median maximum depth to
  `490.774886655666 mm`, but did not meet the package phase boundary:
  mean max depth remained `643.2973898432339 mm`, max depth remained
  `1789.9130899451595 mm`, median correlation was `-0.1876255663636445`,
  and median frozen-duration delta was `-428` days.
- Static legacy inspection localized the missing behavior to the fine-sublayer
  `frostn` state machine: `frostn.for:360-683`, `frzng.for:235-560`, and
  `frwatc.for:89-137`.
- The coarse-front production/test edit was backed out. `SC-SNOWFREEZE-001`
  v57 remains as the active D3 authority amendment; no D3 production behavior
  landed in this pass.

## Review Closure

Claude's post-closeout review findings are accepted. The diagnostic conclusion
from addendum 2e was correct: the scalar model could not satisfy the v150/v151
audit identity by construction. This pass landed the first legitimate phase
line: a layered frozen store with clean additive storage closure and full
43-prefix execution. The package still cannot be marked complete because D3
depth/duration parity is open. The next actionable item is to close
`FDHP01-FROST-DEPTH-HEATFLOW-001` by completing the layered
thermal-resistance/depth-progression port, then rerunning the same cohort and
identity gates.
