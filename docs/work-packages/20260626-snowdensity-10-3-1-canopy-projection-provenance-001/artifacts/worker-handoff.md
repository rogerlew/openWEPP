# Worker Handoff

Status: complete.

Next package should not start gradient melt adjudication by assuming the current
snowbench runtime surface is seasonal. The current state is:

- openWEPP snowbench CoE melt diagnostics consume static
  `generated_openwepp_runtime_surface.cancov`;
- that static value is seeded from fixture initial-condition `cancov`;
- upstream wepppy has seasonal evergreen/mixed/deciduous WEPP projection
  evidence, but openWEPP snowbench does not consume it as a per-day series.

First actionable item for the next package:

Decide whether §10.3.3 Gradient Melt Adjudication requires per-day canopy. If
yes, implement a diagnostic/runtime path carrying daily `cancov` into CoE melt
replay before running the rubric. If no, label the adjudication
static-initial-canopy evidence and exclude seasonal-phenology claims.

