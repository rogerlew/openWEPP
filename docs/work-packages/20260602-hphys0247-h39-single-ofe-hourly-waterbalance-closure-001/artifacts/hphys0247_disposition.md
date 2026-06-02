# HPHYS0247 Disposition

Status: hold

Evidence mode: static + ran

Disposition: `HOLD_PENDING_WB18_WB17_SNOWMELT_MIGRATION`

Static:
- HPHYS0247 did not introduce heuristic/proxy hydrology formulas.
- HPHYS0247 corrected two contract-authoritative defects:
  winter execution was incorrectly gated by sidecar presence, and WB19 lateral
  flow did not follow baseline `meblfc`/`tdvv`/`fffx` lineage.

Ran:
- H39 patched root:
  `/tmp/hphys0247_20260602T062939Z_patched`.
- H39 final root:
  `/tmp/hphys0247_20260602T070132Z_final`.
- H39 semantic comparator result: `semantic_pass=false`.
- Manifest evidence: hourly lane executed for `1461` days; winter active with
  `snow_file_present=false`.
- Tests passed:
  `runtime_inputs::tests::climate_runtime_surface_with_context` (`4`),
  `clim05_snow_runtime_kernel_contract` (`6`), and
  `wb19_lateral_drainage_physics_kernel_contract` (`11`).
- Broad gates passed: `cargo fmt --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, `cargo deny check`, anti-evasion guards,
  `auth11`, and `git diff --check`.

Reasons this is not `GO`:
- H39 semantic parity is not closed.
- WB18 percolation remains materially non-authoritative for H39 early-season
  storage: candidate `Dp=22-24 mm/day` on days 1-4 versus baseline
  `0.24 mm/day`.
- WB17 ET partition remains materially non-authoritative: `Ep` fails
  `1460/1461` rows and `Es` fails `1461/1461` rows.
- Snow execution is now active, but snowmelt/runoff timing remains
  non-authoritative: `Snow-Water`, `RM`, and `Q` still fail semantic parity.
- Dual review was performed and findings were resolved; dual verification was
  not performed.
- Strict contract-first sequencing was not perfectly preserved for all tests;
  this is recorded in `pre-implementation-contract-gate.md`.
