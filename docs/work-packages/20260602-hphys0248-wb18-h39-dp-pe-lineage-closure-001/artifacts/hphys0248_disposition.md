# HPHYS0248 Disposition

Status: hold

Evidence mode: Static + Ran

Disposition: `HOLD_TARGETED_WB18_H39_EARLY_SEASON_DP_PE_CLOSED_FULL_WATERBALANCE_OPEN`

Static:
- HPHYS0248 did not introduce heuristic/proxy hydrology formulas.
- HPHYS0248 corrected the targeted baseline-authoritative WB18 H39
  early-season `Dp`/`Pe` lineage by porting:
  - hourly bottom restrictive-layer effective conductivity from `perc.for`.
  - bottom-layer `fx=1` behavior from legacy hourly `meblfc`.
  - `watbal_hourly`/`purk` deep-seepage publication semantics.
- Dual independent review was dispatched and actionable findings were resolved.

Ran:
- Final evidence root:
  `/tmp/hphys0248_20260602T114714Z_final`.
- H39 first 10 days: baseline `Dp=0.240000 mm/day`, candidate
  `Dp=0.246960 mm/day`, residual `+0.006960 mm/day`.
- H39 `Dp` max residual improved from HPHYS0247 `23.809497` to HPHYS0248
  `0.240000`.
- Full `H1..H39` runtime suite completed: `39/39`.
- Full `H1..H39` semantic comparator report generation completed: `39/39`.
- Semantic pass remains `0/39`.
- Gates passed: `cargo fmt --check`, targeted WB18 test (`15/15`),
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, `cargo deny check`, and `git diff --check`.
- Raw gate logs are stored under `artifacts/gate-logs/`.

Reasons this is not `GO`:
- Full H39 semantic parity is not closed.
- Full `H1..H39` semantic pass remains `0/39`.
- `Dp` still fails all `39` hillslopes by comparator tolerance, despite the
  H39 early-season burst being reduced to baseline scale.
- WB17 `Ep`/`Es`, snow/runoff timing (`Snow-Water`, `RM`, `Q`), aggregate
  storage (`Total-Soil`, `SoilWaterTotal`), and WB19 `latqcc` remain open.
- Separate independent verification agents were not dispatched; this package
  records dual review plus local gate evidence and remains HOLD.
