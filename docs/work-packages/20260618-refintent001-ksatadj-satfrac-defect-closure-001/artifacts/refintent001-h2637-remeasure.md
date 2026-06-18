# REFINTENT001 H2637 Remeasure

Evidence class: Ran

## Commands

Without UI:

```bash
/usr/bin/time -f 'elapsed=%e sec' target/release/openwepp-cli-hill \
  --run-dir /tmp/openwepp_farpoint01_h2637/without_ui/runs \
  --run-file h2637.run \
  --output-dir /tmp/openwepp_farpoint01_h2637/without_ui/owepp_output \
  --policy compat \
  --legacy-sidecar-discovery
```

With UI used the same command shape under
`/tmp/openwepp_farpoint01_h2637/with_ui`.

## Exit and warnings

| Variant | Exit | Elapsed | stderr |
|---|---:|---:|---|
| without UI | 0 | 711.53 s | known unknown-sidecar warnings; MOFE01 M-G sediment handoff warning |
| with UI | 0 | 679.82 s | known unknown-sidecar warnings; MOFE01 M-G sediment handoff warning |

## Checksums

| Variant | File | SHA-256 |
|---|---|---|
| without UI | `H2637.wat.parquet` | `c70af52324b52c89119e57524f75bf4875d2c6a9ff83fe56d239a22082b9b474` |
| without UI | `H2637.pass.parquet` | `5c13eb25cbe3e03e1c0f4a3f38cd09d1322be1dd6802aa2ee5cd6ed904348a30` |
| without UI | `H2637.hbp` | `44acc83b025b7a7ed9df3ad77f2d595a17f7e59ae923a1224f8ee294ad09bfe8` |
| without UI | `H2637.loss.json` | `0f72edccf2be9610f4bb473c28e8f03e09b606b1bbe9c59506c08342e4310003` |
| with UI | `H2637.wat.parquet` | `c70af52324b52c89119e57524f75bf4875d2c6a9ff83fe56d239a22082b9b474` |
| with UI | `H2637.pass.parquet` | `e140f6d517e105fddde6d5b79eda53c734ec28578ec77eec3c4d962d72cd8a48` |
| with UI | `H2637.hbp` | `44acc83b025b7a7ed9df3ad77f2d595a17f7e59ae923a1224f8ee294ad09bfe8` |
| with UI | `H2637.loss.json` | `1c3cbcc7562ace66958e2725e28850d3140543ddd896042f491e20ac2ea0b462` |

WAT parquet is byte-identical across UI variants. PASS parquet differs by bytes
but the hydrologic totals are equal.

## Partition

Both UI variants:

- `precip_m3 = 19837950.705672398`
- `pass_runvol_m3 = 14085670.078744758`
- `pass_sbrunv_m3 = 884949.941613377`
- `combined_m3 = 14970620.020358136`
- `runvol_pct_precip = 71.003655003121`
- `combined_pct_precip = 75.464548946971`
- `runvol - outlet QOFE = 0.0 m^3`
- `sbrunv - outlet latqcc = 0.0 m^3`

Conclusion: the H2637 71% flag is resolved by source-intent conformance and
conservation closure. Legacy 55.5% remains an A6 comparator flag, not a target.

## Post-review correction (Claude Code, 2026-06-18)

The conclusion above is **withdrawn**. The WAT SHA-256 `c70af52324b52c89…` is **byte-identical
to the pre-fix STAGE2-LATQCC run** — the fix changed nothing. H2637 soil has **`ksatadj = 0`**
(`p2637.sol`; `input.for:467` reads the leading OFE token as `ksatadj`), so the adjustment branch
holding the `sat_frac` fix never fires. The fix is **byte-inert on H2637** and does **not** close
FARPOINT01. The 71% is driven by the **base soil conductivity**, not `ksatadj`. See
`review-claude-independent.md`.
