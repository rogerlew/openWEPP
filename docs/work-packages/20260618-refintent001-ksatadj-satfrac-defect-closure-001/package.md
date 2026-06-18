# REFINTENT001-KSATADJ-SATFRAC - WB14 `ksatadj` Saturation-Fraction Defect Closure

Status: **complete-with-correction 2026-06-18** — the `ksatadj` `sat_frac` source-intent fix
landed and is **correct** (closes the `OPENWEPP-DEFECTIVE` verdict; gate-clean; valuable for
`ksatadj = 1` soils). **BUT the FARPOINT01 flag-closure claim is withdrawn:** independent review
found the fix is **byte-inert on H2637** (`ksatadj = 0` there — branch never fires; WAT SHA
identical pre/post), so it does **not** close the FARPOINT01 71% flag, which **re-opens**. The
H2637 71% is driven by the **base soil conductivity**, not `ksatadj`. See
`artifacts/refintent001_disposition.md` (Post-review correction) and
`artifacts/review-claude-independent.md`. Follow-on: a base-conductivity adjudication.

Package type: **Defect-Closure ExecPlan (ADR-0018) — first openWEPP *physics* change of this arc.**
Codex authors the fix; Claude scaffolds/reviews. The diagnosis is **done** (REFINTENT); this
package **corrects** openWEPP's `ksatadj` saturation fraction to satisfy the now-canonical
`SC-SUBHYD-001#INV-SUBHYD-032` source-intent algorithm. **Bit-identity is NOT the gate** — outputs
change by design; the gates are contract conformance + conservation closure + the contract suites.

## The defect (item-1, already localized)

`crates/.../hydrology_phase_lateral_drainage/02_ksat_adjustment.rs:249` forms
`sat_frac = theta_sum / ul_sum` (storage over upper-limit, summed over layers). The ratified
source-intent authority (`INV-SUBHYD-032` / `REF-SUBHYD-KSATADJ-INTENT`, from
`wepp-forest_260430_baseline/src/infpar.for`) requires:

```
avsat   = (st_1 + st_2) / tillay(2) + avsm15            ! top-two tillage-layer total water + residual
caps    : if avsat > avpor      -> avsat = avpor * 0.98
          if avsat >= avpor*avcpm-> avsat = avpor*avcpm * 0.99
sat_frac = min( avsat / (avpor * avcpm), 1.0 )          ! rock-corrected denominator
keff    = ks * 3.6e6 * sat_frac^(2*lambda + 3)          ! 9002+ Saxton-Rawls (already matches)
```

These coincide only degenerately; since `keff ∝ sat_frac^(2λ+3)`, the mis-formed `sat_frac` is a
credible root cause of the H2637 71% lateral magnitude. The **branch formulas (9001/9002+/9003) and
the unit conversion already match** — only the `sat_frac` *operand lineage* is wrong.

## The correction

Rebuild the WB14 `ksatadj` operand lineage so `sat_frac` is formed per `INV-SUBHYD-032`:

1. **Operand lineage** — project the source-intent operands into the `ksatadj` computation (absent
   today): top-two tillage-layer **porosity `por`**, **rock correction `avcpm`** (coarse-particle
   multiplier), **residual `avsm15`** (water at 15 bar), the **tillage geometry `tillay(2)` /
   `dg` weights**, and **storage `st`**. Confirm their provenance against `SC-INFILE-SOIL-001`
   (porosity / rock-fragment / theta lineage); do not invent values.
2. **Numerator** — `avsat = (st_1 + st_2)/tillay(2) + avsm15` (total tillage water + residual),
   top-two-tillage weighted (`weight_i = dg_i/tillay(2)`).
3. **Caps** — apply `avpor*0.98` then `avpor*avcpm*0.99` exactly as the source intent (order matters).
4. **`sat_frac`** — `min(avsat/(avpor*avcpm), 1.0)` (rock-corrected denominator), replacing
   `theta_sum/ul_sum`.
5. Keep the 9001/9002+/9003 branch formulas + unit conversion (already correct).

## What is NOT the goal (ADR-0017)

**Matching legacy's 55.5% is not the target.** Whatever magnitude the *correct source-intent
`sat_frac`* yields is correct by the now-governing authority, whether or not it moves toward the
legacy number. The legacy binary output stays an A6 flag. Acceptance is `INV-SUBHYD-032`
conformance + closure, **not** a comparator delta.

## Gates (bit-identity does not apply — this is a deliberate physics change)

