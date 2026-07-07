# Consumer Path Proof (D15A-P3)

Status: **EXECUTED**.

Evidence mode: mixed, labeled per row: **Ran** = observed in the executed
active H2637 run / test suite this session; **Static** = source chain read.
Paths per `operand-lineage.md` legend.

## Consumer 1 — inter-OFE surface runon (routing owns it)

- Producer source (Static): lane `i`'s solver run inside
  `laned_active_route_lane` (ORCH `laned_active.rs`), producing the rev-24
  conservative outlet bin series.
- In-memory object (Static): `ofe_routing::cascade::UpstreamHandoff`
  (bins + spans + bin_dt + width), held by the executor's phase-2 day loop.
- Runner handoff (Static): none required — the handoff is
  orchestrator-internal by design (the runner only selects and configures).
- Downstream call site (Static): lane `i+1`'s `route_single_ofe` upstream
  integral (OFE `cascade.rs`, the same width-scaled conservative injection
  `run_cascade` uses).
- Output/API surface (Ran): manifest `laned_active` books —
  `total_routed_outlet_m3=371782.96` with day cascade residual ≤ 2.5e-13
  relative on all 610 routed days.
- Negative proof (Ran + Static): the DC01 surface path is dead on active
  lanes — (a) Static: `publish_dynamic_transfer_to_downstream_with_ownership(…,
  true)` never writes `surface_carry_m`/`surface_hourly_weights`; (b) Ran:
  the INV-OFEROUTE-009 double-feed guard (typed hard fail on resolved R4J
  surface runon > 1e-12 m) ran on EVERY lane-day of the 731-day active run
  (both the routed and the zero-source branches call it) and never fired;
  (c) Ran: `sbrunv` (lateral) is preserved (208,153 → 208,133 m³) while the
  surface cascade amplification is gone — exactly the supersede signature.

## Consumer 2 — D13 erosion hourly shape

- Producer source (Static): `laned_active_routed_erosion_weights` — the
  lane's routed outlet bin series, hour-aligned sums, rev-27 tail fold, dry
  all-zero, counted full-mesh-hold degeneracy.
- In-memory object (Static):
  `DirectErosionInputs.routed_hydrograph_runoff_fraction` +
  `hydrograph_shape_authority = RoutedHydrograph`, set on the day frame
  BEFORE the erosion span.
- Runner handoff (Static): none required — producer (executor routing step)
  and consumer (erosion span) share the day frame; the runner sees the
  result via `hourly_runoff_fraction` on published rows.
- Downstream call site (Static): `r7d8_surface_hourly_weights` → the
  `RoutedHydrograph` match arm → `r7d8_routed_hydrograph_hourly_weights`
  (ORCH `erosion.rs:402-447`) with the D13 fail-closed validation (missing →
  `MissingDirectUpstream`; non-unit-sum on wet days / non-zero on dry days →
  `DirectClosureToleranceExceeded`).
- Output/API surface (Ran): `wave1_hourly_weights` propagates the routed
  shape into the Wave-1 hourly plan, the published
  `publication.hourly_runoff_fraction` surface, and the E.3 erosion-inflow
  publisher (`hourly_qout_m2_s`); the active pass parquet erosion surfaces
  (`tdet`, `tdep`, `sedcon_*`) differ from the off run.
- Positive execution proof (Ran): `multi_ofe_wave1_chained=true` — Wave-1 is
  the erosion engine on every lane, and the erosion span ran on all
  731 × 19 lane-days with the authority set to `RoutedHydrograph`; the D13
  validation is fail-closed, so a missing/malformed routed shape on ANY
  wet lane-day would have aborted the run. It did not: the consumer read the
  routed weights everywhere.
- Negative proof (Static + Ran): `r7d8_dc01_surface_hourly_weights` is
  reached only from the `Dc01SourceShape` match arm, which active lane-days
  never carry (the routing step and the zero-source branch both set
  `RoutedHydrograph`); the D13 tests
  (`wave1_span_routed_hydrograph_shape_supersedes_dc01_weights` + the new
  `laned_active` unit tests) pin the supersede direction, and the 1
  full-mesh-hold lane-day consumed the routed SOURCE series under the
  counted rev-27 degeneracy — not the DC01 weight function.

## Consumer 3 — runtime closure (INV-OFEROUTE-012 hard-fail)

- Producer source (Static): per-day `DirectLanedActiveDayBooks` — router
  books from the solver mass ledgers, the INDEPENDENT soil-release ledger
  (`Σ q_runoff × A_i` from the R4A surface), lane books from the R4B
  `storage_downstream_operands`, `ui_SCrunf`-lineage source consumption via
  the three-limb weights, `latqcc` bypass recorded from the terminal lane's
  subsurface projection on ALL days (routed and zero-source).
- In-memory state/frame object (Static): `DirectLanedActiveDayBooks`
  (executor day scope) + `DirectLanedActiveRunSummary` on the run frame.
- Runner handoff (Static): `frame.laned_active_summary` → retained
  publication → manifest `laned_active` block (fail-closed if missing under
  the selector).
