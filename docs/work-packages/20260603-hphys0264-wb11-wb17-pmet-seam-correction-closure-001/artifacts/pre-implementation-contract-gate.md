# Pre-Implementation Contract Gate

Status: completed

Evidence mode: Ran

Ran:

- Command:
  `cargo test -p openwepp-hillslope-orchestrator hphys0264_pmet_evapotranspiration_consumes_evappm_components_without_pt_repartition -- --nocapture`.
- Initial compile attempt exposed a test adapter mismatch and was corrected
  before recording behavioral gate evidence.
- Behavioral red gate failed before production code edits with:
  `PMET mode must pass pmet.ep_m to SWU as Etp, observed 0.0015999999999999999`.

Interpretation:

- The pre-existing WB17 seam still applied the non-PMET
  Priestley-Taylor/LAI partition to migrated PMET demand.
- The failure was contract-relevant because the expected PMET `pmet.ep_m` was
  `0.0034 m`, while the old partition produced `0.0016 m`.
