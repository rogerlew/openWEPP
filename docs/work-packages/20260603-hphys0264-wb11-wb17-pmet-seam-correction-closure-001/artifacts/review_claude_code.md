# Review: Claude Code

Status: complete

Evidence mode: static (source/contract read) + recorded-log read

Static:

- Reviewer: Claude Code (`claude-opus-4-8`), invoked by user post-commit.
- Scope: static correctness review of HPHYS0264 commit `c4d8c86` — the
  branch-aware WB17 PMET seam in
  `hydrology/03_kernel_support_01_kernel_phases.rs`, the summary-accumulator
  `Es` guard change (`crates/openwepp-summary-accumulator/src/lib.rs`), and the
  SC-EVAP-001 / SC-WATBAL-001 amendments.
- Baseline cross-check: the `evappm`/`swu` control flow was verified directly
  against `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:556-559,981`
  and `watbal.for:494-497,921`.
- Continuity: this package was opened to resolve findings 1 and 2 of the
  HPHYS0263 `review_claude_code.md`; this review verifies that resolution.
- Gate evidence read from package artifacts; Claude Code ran no `cargo`
  commands.

## Confirmations (no action)

1. HPHYS0263 findings 1 and 2 are resolved, faithfully. WB17 now branches on
   `wb11_et_seed_branch_evappm`: in PMET mode it consumes `pmet.es_m` for `Es`
   and sets `Etp = pmet.ep_m`, bypassing the Priestley-Taylor partition; PT mode
   retains the old partition. This matches baseline control flow:
   `watbal_hourly.for:556-559` calls `evappm` instead of `evap` in PMET mode,
   and `:981` calls `swu` unconditionally, so feeding PM `ep` into SWU
   (SWU authoritative for final `Ep`) is baseline-faithful. The original bug was
   the PT partition being applied to a PM actual; keeping SWU is correct.
2. Behavioral red-green, quantified: pre-impl gate failed with
   "PMET mode must pass pmet.ep_m to SWU as Etp, observed 0.0016" vs expected
   `0.0034`, i.e. the old PT partition halved the PM transpiration potential.
   Honest note that a test-adapter compile error was corrected before the
   behavioral red was recorded.
3. Day-1 residuals collapsed with the correct partition: H39 `Total-Soil` diff
   `-8.89 -> -0.10 mm`, `latqcc` `+8.73 -> +0.18 mm`; H1/H7 similar. The day-1
   water balance now essentially closes.
4. Honest disposition: `HOLD`, full suite still `0/39`, with the continuation
   correctly redirected to the first large `Ep` divergence day rather than
   day-1 PMET seed selection.

## Findings

1. Medium — the `Es` non-negativity guard was removed (not tolerance-bounded) in
   PMET mode, in two places. The WB17 kernel uses
   `require_flux_range(WB17_SYMBOL_ES, pmet_es_m, None, None)` and the summary
   accumulator switches `Some(0.0)` to `None` when `evappm_pmet_branch`, so both
   now accept arbitrarily negative `Es`. But `pmet.es_m`'s HPHYS0263 derivation
   (`kecon*bpotes/etke + resint`, with `potes>=0`, `kecon>=0`, `bpotes>=0`)
   appears non-negative by construction, so removing the lower bound — rather
   than a roundoff-tolerance snap like the WB15 fix in HPHYS0250 — is
   unjustified in the diff. If only float roundoff is at issue, prefer a bounded
   snap; if material negative `Es` is physically reachable, document why. As
   written, a real negative-`Es` regression would pass silently in PMET runs.

2. Low — deferred-segment line range drifted between packages. HPHYS0263 cited
   `evappm.for:391-454` as the unported redistribution; HPHYS0264 cites
   `:460-523`. Reconcile which lines actually remain unported so the follow-on
   scope is unambiguous.

3. Note (continuation, positive) — the diagnosis frontier has genuinely moved.
   With seed (HPHYS0254), PM demand mode (HPHYS0263), and the PM seam (this
   package) all fixed, day-1 now closes to ~0.1 mm. The defect is no longer
   "everything ~9x low from t=0"; it is a seasonal divergence that onsets
   mid-run, downstream of the still-open storage drain. The disposition's
   redirect to tracing the first large `Ep` divergence day is the right next
   step and will localize the onset faster than another surface fix.

## Notes for disposition owner (Codex)

- Finding 1 is the only correctness item to tighten: restore a bounded `Es`
  lower guard in PMET mode or document why `pmet.es_m` can be materially
  negative. It is a silent-regression risk, not a current wrong value.
- Finding 2 is a documentation reconciliation.
- Finding 3 affirms the continuation plan; no change requested. The day-1
  closure is the first unambiguous structural win in the HPHYS0249-0264 run,
  and the investigation is now correctly pointed at the seasonal onset rather
  than circling individual surfaces.
- This package is a faithful, well-scoped resolution of prior review findings
  with a quantified red gate. Disposition `HOLD` at `0/39` is consistent with
  this review; no overclaim observed.
