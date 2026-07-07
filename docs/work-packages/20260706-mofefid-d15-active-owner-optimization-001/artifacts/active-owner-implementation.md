# Active Owner Implementation (D15A-P2)

Status: **EXECUTED**.

Evidence mode: **Ran** for every executed claim (commands in this artifact and
`gate-results.md`); Static for design/authority statements.

Contract authority: `SC-OFEROUTE-001` rev 27 (amended BEFORE these edits; two
mid-implementation semantics gaps were also resolved contract-first — the
mesh-basis conversion and the full-mesh-hold erosion degeneracy, both recorded
in the rev-27 rows before the corresponding code landed).

## Production edits

Orchestrator (`crates/openwepp-hillslope-orchestrator`):

- `src/direct_runtime/laned_active.rs` (NEW): the active-owner module —
  per-lane static config (`DirectLanedActiveLaneConfig`, rev-20/21 sources),
  the lane-day routed source builder (ADR-0036 weights-times-total over the
  three D12 limbs, from the LIVE day frame, with the supply-reconstruction
  hard-fail), the INV-OFEROUTE-009 double-feed guard (typed
  `DirectKernelGuardFailure` on resolved DC01 surface runon > threshold), the
  rev-27 day-window rule, the routed-erosion-weights mapping (hour-aligned bin
  sums, hour-24 tail fold, dry all-zero, counted full-mesh-hold degeneracy),
  the per-lane routing step (rev-21 operand validation → shared
  `route_single_ofe` → D13 producer flip → books), and the day-closure
  hard-fails with the run summary.
- `src/direct_runtime/03_executor.rs`: `run_day_spans` split into
  `run_day_spans_hydrology` + `run_day_spans_erosion_and_ledger` (default path
  calls both back-to-back — identical span sequence);
  `publish_dynamic_transfer_to_downstream_with_ownership` (surface portion
  suppressed under router ownership, LATERAL carry unchanged);
  `run_laned_active_publication_stream` — the two-phase active day loop
  (phase 1: all lanes' hydrology + lateral-only transfer; window; phase 2:
  per-lane route → erosion-inflow-intake refresh → erosion + ledger → row →
  publishers → commit; day-closure hard-fail per day); dispatch at the top of
  the publication-stream entry; `run_skeleton` fails closed under the
  selector (no silent non-activation).
- `src/direct_runtime/00_core_frames.rs`: `DirectRunFrame.laned_active`
  (config = the in-orchestrator selector) + `laned_active_summary`;
  `DirectDayFrame.laned_active_routing` evidence record.
- `src/ofe_routing/cascade.rs`: extracted `route_single_ofe` +
  public `UpstreamHandoff` so the active path and `run_cascade` share ONE
  routing code path (the shadow's H2637 outputs are bit-identical
  post-refactor — hashes + manifest block re-verified).

Runner (`crates/openwepp-runner`):

- `src/hillslope/laned_active.rs` (NEW): `OPENWEPP_LANED_ACTIVE=1` selector.
- `day_input_and_helpers/00_builders_and_authority.rs`:
  `laned_active_config()` — the SAME fail-closed geometry/coefficient
  extraction as the shadow (selector-parametrized message) plus typed-management
  `canhgt` from the lane authority.
- `05_runner_execution_and_outputs.rs`: shadow/active mutual-exclusion hard
  fail; config attach; fail-closed check that an active run produced the
  summary; manifest `laned_active` block.
- `00_runner_intake_and_lane_setup.rs`: `LanedActiveProvenance` manifest
  struct + threading.

## Two semantics gaps found by the first executed runs (both contract-first)

1. **Mesh-basis seam** (first run failed the day cascade residual): the soil
   books release `q_runoff × lane_area` m³ while the 1-D mesh's plan area is
   `slplen × width`. The routed depth series is now scaled by
   `area / (slplen × width)` (factor 1.0 when equal) so the router owns
   EXACTLY the soil-released volume on any geometry; the router books are
   booked from the solver's own mass ledger. This is the QOFE/Q-class
   area-duality trap caught by the composed day identity — the check did its
   job on day one.
2. **Full-mesh-hold erosion degeneracy** (run then failed at lane 1 day 367):
   the erosion wet-gate (1e-12 m) and the router dry floor (1e-9 m mesh
   depth) are inconsistent at the seam — a tiny-runoff lane-day holds all its
   water on-mesh, discharges zero outlet mass, and no unit-sum outlet shape
   exists. Rev-27 records the DEGENERACY rule: the shape falls back to the
   normalized routed SOURCE series, COUNTED per run
   (`lane_days_erosion_source_shape_degenerate`) — never silent. H2637
   measures exactly **1** such lane-day in 731 days × 19 lanes.

