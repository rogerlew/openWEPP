# STAGE2-BASE-CONDUCTIVITY + BASECOND01 — Independent Review (Claude Code)

Verdict: **Sound, well-disciplined, and honest — the discipline visibly improved after the
ksatadj correction.** base-cond found a real, source-verified defect (vertical `ssc` should be
harmonic, openWEPP used arithmetic), BASECOND01 fixed it correctly, and — crucially — the
disposition **disclosed up front** that the fix is H2637 aggregate-inert and therefore does **not**
close FARPOINT01. No overclaim this time. The 71% localization is now correct: the **lateral**
conductivity lineage is verified end-to-end, so the magnitude is correct-by-construction and the
remaining question is a `CONTRACT-GAP` (item 7).

Evidence mode: **Static + Ran** (verified the legacy intent in source; re-ran the gates).

## The anti-detour gate worked

base-cond's sensitivity probe (`ksat×0.9` moved WAT/PASS checksums, aggregate `latqcc`, `runvol`)
**proved the base conductivity is byte-live** before adjudicating — exactly the check the ksatadj
detour skipped. Good.

## The defect is real and I verified the source intent

base-cond's claim — vertical `ssc` should be **inverse-conductivity (harmonic)**, lateral `ui_ssh`
**arithmetic**, and the two **distinct** — is confirmed in `wepp-forest_260430_baseline/src/input.for`:

- `input.for:760` `ksinv(i) += ddg(j)/ssc2(j)` and `:926` `ssc1 = slayth/ksinv` → **harmonic
  vertical** `ssc1`.
- `input.for:761` `ui_ksari(i) += ddg(j)·ssc2(j)·ui_anisrt(j)` → **arithmetic lateral** `ui_ssh`.

openWEPP aliased both to the arithmetic value (H2637 layer 3 = 270.8 mm/h where vertical should be
**117.96**). That's a genuine soil-physics defect (series vertical flow → harmonic mean), correctly
localized. The discipline that mattered: base-cond explicitly refused to "fix" the **lateral**
surface (which is correct) to chase the aggregate flag, and set the guardrail "don't collapse both
surfaces to one harmonic value." ✓

## The fix (BASECOND01) is correct, contract-grounded, gate-clean

- Production fix in `02_soil_slope.rs` (harmonic vertical, arithmetic lateral, distinct);
  `SC-INFILE-SOIL-001` v0.1.11 amended to govern it; tests assert harmonic vertical (117.96),
  arithmetic lateral (270.8), `wb18_perc_ssc ≠ wb19_lateral_ssh`, an anisotropy≠1 case, and
  unchanged-non-split-layer regressions.
- My independent gates: `cargo check --workspace` clean; the harmonic + non-alias + `soil_runtime`
  (11) tests pass.
- **Honest disposition:** "Aggregate WAT/PASS unchanged… `runvol_pct_precip` remains 71.0036550031206…
  therefore BASECOND01 does **not** close the remaining FARPOINT01 H2637 magnitude flag." This is the
  REFINTENT001 lesson applied — the aggregate-inert result is checked and disclosed, not papered over.
  ✓ (H2637's percolation is evidently limited by a deeper layer, so the split-layer vertical
  correction is inert here while still correct for fixtures it does limit.)

## Where the 71% now stands (the real convergence)

Two conductivity defects are now ruled out as the H2637 driver: `ksatadj` (off; REFINTENT001) and
vertical `ssc` (fixed but inert; BASECOND01). What remains is the **lateral** conductivity lineage —
and base-cond verified it is **correct** end-to-end (raw `ksat` parse correct; `ui_ssh` arithmetic
matches source intent; consumption per HPHYS0257 correct; equation/operands closed by STAGE2-LATQCC).
So the H2637 71% is **correct-by-construction** by a fully-verified lateral lineage. The only open
question is the **absolute magnitude** — a `CONTRACT-GAP` (unvalidatable from existing authority),
the same place STAGE2-LATQCC landed but now with the entire lineage verified, not just bound-checked.

## Disposition

base-cond + BASECOND01 are sound and complete; the vertical `ssc` fix is a correct, valuable
general defect closure. Item 7 (post-BASECOND01 disposition) should formalize the 71% as
`CONTRACT-GAP` / correct-by-construction and resolve the FARPOINT01 flag accordingly — **not** a
defect, with the absolute magnitude a documented external-authority gap. Do not re-edit vertical
`ssc` or make `ui_ssh` harmonic without new authority (the guardrail holds).
