# Base Conductivity Sensitivity Probe

Evidence class: Static + Ran

Verdict: PASS. H2637 base soil conductivity is byte-live and changes the
same aggregate quantities that carry the FARPOINT01 magnitude flag.

## Probe Setup

Static:

- Baseline fixture:
  `/tmp/openwepp_farpoint01_h2637/without_ui/runs/p2637.sol`.
- H2637 soil version is `9002`, and each OFE policy row has `ksatadj = 0`.
- The first OFE conductivity rows are:

| Cumulative depth mm | Raw `ksat` mm/h | Anisotropy |
|---:|---:|---:|
| 200.0 | 60 | 1.0 |
| 560.0 | 330.2755 | 1.0 |
| 1140.0 | 33.0275 | 1.0 |
| 1600.0 | 33.0275 | 1.0 |

Ran:

- Rejected stress probe: `/tmp/stage2_base_cond/ksat_x0_5`.
  It scaled all four H2637 layer `ksat` values to `0.5x` and failed with
  `HKERNEL-WB12-STORAGE-E-003` at `sim_day_index=5637`, elapsed `300.61 sec`.
  This was not used for output deltas.
- Accepted sensitivity probe: `/tmp/stage2_base_cond/ksat_x0_9`.
  It scaled all four H2637 layer `ksat` values to `0.9x` and completed with
  elapsed `695.83 sec`.
- Perturbed soil SHA-256:
  `7416787892932bd870a19b11051d520ab54a8461269fd6044006069bfa39cec2`.
- Command shape:

```bash
target/release/openwepp-cli-hill \
  --run-dir /tmp/stage2_base_cond/ksat_x0_9/runs \
  --run-file h2637.run \
  --output-dir /tmp/stage2_base_cond/ksat_x0_9/owepp_output \
  --policy compat \
  --legacy-sidecar-discovery
```

## Byte Sensitivity

Ran:

| Output | Baseline SHA-256 | `ksat_x0.9` SHA-256 |
|---|---|---|
| `H2637.wat.parquet` | `c70af52324b52c89119e57524f75bf4875d2c6a9ff83fe56d239a22082b9b474` | `c59a7e73dc35056e8c319e44dc64dd629b2b44431226e46c959610efbe10a3ac` |
| `H2637.pass.parquet` | `5c13eb25cbe3e03e1c0f4a3f38cd09d1322be1dd6802aa2ee5cd6ed904348a30` | `265e1d0ef20c24343a2deb42f3e15cc6e2338ec054331bb8ccf02f7e775a64aa` |

Both WAT and PASS outputs changed while row counts and precipitation stayed
fixed.

## Aggregate Deltas

Ran:

| Metric | Baseline | `ksat_x0.9` | Delta | Percent delta |
|---|---:|---:|---:|---:|
| WAT rows | 235961 | 235961 | 0 | 0 |
| PASS rows | 12419 | 12419 | 0 | 0 |
| Precipitation volume, m3 | 19837950.7056724 | 19837950.7056724 | 0 | 0 |
| All-OFE WAT `latqcc`, m3 | 14872633.3781943 | 14867351.1744354 | -5282.20375891 | -0.0355162641651% |
| PASS `runvol`, m3 | 14085670.0787448 | 14080795.8880844 | -4874.19066036 | -0.0346038962514% |
| PASS `sbrunv`, m3 | 884949.941613377 | 884859.279295346 | -90.6623180313 | -0.0102449092054% |
| PASS `runvol+sbrunv`, m3 | 14970620.0203581 | 14965655.1673797 | -4964.85297839 | -0.0331639769872% |
| `runvol` percent of precip | 71.0036550031206 | 70.9790849720086 | -0.024570031112 | -0.0346038962514% |
| Peak WAT `latqcc`, mm | 71.624098767105 | 65.2944885985325 | -6.32961016857 | -8.83726326408% |

Interpretation:

- The base conductivity probe is not inert.
- The raw soil `ksat` lineage is an active driver of both lateral magnitude and
  outlet pass volume.
- The remainder of this package can adjudicate the base-conductivity lineage
  without repeating the `ksatadj` detour.
