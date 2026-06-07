# FQ1 Validation Ledger

Evidence mode: `Ran:`.

Temporary runfiles and outputs were generated under `/tmp/fq1_after/`.

Command shape:

```text
./target/debug/openwepp-cli-hill \
  --run-dir /wc1/runs/al/algebraic-radium/wepp/runs \
  --run-file /tmp/fq1_after/runfiles/pN.toml \
  --output-dir /tmp/fq1_after/outputs/pN \
  --policy compat \
  --legacy-sidecar-discovery
```

Raw ledger: `/tmp/fq1_after/run_status.tsv`.

## Population Result

| Result | Count |
|---|---:|
| Prefixes run | 43 |
| Return code 0 | 42 |
| `H.wat.parquet` emitted | 42 |
| `H.hbp` emitted | 42 |
| `HS-RUNTIME-E-062` failures | 0 |
| Downstream protected-boundary failures | 1 |

The six controls `p8,p13,p22,p23,p26,p28` all returned `0` and emitted both
`H.wat.parquet` and `H.hbp`.

## Concise Ledger

| Prefixes | Status |
|---|---|
| `p1-p10`, `p12-p43` | rc `0`; `H.wat.parquet=yes`; `H.hbp=yes` |
| `p11` | rc `1`; no WAT/HBP; `HKERNEL-WB11-PERC-E-003` at `sim_day_index=162`, `calendar_year=1990`, `julian_day=162` |

`p11` rerun:

```text
p11_rerun rc=1 wat=no hbp=no
CLIHILL-E-011 runtime surface failure for execution_provenance:
HS-SIMPIPE-E-001 ... message_id=HKERNEL-WB11-PERC-E-003 ...
[sim_day_index=162, calendar_year=1990, julian_day=162]
```

Disposition impact: the FQ1 soil mapping blocker is removed, but full 43/43 WAT
publication is held at an out-of-envelope percolation guard.
