# Claude Code Review — FQ3-DC ET Corn-Engagement Closure

Reviewer: Claude Code
Date (UTC): 2026-06-07
Evidence mode: **Static** — read the localization, disposition, validation ledger,
both dual reviews, and the runner / kernel / constants diffs plus the
`SC-PLANT-001` v18, `SC-EVAP-001` v26, `SC-WATBAL-001` v147, and `SC-RUNOFFPART-001`
v40 amendments. The p8/p1 + 36-prefix Corn reruns and the annual-closure CSVs are
Codex's `Ran` evidence, attributed.

Verdict: **Approve.** The annual-crop ET partition now engages and is physically
*right*, not merely nonzero; the fix is contract-first with pinned-baseline
provenance; conservation holds to machine precision; and the disposition is honest
about the `Er` objective-overclaim. Two forward-looking watch-items (a small
perennial-path delta and the runoff merge-seam interaction) — neither blocks commit.

---

## F1 — The fix is correct *and* corrects the partition, not just the zero (primary)

This is the headline. The defect was not "Corn `Ep`=0" in isolation — it was the
whole annual canopy→transpiration partition being dumped to soil evaporation. The
fix repairs the partition:

| p8 (Corn) | legacy | pre-fix openWEPP | post-fix openWEPP |
|---|---:|---:|---:|
| `Ep` | 1831 | 0 | **1938** |
| `Es` | 2764 | 4886 | **2432** |
| `Interception` | (WAT-unavailable) | 0 | **615** |

`Ep` engages (1938 ≈ legacy 1831 — above, which is fine; comparator is a flag) **and**
the over-inflated soil evaporation collapses from 4886 → 2432 (≈ legacy 2764). That
second number is the real proof: water that was wrongly leaving as `Es` is now
transpiring. 36/36 Corn prefixes have nonzero `Ep` and `Interception`; zero-`Ep`
and zero-`Interception` sets are both empty.

The two root causes match the localization exactly and are both real openWEPP defects:

1. **Annual PL activation sentinel was permanently lost.**
   `prepare_pl_runtime_activation_for_scheduler` deleted `pl_schedule_slot_count`
   on a pre-plant day, turning a *day-local* skip into permanent suppression — so
   annual Corn never activated post-`jdplt`. Fix: capture the sentinel before the
   skip and `or_insert` it back into the writeback surface for the next day. Perennial
   (zero-date active slot) was correctly unaffected by the sentinel path.
2. **Scheduler `day` was day-of-month, not Julian.** Annual activation compares
   against `jdplt` (Julian), so the predicate saw `1..31` and stayed pre-plant.
   Fix: `seed_scheduler_calendar_symbols` publishes Julian `day` + simulation `year`.

Both are authority-backed: new `INV-PLANT-026` (annual PL activation persistence,
hard-fail) and a tightened `INV-EVAP-016`, **bidirectionally cross-linked**
(`SC-PLANT-001#INV-PLANT-026` ↔ `SC-EVAP-001#INV-EVAP-016`). Conformance vectors
added for the inactive→active annual lifecycle.

## F2 — WB15 interception cap is legacy-faithful; the RUNOFFPART touch is legitimate

The old guard rejected `vdmt > 0.8 kg m^-2` as an invalid *state* (fail-closed) —
which is exactly what blocked mature Corn (biomass > 8000 kg/ha). The fix matches
pinned-baseline `idat.for:286-291`: the **state** is finite-non-negative valid, and
only the **equation input** is capped (`VE = min(vdmt*10000, 8000)`) before the
Chapter-5 quadratic. This is sound physics: the interception parabola
`0.000627·VE − 3.73349e-8·VE²` peaks near VE≈8397, so capping the input at 8000 keeps
it on the rising limb and prevents the equation from *decreasing* (or going negative)
at high biomass — which is precisely why legacy caps the input. Review B's catch of
the negative-interception risk and its red test
(`fq3dc_wb15_accepts_finite_non_negative_corn_vdmt_above_legacy_cap` with a `vdmt`
that would make the uncapped quadratic negative) is a genuine safety find.

The `SC-RUNOFFPART-001` v40 touch is **correctly classified** (Review B finding 1):
the WB14 interception consumer lives in the runoff-partition contract
(interception-before-infiltration coupling), so mirroring the WB15 domain change
there is consumer-contract mirroring, **not** runoff tuning — no partition equation
or `Q` acceptance changed. Same change reflected consistently in `SC-WATBAL-001`
v147 with matching `REF-...-IDAT-INTERCEPT` provenance.

