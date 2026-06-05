# Worker Handoff

Status: complete

Evidence mode: static + ran

Static:

- HPHYS0304 completed ADR-0016 Required Continuation Order step 1 and queued
  HPHYS0305 for step 2.

Ran:

- Use `artifacts/fixed-baseline-semantic-metrics.md` for H1..H39 semantic
  context.
- Use `artifacts/snow-rm-window-reclassification.md` for the nine target-window
  route decisions.
- Use `docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/package.md`
  as the next execution package.
- Do not patch production snow/melt/forcing/WB13/WB17/WB18/WB19/WB12 code from
  HPHYS0304 aggregate residuals.
- Dual review and dual verification are complete; no unresolved review
  findings remain.
