# Implementation Evidence

Evidence mode: Static/Ran.

## Rust Changes

- Added `SnowMeltModel::CoeWinterThawStateLossV1` with id
  `coe_winter_thaw_state_loss_v1`.
- Added `SnowMeltModel::requires_snow_albedo_state()` and kept albedo required
  only for `CoeShortwaveAlbedoV1`.
- Updated hourly albedo carry so non-albedo melt models return no albedo state.
- Added `CoeMeltModel::CoeWinterThawStateLossV1` to snowbench parser/reporting.
- Updated `openwepp-snowbench` help text to list the diagnostic selector.
- Added retained/released-rain and active-ledger residual operands to snowbench
  CoE melt diagnostic rows.
- Added `OPENWEPP_SNOWDENSITY1037_MELT_MODEL` as a package-bound
  direct-production diagnostic selector for coupled WAT evidence only; absent/
  empty preserves `legacy_coe`, unknown values fail closed, and no parser/
  runfile/user CLI activation was added.
- Added melt-model and water-routing operands to direct snow trace rows for
  coupled WAT proof.
- Mechanically split the direct-production snow/frost authority impl and local
  direct-publication source-guard tests into include fragments to satisfy
  line-count governance without changing compiled module scope or behavior.
- Added the low-density positive-thaw branch delta only for
  `SnowMeltModel::CoeWinterThawStateLossV1`: when legacy would retain positive
  `wmelt` below the `350 kg m^-3` density gate, the candidate emits that positive
  melt as snowpack state loss/routed melt while preserving non-negative snow
  state.

## Focused Test Evidence

- Ran: `cargo test --test snowdensity10_3_7_winter_thaw_melt_response_correction -- --nocapture`
  after diagnostic artifacts were generated.
- Result: PASS after rerun; see `gate-results.md`.

The focused test proves selector parsing, contract/package markers, albedo
isolation, default low-density legacy gate behavior, opt-in low-density thaw
state loss, conservation/routing closure, package-bound coupled WAT diagnostic
plumbing, and diagnostic report markers.