## Executed evidence (Ran)

- Active H2637 endpoint (3 runs, `taskset -c 4`, release):
  `37.50 / 37.48 / 37.44 s` user — **well inside the S5-adjudicated budget**
  (the active path routes rainfall-only lane supplies, which are smaller and
  earlier-windowed than the shadow's DC01-shaped reconstruction).
- Manifest `laned_active` block (deterministic across runs):
  `days_seen=731, days_routed=610, days_uniform_shape=3,
  lane_days_erosion_source_shape_degenerate=1,
  max_supply_reconstruction_rel=7.3e-16, max_day_cascade_residual_rel=2.5e-13,
  max_day_identity_residual_rel=2.4e-13, total_source_m3=373994.99,
  total_routed_outlet_m3=371782.96, total_end_window_storage_m3=3168.46
  (0.85 % of source — the rev-27 reset residual class),
  total_clamp_m3=956.43 (0.26 % of source),
  total_tail_fold_m3=36581.69, total_latqcc_outlet_m3=195192.39`.
  All three rev-27 hard-fail tolerances hold with 3-7 orders of margin ON
  while being LIVE (the two pre-fix failures above prove the hard-fails
  actually fire).
- Default/off byte identity (INV-OFEROUTE-010): post-implementation off run
  reproduces the pre-implementation baseline hashes exactly
  (`H2637.hbp 948faf82…`, `H2637.loss.json 725f5723…`,
  `H2637.pass.parquet f0d1be11…`).
- Shadow bit-identity after the `route_single_ofe` refactor: shadow-on run
  reproduces the same protected-output hashes and a JSON-identical
  `laned_shadow` manifest block.

## Behavioral consequences surfaced for adjudication (Ran, expected)

Active-mode pass surfaces differ from DC01 mode as the contract-authorized
supersede implies — recorded here so no reviewer discovers them by surprise:

- Outlet `runvol` Σ drops `374,033 → 71,941 m³`: per-lane published runvol is
  now the lane-LOCAL rainfall-only kernel product; the actual surface export
  (`371,783 m³`) lives in the routed books/manifest. The watershed-facing HBP
  outlet re-pointing is the NAMED rev-27 follow-on gate.
- `sbrunv` (lateral) essentially unchanged (`208,153 → 208,133 m³`): the
  lateral admission path is untouched, as required.
- Erosion `tdet` Σ drops `5,802 → 23 kg` and `peakro` drops ~11x: erosion's
  water MAGNITUDE operands (local `q_runoff`, peak) shrink because downstream
  lanes no longer re-infiltrate runon; the routed-over water is not part of
  the lane's local erosion water column (D13 is shape-only by design, and the
  package excludes erosion semantic changes). Whether active-mode erosion
  transport should consume routed-water magnitude is a NAMED follow-on
  adjudication (recorded in the worker handoff), not silently changed here.

## Post-review repairs (dual-review fix batch, same package)

- **QA-H2 seam repair**: the mid-implementation mesh-basis fix had re-based
  the closure books onto the solver's own ledger, making checks (b)/(c)
  self-referential. Restored the independent soil↔router SEAM cross-ledger
  hard-fail (`|injected_solver − Σ q_runoff·A|` ≤ 1e-9 rel) and made it
  exact by passing the HOURLY FORCING BREAKPOINTS through
  `route_single_ofe` on the active path (High-2 no-straddle rule; shadow
  passes `&[]`, bit-identity re-verified). Introduction of the seam check
  immediately exposed a real ~0.11 % hour-straddling booking error — now
  structurally eliminated. Day-identity check re-formed on the SOIL-side
  release.
- **CR-M1/M2**: test env-hygiene (harness contract documented; sibling
  selector vars neutralized in every helper).
- **CR-L1**: `total_latqcc_outlet_m3` now covers all days (terminal lane,
  zero-source days included) — and reconstructs the published `sbrunv`
  parquet sum to 1 ulp.
- **CR-L2**: day coordinates in all closure hard-fail details.
- Contract reconciliation: mesh-basis rule, latqcc booking wording, D12
  uniform-fallback active disposition, erosion water-magnitude follow-on —
  all recorded in the rev-27 text (QA-M1..M4).

## Line-count governance (measured, final tree)

`kinematic_wave.rs` 1,639; `03_executor.rs` 1,223; `laned_active.rs` 868;
`cascade.rs` 601. Pre-existing WARN-band file touched:
`00_builders_and_authority.rs` 2,732 (was 2,696; +36 additive lines for the
selector-parametrized geometry builder) — WARN acknowledged; below the
3,000 refactor-required bar; no new file enters the WARN band. Disposition:
acceptable for this package; the builder file is a standing refactor
candidate.
