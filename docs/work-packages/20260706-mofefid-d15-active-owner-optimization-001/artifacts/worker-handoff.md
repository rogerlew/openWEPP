# Worker Handoff (D15A-P5)

Status: **HANDOFF-RECORDED**.

D15A completed opt-in activation (`SC-OFEROUTE-001` rev 27; evidence in this
artifact set; final tree = this package's working-tree diff over `9f536aad`).

## For the operator (ratification items)

1. **Timing adjudication** (`optimization-results.md` S5): the package
   adjudicated `78.8 s` user (H2637 shadow endpoint) as the corrected-scheme
   budget, superseding D14's `29.9 s` (which priced the pre-rev-24 latently
   unstable scheme). The executed ACTIVE endpoint is `37.4 s` — inside every
   historical budget — so the adjudication is load-bearing only for the
   diagnostic shadow surface. Please ratify or redirect.
2. **Codex re-check** (D10B pattern): the dual reviews in this package are
   delegated in-session subagent reviews (recorded in `review-codex.md` /
   `review-qa.md`, dispositioned in `review-disposition.md`); an
   operator-dispatched Codex re-check of the diff is recommended before/at
   merge.

## Next actionable items (in order)

1. **D16 default-promotion adjudication**
   (`20260705-mofefid-d16-default-promotion-adjudication-001`, §6.1): its
   inputs now exist — executed opt-in activation evidence, active endpoint
   timing (`37.4 s` H2637), protected-output identity, and the residual-class
   counters. D16 remains a policy/release gate; nothing here pre-decides it.
2. **Active-mode erosion water-magnitude coupling** (named rev-27 follow-on;
   contract authority: `SC-SED-001` + `SC-OFEROUTE-001`): under active
   routing, downstream lanes no longer re-infiltrate runon, so their LOCAL
   `q_runoff`/`peakro` (erosion's water-magnitude operands) shrink and H2637
   `tdet` drops `5,802 → 23 kg` (~250x). D13 deliberately moved SHAPE only.
   Whether active-mode erosion transport should consume routed-water
   magnitude (the routed hydrograph the lane actually carries, including
   routed-over upstream water) is a physics/contract adjudication — do not
   let it ride into D16 silently.
3. **Watershed-facing HBP outlet re-pointing** (named rev-27 follow-on;
   authority: `SC-RUNOFFPART-001`/`SC-ROUTE-001`): active-mode per-lane
   `runvol` publishes the lane-local rainfall-only product; the hillslope's
   actual surface export lives in the routed books
   (`total_routed_outlet_m3`). Before any watershed consumption of ACTIVE
   hillslope passes, the outlet surface must be re-pointed at routed water —
   contract-first, outside D15A's write set.
4. **Inter-day routed-storage carry** (named rev-27 window row): each active
   day resets the router mesh; the reset mass is counted
   (`total_end_window_storage_m3` = 3,167 m³ ≈ 0.85 % of routed source on
   H2637, seam-fixed run). Adopting an inter-day carry is a design gate to
   consider WITH default promotion.
5. **Shadow-path retirement question**: with the active owner landed, the
   diagnostic shadow (`OPENWEPP_LANED_SHADOW`) duplicates machinery (own
   constants, post-publication reconstruction). Consider folding or retiring
   it after D16 to avoid drift.

## Residual classes (bounded, counted, non-blocking)

- `lane_days_erosion_source_shape_degenerate = 1` (full-mesh-hold; rev-27
  rule) on H2637.
- `days_uniform_shape = 3` under active hydrology (no-authorized-source-shape
  class, D12 posture).
- End-window mesh reset 0.85 % of source; positivity clamp 0.86 % of source
  (seam-fixed trajectory; both booked in the day closures, surfaced in the
  manifest). `total_latqcc_outlet_m3` covers ALL days (routed and
  zero-source) and reconstructs the published `sbrunv` sum to 1 ulp.
- Erosion `tdet` collapse under active mode — see item 2; behavioral, not a
  closure violation.

## Keys for reproduction

- Selector: `OPENWEPP_LANED_ACTIVE=1` (mutually exclusive with
  `OPENWEPP_LANED_SHADOW=1`).
- Fixture recipe + exact commands: `baseline-profile.md`; active evidence
  block: `artifacts/logs/p4_laned_active_block.json`.
- Heavy gate: `cargo nextest run --test laned_shadow_h2637 --run-ignored
  ignored-only --no-fail-fast`.