## F3 — `Er` overclaim honestly retracted (disposition quality)

The package objective (which I authored) named `Er` alongside `Ep`/`Interception`.
Codex correctly disposed this as an **objective overclaim**: upstream FQ-3 showed
legacy `Er`=0 *and* openWEPP `Er`=0 on p8/p1 — `expected-config-zero`, not an
unresolved defect. The closure note, disposition, and ledger all record this rather
than quietly dropping it. This is exactly the truthfulness discipline the program
runs on — my framing bundled `Er` as a symptom; the evidence said otherwise, and the
record now says so plainly.

## F4 — Perennial p1 `Ep` moved 5511 → 5475.2 (watch-item, not a blocker)

The package boundary was "Tah p1 `Ep`≈5511 **must not move**." It moved −36 mm/7yr
(−0.65%). The disposition and both dual reviews assert "perennial non-regressed"
**qualitatively** (nonzero `Ep` + interception) without quantifying this delta.

- Interception is unchanged (643 → 643.36), so the shift is in the
  transpiration/growth path, **not** canopy — almost certainly the now-Julian `day`
  symbol reaching the perennial growth path too (it is seeded for all managements,
  not just annuals).
- This is plausibly a **latent correction** (perennial growth may have been reading
  day-of-month where it should read Julian), in which case 5475 is *more* correct
  than 5511 — but it could equally be an incidental perturbation.

Low risk either way (0.65%, conservation closes at 1e-11). Recommendation: record
the 5511 → 5475.2 delta in the disposition, attribute it to the corrected Julian-day
symbol reaching perennial growth, and classify it (latent correction vs accepted
tolerance) rather than asserting "must not move → satisfied" when it numerically
moved. A one-line honest delta beats an unqualified "non-regressed."

## F5 — Merge seam confirmed real: Corn `Q` dropped 513 → 320.7 (carry-forward)

The seam I flagged when scaffolding (this DC + the landed runoff DC both touch the
WB14 interception consumer) is **real and material**, and the package handled the
*conservation* half correctly — annual closure re-verified across 216 rows
(36×yrs 2..7) at max abs residual **3.16e-11 mm**. But note the *magnitude* coupling:

- p8 Corn `Q`: 513 (runoff DC alone) → **320.7** (with Corn interception). The 615 mm
  interception removes water before infiltration/runoff, so Corn runoff fell — moving
  *away* from legacy (760), to ~42% (was ~67%).
- p1 Tah `Q`: 138 → 138.18 (stable — perennial already intercepted).

This is **not** a defect — mechanism + conservation hold, comparator is a flag
(ADR-0017), and a with-canopy crop *should* intercept and run off less. But it means
the runoff-magnitude follow-on (the `FQ3-DC-RUNOFFPART` F2 watch-item) must
re-baseline on the **with-interception** budget; the runoff DC's "p8 0→513" number is
now superseded by this interaction. Worth an explicit note so the next runoff
characterization doesn't chase a 513→760 gap that is really 320→760 post-canopy.

## F6 — Test churn is legitimate (nit-clear)

`hphys0319`/`hphys0320` integration tests changed only their asserted
`SC-WATBAL-001` version string (146→147) — a necessary consequence of the contract
bump, not scope creep. `SC-EVAP-001` header went 25→26 consistent with its new
changelog row (the older changelog rows' non-monotone version labels are pre-existing
file history, not introduced here).

---

## Recommendation

Approve and commit. The fix is the substantive rung-2 ET work: the annual-crop
canopy→transpiration partition now engages contract-first, `Es` is corrected to
≈legacy, conservation is preserved to machine precision, and the `Er` overclaim is
honestly retracted. Two carry-forwards, neither blocking: (F4) record the perennial
`Ep` 5511→5475.2 delta + its Julian-day cause and classify it; (F5) re-baseline the
runoff-magnitude follow-on on the post-interception Corn `Q` (320.7, not 513). With
this, **both** fundamental rung-2 partition defects — crop ET and runoff — are closed
on a conserving single-OFE foundation. FQ-2 (ledger), the p11 percolation follow-on,
and FQ-4 (frost) remain, with frost last on the now-repaired substrate.
