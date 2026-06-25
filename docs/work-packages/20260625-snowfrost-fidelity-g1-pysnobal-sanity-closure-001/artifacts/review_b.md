# Review B: Rust WAT Projection and Anti-Alias

Evidence mode: Static + Ran.

Reviewer: local Codex review pass B.

Scope:

- `crates/openwepp-runner/src/hillslope/snowbench.rs`;
- `crates/openwepp-runner/src/bin/openwepp-snowbench.rs`;
- `tests/integration/snowfrost_fidelity_g0_pysnobal_bridge_contract.rs`;
- WAT-backed `openwepp_snow.csv` artifacts.

Findings:

1. accepted: projecting openWEPP comparison rows from WAT is the correct
   anti-alias surface. The implementation reads published `Snow-Water` and
   nullable `Snow-Depth` from WAT parquet rather than recomputing snowpack depth
   inside the exporter.

2. accepted: WAT year/month/day fields are not the external date authority for
   this diagnostic CSV. The implementation now maps `sim_day_index` to
   `DailyForcingExport.date`, and fails closed if a WAT row points outside the
   available climate-date range.

3. accepted: the focused projection test avoids forcing normal integration
   tests to run a full 45-year compatibility simulation while still proving
   `Snow-Water` and `Snow-Depth` millimeter values are preserved from WAT.

4. follow-up: the exporter is slow and silent while writing multi-decade hourly
   CSVs. This is not a correctness blocker for G1, but the next snowbench
   tooling package should add progress logging or a row-window export mode if
   PySnobal iteration continues.

Disposition: one follow-up is non-blocking and recorded in the worker handoff;
no accepted finding remains unresolved.
