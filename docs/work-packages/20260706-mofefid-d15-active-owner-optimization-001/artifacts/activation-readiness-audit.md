# Activation Readiness Audit (D15A-P1)

Status: **EXECUTED** (pre-implementation audit; the "After D15A" column is
finalized with the P4 gate evidence).

Evidence mode: Static (source/contract read) unless labeled Ran.

Contract state at audit time: `SC-OFEROUTE-001` rev 27 (this package's
contract-first amendment: selector row, day-window/reset row, erosion tail-fold
rule, activation tolerances — authored BEFORE production code per the
science-contract sequencing rule).

| Precondition | Before D15A (rerun audit, confirmed this session) | Rev-27 authority | After D15A (target; P4 evidence) |
|---|---|---|---|
| `INV-OFEROUTE-010` off-path byte identity | PASS for shadow; no active selector existed | selector row: off = byte-flat, re-verified with the selector present | PASS required (P4 byte-identity gate) |
| `INV-OFEROUTE-011` / GAP-005 | PASS (rev 25/26; D10B tests green in this session's S4 focused run — Ran) | unchanged | PASS (no D-val surface touched; S4 preserved bit-identity) |
| `INV-OFEROUTE-012` `ui_SCrunf` source term in active runtime | HOLD — pure seam helper only | source = the three-limb weights-times-total series from the live day frame (rev-27 status text) | live in the executor routing step |
| `INV-OFEROUTE-012` `latqcc` bypass in closure | HOLD — helper only | closure operand table (operand-lineage.md); in-frame depth × outlet area | live in the day-closure hard-fail |
| `INV-OFEROUTE-012` runtime closure hard-fail | HOLD — nothing invoked it | rev-27 tolerance notes: supply ≤ 1e-9 rel; day cascade residual ≤ 1e-9 rel; day identity ≤ 1e-6 × max operand, clamp booked | live, typed `DirectRuntimeError` |
| DC01 daily-lump surface runon disabled on active lanes | HOLD — `apply_dc01_runon_supply_admission` unconditional | INV-009 runtime form: surface transfer zeroed at the publisher; typed double-feed guard on `runon_input_m`; LATERAL admission explicitly unchanged | live + negative proof (consumer-path-proof.md) |
| Rev-21 friction operands consumed by the real active path | HOLD — shadow only | same sources/guards, consumed by the executor routing step | live (consumer-path-proof.md) |
| D12 source-shape limbs consumed by the real active path | HOLD — DC01/shadow only | the weights function over the three limbs feeds the ACTIVE source; supply reconstruction is the daily-sum closure | live |
| D13 routed hydrograph feeds erosion | PARTIAL — consumer landed, producer absent (`Dc01SourceShape`/`None` in production) | rev-27 producer flip: `RoutedHydrograph` + tail-fold weights on active lanes | live (consumer-path-proof.md) |
| Endpoint timing within an accepted budget | HOLD — 91.6 s vs D14 29.9 s | S5 adjudication: optimized 78.8 s accepted as the corrected-scheme budget (decomposition in optimization-results.md), flagged for operator ratification | adjudicated; active-mode endpoint measured at P4 |
| Active selector exists | HOLD — none | `OPENWEPP_LANED_ACTIVE=1`, publication-stream path, shadow mutual exclusion, fail-closed preflight | live |

## Semantics gaps found and resolved contract-first (P1 outcome)

1. **Clamp booking**: the H2637 baseline proves the positivity-clamp injection
   is ≥ 3.8 % of routed source — a day identity that omits it cannot close.
   Rev-27 tolerance notes bind the clamp as an explicit input-side term.
2. **Day window under inline execution**: the shadow's window rule needs all
   lanes' sources, forcing the two-phase active day loop; recorded as the
   rev-27 window row (fixed-window and per-lane-window alternatives rejected
   in `active-owner-architecture.md`).
3. **Inter-day mesh reset**: end-of-window router storage is booked in day
   `ΔS` then reset — surfaced as a named bounded residual class
   (`routed_end_window_storage_m3`), measured in the P4 evidence; the
   inter-day carry decision is a named follow-on gate.
4. **Erosion 24-hour frame vs 30-hour routed window**: explicit hour-24 tail
   fold, surfaced (`routed_tail_fold_m3`) — no silent truncate/reshape.
5. **Publication scope**: the activation claim covers hillslope-internal
   ownership + manifest/`hourly_runoff_fraction` surfaces; per-lane WB and the
   watershed-facing HBP outlet stay SC-RUNOFFPART-owned (named follow-on) —
   recorded in rev 27 so the boundary is contract-visible, not silent.

No missing or contradictory canonical authority remains for the in-scope
active consumers; implementation may proceed (P2).

## P5 final status

Every "After D15A" target column above is EXECUTED-PASS on the final tree —
see `gate-results.md` (initial + post-review re-verification tables) and
`consumer-path-proof.md`. The rev-27 contract text was reconciled after the
dual reviews (mesh-basis rule, latqcc booking, D12 uniform-fallback active
disposition, seam cross-ledger tolerance, erosion water-magnitude
follow-on).
