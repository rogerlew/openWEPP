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

Generate SNOWFROST-FIDELITY-C diagnostic SFCC/frozen-K comparison curves:

```bash
.venv/bin/python tools/snowfreeze_observed/frozen_k_diagnostics.py \
  --output-json target/snowfrost_fidelity_c/diagnostics.json \
  --output-md target/snowfrost_fidelity_c/diagnostics.md
```

The frozen-K diagnostics are research/comparison surfaces only. They do not
feed production snow/frost runtime physics, do not select texture defaults,
and do not authorize `Qwet`.
