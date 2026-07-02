# Stage 3 Design — Single Start-of-Day Frost Solve (ingress application)

Adjudication: operator ratified 2026-07-01 — proceed; **the frost
observation rubric (INV-SNOWFREEZE-050) is the acceptance bar** (single-solve
ships only if it scores no worse than the current default on the frost
network). H2637 identity is intentionally not preserved.

## Why the shape is forced

The naive alternative — pass the builder outcome to R4A and apply it there —
is structurally rejected by the code's own guards (Ran/Static):
`apply_r4a_frost_layer_projection` **overwrites** `layer.theta_m` with the
solve's absolute `theta_after_m` (runoff.rs:1613), and
`apply_r4a_winter_frost_outcome` hard-asserts (1e-9) that the outcome's
`soil_water_after_frwatc_m` matches the layers it is applied to. Applying a
start-of-day-solved outcome onto post-ET evolved layers would both discard
the day's withdrawals and trip the assert. The outcome must be applied to
the same layer basis it was solved from — the start-of-day layers — which is
also exactly the legacy ordering (`contin.for`: frost before infiltration)
and the contract's once-at-ingress handoff (`INV-SNOWFREEZE-012`).

## Stage 3a — the rewire

1. **Carry the outcome.** `DirectPublicationDayInput` (and the
   constructor-inputs path used by tests) gains
   `winter_frost_outcome: Option<Box<DirectWinterFrostPartitionOutcome>>`,
   filled by the builder from its existing single solve.
2. **New ingress span** `run_r4w_frost_ingress_span`, inserted in
   `run_day_spans` before `r4c_storage_input` (the first water-moving
   phase). Body = today's `apply_r4a_winter_frost_outcome` semantics,
   relocated: winter-column/frost carry, layer projection applied over
   `percolation_inputs.layers` (same `lane.subsurface_layers` basis the
   builder solved from — the 1e-9 self-consistency asserts hold by
   construction), `storage_reconciliation_inputs.frost_liquid_delta_m`,
   hydrology-projection frozen fields, `water.soil_water_m`, and the
   post-frost layers written into `percolation_inputs.layers` +
   `subsurface_compute_inputs.layers` so percolation/ET/subsurface operate
   post-frost (the legacy semantics).
3. **R4A shrinks.** The re-solve
   (`compute_r4a_winter_frost_partition`, `latest_r4a_frost_layers`,
   `r4a_frost_layers_with_local_partition_excess`) and the R4A-side apply
   are deleted. Retained-local-liquid bookkeeping
   (`frost_retained_local_liquid_m` / deferred-excess lineage) stays where
   its consumers need it; R4X keeps its is-winter gate via the day input.
4. **Builder keeps** compute-inputs (gates/guards), infcap, and now the
   outcome. Its previously pre-projected frost day-input fields
   (`frost_storage_liquid_delta_m`, `frost_layer_carry_projection`,
   frozen hydrology-projection fields) become redundant once the ingress
   span is authoritative — removed in **Stage 3b** (a separate
   single-authority cleanup commit) so 3a's behavioral diff stays minimal.
5. **Tests** driving R4A's solve (r7g frost tests) migrate to the new
   boundary: construct the outcome via
   `Wb11HydrologyKernel::compute_direct_winter_frost_partition` in-test and
   drive the ingress span.

## Gates (in order)

1. Full workspace suite green (tests migrated, not weakened).
2. H2637 runs to completion — the in-run conservation/closure guards are
   the first correctness net (a broken rewire fails loudly at the first
   divergent lane-day).
3. **First-divergence evidence**: WAT diff vs the WP-1 baseline outputs,
   first divergent day/field documented (expected — this change is
   output-affecting by design).
4. **The bar — frost rubric before/after**: `observed_harness.py compare`
   on all five frost sites + `classify_residuals.py`, diffed against the
   before-state captured at branch commit `39061021` (in progress at
   design time). No-worse per the INV-SNOWFREEZE-050 cell taxonomy.
5. Endpoint timing (expected ~40 s from the halved dominant block; the
   solve runs on every OFE-day — Stage-1 finding).
6. `compatibility_edge_invocations=0`; fmt/clippy/deny; Codex review.

## Scope amendment (pre-implementation read of the R4A span)

The R4A frost block is three coupled steps, not one:
`reconcile_r4a_frost_runtime` (solve+apply) → runoff partition compute →
`rebalance_r4a_frost_projection_to_storage_target` (only when frost
reconciled). The rebalance exists because the late apply perturbs storage
after the day's fluxes have run; under ingress application the perturbation
precedes the fluxes, so the rebalance is expected to become structurally
unnecessary — its deletion is in 3a's scope, with the in-run closure gates
as the check. The deferred-partition-excess lineage
(`r4a_deferred_local_partition_excess_m`,
`frost_preprojected_local_liquid_m`, R4X's mid-day retained-liquid
injection) is calibrated around the current two-solve shape and must be
re-derived against the ingress shape rather than assumed unchanged — it is
the most likely source of first closure failures during bring-up.

## Before-state rubric (captured, archived)

`artifacts/rubric-before/`: five-site comparison reports + residual
classification at branch `39061021`. Headline (classify_residuals, Ran):
0 defect-attribution-eligible sites, 0 `OPENWEPP-DEFECTIVE`; primaries
`SNOW-CONTROL-FAILED` ×3 (sites 1/2/4, snow-confounded) and
`INCONCLUSIVE` ×2 (sites 3/5, no paired observed snow). Per-site
quantities to hold no-worse after the change: matched rows
392/200/10,643/83/4,356; max abs frost residual 0.247/0.390/–/0.787/– m;
isotherm exceedances 0/0/3,658/0/125; snow-control failures 218/72/–/20/–.

## Implementation correction (recorded for review)

An interim implementation shrank the rewire on a **misread**: the claim that
"the builder already applies the frost layer effects via
`same_day_frost_hydrology_layers`" is false — that helper returns the lane
layers unchanged except for the stale-clear on thaw-complete days (Static:
`01_frost_and_layer_helpers.rs:214-234`). Only the deleted R4A apply ever
applied the layer projection. The shrunken shape therefore shipped freeze-day
deltas into R4B with no layer basis carrying them, and the in-run guard
caught it exactly as designed (Ran: seven snowbench-family tests failed with
`storage_reconciliation.frost_storage_projection_theta_m must be nonnegative`
at site1 day 6 — the first material freeze day).

Corrected to the design's original shape: `apply_r4w_winter_frost_ingress`
applies the carried outcome to the day's layer-input basis
(`percolation_inputs.layers`, mirrored to `subsurface_compute_inputs.layers`)
**between R4C and R4I** — R4C captures the pre-frost `storage_initial`
scalar, so the daily identity `initial + fluxes + frost_delta = final`
closes by construction; all water-moving phases then operate on post-frost
stores (the legacy frsoil-before-infiltration ordering). The ingress carries
the relocated apply semantics wholesale (material/no-material paths,
1e-9 outcome/layer self-consistency asserts — valid by construction on the
solve's own basis — and the frozen-water bound). R4A retains only the
retained-local-liquid bookkeeping.

Verification lesson: the snowbench harness tests acted as the effective
mini-endpoint for this failure class — they run real multi-year site
fixtures in seconds and fired before H2637 was ever attempted.
