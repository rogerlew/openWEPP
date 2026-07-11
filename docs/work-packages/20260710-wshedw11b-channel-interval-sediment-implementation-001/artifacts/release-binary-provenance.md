# Release Binary Provenance

Status: `EXECUTED-PASS`

Evidence mode: `Ran` on the frozen final tree, 2026-07-10.

Command: `cargo build --release -p openwepp-runner --bins`.

The production consumer used for W11B evidence was:

```text
target/release/openwepp-cli-watershed
sha256 0e0ce234d1807dc64f01ac7a579541e72f0c3c08d2c416c39e54df021b1b8357
size 9336832
mtime 2026-07-10 19:46:55.491391372 -0700
```

The runner integration test first passed 2/2 against the current sources. The
same generated two-channel spike/spread fixture directories were then executed
directly with the release watershed binary:

- spike: `/tmp/w11b_cli_spike_1783738025745325877/out_release`
- spread: `/tmp/w11b_cli_spread_1783738026376507618/out_release`

Extracted terminal EBE rows:

| Shape | Runoff (m3) | Peak (m3/s) | Sediment (kg) | Element |
|---|---:|---:|---:|---:|
| spike | 7,088.171478291323 | 2.1122146208271415 | 240 | 2 |
| spread | 7,160.979461604386 | 0.5002525682549819 | 240 | 2 |

Logs: `logs/release-build.log`, `logs/release-binary-stat-sha256.log`,
`logs/w11b-runner-integration.log`, `logs/release-cli-spike.log`,
`logs/release-cli-spread.log`, and `logs/release-cli-extracted-results.log`.
