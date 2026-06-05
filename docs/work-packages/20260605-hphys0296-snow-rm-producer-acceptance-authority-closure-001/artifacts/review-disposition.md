# Review Disposition

Status: executed-hold
Evidence mode: Static

Static:
- Received `artifacts/claude-code-review-findings.md`.
- `CLAUDE-0296-001` accepted: the original acceptance wording was too
  permissive because it allowed correlation plus internal closure to support
  semantic acceptance without per-window root-cause proof.
- `CLAUDE-0296-002` accepted as process debt: dual subagent review remains
  not-run under current delegation policy; the package stays `executed-hold`.

Disposition:
- Amended `SC-SNOWFREEZE-001#INV-SNOWFREEZE-027`,
  `SC-RUNOFFPART-001#INV-RUNOFFPART-024`, and
  `SC-WATBAL-001#INV-WATBAL-071` so material negative raw melt, internal
  snow-state closure, `RM` identity, and closed `Q` are necessary diagnostics
  but not sufficient acceptance authority.
- Required per-window defective-model verdict before any residual leaves the
  failing set:
  - mechanistic `file:line` root cause in both openWEPP and
    `/workdir/wepp-forest_260430_baseline`,
  - reconstruction controlled experiment to named tolerance,
  - independent correctness adjudication,
  - explicit `LEGACY-DEFECTIVE`, `OPENWEPP-DEFECTIVE`, or `UNRESOLVED`
    disposition.
- Updated the HPHYS0296 contract-derived test to guard the tightened wording.
- Six corrected-negative-melt windows remain candidates only; none are accepted
  or re-tiered by HPHYS0296.
- Dual review remains outstanding.
- Package remains `executed-hold`.
