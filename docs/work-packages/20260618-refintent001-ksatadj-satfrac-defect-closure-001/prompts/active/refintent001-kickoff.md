# REFINTENT001-KSATADJ-SATFRAC Kickoff — WB14 `ksatadj` Saturation-Fraction Defect Closure

Execution mode: defect-closure ExecPlan (ADR-0018) — **first openWEPP physics change of this arc.**
Bit-identity is **not** the gate (outputs change by design); the gates are contract conformance +
conservation closure.

Autonomy: execute end-to-end (operand lineage → `sat_frac` correction → INV-SUBHYD-032 conformance
→ conservation closure → non-aliased + guard tests → H2637 re-measure → gates → dual review →
disposition). **Stop at a declared boundary** if conservation can't close or a contract ambiguity
appears — do not silently re-author the contract or chase legacy parity.

## The defect (already localized by REFINTENT)

`…/hydrology_phase_lateral_drainage/02_ksat_adjustment.rs:249` forms `sat_frac = theta_sum/ul_sum`.
The ratified `SC-SUBHYD-001#INV-SUBHYD-032` / `REF-SUBHYD-KSATADJ-INTENT` (from
`wepp-forest_260430_baseline/src/infpar.for`) requires:

```
avsat    = (st_1 + st_2)/tillay(2) + avsm15           ! top-two tillage total water + residual
caps     : avsat > avpor       -> avpor*0.98
           avsat >= avpor*avcpm -> avpor*avcpm*0.99
sat_frac = min(avsat/(avpor*avcpm), 1.0)              ! rock-corrected denominator
keff     = ks*3.6e6 * sat_frac^(2*lambda+3)           ! already matches (9002+)
```

The branch formulas (9001/9002+/9003) + unit conversion **already match** — only the `sat_frac`
operand lineage is wrong (`theta_sum/ul_sum` ≠ `avsat/(avpor*avcpm)` except degenerately).

## The correction

1. **Operand lineage** — project the source-intent operands (absent today): top-two tillage
   `por`, rock correction `avcpm`, residual `avsm15`, `tillay(2)`/`dg` weights, storage `st`.
   Confirm provenance against `SC-INFILE-SOIL-001`; do not invent values.
2. `avsat = (st_1+st_2)/tillay(2) + avsm15`, top-two-tillage weighted.
3. Apply the `avpor*0.98` then `avpor*avcpm*0.99` caps (order matters).
4. `sat_frac = min(avsat/(avpor*avcpm), 1.0)` — replace `theta_sum/ul_sum`.
5. Keep the branch formulas + unit conversion.

## NOT the goal (ADR-0017)

**Matching legacy 55.5% is not the target.** The correct source-intent `sat_frac` magnitude is
correct by the governing authority, whatever it is. Legacy binary output stays an A6 flag.

## Gates (no bit-identity — deliberate physics change)

1. **INV-SUBHYD-032 conformance** (source-intent `sat_frac`; typed reject/HOLD on bad operands).
2. **Conservation closure preserved (CRITICAL safety gate):** the conductivity change shifts
   infiltration/runoff/lateral, but the water balance must still close — MOFE/FARPOINT01 +
   `SC-WATBAL-001` identities hold, no new non-conservation.
3. **Subhyd contract suites pass:** `cas_l4_subhyd_*` + other `SC-SUBHYD-001` invariants.
4. **Non-aliased tests:** component tests where `theta_sum/ul_sum` ≠ `avsat/(avpor*avcpm)`
   (rock fragments, residual, cap-binding) — prove the corrected formula is exercised.
5. **Determinism** (`docs/numerics/`).
6. **Re-run H2637** (both UI + OFE ladder): document the new lateral magnitude/partition, confirm
   closure, **close the FARPOINT01 71% flag** (by conformance, not by the new number).
7. **Rust gates:** fmt; clippy `-D warnings`; `test --workspace`; deny; `git diff --check`; line-count.

## Constraints

- No `qdry`/`ksflag`; no `SC-*` change (HOLD + flag if the fix exposes a contract ambiguity);
  no conservation/transfer/export rework; no legacy-parity chase. Irrigation deferred.
- Truthfulness: conformance, closure, timing are empirical — label `Ran:`. Report the new magnitude
  honestly; do not frame it as "matching legacy."

## Required reading

- `docs/work-packages/20260618-refintent001-ksatadj-satfrac-defect-closure-001/package.md`
- `docs/work-packages/20260618-refimpl-intent-authority-ksatadj-subhyd-001/artifacts/{ksatadj-intent-extraction,ksatadj-openwepp-vs-intent,sc-subhyd-ksatadj-anchor,review-claude-independent}.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md` (INV-SUBHYD-032, REF-SUBHYD-KSATADJ-INTENT)
- `docs/decisions/0024-...intent-authority.md`, `0017-...comparator-is-flag-not-target.md`, `0018-defect-closure-execplans-conversion-rule.md`, `docs/defect_closure_execplans.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`, `SC-WATBAL-001.md`
- `wepp-forest_260430_baseline/src/{infpar,input}.for`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage/02_ksat_adjustment.rs`
- `docs/numerics/README.md`, `docs/standards/rust-scientific-coding-standard.md`, `AGENTS.md`, `crates/AGENTS.md`
