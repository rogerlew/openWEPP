# Selector Policy

Status: COMPLETE. Evidence mode: Static + Ran.

The package implements selector-first staging:

- `OPENWEPP_LANED_ACTIVE_IMPLICIT=1` is a hybrid request, not unconditional
  hybrid execution.
- A routed lane-day selects hybrid only when the post-growth active cell
  operands are exact bare-skin eligible: no Manning override, no active
  roughness-element addend, and no active vegetation addend.
- A routed lane-day that is not exact bare-skin falls back to the plain
  rev-27 active route while preserving the same active ownership,
  no-DC01-double-feed guard, D13 erosion consumer, and day closure checks.
- The selector is a pure function of run inputs already present in the active
  lane-day: static friction/cover operands and post-growth daily LAI/canhgt.
- Wall time, host load, measured profile counters, and mid-run observed
  iteration counts are evidence only; they cannot influence routing output.

This policy can lift selected-cohort timing no-harm by preserving the known
H2637 bare-skin win and avoiding generic non-bare solve-cost regressions. It
does not close non-bare solve-cost viability or default promotion.

Execution result: the policy lifted selected-cohort timing no-harm for opt-in
hybrid request at current mesh. H2637 selected hybrid for `11590/11590`
requested lane-days; the three non-bare selected members fell back to active
plain for `7299/7299` requested lane-days.
