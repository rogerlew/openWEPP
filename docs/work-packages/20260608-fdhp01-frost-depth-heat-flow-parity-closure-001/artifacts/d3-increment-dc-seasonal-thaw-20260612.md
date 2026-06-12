# D3 Increment Dc Seasonal Heat/Thaw Attempt Evidence

Status: executed-hold, backed out

Evidence mode: Static + Ran

Date: 2026-06-12

## Scope

Increment Dc attempted to land the staged-plan F1/F2 fixes in one increment:

- Replace the synthetic lower-front heat floor (`max(7 degC, daily air
  midpoint)` over a fixed 1 m path) with the legacy seasonal `tmpcft`/`tmpbl`
  damped wave and zero gate.
- Rebuild top/bottom thaw helpers to recompute thaw resistance within the hour,
  including a surface-to-thaw-front top-thaw path.
- Add the contract/test coverage for those behaviors.

The production, contract, and test edits from this attempt were backed out
because the increment violated the staged plan's D2 hard stop and failed the D3
depth/duration gate. The committed tree remains at the Increment Db production
boundary (`SC-SNOWFREEZE-001` v63).

## Local Evidence

Ran:

| Command / Gate | Result |
|---|---|
| `cargo test --test clim06_frost_frozen_soil_kernel_contract fdhp01_dc_ -- --nocapture` before production edits | Failed as expected: synthetic `qdry` floor produced `14.7 W/m2` where Dc expected zero-gated seasonal heat; one-hour thaw advanced too far. |
| `cargo fmt --check` after Dc implementation | Pass |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract fdhp01_dc_ -- --nocapture` after Dc implementation | Pass, 3 tests |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture` after Dc implementation | Pass, 37 tests |
| `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` after Dc implementation | Pass, failed-attempt binary SHA `772b5778f710619c7e1a99da82c69417d67c8ee15a3ff0a9bb44afe2886e3e21` |
| Focused p35 guard rerun | Initially failed on `frost.runtime_slsw_theta_0002_0006=0.1749999999716694` below residual by `2.9e-11`; a bounded fine-theta boundary canonicalization removed the p35 guard failure. |
| Temporary diagnostic source search | Pass after backout; no `FDHP01_DC_DIAG` diagnostic remains in source. |
| Comparator-suite runner | Not used; user explicitly requested no comparator subagent because GPT-5.3-Codex-Spark weekly quota was exhausted. Parent ran local CLI/DuckDB/PyArrow comparisons. |

The focused p35 finding is numeric texture at the fine-layer theta lower bound,
not a science relaxation. It is recorded because the same narrow canonicalizer
will be needed if Dc is reintroduced, but it was backed out with the rest of the
failed production diff.

## Cohort Evidence

Run root:
`/tmp/fdhp01_increment_dc_cohort_20260612T062840Z`.

Generated reports copied into this package:

- `fdhp01_increment_dc_execution_summary_20260612.json`
- `fdhp01_increment_dc_run_status_20260612.tsv`
- `fdhp01_increment_dc_annual_closure_residuals_20260612.csv`
- `fdhp01_increment_dc_depth_metrics_20260612.csv`
- `fdhp01_increment_dc_frozwt_frdp_ratio_20260612.csv`
- `fdhp01_increment_dc_activation_summary_20260612.csv`

The cohort ran `43/43` clean and emitted `43/43` WAT files, so the failure is
not a runtime guard failure. It is a physics/accounting gate failure:

- Years 2-6 independent `Total-Soil + frozwt` additive-storage residual:
  max abs `0.2706094484356498 mm` (`p34`, year 2). This materially regresses
  Db's accepted WAT-publication texture (`1.9976620946327017e-07 mm`) and
  violates the D2 hard stop.
- p1/p20 spot checks also regress from `~1e-13 mm` to
  `0.022261272887243777 mm` and `0.023365382872134077 mm`, respectively.
- p43 year 2 regresses to `-0.24479853886504088 mm`.
- Depth grows too deep: mean maximum depth `1062.5086535449198 mm`, median
  `1044.4140627263175 mm`, range `763.4002205550781..1799.9999999999998 mm`.
- Profile-bound pinning returns for one prefix (`1/43`), with minimum margin
  `2.2737367544323206e-13 mm`.
- No prefix remains inside the legacy `240..503.2 mm` maximum-depth envelope
  (`0/43`).
- Depth correlation improves materially to median `0.6595441080376998`, but
  this is not acceptable because conservation and the depth envelope fail.
- Frozen duration flips from Db's under-persistence to severe over-persistence:
  median open-minus-legacy `+751` days, range `+724..+794`.
- Days above `200 mm` also regress: full-WAT median `1306` days.

## Disposition

Increment Dc failed before package acceptance. F1/F2 as implemented are not a
valid one-increment close: they improve timing correlation but reopen D2
closure and push depth/duration past the physical envelope. The failed run
localizes the next work to a smaller split:

- Keep the seasonal `tmpbl` lower-front heat expression as a plausible authority
  target, but reintroduce it behind an accounting-preserving gate before thaw
  dynamics are changed.
- Investigate the `~0.27 mm` years 2-6 additive-storage residual introduced by
  the Dc thaw/seasonal coupling; do not proceed to duration tuning while that
  residual exists.
- Preserve Db's accepted boundaries: independent WAT ledger at
  `~2e-7 mm`, zero production profile pins, all prefixes inside the maximum
  depth envelope, and all C1b/C2 capacity/overflow guards.

All Increment Dc production, contract, and test edits were backed out. FDHP01
remains `executed-hold` at the Increment Db boundary. The next increment should
split Dc into an accounting-preserving seasonal heat reintroduction and a
separate thaw-timing pass only after the D2 ledger remains at the Db floor.
