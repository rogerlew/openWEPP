# Implementation and Test Evidence

Status: completed

Evidence mode: Static + Ran

Static:
- Implemented WB18 hourly bottom-layer baseline lineage in
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`:
  - collects `dg_####` for per-layer percolation.
  - requires `ui_bdrkth` when `slflag=1` and hourly lane is active.
  - uses `Ksi_eff = (dg_i+ui_bdrkth)/(dg_i/Ksi + ui_bdrkth/kslast)` for
    hourly restrictive bottom seepage.
  - forces `fx=1` for hourly bottom-layer seepage per baseline `meblfc`.

Ran:
- `cargo test --test wb18_percolation_physics_kernel_contract -- --nocapture`:
  pass (`15/15`).
- `cargo build -p openwepp-runner --bin openwepp-cli-hill`: pass.
- `H1..H39` runtime suite: pass (`39/39`) at
  `/tmp/hphys0248_20260602T114714Z_final`.
- `H1..H39` semantic comparator suite: pass as executed (`39/39` report
  generation), semantic pass `0/39`.
