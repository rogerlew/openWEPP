# REFIMPL-INTENT-AUTHORITY + ksatadj/SC-SUBHYD-001 — Independent Review (Claude Code)

Verdict: **Sound `OPENWEPP-DEFECTIVE` — I independently verified both sides of the divergence
against source.** The intent-authority methodology worked: it turned the STAGE2-LATQCC
`CONTRACT-GAP` into a specific, fixable defect that is a plausible root cause of the H2637 71%
lateral magnitude. ADR-0024 is faithful and airtight; `INV-SUBHYD-032` is well-authored;
discipline (intent-not-behavior, no fix) held. **Two gates remain: operator ratification of
ADR-0024, and the defect-closure follow-on.**

Evidence mode: **Static** (independently read the legacy Fortran *and* the openWEPP Rust, plus
the ADR + contract amendment).

## The defect is real — I checked both sides, not just the artifact

- **Legacy intent (verified in `wepp-forest_260430_baseline/src/infpar.for`):**
  `avsat = (st(1)+st(2))/tillay(2) + avsm15`; caps `avsat ≤ avpor·0.98` then
  `avsat ≥ avpor·avcpm → avpor·avcpm·0.99`; **`sat_frac = avsat/(avpor·avcpm)`**; feeding
  `keff = ks·3.6e6·sat_frac^(2λ+3)` (Saxton-Rawls, `solwpv≥9002`). The "CAS / A. Srivastava"
  comments confirm it's the forest empirical adaptation — exactly the implementation-is-spec case
  ADR-0024 governs. The extraction matches the source.
- **openWEPP (verified in `…/hydrology_phase_lateral_drainage/02_ksat_adjustment.rs:249`):**
  `sat_frac = theta_sum / ul_sum`, accumulating `theta_sum += theta.max(0)` and `ul_sum += ul`.
  No `por`/`cpm` rock correction, no `avsm15` residual, no caps, and a sum-over-layers instead of
  the top-two-tillage weighted average.
- **They diverge structurally,** not degenerately: different denominator (`Σul` vs rock-corrected
  `avpor·avcpm`), different numerator (`Σθ` vs total water + residual), missing caps, different
  layer treatment. The "only coincide in restricted degenerate cases" claim is correct. Since
  `keff ∝ sat_frac^(2λ+3)`, a mis-formed `sat_frac` propagates directly into the equivalent
  conductivity and the lateral-flow magnitude — a credible mechanism for the 71%. ✓

## Governance is well-built

- **ADR-0024** faithfully encodes the operator directive: source *intent* (the cited static
  algorithm, with provenance) as an **A0 anchor** for empirical models lacking external authority;
  explicitly **"not the legacy binary output / replay / comparator delta"**; **"ADR-0017 is
  preserved… legacy binary behavior remains A6 investigation evidence only"**; "a provenance basis
  for existing A0 authority, **not a new rank**"; and the honest fallback "encode the intended
  algorithm when intent is clear, or **hold** if it is not." The A0/A6 line is airtight. ✓
- **`SC-SUBHYD-001` amendment** (version 33): `REF-SUBHYD-KSATADJ-INTENT` anchor with
  `file:line:commit` provenance; the algorithm encoded faithfully; `INV-SUBHYD-032` (hard-fail)
  requiring the source-intent `sat_frac` + branch formulas + unit conversion + typed rejection,
  with "legacy binary output magnitude, disabled branches, non-conservation artifacts are
  non-authoritative." Authored per the contract procedure. ✓
- **No fix applied** (scope-correct); routed to defect `REFINTENT001-KSATADJ-SATFRAC`. ✓

## The two gates before this is done

1. **Operator ratification of ADR-0024.** The ADR is marked `Status: Accepted` with Roger Lew as
   ratifier — but the actual text is being seen now. The principle was directed by the operator
   ("the code-implementation intent should be the authority… general for qdry/ksflag"); the text
   faithfully captures it. This is a **confirm**, not a re-decision — but the canonical contract
   change should not be promoted until it is explicit.
2. **The defect-closure follow-on** (`REFINTENT001-KSATADJ-SATFRAC`): rebuild the WB14 `ksatadj`
   operand lineage so it computes the source-intent `sat_frac` (rock-corrected `avpor·avcpm`
   denominator, total-water + residual numerator, the two caps, top-two-tillage averaging), with
   **non-aliased tests** where the surrogate and intended formula differ. The FARPOINT01 71% flag
   stays open until that lands and is re-run.

## One honesty caveat to preserve

Fixing `sat_frac` will change the H2637 lateral magnitude, but **matching legacy's 55.5% is not
the goal** (ADR-0017). The goal is the *correct algorithm*; whatever magnitude the source-intent
`sat_frac` yields is correct by the now-governing authority, whether or not it moves toward the
legacy number.

## Disposition

The adjudication + contract authoring are **sound and complete**; land them on operator
ratification of ADR-0024. Then scaffold `REFINTENT001-KSATADJ-SATFRAC` (the defect-closure fix).
This is a textbook win for the contract-first / intent-authority approach.
