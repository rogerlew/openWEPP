# MOFE-MAGPARITY01 — Independent Review (Claude Code)

Verdict: **Sound, complete, and a model of the closure-not-magnitude adjudication.** The
"no defect → Stage-2 lateral/subsurface" verdict is well-evidenced, rests on cited
invariants/bounds and real external authority (not the legacy delta), holds the ADR-0017
discipline, and — importantly — *localizes* the residual magnitude to a specific owner rather
than vaguely deferring. No fix, no contract change, exactly the adjudication shape scoped.

Evidence mode: **Static** (all six artifacts + the per-OFE decomposition arithmetic).

## Why I trust the "no defect" verdict

The danger with a "no defect, defer to Stage-2" outcome is that it's the *comfortable* answer
— so I checked it doesn't paper over a real defect. It doesn't:

- **The decomposition is exact.** `runvol` (14,085,670 m³) reconstructs as 97,987 m³ local
  surface + 13,987,683 m³ routed `latqcc` (OFE1-18) with residual **<1e-6 m³**. The 71% is
  *not* surface over-generation — it is ~99% routed lateral flow (local surface is 0.7%).
- **The partition machinery is ruled out with hard identity evidence, not assertion:**
  adjacent `UpStrmQ` carry 2.27e-13 mm, `SubRIn` exact, area ratio exactly 1.0 (equal-area
  OFEs, so area inflation is structurally impossible here), export duality 5.46e-12 m³, and the
  manifest closure gates (transfer 0.0, per-element 7.96e-13, hillslope-total 1.62e-13). So
  `INV-RUNOFFPART-028`, the QOFE/Q area-duality hazard, conservation closure, and export
  scaling are each **PASS** — none is the source.
- **The reframe is the real insight, and it's correct.** The *combined* `runvol+sbrunv` is
  **75.5% (openWEPP) vs 72.2% (legacy without_ui)** — only ~3.3% apart. The visible 71% vs
  55.5% gap is a **surface-vs-lateral partition** difference (openWEPP routes lateral into
  outlet surface runoff; legacy egresses more as terminal `sbrunv`), not gross over-generation.
  That correctly relocates the question to lateral generation + re-infiltration *physics* (WB19),
  which is a magnitude question over an already-closed structure — textbook closure-not-magnitude.

## The external-authority check is genuine

Not "bounded so fine." It cites WEPP Ch.4/5 (rainfall-excess + subsurface lateral as a separate
process family) and the repo's annotated forest-hydrology authority — **Dun et al. (2009)**
(WEPP forest lateral flow + hillslope-to-channel transfer) and **Srivastava (2013)** (Priest
River: *no simulated surface runoff*, outlet streamflow from subsurface lateral/baseflow). That
literature directly supports a lateral-dominant partition on a wet (2825 mm/yr) forest hillslope
like H2637, and — correctly — provides **no hard coefficient** mandating 55.5% or 71%. So
openWEPP's 71% is not physically impossible, the comparator delta stays a valid flag, and the
owner is Stage-2 lateral/subsurface magnitude. The plausibility basis is real, not the legacy number.

## Discipline held

Per-term verdicts use the ADR-0017 taxonomy (`LEGACY-DEFECTIVE` for with_ui's 127.7%/152.6%;
`UNRESOLVED` Stage-2 flag for the bounded delta — *not* `OPENWEPP-DEFECTIVE`). No "match 55.5%"
anywhere. The handoff opens **no** Defect-Closure ExecPlan (correct — no in-envelope defect, no
contract gap) and instead names a well-scoped Stage-2 follow-on `STAGE2-LATQCC-H2637-MAGNITUDE`
with the right physics owner (WB19 lateral/drainage, restrictive-layer/conductivity controls),
the evidence it needs (per-OFE WB19 operands behind `latqcc`, not WAT/PASS aliases), and explicit
non-goals (don't reopen INV-028, don't target legacy with_ui, don't touch the passed duality).

## Disposition

Land the record (docs-only — no code/contract touched, none in scope). ROADMAP item 1 closes
complete; the residual routes to the Stage-2 physics-magnitude item as `STAGE2-LATQCC-H2637-MAGNITUDE`.
This adjudication is the template for the remaining Stage-2 magnitude triage: prove the machinery
is closed with identity evidence, localize the magnitude to a named process owner, route it with
legacy as a flag.
