# Disposition

Evidence mode: Static + Ran.

Final disposition: COMPLETE as characterization.

Closed scope:

- Scaffolded SNOWFROST-FIDELITY-A.
- Added `tools/snowfreeze_observed/classify_residuals.py`.
- Ran all five pilot sites through the direct observed harness.
- Generated residual classification artifacts.
- Classified zero sites as eligible for frost-defect attribution.

Package conclusion:

- `site1_sleepers_south_field_vt`, `site2_sleepers_w9_hardwood_vt`, and
  `site4_ggd498_morris_mn` are `SNOW-CONTROL-BLOCKED`: frost-tube residuals are
  visible, but observed snow depth cannot be paired with modeled snow depth.
- `site3_scan_mandan_nd` and `site5_reynolds_creek_us_rls_id` are
  `INCONCLUSIVE`: soil-temperature isotherm upper-bound signals are present, but
  they are not magnitude targets and snow-control is missing.
- No site is `OPENWEPP-DEFECTIVE`.

Direct consequence:

The next work must expose modeled snow depth and rerun this classification
before field residuals are attributed to frost physics. No snow/frost physics
change, `Qwet` activation, or direct default activation is authorized by this
package.
