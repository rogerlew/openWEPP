# Snowfreeze Observed Harness

Observation-backed frost-depth validation tooling for
`SC-SNOWFREEZE-001 INV-SNOWFREEZE-047`.

Normal tests are offline and use the checked-in normalized corpus under
`tests/fixtures/snowfreeze_observed/observations/`.

Refresh the local raw cache explicitly:

```bash
.venv/bin/python tools/snowfreeze_observed/observed_harness.py \
  --cache target/snowfreeze_observed fetch
```

Regenerate and validate the normalized corpus:

```bash
.venv/bin/python tools/snowfreeze_observed/observed_harness.py \
  --cache target/snowfreeze_observed normalize \
  --observations-dir tests/fixtures/snowfreeze_observed/observations

.venv/bin/python tools/snowfreeze_observed/observed_harness.py \
  validate --observations-dir tests/fixtures/snowfreeze_observed/observations
```

Run one comparison:

```bash
.venv/bin/python tools/snowfreeze_observed/observed_harness.py \
  compare --site site1_sleepers_south_field_vt \
  --observations-dir tests/fixtures/snowfreeze_observed/observations \
  --output-dir target/snowfreeze_observed_compare_site1
```

`compare` defaults to the `direct-production-executor` runtime surface.
`--runtime compatibility` is available only for flagging/debugging and is not
the observation acceptance target.

The comparison verdict remains `UNRESOLVED` while mechanism attribution is
blocked. WAT `Snow-Depth` is the modeled snow-depth diagnostic for
`TOL-SNOWFREEZE-009`; WAT `Snow-Water` is SWE and is not a snow-depth
diagnostic.

Classify frost residuals and audit snow-depth fidelity:

```bash
.venv/bin/python tools/snowfreeze_observed/classify_residuals.py \
  --observations-dir tests/fixtures/snowfreeze_observed/observations \
  --output-json target/snowfreeze_residual_classification.json \
  --output-md target/snowfreeze_residual_classification.md \
  target/snowfreeze_observed_compare_*/comparison_report.json

.venv/bin/python tools/snowfreeze_observed/snow_depth_audit.py \
  --observations-dir tests/fixtures/snowfreeze_observed/observations \
  --output-json target/snowfreeze_snow_depth_audit.json \
  --output-md target/snowfreeze_snow_depth_audit.md \
  target/snowfreeze_observed_compare_*/comparison_report.json
```

The snow-depth audit is bound to
`SC-SNOWFREEZE-001 INV-SNOWFREEZE-048`. It publishes signed residual
direction, modeled-over/under-observed counts, adjacent-day timing checks, and
depth-vs-SWE anti-alias checks. Failed snow control blocks frost attribution
and routes to snow-depth fidelity unless the audit reports a correspondence
blocker.

Generate SNOWFROST-FIDELITY-C diagnostic SFCC/frozen-K comparison curves:

```bash
.venv/bin/python tools/snowfreeze_observed/frozen_k_diagnostics.py \
  --output-json target/snowfrost_fidelity_c/diagnostics.json \
  --output-md target/snowfrost_fidelity_c/diagnostics.md
```

The frozen-K diagnostics are research/comparison surfaces only. They do not
feed production snow/frost runtime physics, do not select texture defaults,
and do not authorize `Qwet`.

Compare pinned legacy WEPP snow outputs with current openWEPP and observed
snow-depth rows:

```bash
.venv/bin/python tools/snowfreeze_observed/legacy_snow_compare.py \
  --observations-dir tests/fixtures/snowfreeze_observed/observations \
  --output-dir target/snowfrost_fidelity_f_legacy_compare \
  --output-json target/snowfrost_fidelity_f_legacy_compare.json \
  --output-md target/snowfrost_fidelity_f_legacy_compare.md
```

The legacy comparison helper treats legacy agreement as flag evidence only.
Legacy WAT `Snow-Water` is SWE, not physical depth. Date-aligned legacy
physical snow depth is captured from temporary replay daily-winter hour-24
rows; large graphics `treal(73)=snodpy*1000` and `treal(75)=densg` are retained
as sparse operand provenance.

Generate PySnobal-ready diagnostic forcing from WEPP/openWEPP inputs:

```bash
cargo run -p openwepp-runner --bin openwepp-snowbench -- export-pysnobal \
  --run-dir tests/fixtures/snowfreeze_observed/site1_sleepers_south_field_vt \
  --output-dir target/snowfrost_fidelity_g0/site1
```

Run PySnobal against exported G0 forcing directories:

```bash
PYSNOBAL_PYTHON=/tmp/pysnobal-g0-venv/bin/python \
  .venv/bin/python tools/snowfreeze_observed/pysnobal_compare.py \
  --input-root target/snowfrost_fidelity_g0 \
  --output-json target/snowfrost_fidelity_g0/pysnobal_site_summary.json \
  --output-md target/snowfrost_fidelity_g0/pysnobal_site_summary.md
```

The G0 bridge is diagnostic only. It uses SIMIMPL28 hourly forcing lineage for
air temperature, radiation, rain/snow partition, and `snow.txt` new-snow
density, then labels longwave, net-shortwave, precipitation temperature, and
constant ground-temperature lanes as proxies. PySnobal output is not openWEPP
correctness authority.
