# Claude Code Review — FQ3-DC RUNOFFPART Q/QOFE Underproduction Closure

Reviewer: Claude Code
Date (UTC): 2026-06-07
Evidence mode: **Static** — read the localization, validation ledger, disposition,
the `SC-RUNOFFPART-001` v39 amendment, and the `03_kernel_support_00/01` diffs.
The p8/p1 + 42-prefix reruns and closure CSVs are Codex's `Ran` evidence,
attributed.

Verdict: **Approve.** Exemplary DC-ExecPlan: real mechanism localized, the
conservation trap caught and corrected, a contract-first authority-backed fix
landed, runoff now engages, and the rung-1 closure still holds to machine
precision. One calibrated watch-item (residual magnitude vs legacy) and a
merge-seam note.

---

## F1 — Sound, contract-first fix; conversion rule honored (primary)

- **Real mechanism, not my guess.** I had hypothesized gentle storm-intensity
  disaggregation; the actual cause is a **storage-limited infiltration cap not
  enforced**. On p8 1990-08-25, 24.6 mm rain was fully infiltrated with the top
  two WB18 layers at/near `ul`, while legacy produces ~20.4 mm runoff. openWEPP
  over-absorbed event liquid because it did not apply the upper-storage condition
  before publishing same-pass infiltration.
- **Fix verified in the diff:** `resolve_wb14_top_two_layer_storage_available`
  sums `(ul − theta).max(0)` over the top two layers; `apply_wb14_storage_limit_
  to_infiltration` does `infiltration.min(available_storage)` so the
  storage-limited residual becomes runoff; and `resolve_wb14_producer_published_
  infiltration` consumes the WB18-produced same-pass infiltration. Matches the
  contract.
- **Conservation trap caught and corrected.** The localization openly records that
  the first pass produced runoff but *broke* annual closure (WB14 recomputed
  infiltration after ET/lateral mutation), and the correction is consume-not-
  recompute — the same discipline as WBVAL05. The package's "do not create runoff
  by breaking the balance" boundary held.
- **Authority-backed, not comparator-tuned.** `INV-RUNOFFPART-027` cites WEPP
  **Eq. [4.3.2]** (REF-RUNOFFPART-CH4-RAINEX/INFIL), is `hard-fail`, and uses real
  `wb18_perc_theta` vs `wb18_perc_ul` state with typed errors on malformed
  symbols. The top-two-layer storage limit is a Chapter-4 mechanism, not a fudge.
- **Validated:** `Q>1e-6` 7/42 → **42/42** (p8 0→513 mm, p1 0→138 mm); annual
  closure across 252 rows (42×6 yr) stays at **~1e-11 mm** (machine noise). Runoff
  engaged *and* conservation preserved — the acceptance is met.

This is the conversion rule working: in-envelope root cause + Ch4 authority → a
landed, validated fix that keeps rung-1 closed.

## F2 — Residual magnitude vs legacy (calibrated watch-item, not a defect)

openWEPP runoff is now nonzero but materially below legacy: p8 513 vs 760 mm,
p1 138 vs 278 mm (~60–67%). Per ADR-0017 this is **correctly accepted** — the
mechanism engages and conserves, and legacy is a flag, not a target; the package
did not tune to legacy. But worth recording: this fix addressed the
**saturation/storage-excess** runoff component; the remaining gap may be the
**infiltration-excess (intensity-driven)** component still under-producing — i.e.
my earlier storm-intensity hypothesis, now plausibly the *second* runoff path. It
is not a blocker (mechanism + conservation are satisfied), but runoff magnitude
matters downstream (erosion), so the gap is worth a future characterization —
ideally as an explicit follow-on rather than left implicit.

## F3 — Top-two-layer scope (minor)

`INV-RUNOFFPART-027` is authority-cited (Eq. 4.3.2) but the "top-two-layer"
specificity is `[INFERENCE]`-tagged. Worth a one-line confirmation that Eq. 4.3.2's
upper-storage condition is over exactly the top two layers (vs the wetting-front
layer or full profile) so the layer count is provenance-pinned, not a tuned choice.

## F4 — Merge seam with the corn-ET DC (coordination)

This DC edits `03_kernel_support_01_kernel_phases.rs` (and `_00`), and the parallel
`FQ3-DC-ET-CORN-ENGAGEMENT-001` (Codex) operates on the ET path in the same
kernel-phases file. They are different mechanisms (runoff partition vs crop-ET
engagement) but adjacent code — watch the seam when the corn-ET DC lands; a
re-run of *both* the runoff closure (Q + conservation) and the corn-ET check after
the second one merges is the safe confirmation.

---

## Recommendation

Approve and commit (the runoff cap + consume fix, `SC-RUNOFFPART-001` v39, the
hyetograph kernel test). No rework. Two carry-forwards: (1) the residual runoff
magnitude vs legacy — consider a follow-on characterization of the
intensity-driven (infiltration-excess) component; (2) revalidate Q + conservation
after the corn-ET DC merges (shared kernel-phases file). With this, the two
fundamental rung-2 partition defects (crop ET, runoff) are both being closed on a
conserving foundation — FQ-4 (frost) remains last, on the repaired substrate.
