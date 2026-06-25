# Worker Handoff

Status: `COMPLETE-SNOWDENSITY-03-OFFLINE-PHYSICS-CORE`.

Next route: `SNOWDENSITY-04 Offline Adjudication Loop`.

## What Exists

- Offline `physics_bulk_candidate_v1` implementation in Rust snowbench.
- CLI command:

```bash
target/debug/openwepp-snowbench physics-bulk \
  --run-dir tests/fixtures/snotel_observed/snotel_mica_creek_st_joe_id \
  --output-dir target/snowdensity03_probe
```

- Five-site SNOTEL profile harness:

```bash
.venv/bin/python tools/snowfreeze_observed/physics_bulk_snotel_profile.py \
  --observations-dir tests/fixtures/snotel_observed/observations \
  --output-dir target/snowdensity03_physics_bulk \
  --snowbench-binary target/debug/openwepp-snowbench
```

- Committed baseline profile artifacts:
  `physics-bulk-snotel-profile.json` and
  `physics-bulk-snotel-profile.md`.

## Current Evidence

- The candidate runs all five SNOTEL fixtures without site constants.
- It emits finite daily SWE/depth/density/cold-content/liquid-release series.
- It preserves mass closure in focused tests and fixture summaries.
- It is confined to snowbench/diagnostic surfaces.
- It does not yet beat the rubric: forcing-robust counts are `fail=24`,
  `marginal=13`, `pass=3`, `strong=5`, `unavailable=15`.

## SNOWDENSITY-04 Instructions

- Iterate only inside the ratified `INV-SNOWFREEZE-051` physics envelope.
- Do not add per-site constants, SSD fitting, default activation, runtime parser
  coupling, or production publication changes.
- Compare candidate profiles against legacy/openWEPP/PySnobal evidence using the
  v74/v75 rubric profile, not a scalar score.
- Treat PySnobal and legacy as comparators, not acceptance authority.
- If the bulk envelope cannot materially improve forcing-robust cells, close
  with a documented fail route and recommend the smallest next physics escalation
  such as two-layer thermal state, not arbitrary coefficient tuning.
