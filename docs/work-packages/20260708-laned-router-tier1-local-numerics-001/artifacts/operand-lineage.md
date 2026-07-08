# Operand Lineage

Status: `EXECUTED`

Lineage:

- `h`: solver state `KinematicWaveSolver.depth_m`.
- `q`: pre-step solver state `discharge_m2_s`, then rev-47 local
  `alpha_q_celerity` equilibrium result, then committed `discharge_m2_s`.
- friction operands: native `routing_coefficients` plus live rainfall/LAI/Hc
  sources as already bound by `SC-OFEROUTE-001` rev 21/36/46.
- `Re`: local `q / nu`; selects laminar vs Hirsch skin branch.
- `Fr`: built only when wave resistance is active.
- `alpha`: `sqrt(8g/f_eq) sqrt(S_o)` or closed-form equivalent.
- `celerity`: rev-47 analytic `dq/dh`; consumed by `prepare_step_alpha`,
  CFL `dt`, Courant evidence, and TVD dissipation.
- outlet hydrograph and bins: unchanged `RoutingResult` surfaces.
- active H2637 consumer: default-active runner attaches
  `DirectRunFrame.laned_active`, calls `laned_active_route_lane`, then
  `route_single_ofe_with_step_trace`, then the shared
  `KinematicWaveSolver::run_with_options_and_step_trace` path.