1. **`INV-SUBHYD-032` conformance:** `sat_frac` formed per the source-intent algorithm; typed
   reject / `HOLD` on missing/non-finite/ambiguous operands (the invariant's guard posture).
2. **Conservation closure preserved (CRITICAL):** the `ksatadj` change shifts the lateral
   conductivity → infiltration/runoff/lateral magnitudes shift, but the **water balance must still
   close** — MOFE / FARPOINT01 conservation + `SC-WATBAL-001` identities hold (no new
   non-conservation). This, not bit-identity, is the load-bearing safety gate.
3. **Subhyd contract suites still pass:** `cas_l4_subhyd_*` (lateral response, withdrawal caps,
   FC/WP consistency) + the other `SC-SUBHYD-001` invariants — the fix must not break them.
4. **Non-aliased tests:** add component tests on inputs where `theta_sum/ul_sum` ≠
   `avsat/(avpor*avcpm)` (rock fragments `avcpm<1`, non-degenerate residual, cap-binding cases),
   proving the corrected formula is exercised and the surrogate is gone.
5. **Determinism** (`docs/numerics/`): no FP-reduction reorder, no per-OFE sequencing change,
   pinned-seed reproducible.
6. **Re-run H2637** (both `wepp_ui` variants + the OFE ladder): document the **new** lateral
   magnitude and the runoff partition, confirm conservation still closes, and **close the
   FARPOINT01 71% flag** (resolved by INV-SUBHYD-032 conformance, not by the new number).
7. **Rust gates:** fmt; clippy `-D warnings`; `test --workspace`; deny; `git diff --check`;
   line-count.

## Scope

In scope: the WB14 `ksatadj` `sat_frac` operand-lineage + formula correction; the operand
projection; non-aliased + guard tests; the H2637 re-run + FARPOINT01 flag resolution.

Out of scope:

- **No `qdry`/`ksflag`** (future ADR-0024 applications).
- No `SC-*` change (INV-SUBHYD-032 is the authority already; if the fix reveals a contract
  ambiguity, **HOLD** and flag — do not silently re-author).
- No conservation/transfer/export rework (MAGPARITY01-settled); no chasing legacy 55.5%.
- Irrigation deferred.

## Acceptance Criteria

- WB14 `ksatadj` `sat_frac` = source-intent `avsat/(avpor*avcpm)` with caps + top-two-tillage
  averaging; `theta_sum/ul_sum` surrogate removed.
- **`INV-SUBHYD-032` satisfied**; guard posture (typed reject/HOLD) intact.
- **Conservation closure holds** on MOFE/FARPOINT01/H2637 (the safety gate).
- Subhyd contract suites + non-aliased tests pass; determinism preserved; Rust gates pass.
- H2637 re-run documented (new lateral magnitude + partition + closure); **FARPOINT01 71% flag
  closed** by conformance.
- Evidence labeled Static / Ran. If the fix cannot close conservation or `INV-SUBHYD-032` without
  a contract ambiguity, **stop at a declared boundary** and record the defect-shaped handoff.

## Deliverables

- `artifacts/refintent001-operand-lineage.md` (the projected `por`/`avcpm`/`avsm15`/`tillay`/`st` lineage + SC-INFILE-SOIL-001 provenance)
- `artifacts/refintent001-satfrac-correction.md` (the formula change, before/after)
- `artifacts/refintent001-inv-subhyd-032-conformance.md`
- `artifacts/refintent001-conservation-closure.md` (MOFE/FARPOINT/H2637 closure after the change)
- `artifacts/refintent001-h2637-remeasure.md` (new lateral magnitude + partition; FARPOINT01 flag resolution)
- `artifacts/refintent001-nonaliased-tests.md`
- `artifacts/refintent001-gate-results.md`
- `artifacts/refintent001-line-count-governance.md`
- `artifacts/refintent001-review-a.md`
- `artifacts/refintent001-review-b.md`
- `artifacts/refintent001-worker-handoff.md`
- `artifacts/refintent001_disposition.md`

## Dependencies

- `docs/work-packages/20260618-refimpl-intent-authority-ksatadj-subhyd-001/artifacts/{ksatadj-intent-extraction,ksatadj-openwepp-vs-intent,sc-subhyd-ksatadj-anchor,review-claude-independent}.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md` (`INV-SUBHYD-032`, `REF-SUBHYD-KSATADJ-INTENT`)
- `docs/decisions/0024-reference-implementation-intent-authority.md`; `0017-...comparator-is-flag-not-target.md`; `0018-defect-closure-execplans-conversion-rule.md`; `docs/defect_closure_execplans.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md` (operand provenance: porosity, rock fragment, theta)
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` (closure safety gate)
- `docs/specifications/external-authority/suites/cas_l4_subhyd_*` (must still pass)
- `wepp-forest_260430_baseline/src/{infpar,input}.for` (the source-intent algorithm)
- `crates/.../hydrology_phase_lateral_drainage/02_ksat_adjustment.rs` (the fix site)
- `docs/numerics/README.md`; `docs/standards/rust-scientific-coding-standard.md`; `AGENTS.md`; `docs/work-packages/AGENTS.md`; `crates/AGENTS.md`

## Subagent Requirement

None required. If the operator authorizes subagents, the operand-provenance mapping and the H2637
closure re-run are parallelizable. Run gates + the closure checks locally; record evidence.

## Autonomy

Execute end-to-end through the operand lineage, the `sat_frac` correction, `INV-SUBHYD-032`
conformance, conservation closure, non-aliased + guard tests, the H2637 re-measure, gates, dual
review, and disposition. **Stop at a declared boundary** if conservation cannot close or the fix
exposes a contract ambiguity (do not silently re-author the contract or chase legacy parity). The
deliverable is a conforming, conservation-closed `ksatadj` `sat_frac` + the resolved FARPOINT01 flag.
