# Claude Code Review — FQ-4 ksflag Frost Activation + Closure

Reviewer: Claude Code
Date (UTC): 2026-06-07
Evidence mode: **Static** — read the localization, disposition, validation ledger,
both dual reviews, the compliance checklist, and the `SC-SNOWFREEZE-001` v53, kernel
(`03_kernel_support_00:2890`), and runner (`mod.rs:5479`) diffs; plus a direct read
of the frost-depth driver (`:3304`–`:3335`), the `qsrf`/`quf`/`harmonic` heat-flow
symbols, and `GAP-SNOWFREEZE-002`. The 43-prefix population, p8 paired on/off, and
closure CSVs are Codex's `Ran` evidence, attributed.

Verdict: **Approve.** The activation defect is correctly localized to a
provenance-flag-as-gate, fixed contract-first by extending the *existing* snow-sidecar
posture to frost, validated population-wide, and conservation holds under frost to
machine precision. One substantive carry-forward (the now-active frost *magnitude*
rests on an acknowledged-incomplete depth model) and two minor notes.

---

## F1 — Correct root cause, authority-consistent fix (primary)

The defect was **not** my static lead (temp wiring / freeze-index proxy / frozen-water
derivation) — it was upstream of all of them: `resolve_active_frost_coupling` returned
inactive whenever `frost.options.frost_file_present` was `0`, *before* consulting
`wintRed`. algebraic-radium runfiles carry no `frost.txt`, so the parser supplies valid
missing-file defaults (`wintRed=1`) but sets `frost_file_present=false` — and that
provenance flag was promoted into an activation gate. With the gate off, the
freeze-index depth path never ran, so `frdp=frozwt=0` and the conductivity gate never
bit. That fully explains the FROSTVAL01 on/off identity.

The fix is the right shape and authority-consistent:

- `frost_file_present` is now **validated** (finite, in `[0,1]`, integer) but **no
  longer gates**; activation keys on `wintRed` (`03_kernel_support_00:2890`).
- The runner manifest mirrors it: `frsoil_active = wint_red_enabled` (was
  `frost_file_present && wint_red_enabled`, `mod.rs:5479`) — so provenance and the
  active flag are consistent.
- `SC-SNOWFREEZE-001` v53 extends `INV-SNOWFREEZE-009` so frost-sidecar presence is
  provenance only and must not suppress frozen soil when `wintRed=1` and thermal
  triggers are active, cross-linked to `-012`/`-013`.

This is the **same defect class snow already had** — `INV-SNOWFREEZE-009` long said
"activation depends on runtime state/forcing triggers, not snow-sidecar presence
alone." Frost simply hadn't had the principle applied. Extending it is exactly right,
not a new heuristic.

Validation is convincing: 43/43 prefixes emit WAT with `frsoil.active=true` and
nonzero `frozwt` (max 27.5–31.0 mm across the population); p8 paired on/off now
**diverges** (frost-on `sum(frozwt)=28902` mm-day vs off `0`; `Q` +393 mm,
`latqcc` −62 mm) — the gate bites and is no longer identical; and annual closure over
258 rows (43×yrs 2..7), using `SoilWaterTotal = Total-Soil + frozwt`, holds at max abs
**3.22e-11 mm**. The FROSTVAL01 `frost-break` artifact is explicitly withdrawn and the
ledger rebuilt on the full WAT identity (the FQ-2 fold-in, done). The p8 pre-fix
`Q=320.7` matches the corn-ET DC's post-interception number — the baseline chained
correctly.

## F2 — The now-active frost *magnitude* rests on an acknowledged-incomplete depth model (carry-forward, not a blocker)

FQ-4 correctly closed frost **activation** and **conservation** — its declared scope.
But by activating frost it moved two things from dormant to load-bearing, and neither
the package nor the dual reviews reconciled them against the contract:

1. **Frost depth is a freeze-index proxy.** `frdp_m = max(prior, 0.20 · clamp(−mean_temp/6))`
   (`:3307`–`:3317`) — a daily-mean-temperature index, not the layered heat-flow
   formulation `INV-SNOWFREEZE-006`/`-012` describe (`Qsrf`/`Quf`, harmonic-mean
   conductivity, Eq. [3.8.1]–[3.8.4]). openWEPP *does* carry `qsrf`/`quf` symbols and a
   `harmonic_mean` path, but the depth that drives the conductivity gate comes from the
   index proxy. **In fairness this is contract-acknowledged:** `GAP-SNOWFREEZE-002`
   already flags "frost hourly/process implementation and contract-derived frost
   execution parity remain open." So this is not a hidden hard-fail violation — it is a
   known-open area that FQ-4 has now made materially active.
