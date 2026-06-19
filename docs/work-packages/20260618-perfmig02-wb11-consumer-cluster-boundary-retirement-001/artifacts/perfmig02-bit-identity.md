# PERFMIG02 Bit Identity

Static: compared PERFMIG02 manifests against PERFMIG01 final manifest and inspected output paths.

Ran: focused identity tests plus H2637 output comparisons below.

## Focused Identity Gates

Ran:

```text
cargo test -p openwepp-hillslope-orchestrator perfmig01_wb11_warm_rain_indexed_writeback_is_bit_identical -- --nocapture
cargo test -p openwepp-hillslope-orchestrator perfmig01_scheduler_applies_indexed_writeback_payload -- --nocapture
```

Result: both passed.

These preserve the PERFMIG01 exact materialized map and `f64::to_bits()` identity fixtures after the
PERFMIG02 helper/materialization changes.

## H2637 Manifest Evidence

PERFMIG02 final-code repeat run:

- binary SHA: `d4f7603e79fdf415e3e4123a2baa7df19a6cb7780e8d01206bfaad6ef012d63b`
- manifest: `/tmp/perfmig02-final/current/h2637_same_manifest_repeat/openwepp_hillslope_run_manifest.json`
- output anchor used by runfile: `/tmp/perfmig01-final/current/anchor/h2637_same/`

PERFMIG01 final manifest:

- binary SHA: `711a439a6df782cfaaeb9ca987a49cbaad098d0585ef327c2105bfb104f6b579`
- manifest: `/tmp/perfmig01-final/current/h2637_same_manifest/openwepp_hillslope_run_manifest.json`

Manifest checksum comparison:

| Output | PERFMIG01 final | PERFMIG02 | Verdict |
|---|---:|---:|---|
| `H2637.hbp` | `44acc83b025b7a7ed9df3ad77f2d595a17f7e59ae923a1224f8ee294ad09bfe8` | `44acc83b025b7a7ed9df3ad77f2d595a17f7e59ae923a1224f8ee294ad09bfe8` | same |
| `H2637.loss.json` | `4d4421a2dcc1275af607059605249517d7f605f4431644aa4e675966daf8e021` | `4d4421a2dcc1275af607059605249517d7f605f4431644aa4e675966daf8e021` | same |
| `H2637.pass.parquet` | `76ae0f4d995f1b7ab0fbf96faf2cd30a9cd4ddcb8a4cdfde18e08aedfc206a68` | `0c6381744342930316e11cb4ef70a731bb595da5ea75a6137f1110bf5c3056b7` | container checksum differs; Arrow rows equal below |
| `H2637.plot.parquet` | `1419d03fad4b5f8dbd8aad6aabae95a6c10934a9e4d7f8ef65437968a12926d6` | `1419d03fad4b5f8dbd8aad6aabae95a6c10934a9e4d7f8ef65437968a12926d6` | same |
| `H2637.wat.parquet` | `c70af52324b52c89119e57524f75bf4875d2c6a9ff83fe56d239a22082b9b474` | `c70af52324b52c89119e57524f75bf4875d2c6a9ff83fe56d239a22082b9b474` | same |

## Independent Output Comparison

Ran:

```text
cmp -s /tmp/perfmig01/current/anchor/h2637_same/H2637.hbp /tmp/perfmig01-final/current/anchor/h2637_same/H2637.hbp
cmp -s /tmp/perfmig01/current/anchor/h2637_same/H2637.wat.parquet /tmp/perfmig01-final/current/anchor/h2637_same/H2637.wat.parquet
.venv/bin/python - <<'PY'
import pyarrow.parquet as pq
...
PY
```

Result:

```text
hbp_cmp=0
wat_cmp=0
pass rows 12419 12419 cols 17 17
pass schema_equal True
pass table_equal True
wat rows 235961 235961 cols 34 34
wat schema_equal True
wat table_equal True
```

Verdict: HBP and WAT are byte-identical against the independent PERFMIG01 output copy. PASS is Arrow-equal
with metadata ignored, matching the already documented parquet container nondeterminism posture from prior
perf packages. The PERFMIG01-final and PERFMIG02 manifests also agree on HBP, loss, plot, and WAT checksums.
