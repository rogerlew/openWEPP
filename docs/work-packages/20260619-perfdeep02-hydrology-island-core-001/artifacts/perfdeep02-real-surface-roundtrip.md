# PERFDEEP02 Real-Surface Roundtrip

Evidence class: Static + Ran.

## Scope

PERFDEEP02 added an opt-in runner diagnostic for the carried PERFDEEP01
real-surface roundtrip condition:

- env var: `OPENWEPP_PERFDEEP02_FRAME_ROUNDTRIP_PATH`
- stages: `single_pre_scheduler`, `single_post_scheduler`, `mofe_pre_scheduler`
- assertion: `HillslopeDayFrame::seed_from_writeback_surface` followed by
  `assert_shadow_roundtrip_bits` must be `to_bits()` identical for every state
  and flux symbol.

The hook fails closed through `HillslopeCliError::RuntimeSurfaceFailure` if
frame seed or shadow roundtrip fails.

## Focused Unit Evidence

Ran:

```text
cargo test -p openwepp-hillslope-orchestrator perfdeep0 -- --nocapture
```

Result: passed, 5 tests.

The new coverage includes
`perfdeep02_frame_roundtrip_covers_production_outputs_and_full_mofe_families`,
which merges a production WB11 warm-rain runoff writeback into a frame surface,
adds all 24 slots for each MOFE hourly carry family, probes typed unit wrappers,
and verifies seed/flush bit identity.

## H2637 Real-Surface Evidence

Pre-final diagnostic run:

```text
env OPENWEPP_PERFDEEP02_FRAME_ROUNDTRIP_PATH=/tmp/perfdeep02/frame_roundtrip/h2637_final.jsonl \
  /usr/bin/time -f "h2637_perfdeep02_final_roundtrip\t%e\t%M" \
  target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file /tmp/perfdeep01/runfiles/h2637_same_current.run \
  --output-dir /tmp/perfdeep02/final/h2637_same \
  --policy compat \
  --legacy-sidecar-discovery
```

Result: passed for frame roundtrip identity. This run predates the final
fail-closed production gate and is not used as a final endpoint-performance
verdict.

- elapsed: `4278.49 s`
- max RSS: `233008 KB`
- diagnostic rows: `235961`
- stage coverage: `235961 mofe_pre_scheduler`
- maximum state symbols in a row: `4038`
- maximum flux symbols in a row: `48`
- maximum MOFE hourly family coverage: 24/24 for upstream saturation,
  current saturation, upstream lateral, and current lateral families
- mismatch search:
  `rg -n '"state_mismatch_count":[1-9]|"flux_mismatch_count":[1-9]'`
  returned no rows.

Verification commands:

```text
wc -l /tmp/perfdeep02/frame_roundtrip/h2637_final.jsonl
rg -n '"state_mismatch_count":[1-9]|"flux_mismatch_count":[1-9]' /tmp/perfdeep02/frame_roundtrip/h2637_final.jsonl
jq -r '.stage' /tmp/perfdeep02/frame_roundtrip/h2637_final.jsonl | sort | uniq -c
```

The runner emitted the expected MOFE01-MG water-transfer-only qin sidecar
warning; it did not affect frame roundtrip identity.

Final package note: after the dense island endpoint failed, the island was
disabled by default behind `OPENWEPP_PERFDEEP02_FRAME_ISLAND=1`. The roundtrip
hook remains available independently through
`OPENWEPP_PERFDEEP02_FRAME_ROUNDTRIP_PATH`.
