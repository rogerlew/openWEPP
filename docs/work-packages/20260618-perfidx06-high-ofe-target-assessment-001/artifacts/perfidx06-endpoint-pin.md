# PERFIDX06 endpoint pin

Evidence: Ran + Static.

## Endpoint

Ran:

```text
git status --short --branch
git rev-parse HEAD
sha256sum /tmp/perfidx04/current/bin/openwepp-cli-hill
sed -n '1,220p' /tmp/perfidx04/current/bin/openwepp-cli-hill.json
cat /proc/sys/kernel/perf_event_paranoid
perf --version
```

Observed:

| Item | Value |
| --- | --- |
| Working tree | clean before measurement |
| Current repo HEAD | `2df0dcea3118e7cd6901dc07e2fd4b815c4359b5` |
| Measured binary | `/tmp/perfidx04/current/bin/openwepp-cli-hill` |
| Measured binary SHA256 | `82c6cac78ed6b138b1b05750012082c1f8045602cf34004862adc48407d53e3c` |
| Binary sidecar source commit | `e9c52a577300465a0899cb595fc4e60f37f47717` |
| Sidecar build time | `2026-06-17T22:57:23.297300664Z` |
| Perf access | `/proc/sys/kernel/perf_event_paranoid = 0` |
| Perf version | `perf version 6.8.12` |

Interpretation: the measured executable is the persisted PERFIDX04 endpoint binary. The
current repo HEAD is later than the binary source commit because PERFIDX05 was held/discarded
and package documentation advanced; no production Rust edits were present before PERFIDX06
measurements.

## Legacy Comparator Pin

Ran:

```text
sha256sum \
  /home/workdir/wepp-forest_260430_baseline/release/wepp_260430 \
  /home/workdir/wepp-forest_260430_baseline/release/wepp_260430_hill
```

Observed:

| Item | Value |
| --- | --- |
| Legacy hillslope binary | `/home/workdir/wepp-forest_260430_baseline/release/wepp_260430_hill` |
| Legacy hillslope SHA256 | `3b2fdd2b7a9e264b84f1e7b161dfb0730d49d3cb652218139efeb3ba17d7a160` |
| Legacy full binary SHA256 | `b11c94d8ba19deb941e4221977c4ec3437f41f612a2b816beacaca1b15563292` |
| H2637 legacy runfile | `/tmp/openwepp_farpoint01_h2637/without_ui/runs/p2637.run` |
| H2637 with-UI legacy runfile | `/tmp/openwepp_farpoint01_h2637/with_ui/runs/p2637.run` |

## Raw PERFIDX06 Evidence

Raw measurement files are under `/tmp/perfidx06`:

- `/tmp/perfidx06/artifacts/wallclock-times.tsv`
- `/tmp/perfidx06/artifacts/legacy-wallclock-times.tsv`
- `/tmp/perfidx06/artifacts/ratio-summary.tsv`
- `/tmp/perfidx06/artifacts/output-hash-compare.tsv`
- `/tmp/perfidx06/artifacts/output-semantic-compare.tsv`
- `/tmp/perfidx06/perf-h2637-endpoint.data`
- `/tmp/perfidx06/perf-h2637-endpoint-report-nochildren.txt`
- `/tmp/perfidx06/perf-h2637-endpoint-report-children.txt`
