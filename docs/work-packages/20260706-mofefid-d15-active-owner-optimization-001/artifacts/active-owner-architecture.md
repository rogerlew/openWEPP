# Active Owner Architecture (D15A-P0)

Status: **EXECUTED** (authored before production edits).

Evidence mode: Static (design over the traced source; anchors per
`operand-lineage.md`). Contract authority: `SC-OFEROUTE-001` rev 27 (amended
first, this package).

## Selector

`OPENWEPP_LANED_ACTIVE=1` — explicit opt-in on the production
publication-stream path (`execute_direct_publication_stream`, the path the
hillslope CLI drives). Chosen over a `.run` schema flag because the package
excludes public input-schema expansion and the env-selector pattern is the
established Lane D opt-in surface. Fail-closed preflight before streaming:

- complete native `routing_coefficients` for every scheduled lane (reuses the
  shadow's fail-closed geometry builder);
- mutual exclusion with `OPENWEPP_LANED_SHADOW=1` (the shadow's
  published-row reconstruction basis is DC01-shaped; over an active run it
  would mis-reconstruct — hard error, not silent precedence);
- hourly-lane carries present (`seam_require_hourly_lane` posture — enforced
  structurally: the direct runtime is the hourly lane; the routing step
  hard-fails if the R4O hourly projection is absent).

Default/off isolation: with the env unset, no active config is constructed,
the executor takes the existing single-pass loop (identical code path), and
protected outputs stay byte-identical (`INV-OFEROUTE-010` gate re-run in P4).

## Execution-order change (the two-phase active day loop)

DC01-mode loop (unchanged when inactive): per day, per lane —
seed → all spans → row → publishers → commit.

Active-mode loop (publication-stream only): per day —

**Phase 1 (hydrology, lane order preserved):** for each lane: seed day frame →
apply day input → hydrology spans (R5B…R4PQZ, everything before the erosion
span) → publish the dynamic transfer with the SURFACE portion zeroed and the
LATERAL portion unchanged (DC01-disable point) → hold the day frame.
Lane `i+1`'s R4J therefore sees zero surface runon (router owns it) and the
unchanged lateral carry (`ui_LfCrf` stays subsurface).

**Window:** after phase 1, the day's routing window is
`(last active source hour over all lanes) + 6 h` (rev-27 window row) — the
same rule the shadow used, computable only once all lanes' sources exist;
this is why the loop is two-phase.

**Phase 2 (routing + erosion + ledger, lane order preserved):** for each lane:
- build the source series (DC01 weights × `q_runoff_m`, the three D12 limbs)
  with the supply-reconstruction hard-fail;
- validate rev-21 dynamic operands (same guards as the shadow builder, typed
  errors);
- run the kinematic-wave solver via the SAME single-OFE routine the cascade
  uses (extracted `route_single_ofe`), upstream BC = previous lane's routed
  bin series, width-scaled;
- double-feed guard: typed hard fail if the lane's
  `runoff_shadow_projection.runon_input_m > 0` (INV-OFEROUTE-009 runtime
  form — DC01 surface admission must be dead on active lanes);
- build the routed erosion weights (hour-aligned bin sums; tail fold; dry
  all-zero) and set `hydrograph_shape_authority = RoutedHydrograph` +
  `routed_hydrograph_runoff_fraction` on the day frame BEFORE the erosion
  span;
- erosion span + R3B ledger span → row → erosion-inflow publisher → commit.

**Day closure (hard-fail):** after the terminal lane: (a) per-lane supply
reconstruction already checked; (b) day cascade residual
(router books) ≤ 1e-9 relative; (c) the assembled hillslope-day identity
(operand-lineage table) ≤ 1e-6 × max operand magnitude — typed
`DirectRuntimeError` on violation. Counters accumulated for the manifest
block: source, routed outlet, mesh end-window storage, clamp, tail fold,
max residuals, days routed.

## State ownership

- Per-lane static config: `DirectLanedActiveConfig` (orchestrator type), built
  by the runner from the shadow geometry builder + lane-authority `canhgt`,
  attached to `DirectRunFrame`.
- Routed handoff between lanes: held by the executor's day-scope state (the
  previous lane's `RoutingResult` bin series + width), not on lane state (it
  is day-transient).
- Per-lane-day routed outputs for evidence: `DirectLanedActiveDayRouting`
  (outlet volume, mesh ΔS, clamp, tail fold, weights) stored on the day frame
  so the runner's row consumer can build the manifest `laned_active` block.

## Erosion consumer handoff

`RoutedHydrograph` authority + weights set on `day_frame.erosion_inputs`
before the erosion span; `r7d8_surface_hourly_weights` then selects the D13
routed consumer with its landed fail-closed validation; `wave1_hourly_weights`
propagates the routed shape to the Wave-1 hourly plan, the published
`hourly_runoff_fraction` surface, and the E.3 erosion-inflow publisher.
Negative proof: on active lanes the DC01 weight function is not invoked for
the Wave-1 shape (the authority match arm short-circuits it).

## Timing posture

The active path routes the same event days over the same windows as the
shadow (same window rule, same solver, same mesh resolution), so the S5
adjudicated budget (~79 s H2637) is the expected active cost; P4 measures the
real active endpoint.

## Publication scope of the activation claim (explicit boundary)

Active mode owns surface water inside the hillslope production runtime:
inter-OFE surface runon (DC01 surface admission disabled), the erosion hourly
shape, and the day-closure books, with routed evidence published via the
manifest `laned_active` block and the routed-shaped `hourly_runoff_fraction`
output. Per-lane WB publication surfaces (runvol/Q/QOFE and the
watershed-facing HBP outlet surface) remain `SC-RUNOFFPART-001`-owned
lane-local products; re-pointing them at routed water is a NAMED follow-on
gate (recorded in rev 27 and the worker handoff) requiring
SC-RUNOFFPART/SC-ROUTE authority outside this package's contract write set.
This is not a double-feed: active-lane published runoff is the lane's own
rainfall-only kernel product, and the routed path is the only surface-runon
carrier.

## Rejected alternatives

- Fixed 30 h window for every lane-day (kills the two-phase need): routes the
  draining mesh far past the 6 h tail on early-source days — measured-class
  cost blowup; also diverges from the shadow's recorded window semantics.
- Per-lane adaptive windows: downstream windows would have to grow ~6 h per
  lane (upstream tails), or drop upstream tail mass (handoff conservation
  violation).
- Routing as a post-hoc runner collector (shadow-style) with production
  writeback: producer-only evidence; cannot feed the erosion span or the R4J
  disable point — exactly the compatibility-wrapper shape the package
  excludes.
- `.run`-file selector: public input schema expansion, excluded.
