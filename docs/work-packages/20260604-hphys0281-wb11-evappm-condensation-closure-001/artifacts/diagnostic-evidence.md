# Diagnostic Evidence

Status: completed
Evidence mode: static + ran

## Pre-Implementation Reproduction

Ran: `cargo test -p openwepp-runner hphys0281 -- --nocapture` after contract/test authoring and before production edits.

Observed failures:
- `hphys0281_wb11_evappm_seed_publishes_condensation_storage_return`: `pmet.es_storage_return_m` missing.
- `hphys0281_wb13_publication_canonicalizes_roundoff_negative_es_without_evappm_clamp`: WB13 rejected within-tolerance negative `Es` unless EVAPPM branch clamp mode was present.

Ran: `cargo test -p openwepp-hillslope-orchestrator hphys0281 -- --nocapture` after contract/test authoring and before production edits.

Observed failure:
- `hphys0281_pmet_evapotranspiration_applies_condensation_storage_return`: top-layer storage did not include explicit PMET condensation return.

## Characterized Vector

Static + computed locally from the HPHYS0281 cold-day vector:
- `tdpt=-1.0 C`, `tmax=-1.6 C`, so `ed=0.567751891176205 kPa > emaxt=0.543231628934308 kPa`.
- `ee=0.370052901424082 kPa`, so `(ee - ed) < 0`.
- `etorc_mm=-0.525767777513258`.
- `potes_m=-0.000068293015672`.
- Required PMET storage return is `0.000068293015672 m` with published `pmet.es_m = 0`.

## Contract Authority

Static: pinned `/workdir/wepp-forest_260430_baseline/src/evappm.for:461-472` computes `xx = es - resint`, clears `resint`, and when `xx < 0` returns `-xx` to `st(1,iplane)` instead of extracting soil water. HPHYS0281 implements this in openWEPP as non-negative `pmet.es_m` plus positive `pmet.es_storage_return_m`, consumed by WB17 during the ET phase so earlier hydrology phases do not see premature seed-time storage mutation.
