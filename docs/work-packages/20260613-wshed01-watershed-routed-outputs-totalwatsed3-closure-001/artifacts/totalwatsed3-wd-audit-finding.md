# W-D totalwatsed3 Audit Finding

Status: W-D executed-hold

Evidence mode: Ran + Static

## Summary

W-D found and corrected three openWEPP publication defects in the W-C
`totalwatsed3.parquet` producer:

- Exact totalwatsed3 volume columns (`P`, `RM`, `Q`, `Dp`, `latqcc`, `QOFE`,
  `Ep`, `Es`, `Er`) were emitted as mm depths despite schema metadata declaring
  `m^3`. Depth aliases remain mm.
- MOFE `latqcc` was summed across internal OFEs. The producer now counts only
  the outlet-facing OFE per WAT file/day/`wepp_id` when OFE identifiers are
  present.
- Optional WAT profile/interception fields were available in source shards but
  not published, causing false profile-cap audit violations and missing
  interception in closure.

These repairs are keepable, but W-D does not meet the conservation acceptance
gate. The independent wepppy audit still reports a material positive residual,
so the increment remains held.

## Audit Evidence

Configured run:

```text
cargo run -q -p openwepp-runner --bin openwepp-cli-watershed -- \
  --run-dir /tmp/openwepp_wshed01_wa/watershed/run \
  --run-file case.run \
  --output-dir /tmp/openwepp_wshed01_wd_configured/output \
  --policy compat
```

Legacy-discovery run:

```text
cargo run -q -p openwepp-runner --bin openwepp-cli-watershed -- \
  --run-dir /tmp/openwepp_wshed01_wa/watershed/run \
  --run-file case.run \
  --output-dir /tmp/openwepp_wshed01_wd_legacy/output \
  --policy compat \
  --legacy-sidecar-discovery
```

Both runs exited `0`. The legacy-discovery path emitted the expected
sidecar-discovery warnings and used deterministic fallback channel globals.

wepppy audit command:

```text
/home/workdir/wepppy/.venv/bin/python \
  /home/workdir/wepppy/tools/totalwatsed3_daily_closure_audit.py \
  <output>/interchange/totalwatsed3.parquet \
  --output-dir <output>/audit \
  --top-n 20
```

Configured and legacy-discovery audit results match:

```text
rows=2192
max_reported_runoff_mm=39.953120
max_reconstructed_runoff_mm=39.953120
max_runoff_to_precip_reported_pct=31357.226648
closure_reconstructed_with_storage_total_mm=2950.498418
closure_reconstructed_with_storage_pct_of_precip=17.772166
interception_reported_total_mm=551.502748
closure_reconstructed_with_enriched_storage_total_mm=2950.498140
closure_reconstructed_with_enriched_storage_pct_of_precip=17.772164
soilwatertotal_vs_legacy_max_abs_mm=4.840523
profile_violations_days=fc_gt_porosity:0,wp_gt_fc:0,soilwater_gt_porosity:0,soilwater_lt_wp:0
```

Audit artifacts:

- `/tmp/openwepp_wshed01_wd_configured/audit/daily_closure_audit_summary.json`
- `/tmp/openwepp_wshed01_wd_configured/audit/daily_closure_audit_top_days.csv`
- `/tmp/openwepp_wshed01_wd_legacy/audit/daily_closure_audit_summary.json`
- `/tmp/openwepp_wshed01_wd_legacy/audit/daily_closure_audit_top_days.csv`

## Remaining Blocker

The residual is not a hollow exact-zero identity. It is a real positive
whole-run closure miss:

`P - (Runoff + Lateral Flow + ET + Percolation + Interception) - DeltaStorage`
= `2950.498418 mm`.

The remaining openWEPP publication gap is daily PASS runoff lineage. wepppy
`totalwatsed3.py` derives `Runoff` from PASS `runvol`, then combines it with
WAT hydrology and storage terms. The current openWEPP WAT-backed producer still
fills `runvol` from WAT `Q`; the exact `runoff_consistency_mm` noise in the
audit therefore proves only self-consistency between `runvol` and the same WAT
source, not conservation closure.

Static source check:

- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`
  emits HBP event payloads with duration, peak runoff, detachment, and
  deposition fields, but the six event i64 volume slots are currently written
  as zero.
- `crates/openwepp-input-contract/src/parsers/hbp/payload_validator.rs` parses
  the same payload shape and does not expose daily PASS runoff volume.
- `crates/openwepp-runner/src/watershed_wat.rs` therefore has no independent
  daily PASS `runvol` input to hand to totalwatsed3.

W-D-REDO must expose or reconstruct canonical daily PASS runoff volume from
HBP/PASS publication authority, then rerun the same audit. Do not force closure
by changing WAT term selection unless the PASS lineage and SC authority support
that mapping.
