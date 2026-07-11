# Release Binary Provenance

Status: `PASS`

Evidence mode: `Ran`

Delegated heavy runner command:

```text
cargo build --release -p openwepp-runner --bin openwepp-cli-watershed
```

- exit: `0`
- binary: `/home/workdir/openWEPP/target/release/openwepp-cli-watershed`
- SHA-256: `0e0ce234d1807dc64f01ac7a579541e72f0c3c08d2c416c39e54df021b1b8357`
- size: `9,336,832` bytes
- mtime: `2026-07-10 19:46:55.491391372 -0700`

No production source changed after W11B, so Cargo correctly retained the same
binary hash and mtime. The W11C test harness then invoked this exact absolute
path via the test-only `OPENWEPP_W11C_WATERSHED_CLI` selector.

Exact evidence command:

```text
OPENWEPP_W11C_WATERSHED_CLI=/home/workdir/openWEPP/target/release/openwepp-cli-watershed \
  cargo nextest run -p openwepp-runner \
  --test mt3_hbp_hourly_consumer_contract \
  wshedw11c_hourly_routing_sanity_matrix --no-capture
```

Final-tree release matrix run ID: `29024159-9f78-4506-9918-09c7f007af0d`;
`1/1 PASS` in 2.836 seconds, with 35 result rows and the same `SANITY-FAIL`
findings as the corrected debug matrix.

Logs:

- `logs/release-build-watershed.log`
- `logs/release-binary-sha256.log`
- `logs/release-binary-stat.log`
- `logs/release-sanity-matrix.log`
