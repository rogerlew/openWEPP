# Required Reading

Evidence mode: Static.

- `docs/planning/snow-frost-fidelity-strategy.md` section 11 step 3.
- `docs/work-packages/20260629-frost-step1-current-snow-control-rerun-001/`.
- `docs/work-packages/20260629-frost-step2-sleepers-attribution-001/`.
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
  `INV-SNOWFREEZE-047`, `INV-SNOWFREEZE-048`, `INV-SNOWFREEZE-050`,
  `TOL-SNOWFREEZE-009`, and `GAP-SNOWFREEZE-002`.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost.rs`.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs`.
- `tests/fixtures/snowfreeze_observed/site2_sleepers_w9_hardwood_vt/p3.man`.
- `tests/fixtures/snowfreeze_observed/site1_sleepers_south_field_vt/`.
- `tests/fixtures/cancov_forest/hubbardbrook_deciduous_nh/p10.man`.
- `docs/backlog/20260626-frost-daylength-canopy-decline-hemisphere-robust.md`
  surface residue / litter cover section.

Static findings:

- `frost_surface_heat_path` adds residue thermal resistance as
  `residue_depth_m / residue_conductivity_w_m_k`, so the frost solver consumes
  `residue_depth_m` as a surface thermal resistance term.
- `frost_entry.rs` can emit `OPENWEPP_R7G_FROST_TRACE_PATH` records containing
  the solver-side `residue_depth_m`.
- Step 2 narrowed `GAP-SNOWFREEZE-002` to Sleepers timing candidate defects:
  South Field `4` thaw-late cells; W9 Hardwood `14` early-onset/thaw-late
  cells.
- W9 `p3.man` is inert `Tah_4899`. South Field `p1.man` is bromegrass, while
  `pmetpara.txt` includes a `Tah_4899` forest line; the package treats this as
  a diagnostic nuance, not as authority to mutate the fixture.
