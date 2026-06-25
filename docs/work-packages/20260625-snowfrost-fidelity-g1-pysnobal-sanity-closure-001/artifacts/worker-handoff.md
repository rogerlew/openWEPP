# Worker Handoff

Status: complete.

Evidence mode: Static + Ran.

Next recommended route: SNOWFROST-FIDELITY-G2 PySnobal diagnostic assessment.

First actionable task: close a comparison/adjudication package that compares
G1 PySnobal selected-lane snow depth against observed snow-depth controls,
current openWEPP WAT `Snow-Depth`, and the pinned legacy snow-depth assessment
from SNOWFROST-FIDELITY-F. The package must keep PySnobal as diagnostic
hypothesis evidence only and must not promote PySnobal as correctness authority
or production dependency.

Required inputs:

- `artifacts/pysnobal_site_sane_summary.json`;
- `target/snowfrost_fidelity_g1/site*/openwepp_snow.csv`;
- SNOWFROST-FIDELITY-E snow-depth correspondence/direction audit;
- SNOWFROST-FIDELITY-F legacy snow-depth output capture;
- `SC-SNOWFREEZE-001#INV-SNOWFREEZE-048`.

Non-blocking tooling follow-up:

- `openwepp-snowbench export-pysnobal` is slow and silent while writing
  multi-decade hourly forcing CSVs. Add progress logging or a row-window export
  mode before broad PySnobal sensitivity sweeps.

Do not:

- classify openWEPP snow/frost as defective from PySnobal agreement alone;
- tune production snow/frost physics from PySnobal residuals;
- use the Morris `Tg=-0.5 degC` failed sensitivity lane as a blocker for the
  selected-lane site-sane route;
- add PySnobal as a required Rust workspace or production runtime dependency.
