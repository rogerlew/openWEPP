# Consumer Path Proof

Status: `EXECUTED`

Real consumer path proven by H2637 active gates:

- native `ow-lanuse-1` management with 19 `routing_coefficients` blocks;
- no explicit active selector needed under rev 46 complete-coefficient default;
- runner attaches `DirectRunFrame.laned_active`;
- executor calls `run_laned_active_publication_stream`;
- per-lane routing calls `laned_active_route_lane`;
- `route_single_ofe_with_step_trace` invokes `KinematicWaveSolver` and rev-47
  local numerics;
- manifest carries `laned_active`, not `laned_shadow`.

Negative proof:

- `cargo nextest run --workspace --profile full --test laned_shadow_h2637`
  passed `8/8`, retaining selector conflict and disabled/off guards.
- The ignored active-owner vector passed default-active and explicit-active
  identity while explicit-disable produces no active block; final rerun was
  `1 passed`, `9 skipped`, `447.438 s`.