- Downstream call site (Static): `laned_active_enforce_day_closure` at each
  day boundary — typed `DirectKernelGuardFailure` with day coordinates.
- Output/API surface (Ran, seam-fixed run): manifest residual maxima
  supply `7.3e-16`, router-internal `2.5e-13`, SEAM cross-ledger `5.0e-14`,
  day identity `2.5e-13`; `total_latqcc_outlet_m3=208132.8460294917`
  (ALL-days scope, CR-L1 fixed).
- INDEPENDENT-RECONSTRUCTION proof (Ran): `total_latqcc_outlet_m3` equals
  the PUBLISHED pass-parquet `sbrunv` column sum (`208132.8460294918`) to
  1 ulp — the closure evidence reconstructs from produced outputs, not from
  producer-internal counters (QA-H2's demanded bar).
- LIVENESS proof (Ran): during implementation the first executed run FAILED
  at the day residual (the mesh-basis aliasing) and the corrected run FAILED
  at lane 1 day 367 (full-mesh-hold shape). QA review then found the
  post-basis-fix books had become solver-self-referential; the SEAM
  cross-ledger check restores the independent guard, and its introduction
  immediately exposed a real ~0.11 % hour-straddling booking error
  (eliminated by the hourly breakpoints). Three real defects caught by this
  check class, all fixed contract-first.
- Negative proof (Static): no compatibility path computes the closure; the
  checks live only on the active loop and cannot be satisfied by the shadow
  or default paths (which never construct the books).

## Consumer 4 — rev-21 friction operands on the active path

- Producer sources (Static): native management `routing_coefficients`
  (fail-closed builder shared with the shadow), Wave-1 seed geometry,
  typed-management `canhgt` (lane authority), live
  `wb14_hourly_rainfall_m / 3600` and post-growth
  `evapotranspiration_compute_inputs.leaf_area_index` from the day frame.
- In-memory state/frame object (Static): `DirectLanedActiveLaneConfig` on
  `DirectRunFrame.laned_active` (statics) + the live `DirectDayFrame` fields
  (dynamics) + the per-lane-day `CellParameters` mesh.
- Runner handoff (Static): `laned_active_config()` (day-input builder) →
  `frame.laned_active` before streaming.
- Output/API surface (Ran): the routed evidence block itself (every routed
  number downstream of these operands) + typed failures naming the operand
  on violation.
- Downstream call site (Static): `laned_active_route_lane` builds
  `CellParameters` per lane-day from exactly these sources with the rev-21
  guards (LAI ≥ 0 finite; `h_c > 0` required when `LAI > 0`; rainfall slots
  finite non-negative — typed failures).
- Negative proof (Ran): `h2637_active_fails_closed_without_routing_coefficients`
  passes — a legacy management without the native extension cannot start an
  active run; no placeholder operand path exists in the active step.

## Consumer 5 — D12 source-shape limbs on the active path

- Producer source (Static): `dc01_surface_runoff_hourly_weights` (the ONE
  shape authority, ORCH `runoff.rs:1405`) over
  `wb14_hourly_excess_m` + `hourly_saturation_carry_m` (`ui_SCrunf` lineage)
  + `hourly_routed_melt_m` (rev-22 limb), consumed PRE-publication from the
  live day frame by `laned_active_lane_source`.
- Daily-sum closure (Ran): `max_supply_reconstruction_rel = 7.3e-16` across
  all routed lane-days (hard-fail tolerance 1e-9) — the weights-times-total
  series closes to the lane-local supply exactly.
- In-memory state/frame object (Static): the weights array +
  `LanedActiveLaneSource.depths_m` in the executor's window pass and routing
  step.
- Runner handoff (Static): none required (orchestrator-internal by design;
  the runner supplies only the static config).
- Output/API surface (Ran): `total_source_m3` + the supply-reconstruction
  maximum in the manifest block.
- Negative proof (Static + Ran): the uniform-fallback class is COUNTED and
  contract-dispositioned (rev-27 D12 row: counted production residual class,
  no fidelity authority) — `days_uniform_shape=3`,
  `lane_days_erosion_source_shape_degenerate=1`; no unauthorized shape
  source exists in the active step (the weights function is the single
  authority, same as DC01/shadow).

## "What still reads the old path?" sweep (Static)

- DC01 surface admission: dead on active lanes (guard + suppressed
  publisher); ALIVE by design on the default path (`INV-OFEROUTE-010`).
- DC01 LATERAL admission: alive on active lanes BY CONTRACT (`ui_LfCrf`
  stays subsurface; the router supersedes surface runon only).
- DC01 erosion shape: alive only for default/off (`Dc01SourceShape` arm).
- Published per-lane `runvol`/`Q`/`QOFE` and the watershed-facing HBP outlet:
  still the SC-RUNOFFPART lane-local kernel products — NOT re-pointed at
  routed water; this is the explicit rev-27 named follow-on gate, recorded in
  the contract, the architecture artifact, and the worker handoff (not a
  silent compatibility read).
