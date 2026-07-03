# Claude Independent Review — DFF-WS2 `ksatadj` direct-runtime port

Evidence: **Static** (independent source + contract read; the source-intent
algorithm math-verified against `SC-SUBHYD-001#INV-SUBHYD-032`) + a **Static**
mechanical-diff subagent for the 2255-line split and the guard retargeting.
Gates were **not** re-run in this pass — Codex's run reported fmt/clippy/full
nextest (1258 passed)/deny/diff-check PASS; not re-verified here. Complements
`review-codex.md` (I concur with all three of its dispositions).

Verdict: **the port is correct and genuinely live; merge-ready modulo one
design decision and two test/coverage strengthenings — none are blockers.**

## What holds (independently verified, Static)

- **Evaluator matches the source-intent** (`support_helpers_mod/ksatadj.rs`):
  `sat_frac` uses the rock-corrected `avsat/(avpor*avcpm)` with the exact caps
  (`avsat>avpor → avpor*0.98`; `avsat≥avpor*avcpm → *0.99`), **not** the
  `theta_sum/ul_sum` surrogate — and a non-aliased conformance test
  (`..._uses_source_intent_avsat_not_ul_surrogate`) proves the divergence, as
  `INV-SUBHYD-032` requires. 9001 exponential recovery, 9002+ Saxton-Rawls
  `2λ+3` (`λ=(ln θfc−ln θdr)/(ln1500−ln33)`), and the 9003 `lkeff` floor all
  match; unit conversion goes through the typed `ProcessRateMillimetersPerHour`
  boundary. Missing/non-finite/out-of-range operands are typed hard-fails.
- **High-finding guard removal is contract-justified.** The removed
  `theta ≤ upper_limit` / `fc ≤ upper_limit` rejects are not in the source
  intent — it *caps* `avsat` for saturated storage rather than rejecting it. The
  regression vector (`..._caps_avsat_instead_of_rejecting_ul_excess`, `sat_frac`
  clamps to 0.98) locks this in. I concur with the disposition.
- **The port is a real consumer, not shadow.**
  `DirectProductionInfiltrationAuthority::inputs` (`00d:342-356`) uses the
  ksatadj conductivity when `ksatadj_policy` is `Some` (populated from
  `soil.ksatadj`, `00_builders:1538`), feeding the Green-Ampt solver; the
  ksatadj-off `else` branch is byte-identical to the prior frost→base chain.
- **SC amendment (v34) is sound:** `BR-SUBHYD-KSATADJ-GUARD` HOLD → runtime
  typed hard-fail; `INV-SUBHYD-032` "or governance HOLD" clause dropped;
  `GAP-SUBHYD-002` honestly narrowed (`Keff_ksatadj` aliases fixed, rest pending).
- **Split + guard retargeting are clean** (subagent, exhaustive diff): the
  2255-line move is byte-identical; guard corpora are add-only/union, no marker
  assertion weakened (negatives are stricter).

## Findings

### Medium (design/spec) — frost × ksatadj composition is unspecified and made implicitly
For a `ksatadj = 1` soil the branch uses the ksatadj conductivity and **discards
`frost_infcap_m_s` entirely** (`00d:342-345`). So on a frozen day a burned soil
is **not** frost-limited — frost activation is on (`ksflag` decoupled, FQ-4) but
its infiltration effect is thrown away for exactly the soils WS-2 targets. Codex
frames this as intended ("active ksatadj overrides the frost/base fallback"), but
that override is not reconciled with the campaign's "keep frost on" intent, and
`INV-SUBHYD-032` does not specify the frost×ksatadj composition. Physically,
`ksatadj`-overrides over-predicts infiltration whenever frost is the more
limiting term; `min(ksatadj_keff, frost_infcap)` would be the conservative
composition. This should be an **explicit, contract-recorded decision**, not an
implicit code choice.
*Caveat:* I did not trace the legacy `infpar.for`/daily-loop frost-vs-ksatadj
sequencing, so this is a **plausible** gap flagged for decision, not a confirmed
defect.

### Low/Medium (test strength) — the p313 end-to-end test is a smoke test
`dff_ws2_ksatadj_direct_runtime.rs` asserts the run completes and HBP/loss/wat
outputs exist — it does **not** assert ksatadj was active or changed the result;
it would pass even if ksatadj were silently inert on the production path. The
logic *is* covered at unit/authority level (incl. the frost-override unit test),
so this is not a coverage gap, but the e2e wouldn't catch a regression that
disables ksatadj in production. Recommend it additionally assert a
ksatadj-specific effect (a diagnostic/manifest marker that the ksatadj branch
fired, or output divergence vs a ksatadj-off variant).

### Low (latent guard coverage) — one sibling guard not retargeted
`direct_publication_source_guards.rs:26-32`
(`compatibility_runtime_deletion_removes_obsolete_transition_modes`) still scans
only `00_builders_and_authority.rs`; its forbidden-surface markers now live in
`00c`/`00d` after the split. Currently benign (those markers are absent from
`00c`/`00d`/`00b`), but the guard would no longer catch a reintroduced obsolete
surface placed in the split files. Add `00c`/`00d` to that guard's `sources`
array for parity with the retargeted corpora.

## Suggested disposition
Land the port. Before/with merge: (1) make the frost×ksatadj composition an
explicit decision — either affirm `ksatadj`-overrides-frost with a one-line
`INV-SUBHYD-032` note + rationale, or switch to `min(ksatadj_keff, frost_infcap)`;
(2) strengthen the p313 e2e to assert ksatadj effect; (3) retarget the one
sibling guard's `sources`. Only (1) is a design question; (2)/(3) are mechanical.

## Re-review — all three dispositions verified (Static, 2026-07-03)
Codex accepted and fixed all three (`review-claude-disposition.md`); re-verified:
- **#1 frost×ksatadj:** `00d:352-362` now `min(Keff_ksatadj, frost_limit)` when a
  positive frost cap exists, else `Keff_ksatadj`. Sound (`Keff_ksatadj =
  base·sat_frac^exp < base`, so it binds on non-frozen days; frost caps frozen
  days). Recorded in `SC-SUBHYD-001` v34: `Keff_ksatadj` = pre-frost surface,
  `wb14_effective_conductivity_m_s = min(Keff_ksatadj, frost_infcap_m_s)`, and
  `INV-SUBHYD-032` gains a frost-cap-composition validator. Concur.
- **#2 e2e:** now asserts `evaluation_count > 0` (active) and `== 0` (disabled);
  a silent-disable regression fails. Good proxy.
- **#3 guard:** `direct_publication_source_guards.rs` `sources` now includes
  `00c`/`00d`. Fixed.

**Validation-scope note (not a defect):** Codex disclosed p313 outputs are
byte-identical ksatadj-on vs off — a property of its cold/dry MORAN-WY climate
(no infiltration-excess runoff, so conductivity changes don't move the output),
confirmed *not* a min-masking artifact. So p313 anchors "ksatadj fires + wiring
live + conservation closes," but the **burn OUTPUT effect** (disturbed → more
runoff) is **not demonstrated** by this fixture and is deferred to WS-3's
directional burn matrix — which needs a runoff-generating (warm, intense-storm)
climate, not MORAN WY. Flag for WS-3 fixture selection.

**Re-review verdict: WS-2 execution is correct and merge-ready.**
