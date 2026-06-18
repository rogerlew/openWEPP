# STAGE2-BASE-CONDUCTIVITY-H2637-MAGNITUDE Kickoff — Base Conductivity Adjudication

Execution mode: Stage-2 physics-magnitude adjudication — produce a VERDICT, not a fix.

Autonomy: execute end-to-end (sensitivity probe → lineage extraction → source-intent + contract
check → plausibility → verdict → handoff). **Do not fix; do not chase legacy parity.**

## Why you're here — the corrected localization

REFINTENT001 fixed openWEPP's `ksatadj` `sat_frac` but it was **byte-inert on H2637** (`ksatadj = 0`
there; post-fix WAT byte-identical to pre-fix). So the H2637 71% lateral magnitude is **not**
`ksatadj`-driven. STAGE2-LATQCC already saw it: "the lateral conductivity is the soil conductivity
exposed to WB19, not a `ksatadj` override." The 71% rides on the **base soil conductivity**
(`Ke`/`ssc`) — adjudicate that.

## THE ANTI-DETOUR GATE (do this first)

**Before adjudicating, prove the base conductivity actually drives the H2637 magnitude:** perturb
the base lateral conductivity and confirm `runvol`/`latqcc` **changes** (byte-sensitivity). The
`ksatadj` detour happened because nobody confirmed the model was active for the fixture. **If the
base conductivity is also byte-inert, STOP and re-localize** — do not adjudicate an inert quantity.

## The question

Is openWEPP's H2637 base lateral/percolation conductivity (`Ke`/`ssc`) — soil-file `ksat` → the WEPP
200 mm runtime-layer normalization → layer averaging → `wb18_perc_ssc` → `wb19 Ke` — correct under
`SC-INFILE-SOIL-001` / `SC-PERC-001` / `SC-SUBHYD-001` and the source-intent soil lineage, or is a
step inflated?

## Method

1. **Sensitivity probe** (gate above).
2. **Lineage extraction:** soil `ksat` (H2637 ≈ `60, 330, 33, 33 mm/h`) → 200 mm normalization
   (`input.for:748-928`) → `ssc` → `Ke` (~`9.2e-5 m/s` peak per STAGE2-LATQCC). Values + transforms.
3. **Source-intent + contract check:** normalization + averaging match legacy `input.for`/`infpar.for`
   (`avks` harmonic mean) + `SC-INFILE-SOIL-001`? Is `ssh == ssc` (lateral = vertical, anisotropy 1.0)
   the intended wiring, or should a lateral/anisotropy factor apply?
4. **Plausibility:** is the `Ke` defensible for H2637 forest soil (WEPP Ch.5/6 + forest authority)?
5. **Verdict** (ADR-0017 taxonomy), citing the transform/invariant/authority, legacy a flag.

## Verdict + handoff

- `CORRECT` → base conductivity follows correctly → **resolve the FARPOINT01 71% flag**.
- `OPENWEPP-DEFECTIVE` → a normalization/averaging/wiring step is wrong → Defect-Closure ExecPlan
  item-1 (ADR-0018). No fix here.
- `CONTRACT-GAP`/`UNRESOLVED` → name the authority needed; if it is a provisional model lacking
  external authority, the ADR-0024 source-intent-as-authority route applies (behavior stays a flag).

## Constraints / truthfulness

- No `ksatadj` (off for H2637); no fix; no `SC-*` change (gap flagged); no legacy 55.5% target; no
  conservation/transfer rework. Irrigation deferred.
- Label evidence Static / Ran. A verdict cites the transform/invariant/authority, not the legacy
  delta. **Confirm the driver is live (byte-sensitive) before attributing the magnitude to it.**

## Required reading

- `docs/work-packages/20260618-stage2-base-conductivity-h2637-magnitude-001/package.md`
- `docs/work-packages/20260618-refintent001-ksatadj-satfrac-defect-closure-001/artifacts/{review-claude-independent,refintent001_disposition}.md`
- `docs/work-packages/20260618-stage2-latqcc-h2637-magnitude-001/artifacts/{latqcc-equation-correctness,latqcc-operand-plausibility}.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`, `SC-PERC-001.md`, `SC-SUBHYD-001.md`
- `docs/decisions/0024-...intent-authority.md`, `0017-...comparator-is-flag-not-target.md`, `0018-defect-closure-execplans-conversion-rule.md`
- `wepp-forest_260430_baseline/src/{input,infpar}.for` (`input.for:748-928`; `infpar.for` `avks`)
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
- `AGENTS.md`, `docs/work-packages/AGENTS.md`, `docs/numerics/README.md`
