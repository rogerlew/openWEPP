# B02 Gate Log

Evidence class: **Ran** (2026-07-02). Reproducible from the committed
binary + H2637 fixture; parquet regenerable-not-vendored.

## Single-OFE byte-identity (clean, same run_name)

marcell_conifer_mn, pre-B02 binary vs B02 binary, identical `run.toml`:
**MATCH on all five outputs** (H.hbp, H.loss.json, H.pass.parquet,
H.wat.parquet, H.plot.parquet). The earlier apparent loss.json/plot diff
was a `run_name` confound (both files embed run metadata); removed by using
one run_name for both runs.

## H2637 (multi-OFE) — semantics + invariance

| Check | Result |
|---|---|
| `H.pass.parquet` sha256 vs pre-B02 | **byte-invariant** (runvol + peak preserved) |
| WAT `QOFE == Q` on every row | **True** |
| Q, latqcc, Total-Soil, Ep, Dp, Snow-Water vs pre-B02 | **unchanged** |

## QOFE changed-row count — the two numbers reconciled (CX-002)

The reviewer's count (87,791) and the package's (53,298) are both correct;
they measure different things against the **same** pre-B02 baseline
(`dc01-m3`, the DC01-era binary — QOFE-identical to pre-B02):

| Count | Value | Meaning |
|---|---:|---|
| Material change (`|Δ| > 1e-9 mm`) | **53,298** | physically meaningful QOFE changes |
| Bit-level change (`to_bits !=`) | **87,791** | any float difference incl. sub-ULP |

The material count is triply self-consistent: it equals the rows where
pre-B02 `QOFE != Q` (53,298) and the rows with `runoff > 0 on OFE > 1`
(53,298) — the OFEs whose local length differs from the cumulative length.
The 34,493 extra bit-level rows are OFE>1 rows with tiny (<1e-9 mm)
near-zero-runoff QOFE shifts from the denominator change. **Baseline
matters:** vs pre-DC01 (`postmerge`) the count is 219,529 (conflates DC01's
runoff changes); the correct baseline is pre-B02.

Reproduce: `c03`-style read of `b02-m/out` vs `dc01-m3/out` WAT parquet
(`abs(new.QOFE - old.QOFE) > 1e-9` vs `.view('int64') !=`).
