# REFINTENT001-KSATADJ-SATFRAC — Independent Review (Claude Code)

Verdict: **The `ksatadj` `sat_frac` fix is correct and worth keeping — but the package's headline
claim that it "closes the FARPOINT01 71% flag" is UNSUPPORTED.** The fix is **byte-inert on
H2637 because `ksatadj = 0` there** (verified two ways). It changes nothing about the 71% lateral
magnitude, which is driven by the **base soil conductivity**, not `ksatadj`. FARPOINT01 must stay
**open**, and the disposition + ROADMAP must be corrected.

Evidence mode: **Static + Ran** (read the diff, the soil file, the legacy read statement; compared
the pre/post WAT SHA).

## The fix itself is good (keep it)

Genuine, correct defect closure: `theta_sum/ul_sum` is removed; `sat_frac = avsat/(avpor·avcpm)`
with rock-corrected denominator, `avsm15` residual, both caps, depth-weighted `por`/`cpm` operands
(projected from the soil lineage, typed-failure on missing), and a non-aliased test (`0.41/0.55`
vs the old `0.06/0.40`). The 9001/9002+/9003 branches + unit conversion are preserved. For a
**`ksatadj = 1`** forest soil (disturbed/burned), this is a real, valuable correction. Land it.

## But it does not touch H2637 — the flag-closure claim fails

1. **Byte-identical outputs.** Post-fix H2637 `H2637.wat.parquet` SHA-256 is
   `c70af52324b52c89119e57524f75bf4875d2c6a9ff83fe56d239a22082b9b474` — **identical** to the
   pre-fix STAGE2-LATQCC run (same SHA, reported in `latqcc_disposition.md` and
   `latqcc-legacy-flag.md`). HBP identical too. The fix changed nothing across 235,961 rows.
2. **`ksatadj = 0` for H2637.** Every OFE block in `p2637.sol` begins `0  'forest'  'sand loam'
   1.5  0.3`, and legacy `input.for:467` reads that leading token as `ksatadj(iplane)`
   (`read (11,*) ksatadj(iplane), luse, stext, ksatfac(iplane), ksatrec(iplane)`). So
   **`ksatadj = 0`** — openWEPP's `if ksatadj == 1` branch (which contains the `sat_frac` fix) is
   **skipped** for H2637. The soil has rock fragments (`cpm ≠ 1`), so a firing branch *would* have
   changed `sat_frac`; it didn't, because it never ran.

The disposition reports "`runvol` remained 71.003655%" as if it were a magnitude that happens to
be stable, **without disclosing that the entire output is byte-identical** — i.e. that the fix had
zero effect on the flagged fixture. A defect-closure cannot "close" the flag it was scoped to
resolve while changing nothing about the flagged quantity.

## What this re-frames

The whole `latqcc` → `ksatadj` localization mis-attributed the H2637 magnitude. STAGE2-LATQCC
already saw the truth and we read past it: "the lateral conductivity is the soil conductivity
exposed to WB19, **not** a larger traced `ssh` override." That is exactly the `ksatadj = 0`
signature — H2637's lateral conductivity is the **base** soil conductivity (`ssc`/`Ke` straight
from the soil file), and `ksatadj` was never in play. So:

- The `ksatadj` algorithm defect REFINTENT found is **real but irrelevant to H2637** (and to the
  FARPOINT01 flag). It matters for `ksatadj = 1` disturbed/burned forest soils.
- The H2637 **71% is driven by the base soil conductivity**, which is **still un-adjudicated**.
  ADR-0024 / `INV-SUBHYD-032` did not, in fact, govern the H2637 magnitude.

## Recommendation

1. **Land the `ksatadj` fix** — it is a correct defect closure (valuable for `ksatadj = 1`),
   gate-clean. Do **not** discard it.
2. **Correct the disposition + ROADMAP:** the fix is **byte-inert on H2637 (`ksatadj = 0`)**; it
   does **not** close FARPOINT01. Re-open the flag.
3. **Adjudicate the actual driver** — the H2637 **base lateral/percolation soil conductivity**
   (the `Ke`/`ssc` lineage, the soil-file `ksat` + the WEPP 200 mm runtime-layer normalization),
   under `SC-SUBHYD-001` / `SC-INFILE-SOIL-001`, with the same intent-vs-behavior discipline. That
   is where the 71% actually lives.

This is a good fix attached to a wrong conclusion. Keep the former; correct the latter.
