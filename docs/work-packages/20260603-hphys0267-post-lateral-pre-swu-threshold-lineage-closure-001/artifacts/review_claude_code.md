# Review: Claude Code

Status: complete

Evidence mode: static (source/contract/artifact read) + recorded-log read

Static:

- Reviewer: Claude Code (`claude-opus-4-8`), invoked by user post-commit.
- Scope: static review of HPHYS0267 commit `82cce0f` — the trace-only
  post-lateral/pre-SWU threshold observability in
  `crates/openwepp-runner/src/hillslope/mod.rs`, the SC-WATBAL-001 /
  SC-SUBHYD-001 / SC-EVAP-001 amendments, and the diagnosis/classification
  artifacts.
- Baseline cross-check: the top-down realized-withdrawal authority was checked
  against `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:774-824`
  (consistent with the `fzdrfc` floor logic at 785-814 verified in the
  HPHYS0252 review).
- Continuity: extends the HPHYS0259/0265/0266 diagnostic chain and the
  2026-06-03 frost/`ksflag`/`ksatadj` audit (`docs/audits/`).
- Gate evidence read from package artifacts; Claude Code ran no `cargo`
  commands.

## Confirmations (no action)

1. The H7 anomaly from HPHYS0266 is resolved as baseline-faithful. H7's realized
   withdrawal from layer `0007` — outside the capacity-active set `0008,0009`
   and overlapping a SWU-stressed layer — is authorized by pinned baseline
   top-down realized withdrawal (`watbal_hourly.for:774-824`: withdraw top-down
   from any layer with `st > fzdrfc` after active-layer capacity calculation).
   0266 flagged H7 as the "best next target"; 0267 traced it and correctly
   classified it `BASELINE_TOPDOWN_WITHDRAWAL_FROM_NONACTIVE_CAPACITY_LAYER`,
   not a defect. That is genuine closure of a specific lead.
2. WB17 and WB19 realized-lateral identities close at all three first-divergence
   rows; pre/post-lateral withdrawal deltas reconcile. The post-lateral/pre-SWU
   threshold seam is ruled out as non-actionable.
3. Trace-only and honest: no production physics changed; full-suite metrics are
   byte-identical to HPHYS0264/0266 (`Ep` 56132, `Total-Soil` 149.44), and the
   disposition states plainly that metrics are a continuation baseline, not a
   closure claim. Contracts were amended before the (trace-only) edits.

## Findings

1. High (strategic, cross-WP) — the diagnostic loop has converged but not been
   acted on. HPHYS0259 (trace), 0265 (localization), 0266 (classification), and
   0267 (threshold lineage) have each ruled out a liquid-water seam (ET/SWU,
   aggregate recompute, realized lateral, threshold eligibility) and each ends
   with the same continuation pointer: "upstream material storage magnitude and
   snow/runoff." Four packages have now independently re-derived that the cause
   lies *outside* the kernels being instrumented — yet no package has entered
   the snow/storage-input domain. The next package should act on the
   convergence (SWE-ingestion provenance and `ksatadj` forest conductivity per
   the frost audit), not perform a fifth elimination in the liquid-water
   subsystem.

2. Medium (mis-anchoring) — the localization anchors on the wrong day. The
   diagnosis pins the first `|Ep diff| > 0.05 mm` crossing, which is mid-January
   (H1 Julian 15, H7 11, H39 22) at a trivial ~0.05 mm. But the classification
   table's own "First >1 mm" column shows the first *material* divergence onsets
   at **Julian 99 / 99 / 115 — i.e. April**, building to the 7.78 mm max. The
   material residual lives in the **spring-snowmelt window**, not mid-January.
   Anchoring four packages of analysis on a near-zero January crossing plausibly
   explains the recurring "identities close / context only / no defect" result —
   they examine a day where the residual is ~0. Re-anchor on the first >1 mm
   (April) divergence; that is where the physics actually breaks.

3. Note (triangulation with the frost/snow audit) — 0267's "upstream storage
   magnitude + snow/runoff" conclusion, the April-melt onset of the material
   divergence (finding 2), the untouched 562 mm `Snow-Water` residual, and the
   ingested-SWE-vs-computed-`snowd` mismatch are now triangulating on the same
   subsystem. The two open audit questions are the actionable next steps:
   (a) where openWEPP's `snow.runtime_swe` comes from vs the baseline's internal
   winter SWE, and (b) whether openWEPP models the `ksatadj` forest conductivity
   adjustment. Frost itself is off on both sides (shared assumption), so it is
   not the divergence — snowpack/melt and forest conductivity are.

## Notes for disposition owner (Codex)

- Findings 1 and 2 are the priority and are coupled: re-anchor the trace on the
  first >1 mm (April) divergence, and point the next package at the
  snow/storage-input domain rather than another liquid-water seam. The
  diagnostic discipline here is excellent; the risk is spending it in the wrong
  subsystem.
- The per-package craftsmanship remains high — H7 was a real lead and was
  resolved cleanly against pinned baseline authority, and the trace-only /
  no-defect disposition is truthful. The concern is strategic targeting, not
  correctness.
- Disposition `HOLD` at `0/39` is consistent with this review; no overclaim
  observed.
