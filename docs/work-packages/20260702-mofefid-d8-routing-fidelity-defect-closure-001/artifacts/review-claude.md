# Claude Code — Independent Review of D8 Execution

Date: 2026-07-02. Reviewer: Claude Code (contract + science review; the D1–D7
executor, reviewing Codex's D8). Evidence: **Static** (read the diffs,
`package.md`, `forcing-operand-audit.md`, contract rev-9 diff) + **Ran** (grep
shadow-first; R-63 unit check). Disposition: **APPROVE — merge-ready with 2
minor non-blocking traceability findings.**

## Verdict: sound, honest, well-evidenced

All four items close with defensible verdicts, and the DC discipline held:

- **D8-1 (corrected) — verified.** R-63 line 69 **explicitly** states "the
  rainfall intensity (m/s), I" for eq. (2), so the SI `m/s` convention is
  genuinely confirmed-against-R-63 (secondary), not self-referential. The
  frozen-library caveat on the 3393 primary coefficient is correctly retained
  (`GAP-OFEROUTE-002`). Removing the silent `.max(0.0)` (negative `I` → NaN,
  fail-loud; callers validate) is a legitimate safety improvement.
- **D8-2 (corrected metric + declared boundary) — strong.** The sampler bug
  (step-end values stamped onto crossed sample times) was real; the
  interpolation fix is correct. Notably this **supersedes my D7 Case-4
  claim**: my "timing/rise reproduce at k_o=200" was itself an artifact of the
  buggy sampler — corrected, sampled `t_peak` is ~37 s (not 28 s), NS ~0.26,
  resolution-sensitive. `GAP-OFEROUTE-005` is exemplary: an in-envelope
  α-iteration fix was **tried and rejected** because it broke steady/cascade
  conservation, so no surrogate landed — the HOLD-legitimacy conditions are
  met and the boundary ties correctly to the frozen-library TVD primaries
  (`GAP-OFEROUTE-001`).
- **D8-3 — honest.** Case 2 operand-limited is well-evidenced (Ks=20 → NS 0.45;
  plausible Ks=10 → NS 0.96 — not tuned into a pass, verdict rests on the
  uncertain operand). Case 3 declared boundary correctly identifies the S0
  magnitude anomaly (enhanced peak exceeds the plot `I·L`).
- **D8-4 (operand-limited) — rigorously decomposed.** The routing-only (Ks=0)
  diagnostic rising in 77.4 s vs the Green-Ampt default 5000 s is a clean
  forcing-first proof that routing celerity is not the source. This is the
  forcing-before-attribution rule applied exactly as intended.
- **API improvement:** `DvalRun` now derives peak/time from `sampled_peak`, so
  the internal-vs-sampled metric disagreement is resolved at the source, not
  merely documented.

Shadow-first preserved (grep: no `ofe_routing::` in `direct_runtime/` or
`openwepp-runner/`). Copyright preserved (no vendored series; cited scalars +
sha256). Contract rev 9 dispositions each touched INV/GAP with revision
entries; `INV-OFEROUTE-011` honestly reads "ZERO cases cleanly reproduce."

## Findings (minor, non-blocking)

**CL-D8-1 — Disclose the sampler change's cross-cutting effect.** The D8-2
interpolation changes the **sampled outlet hydrograph for every routing run**
(D4/D5/D6 + the D5 cascade handoff, which reads the outlet hydrograph), not
only Iwagaki. All tests pass and mass conservation is unaffected (the ledger
uses the solver's internal accumulation, not the samples), and it is an
accuracy improvement — but this behavior change to already-landed D4/D5
surfaces should be stated explicitly in the package (one line: "D8-2 changes
sampled hydrograph values for all routing runs; conservation/CFL unaffected;
cascade handoff now interpolated; D4/D5/D6 suites green").

**CL-D8-2 — Point the superseded D7 Case-4 claim at D8.** Once D7 and D8 are
both on main, the D7 `execution-report.md` will still say Case 4's "timing/rise
reproduce at k_o=200," which D8 has shown to be a sampler-bug artifact. D8's
`INV-OFEROUTE-011` and the renamed test correctly carry the corrected state; a
one-line superseded-pointer in the D8 report (or a note appended to the D7
report on merge) closes the contradiction so the two package reports don't
silently disagree.

## Recommendation

Merge-ready. The two findings are documentation/traceability only — no
correctness or physics issue. Optionally address CL-D8-1/CL-D8-2 with two
one-line notes before merge; neither blocks.
