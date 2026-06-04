# Claude Code Review Findings — HPHYS0287

Reviewer: Claude Code (independent review; implementation deferred to Codex)
Verdict: **APPROVE as a guard-hardening package** — with a significant flag that
the package did not deliver its stated physics objective and that objective is
now overdue.
Evidence mode: static (diff + contract + localization) + ran (report
cross-check, focused + adjacent tests).

## Summary

HPHYS0287 was scoped to correct snow liquid retention / melt-release / runoff
*magnitude* (the mechanism expected to move `Q`/`RM`/`Snow-Water`). It instead
delivered a fail-open -> fail-closed runtime snow-state guard fix with zero
valid-run metric movement. The guard fix is correct and reinforces fail-closed
discipline; the disposition is honest that valid-run residuals are unchanged.
The substantive concern is strategic: the snow-magnitude defect has now been
deferred across four consecutive packages.

## What I verified

Ran (this review):
- Suite metrics match the on-disk report at
  `/tmp/hphys0287_full_release_after_review_20260604T221027Z/reports/hillslope_semantic_summary.md`:
  `Total-Soil` mean `61.115200`; `Q` `0.552218`; `RM` `0.248018`; `Snow-Water`
  `2.899431`. The disposition's numbers are truthful.
- `Q`/`RM`/`Snow-Water` are byte-identical to HPHYS0284/0285/0286. The
  `Total-Soil` 71.75 -> 61.12 improvement is attributable to HPHYS0286, not
  HPHYS0287.
- Focused `hphys0287_snow_liquid_partition_guard_contract`: 7 passed.
- `clim06_frost_frozen_soil_kernel_contract`: 11 passed (fixture snow-state
  vector completion).
- CLAUDE-0285-001 remediation confirmed in HEAD (`ccb8beb`): overdraw bounded by
  `SIMIMPL29_SNOWPACK_STATE_LOSS_OVERDRAW_TOLERANCE_M = 0.005` with fail-closed
  beyond tolerance — the bounded-tolerance option recommended in the HPHYS0285
  review.
- I did not independently rerun `cargo test --workspace`; the package
  gate-results and disposition claim it passes, and I ran the focused + adjacent
  subset.

Static (read + reasoned):
- The fix introduces a shared `validate_runtime_snow_state_domains` consumed by
  `resolve_active_snow_coupling`, `compute_same_pass_wb14_infiltration_lineage`,
  and `run_runoff_reconciliation`. It fails closed for missing projected snow
  vector members, non-finite values, material negative SWE/depth/density/settle
  count, and density above the snow-density cap. It returns explicit no-snow only
  when no snow option/control/runtime projection exists at all.
- This closes the same class of defect as CLAUDE-0285-001: a material-negative
  projected snow state previously could be classified as inactive stale snow and
  silently zeroed before WB12 same-pass infiltration and WB14 runoff
  reconciliation. Hardening it fail-closed is the right direction and aligns with
  the package's own stated intent to preserve fail-closed guards.

## Findings

### CLAUDE-0287-001 [MEDIUM] — Objective/title mismatch; package is robustness, not parity progress

The package objective and title describe a valid-run snow liquid
retention/runoff/infiltration *magnitude* correction. The delivered change is a
guard/robustness fix with zero valid-run movement (`Q`/`RM`/`Snow-Water`
unchanged; `Total-Soil` improvement owned by HPHYS0286). This should be recorded
as robustness, not parity progress, so the work-package ledger does not imply the
snow-magnitude residual was addressed. The disposition is honest about this; this
finding asks that the package framing match the delivered scope.

### CLAUDE-0287-002 [MEDIUM-HIGH, strategic] — Snow-magnitude fix deferred across four consecutive packages

`Q`/`RM`/`Snow-Water` mean-abs-diff have been unchanged across HPHYS0284,
HPHYS0285, HPHYS0286, and HPHYS0287. The snow liquid retention/runoff magnitude
defect has been named the "next target" since the HPHYS0283 worker handoff and
keeps being routed around by adjacent plumbing/guard fixes. HPHYS0287 is the
starkest case: scoped directly at the magnitude defect, it fixed a (real, latent)
fail-open seam and re-deferred the magnitude work.

Fixing the fail-open seam before tuning magnitude is defensible. The concern is
the pattern: diagnosis repeatedly re-confirms the same target while the
parity-moving fix slips package to package. The next package should be the actual
baseline-authoritative `winter.for` rain-on-snow retention/release and
`runoff.for`/melt-partition magnitude work, anchored on H1/H7/H39, preserving the
HPHYS0287 fail-closed guard — not a fifth lap of adjacent hardening.

### CLAUDE-0287-003 [LOW, process] — HPHYS0286 and HPHYS0287 both uncommitted and intermingled

At review time both packages are `executed-hold` and uncommitted simultaneously;
their working-tree changes are intermingled, and the suite report reflects the
combined 0286+0287 state. HPHYS0287's individual contribution cannot be isolated
in a running binary. Recommend committing HPHYS0286 before layering HPHYS0287 so
the two can be reverted/attributed independently.

### CLAUDE-0287-004 [POSITIVE] — Reviews and discipline held

Dual review caught real issues and fixed them: Lovelace found the validator
initially defaulted absent `snow.runtime_*` fields to `0.0` (masking partial
projected vectors) and corrected it to require the full vector when any snow
state is projected; Gibbs expanded test breadth to seven vectors covering
depth/density/settle/non-finite/over-cap/partial/roundoff. Combined with the
CLAUDE-0285-001 re-bounding in `ccb8beb`, the fail-closed-over-canonicalize
discipline is holding across this snow sequence.

## Continuation read

The remaining parity blockers are now well-isolated:
- Snow liquid retention/runoff *magnitude* (`Q`/`RM`/`Snow-Water`) — untouched
  for four packages; the standing physics target.
- `Total-Soil`/`Ep` late-season wetness (e.g. H1 2015) — points at WB17
  ET/drainage withdrawal from corrected storage.

Recommended next package: baseline-authoritative `winter.for` rain-on-snow
liquid retention/release and `runoff.for` melt/rain partition magnitude, anchored
on H1/H7/H39, preserving the HPHYS0287 fail-closed snow-state guard.
