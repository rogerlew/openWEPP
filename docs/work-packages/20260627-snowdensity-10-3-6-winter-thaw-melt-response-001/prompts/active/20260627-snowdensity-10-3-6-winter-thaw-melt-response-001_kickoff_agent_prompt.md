# Kickoff Prompt - SNOWDENSITY-10.3.6

Execute `docs/work-packages/20260627-snowdensity-10-3-6-winter-thaw-melt-response-001/package.md`.

The objective is to diagnose the 10.3.4 rank-2 winter-thaw melt-response
hypothesis by comparing observed snow-depth ablation intervals with modeled CoE
thaw-window melt response. Use diagnostic `legacy_coe` replay only; do not
change production physics.

Guardrails:

- No production physics changes.
- No default activation or parser/runfile/user CLI selector.
- No fixture edits, public output schema changes, site constants, tuning,
  density, phase, frost, longwave, or rain-heat correction.
- HJ Andrews and Hubbard Brook are observation-blocked diagnostic-only surfaces.
- Close only with current evidence for every package acceptance gate.
