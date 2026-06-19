# PERFDEEP02 Gate Results

Evidence class: Ran.

## Rust Gates

Ran:

```text
cargo fmt --check
```

Result: passed.

Ran:

```text
cargo clippy --workspace --all-targets -- -D warnings
```

Result: passed.

Ran:

```text
cargo test --workspace
```

Result: passed.

Ran:

```text
cargo deny check
```

Result: passed (`advisories ok, bans ok, licenses ok, sources ok`).

Ran:

```text
cargo test -p openwepp-hillslope-orchestrator perfdeep0 -- --nocapture
```

Result: passed, 5 tests.

Ran:

```text
cargo build --release -p openwepp-runner --bin openwepp-cli-hill
```

Result: passed.

## H2637 Runtime Gate

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

Result: passed for frame roundtrip identity only. This run predates the final
fail-closed production gate and is not used as a final endpoint-performance
verdict.

- elapsed: `4278.49 s`
- max RSS: `233008 KB`
- diagnostic rows: `235961`
- stage coverage: `235961 mofe_pre_scheduler`
- frame roundtrip mismatches: `0`
- max row shape: `4038` state symbols, `48` flux symbols
- max MOFE hourly family coverage: 24/24 for all four tracked carry families

Post-run checks:

```text
wc -l /tmp/perfdeep02/frame_roundtrip/h2637_final.jsonl
rg -n '"state_mismatch_count":[1-9]|"flux_mismatch_count":[1-9]' /tmp/perfdeep02/frame_roundtrip/h2637_final.jsonl
jq -r '.stage' /tmp/perfdeep02/frame_roundtrip/h2637_final.jsonl | sort | uniq -c
```

Production opt-in endpoint attempts:

```text
/usr/bin/time -f "h2637_perfdeep02_final_nohook\t%e\t%M" \
  target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file /tmp/perfdeep01/runfiles/h2637_same_current.run \
  --output-dir /tmp/perfdeep02/final/h2637_same_nohook_manifest \
  --policy compat \
  --legacy-sidecar-discovery
```

Results:

- whole-frame logical flush attempt: completed but failed endpoint,
  `2417.14 s`, `235700 KB`;
- deferred dirty-id flush attempt: terminated after `23:36` elapsed because it
  had already exceeded 2x the PERFDEEP01 endpoint;
- direct indexed-frame seeding attempt: terminated after `25:27` elapsed because
  it had already exceeded 2x the PERFDEEP01 endpoint.

Final code is fail-closed: the dense island is disabled by default and requires
`OPENWEPP_PERFDEEP02_FRAME_ISLAND=1`. No final default-production H2637 endpoint
rerun was required for `NO-GO`; the full workspace integration suite covers the
default path with the dense island disabled.

## Notes

No output-identity claim is made for the final opt-in implementation because
the endpoint gate failed before a final completed H2637 run. The pre-final
diagnostic run wrote outputs under `/tmp/perfdeep01/current/h2637_same` because
the runfile uses absolute output paths; that run showed HBP, loss, plot parquet,
and WAT parquet checksum identity against the PERFDEEP01 determinism snapshot,
and PASS parquet row/schema identity despite byte-checksum drift.
