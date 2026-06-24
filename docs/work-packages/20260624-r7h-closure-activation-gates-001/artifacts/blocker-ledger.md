# Blocker Ledger

Evidence class: Static/Ran.

## R7H-001: Direct H2637 day-10 storage projection closure

State: closed in this package.

Observed:

- Command: H2637 direct default-candidate release run using
  `/tmp/openwepp_farpoint01_h2637/without_ui/runs/h2637.run`.
- Failure: lane `1`, day `10`,
  `hydrology_projection.aggregate_storage_delta_m`.
- Mechanism: the no-material frost branch added
  `frwatc_net_liquid_delta_m` into R4B `frost_liquid_delta_m` while emitting no
  coarse layer mutation. This made scalar storage and layer aggregate disagree.

Correction:

- No-material frost outcomes preserve typed fine/shadow state but do not mutate
  coarse storage through `frost_liquid_delta_m`.
- Focused regression:
  `cargo test -p openwepp-hillslope-orchestrator r7g_ -- --nocapture`.

## R7H-002: Direct H2637 day-25 stale frozen layer projection

State: closed in this package.

Observed:

- Failure after R7H-001: lane `2`, day `25`,
  `hydrology_projection.frozen_layer_storage_m`.
- Diagnostic rerun:
  `aggregate_storage_from_layers_m=0.4953069572576004`,
  `storage_reconciled_m=0.49530695725760054`,
  `frozen_layer_storage_m=0.0007513880146886001`,
  `projected_frozen_soil_water_m=0`.
- Mechanism: current no-material frost projection exposed zero projected
  frozen-water authority, but the terminal layer vector still carried stale
  coarse frozen fields.

Correction:

- No-material frost clears stale coarse `frozen_depth_m`/`frozen_water_m` only
  when present, preserving aggregate layer storage.
- Focused regression:
  `r7g_inactive_no_material_frost_clears_stale_coarse_projection_without_storage_delta`.

## R7H-003: H2637 direct performance and protected parity

State: held.

Observed:

- Direct default-candidate endpoint after fixes:
  `r7h_direct_default_after_no_material_fast_path 113.53 1083636`.
- Gate budget: `<=91.2 s` from the R7G `<=10x` legacy reference.
- Manifest:
  `compatibility_edge_invocations=0`,
  `day_frame_commits=235961`,
  `scheduler_kernel_executed=false`,
  `publication_source=direct-publication-frame`.
- Retained compatibility-capture comparison, non-authoritative for current-code
  parity but useful blocker characterization:
  HBP, WAT, PASS, loss, and plot all differ.

Disposition:

- `HOLD-R7H-H2637-DIRECT-PERFORMANCE-AND-PROTECTED-PARITY`.
- Default activation remains disabled.
- First follow-up correction: profile/remediate the active winter/frost
  execution path after the no-material closure fixes, then rerun same-binary
  H2637 compatibility/direct/rollback/parity matrix. Activation remains blocked
  until performance and protected parity are both green.