2. **The conductivity bite is very strong — but it is legacy-faithful (provenance
   correction).** `infcap_frz = 9.17e-11` vs `ssc = 9.17e-06` on p8 — a ~10⁵× reduction
   (`kfactor ≈ 1e-5`), near-impermeable frozen soil, roughly doubling p8 runoff
   (320.7 → 714.0). **My initial "suspect magnitude" framing of this was wrong.** A
   provenance check (`frost.rs:18-20` vs `frost.spec.md` scenario A and legacy
   `infile.for`/`getfreezecond.for`) shows openWEPP's defaults (`kfactor1=kfactor2=1e-5`,
   `kfactor3=0.5`) **are the documented WEPP defaults**, and an annual crop (Corn)
   correctly selects `kfactor(1)` = "concrete frost" = intentionally near-impermeable.
   So the strong bite (and the Q doubling) is the **intended** concrete-frost behavior,
   not an openWEPP magnitude error. (Legacy `getfreezecond.for` carries a known
   comment-vs-code kfactor-index inconsistency, spec ref E-WF-05, but the annual path
   lands on the small kfactor either way; openWEPP's `min()` fallback also lands at 1e-5.)

The **real** openWEPP-specific divergence is the frost **depth model**, not the
conductivity magnitude:

- **Legacy** `frostn.for` computes frost depth from a layered **energy-balance
  heat-flow** model (Dun-2008 fine 1 cm/2 cm sublayers, energy flow between frozen
  layers, frost depth up to **1.0 m**).
- **openWEPP** uses a **freeze-index proxy** — `frdp = 0.20 · clamp(−mean_temp/6)`,
  hard-capped at **0.20 m** — a different mechanism, ~5× shallower max. The `qsrf`/`quf`
  heat-flux symbols exist but are derived *downstream of* the proxy depth, not driving
  it. `GAP-SNOWFREEZE-002` already acknowledges the frost process/parity is open, and
  the proxy diverges from `INV-SNOWFREEZE-006`/`-012`.

So "frost activates and conserves" is **necessary but not sufficient** — the same
conservation-vs-magnitude split that has run through this entire arc (snow, corn ET,
runoff) now applies to frost, but the locus is the **depth model**, not the kfactor.
Because the kfactor bite is near-total whenever any frost exists, the crude proxy (when
frost forms, how deep, the 0.20 m cap, the 6 °C scale) now controls frost timing/extent.
Recommendation: track the **frost depth model** (freeze-index proxy vs the legacy
heat-flow chain, `GAP-SNOWFREEZE-002`) as the frost analog of the deferred
snow-magnitude boundary, gated by a comparator sanity check (does `wepp_260606_hill`
produce comparable frost depth/duration on this cohort?). This belongs in the handoff
as an explicit follow-on — it is the more consequential open frost question now that
activation is fixed.

## F3 — Population went 42→43 emitters: confirm p11 is resolved, not masked (minor)

The repaired substrate was 42/43 runnable (p11 held at `FQ1-P11` percolation
`HKERNEL-WB11-PERC-E-003` at J162). Post-FQ-4 the ledger reports **43/43** WAT
emitters. The likely cause is benign-but-worth-stating: frozen-soil infiltration
reduction lowers the water reaching the deep-seepage path, so the J162 percolation
failure no longer triggers. That means p11 may now emit because the percolation defect
is **masked by frost**, not fixed. Recommend a one-line note so `FQ1-P11` is not
considered closed on the strength of this — it could re-surface in warm/non-frozen
conditions or on a substrate where frost does not bite.

## F4 — Comparator-flag ownership step not reported (minor)

The scaffold's M1 step 4 asked for a `wepp_260606_hill` run to confirm legacy produces
`frozwt`>0 (the ownership flag). The localization established ownership instead from the
self-evident gate defect + cold temperatures + the kernel's demonstrated capability —
which is **acceptable** here (the provenance-flag-as-gate is unambiguous, and ADR-0017
makes the comparator optional). But running it would have cost little and would double
as the F2 magnitude sanity check. Noting for completeness, not as a gap.

---

## Recommendation

Approve and commit. The rung-2 frost **activation** target is met: a provenance flag
wrongly gating frost is removed contract-first, frost engages across the population, the
FROSTVAL01 artifact is withdrawn, and conservation holds under frost at machine
precision. One carry-forward to record in the handoff before moving on: **frost
magnitude** (the freeze-index depth proxy + `kfactor` conductivity bite, `GAP-SNOWFREEZE-002`)
is the frost analog of the deferred snow-magnitude question — track it with a comparator
sanity check rather than treating activation+conservation as the whole story. Plus the
two minor notes (verify p11 is resolved not frost-masked; the comparator ownership step
was inferred). With activation closed, the rung-2 ladder is: frost-magnitude as a
Stage-2-like deferral → **MOFE (rung-3)** → snow magnitude (Stage-2).
