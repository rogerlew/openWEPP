# Frost Single-Solve (sub-5× WP-2, finding F1)

Status: SCAFFOLDED — blocked on WP-1
(`20260701-hillperf-mechanical-winter-overhead-001`) merging, and on a WP-1
exit re-profile confirming F1's residual weight.

Package id: `20260701-hillperf-frost-single-solve-001`

## Objective

Resolve finding F1 of
`docs/backlog/20260701-hillslope-sub5x-performance-assessment.md`: the hourly
winter frost partition (`compute_direct_winter_frost_partition`) is solved
twice per winter OFE-day — once in the runner day-input builder from
start-of-day lane state (to pre-project frozen infiltration capacity,
storage liquid delta, and same-day hydrology layers), and once in the R4A
runoff span from post-ET evolved layer state (the outcome that feeds
publication and carry). Same controls/thermal/hourly inputs per day; only
the soil-liquid/layer state differs. Snow is single-solved (builder-side);
frost's second solve is R7G-retrofit residue, not a ratified two-pass design.

## Execution model (operator-ratified 2026-07-01)

Claude implements; Codex reviews before close. **One planned operator
touchpoint:** if Stage 1 shows material divergence between the two solves,
implementation stops for a yes/no on the adjudication memo. If the solves
agree within contract tolerance, the package proceeds autonomously as a
refactor.

## Stages

1. **Paired-solve diagnostic (decides everything).** Instrument both solves;
   dump paired outcomes across all H2637 winter OFE-days (and the committed
   carnivorous-adobo fixture for a second climate): day-by-day deltas on
   frost depth, frozen water, `infcap_frz_m_s`, `storage_liquid_delta_m`,
   layer projections. Artifact: divergence distribution (max, p99, count
   above contract tolerance).
2. **Adjudication.**
   - Equivalent within tolerance → single-solve is a refactor; record the
     evidence; proceed to Stage 3 with the *executor-side* solve as the
     survivor (its outputs are today's publication/carry authority, so
     H2637 byte identity is expected to hold — gate on it).
   - Materially divergent → one-paragraph memo to the operator: where they
     diverge, what `SC-SNOWFREEZE-*` and the legacy `frsoil` daily ordering
     support (verify against the pinned 260430 baseline — static claim to
     confirm, not assume), recommendation, yes/no. Output changes are then
     expected; identity gate is replaced by conservation/closure + frost
     observation-suite re-run + first-divergence WAT evidence.
3. **Implementation.** Restructure to one solve per (lane, day) whose
   outcome feeds both the pre-phase inputs (frozen infiltration capacity et
   al.) and R4A (mirroring the snow single-solve pattern). No wrapper around
   the old shape; the losing solve path is deleted.
4. **Contingency (only if the post-F1 endpoint is still above target):**
   F4 (incremental fine-layer depth tracking; `round` audit) and the
   deferred WP-1 tail (F3-narrowed forcing-core sharing; span-report
   projection drop).

## Gates

- Entry: WP-1 merged; fresh 3-rep baseline + re-profile on the merged state.
- Stage-1 artifact complete before any production edit.
- Stage-3: full workspace gates; H2637 identity if Stage 2 ruled
  equivalence, else the divergence-documented gate set above; endpoint
  timing (3-rep, quiet window); `compatibility_edge_invocations=0`.
- Exit: Codex review dispositioned; backlog assessment updated; ROADMAP note.

## Context from WP-1 (2026-07-01)

WP-1 landed the mechanical lane at **45.85 s indicative (4.75× same-host
legacy)** — under the 5× bar. F1 is therefore no longer required for the
gate; it remains the largest single remaining lever (each winter day still
pays a full duplicate hourly solve) and the path toward the deeper floor
argued in the array-native spec. Its Stage-1 diagnostic is cheap and decides its own
fate; run it before deciding to descope.
