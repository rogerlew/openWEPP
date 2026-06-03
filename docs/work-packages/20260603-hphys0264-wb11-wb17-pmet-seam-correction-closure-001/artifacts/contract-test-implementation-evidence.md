# Contract-Test Implementation Evidence

Status: completed

Evidence mode: Static + Ran

Static:

- Added orchestrator contract test
  `hphys0264_pmet_evapotranspiration_consumes_evappm_components_without_pt_repartition`
  to prove PMET mode consumes `pmet.es_m`/`pmet.ep_m` and rejects the old
  Priestley-Taylor partition result.
- Added orchestrator contract test
  `hphys0264_pmet_evapotranspiration_preserves_negative_evappm_soil_evaporation`
  to prove signed EVAPPM `es` remains signed `Es` and is returned to top-layer
  storage.
- Added summary-accumulator contract test
  `wb13_row_allows_negative_soil_evaporation_only_for_evappm_pmet_branch` to
  keep signed `Es` branch-scoped.

Ran:

- Pre-implementation red gate failed as expected after contract/test authoring:
  `cargo test -p openwepp-hillslope-orchestrator hphys0264_pmet_evapotranspiration_consumes_evappm_components_without_pt_repartition -- --nocapture`.
- Post-implementation focused tests passed:
  `cargo test -p openwepp-hillslope-orchestrator hphys0264 -- --nocapture`.
- Post-implementation summary test passed:
  `cargo test -p openwepp-summary-accumulator wb13_row_allows_negative_soil_evaporation_only_for_evappm_pmet_branch -- --nocapture`.
