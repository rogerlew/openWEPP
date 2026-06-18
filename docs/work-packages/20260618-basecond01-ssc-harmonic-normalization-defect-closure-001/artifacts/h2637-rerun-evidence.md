# H2637 Rerun Evidence

Evidence class: Ran

Status: complete.

Run:

```text
/usr/bin/time -f 'elapsed=%e sec' target/release/openwepp-cli-hill \
  --run-dir /tmp/openwepp_farpoint01_h2637/without_ui/runs \
  --run-file h2637.run \
  --output-dir /tmp/basecond01/h2637_no_ui/owepp_output \
  --policy compat \
  --legacy-sidecar-discovery \
  > /tmp/basecond01/h2637_no_ui/stdout.log \
  2> /tmp/basecond01/h2637_no_ui/stderr.log
```

Result:

- Exit status: `0`.
- Elapsed: `659.13 sec`.
- Manifest:
  `/tmp/basecond01/h2637_no_ui/owepp_output/openwepp_hillslope_run_manifest.json`.
- Binary SHA256:
  `c6f05b9121310545788af99d9605943f1a39be0931ea607c6f36a8d4775d8c0d`.
- Binary sidecar SHA256:
  `7b5eff87d5f46b39c3bd789f6b17f0f5d203907fbc24bdce9fb74f328a1716f7`.
- Source commit recorded by manifest:
  `2963f1c4a9f825de95a2a74c6a1b64b0b68889f8`.
- Worktree state: dirty. The production-code diff for
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
  is identified by SHA256
  `f2f648cd4734d7b6998911df49fe5a2980eae79a0b1ba86afd7fe0692d620e0e`.
  Later post-review edits added tests and documentation only; they did not
  alter the release binary production path used for this H2637 rerun.

Post-BASECOND01 output checksums:

| Output | SHA256 |
|---|---|
| WAT | `c70af52324b52c89119e57524f75bf4875d2c6a9ff83fe56d239a22082b9b474` |
| PASS | `87e1266a95354bde28db8af44812965876eb925957d7f0589fdf3437a2a68cdc` |

Aggregate deltas against pre-BASECOND01 recorded metrics:

| Metric | Pre-BASECOND01 | Post-BASECOND01 | Delta | Percent delta |
|---|---:|---:|---:|---:|
| `wat_rows` | 235961 | 235961 | 0 | 0 |
| `pass_rows` | 12419 | 12419 | 0 | 0 |
| `precip_m3` | 19837950.7056724 | 19837950.7056724 | 0 | 0 |
| `latqcc_all_ofe_m3` | 14872633.3781943 | 14872633.3781943 | 0 | 0 |
| `pass_runvol_m3` | 14085670.0787448 | 14085670.0787448 | 0 | 0 |
| `pass_sbrunv_m3` | 884949.941613377 | 884949.941613377 | 0 | 0 |
| `pass_combined_m3` | 14970620.0203581 | 14970620.0203581 | 0 | 0 |
| `runvol_pct_precip` | 71.0036550031206 | 71.0036550031206 | 0 | 0 |
| `sbrunv_pct_precip` | 4.46089394385045 | 4.46089394385045 | 0 | 0 |
| `combined_pct_precip` | 75.464548946971 | 75.464548946971 | 0 | 0 |
| `peak_wat_latqcc_mm` | 71.624098767105 | 71.624098767105 | 4.2632564145606e-14 | 5.95226535195e-14 |

Interpretation:

- BASECOND01 closes the vertical `ssc` source-intent defect in the runtime
  projection.
- The H2637 no-UI aggregate magnitude is byte-inert to this correction within
  recorded metric precision.
- The remaining FARPOINT01 H2637 magnitude flag is not closed by vertical
  `ssc`; follow-on disposition must start from the fact that both `ksatadj`
  source intent and vertical `ssc` source intent have now been corrected.
