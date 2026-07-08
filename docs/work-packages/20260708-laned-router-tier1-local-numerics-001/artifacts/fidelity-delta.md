# Fidelity Delta

Status: `EXECUTED`

Rev 47 intentionally does not claim bit identity with rev 46.

Correctness/fidelity surfaces run:

- `cargo test -p openwepp-hillslope-orchestrator ofe_routing::kinematic_wave --lib`
  -> `26 passed`.
- `cargo test -p openwepp-hillslope-orchestrator ofe_routing::friction --lib`
  -> `9 passed`.
- `cargo test -p openwepp-hillslope-orchestrator ofe_routing::d10b_reconciliation_tests --lib`
  -> `11 passed` in `142.01 s`; includes the Iwagaki oracle ladder, TV(q)
  bound, and 19-OFE conservation convergence.
- `cargo nextest run --workspace --profile full --test laned_shadow_h2637`
  -> `8 passed`, `2 skipped`.
- ignored H2637 active-owner vector
  `h2637_native_active_owner_routes_and_closes --run-ignored ignored-only`
  -> `1 passed`, `9 skipped`, `447.438 s`.

Pre-change comparison:

- Baseline binary: detached worktree at commit `46532c28`, run once on the same
  H2637 active fixture class.
- Daily pass rows/schemas: both `731` rows; key columns equal.
- Hydrograph/output magnitude proxies:
  - `runvol`: bit-identical, annual sum `71940.618224977254 m^3`.
  - `peakro`: bit-identical, annual sum `0.0026841043823428255 m^3/s`.
  - `sbrunv`: bit-identical, annual sum `208132.8460294918 m^3`.
- Active router volume/timing-shape surfaces:
  - `total_source_m3`: unchanged `374423.35262127215`.
  - `total_routed_outlet_m3`: `371256.03024551831` ->
    `371254.76026497723`, delta `-1.2699805410811678 m^3`.
  - `total_end_window_storage_m3`: `3167.3223757570549` ->
    `3168.5923562929565`, delta `+1.2699805359015954 m^3`.
  - `total_tail_fold_m3`: `36426.084420240768` ->
    `36531.174110474778`, delta `+105.08969023401005 m^3`.
  - `lane_days_erosion_source_shape_degenerate`: unchanged `1`.
- Closure maxima remain below the rev-27 acceptance envelope:
  - cascade residual `2.5119138589807493e-13`;
  - seam residual `2.2525144908845638e-13`;
  - identity residual `2.650254651666847e-13`.
- Sediment response surfaces changed only on day `44`: annual `tdet` delta
  `-0.45379142937636985 kg`; `sedcon_*` max relative delta
  `0.0044339259231856052`.
- `H2637.loss.json` hash is identical; `H2637.pass.parquet`, `H2637.hbp`, and
  manifest hashes differ as expected for a non-bit-identity numerics change.

Endpoint timing changes materially: active median `37.48 s` -> `11.90 s` user.
Acceptance rests on oracle/conservation/H2637 closure gates and the named
deltas above, not bit identity.
