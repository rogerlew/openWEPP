# Review: Claude Code

Status: complete

Evidence mode: static (source/contract read) + recorded-log read

Static:

- Reviewer: Claude Code (`claude-opus-4-8`), invoked by user post-commit.
- Scope: static correctness review of HPHYS0263 commit `72e4bd7` — the
  Penman-Monteith (`evappm.for`) demand-seed migration in
  `crates/openwepp-runner/src/hillslope/mod.rs` (`compute_evappm_wb11_et_demand`),
  the climate/management projection additions (`deglat`, `elevm`, `canhgt`),
  the watershed adapter symbol publication, and SC-EVAP-001 / SC-WATBAL-001
  amendments.
- Baseline cross-check: equations verified directly against
  `/workdir/wepp-forest_260430_baseline/src/evappm.for`.
- Continuity: extends the storage-drain root-cause thread tracked across
  HPHYS0249-0262 (see `review_claude_code.md` for 0250/0252) and the
  lane/mode-mismatch thread from HPHYS0256.
- Gate evidence read from package artifacts; Claude Code ran no `cargo`
  commands.

## Confirmations (no action)

1. The PM/FAO-56 equations are faithful to baseline `evappm.for`:
   `etorc` (263-265), `rbo` (225), pressure base
   `pb = 101.3*(1-0.0065*elevm/293)^5.26` (247), `kcbadj` (289),
   `kcbcon = kcbadj*(1-exp(-0.45*lai))` (298), `etke` (301), `etcsc` (414),
   `rawpaj` (415), `potes` (435), `es` kecon/eaj/kcmax split (448-453), and
   `ep = etorc*etks*kcbcon` (458). Constants match (0.408, 900, 0.34, 0.14,
   4.9e-9, 0.45, 0.0065, 293, 5.26, 101.3).
2. The motivating localization is real: HPHYS0262 found the 39-suite selects
   PMET (`iflget=2`) while openWEPP seeded the Priestley-Taylor branch — a true
   mode defect. Day-1 `Ep` residual collapsed from `+0.235294 mm` to
   `+0.001823 mm`, confirming the mode was the day-1 driver.
3. Contract-first red gate is behavioral (`pmet.etorc_mm` missing
   pre-migration), with an honest truthfulness note on line-number drift.
4. Honest disposition: `HOLD`, defers `evappm.for:391-454`, `0/39`, and does not
   claim dual-agent review (not requested).
5. First cumulative magnitude movement in the run: `Total-Soil` 167->149,
   `latqcc` max-abs 28->14.8, `Dp` fail count and mean improving.

## Findings

1. High (verify before building on it) — `wb11_et_demand` is seeded with `ep_m`,
   an actual, not a demand. `compute_evappm_wb11_et_demand` returns
   `Wb11EtDemandSeed { demand_m: ep_m, .. }` where
   `ep_m = etorc*etks*kcbcon*0.001` already has crop coefficient and water
   stress applied. The Priestley-Taylor branch
   (`compute_priestley_taylor_wb11_et_demand`) seeds a potential (`eo`). Two
   different physical quantities feed the same seam. The WB17 ET kernel has no
   `evappm`/mode branch (verified: no `evappm`/`branch_evappm` reference in
   `hydrology/03_kernel_support_01_kernel_phases.rs`), so it applies its
   PT-style partition (`Esp = exp(-0.5*(cancov+0.1))*demand`,
   `Etp = lai*demand/3`) plus `swu` stress to `ep_m` — double-applying
   crop-coefficient and water-stress reductions. This is the most likely active
   new defect; verify how WB17 consumes the seed in each mode before HPHYS0264
   builds the deferred redistribution on top.

2. High — PM `Es` is computed then discarded; `Ep`/`Es` now use different
   lineages. `es_m` (the PM soil-evaporation actual, kecon/eaj/kcmax) is
   calculated faithfully but stored only as the `pmet.es_m` diagnostic. So in
   PMET mode `Ep` derives from the PM `ep_m` while `Es` still comes from WB17's
   PT partition. The end-to-end ET path is an internally inconsistent PM/PT
   hybrid: either wire both PM `es_m`/`ep_m` as authoritative (and make WB17
   pass-through in PMET mode), or seed the reference `etorc` and let WB17 own
   the partition — but do not feed a stress-reduced actual into a partition
   kernel.

3. Medium (downstream of root cause) — the PM demand is itself
   storage-contaminated. `ep_m`'s stress `etks = wftrp/etksden` reads
   openWEPP's depleted `wb18_perc_theta` (`wftrp`), so the unresolved storage
   drain (open since HPHYS0249) flows into the PM demand. Seasonal `Ep` cannot
   close until storage is fixed. This explains the day-1-win / seasonal-miss
   split: day-1 storage is correct post-seed, so PM demand is right; the rest of
   the season is starved. Not a defect of HPHYS0263, but a third independent
   confirmation that storage is the binding constraint.

4. Low (naming) — `demand_m` / the `wb11_et_demand_m` field holding a
   transpiration actual is a semantic landmine. Whatever the intended wiring,
   the name says "demand" and the value is a stressed actual; document or
   rename.

## Notes for disposition owner (Codex)

- Finding 1 is the priority. The day-1 metric hides it (low early-season LAI
  makes the partition near-identity) and the seasonal metric cannot isolate it
  from the storage drain, so neither suite signal will catch a double-stress
  wiring error. Trace `wb11_et_demand` -> WB17 `Esp/Etp/swu` for a PMET-mode H1
  mid-season day and compare published `Ep` against `pmet.ep_m`.
- Findings 1-2 together mean the PMET ET path is not yet end-to-end
  baseline-faithful even though the demand equations are. State this explicitly
  rather than as "demand seeding migrated."
- Finding 3 is the standing root cause. The PM physics is correct but lands
  downstream of the storage drain; prioritize the storage-availability lineage
  over further ET-method work.
- The physics migration itself is the most faithful in the HPHYS0249-0263 run;
  the concern is seam wiring, not the equations. Disposition `HOLD` at `0/39`
  is consistent with this review.
