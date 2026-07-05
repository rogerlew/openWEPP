# Ground-Cover Authority Defect Closure — Execution & Evidence

Author: Claude Code, 2026-07-05. Evidence: **Ran** (instrument reruns) +
**Static** (legacy source resolution). Branch:
`erosion-cover-authority-defect-closure`.

## 1. The cvrcal question — RESOLVED: (a) recomputed-from-pools

- `init1.for:295-297`: pools seeded from declared covers (log inverse).
- `covcal.for:160-176`: covers re-derived from pools (+`strcov`), clamps.
- `decomp.for:977` evolves pools then calls `covcal`; no-decomp
  scenarios hold pools ⇒ covers constant at declared values.

## 2. The fix (merged design)

Ground pools (`interrill`/`rill`) as decomposition runtime state: day-0
seeded by the `init1.for` inverse from the projection's declared
`inrcov`/`rilcov` + the residue plant's `cf` (newly exported as
`residue_cover_factor_cf`); evolved by the surface decay factor with
litter added to both and Burn/Remove/Grazing applied (Cut labeled
no-op on ground pools); covers re-derived daily via the `covcal.for`
forward form (0/0.999 clamps); the residue partition publishes the
split covers; the erosion daily state consumes them (formerly both
fields read a hardcoded-zero composite). `strcov` (standing mat) is 0 —
labeled, additive-only, conservative in the closed defect's
fail-direction.

## 3. Evidence (Ran, full-length source inputs)

| Instrument | Pre-fix | Post-fix | Legacy |
|---|---|---|---|
| p61 dominant event (kg/m, per-width delivery, matched cut-point) | 25.1 (~6× over) | **3.97** | 4.2 |
| p102 outlet detachment (kg/m/yr, aggregate scope) | ~84 | **17.4** | ~19.4 |

The p61 residual (−6%) is consistent with the E.5 water cut (openWEPP
0.73× legacy runoff on that event). The `GAP-SED-009` predicted band
("into the legacy order, ~0.6–1×") is hit on both instruments.

Guards: the seed/derive round-trip identity (exact), pool constancy
under no-decay/no-litter, the covcal clamps, and the p61
dominant-event export band `[0.5, 12]` kg/m (excludes the bare-soil
regression class ~25 by a wide margin; magnitudes are not acceptance
oracles per ADR-0017 — the band is an order guard).

## 4. Small-event gate — ADJUDICATED FAITHFUL, divergence re-attributed

`wave1_day_routes_sediment` is legacy-exact against `contin.for:970-973`
(`runoff ≤ 0.010 m ∧ peakro ≤ 2.78e-6 m/s ⇒ passby`, same constants,
same AND). The E.5 small-event divergence (legacy routes a 3.9 mm
event, openWEPP gates its 4.4 mm counterpart) is the **WB16 `peakro`
operand** (openWEPP ~1.1–1.4e-6 vs legacy evidently > 2.78e-6 on those
days) — a water-side Investigation flag, bounded to trace events, NOT
an erosion gate defect. Recorded; not fixed here (water scope).

## 5. Contracts

`SC-RESIDUE-001` rev 12 (`INV-RESIDUE-020`); `SC-SED-001` rev 50 —
**`GAP-SED-009` CLOSED** with the re-judgment text carrying both
instruments' numbers and the re-attribution.

## 6. Full-suite finding: the TOL-SED-006 envelope correction

The branch-head full suite caught a REAL fidelity error in my rev-47
tolerance: the G0 pysnobal fixture (once the cover fix activated real
enrichment paths) hit `frcflw_unit_sum` at a transport-capacity-limited
profile — where the do-10 blend's floored `rillod` legitimately pushes
the composition sum above 1 by percent scale. Legacy NEVER
re-normalizes after blends and its ER consumes the raw sum
(`enrich.for` has no gate). `TOL-SED-006` is corrected (SC-SED-001
rev 51) to the corruption envelope `[0.5, 1.5]`; the published
per-class split re-normalizes at the publication boundary, preserving
the `TOL-SED-005` closure; the enriched-override publication gate uses
the same envelope. p61/p102 evidence unchanged (3.965 kg/m re-verified
post-correction).

## 7. Codex review round 1 — response record (2026-07-05)

Five findings (3 Medium, 2 Low), all CONFIRMED and fixed:
1. **Medium — the Cut ground-pool rule was source-inaccurate:**
   `decomp.for:689-693` ADDS the cut standing-mat mass to `rilrm`/
   `rigrm`/`rmogt`; my "Cut does not apply" no-op encoded the wrong
   source rule. Fixed: Cut adds the cut mass to both ground pools; our
   topology has no standing mat, so the cut-mass basis is the
   surface-pool transfer (`surface·cut_transfer_fraction`) — the
   labeled mapping with the source-true addition rule. SC-RESIDUE-001
   rev 13. (The production builder drives actions `None`, so the p61
   evidence is unaffected — as the review noted.)
2. **Medium — new operands unvalidated at the input boundary:** the
   ground-pool seeds and `cf` now fail closed in
   `validate_pool_and_rate_domain` (nonnegative-finite).
3. **Medium — the composite `cover_fraction` claim:** now COMPUTED as
   the `covcal.for:176` `rescov` area-weighted blend
   (`w = (rspace − width)/rspace` threaded from the projection through
   the authority); the input pass-through is superseded (the partition
   consumer test asserts the computed blend).
4. **Low — stale roadmap/catalog:** ROADMAP's E.5 forward-item text now
   records the executed closure; the WP is listed in
   `docs/work-packages/README.md` Current Active/Held.
5. **Low — line-count WARN:** disposition recorded in `package.md`
   (the seed-authority fan-out file; split rides the next structural
   runner refactor).
