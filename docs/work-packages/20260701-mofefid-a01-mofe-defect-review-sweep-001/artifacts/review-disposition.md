# Review Disposition

Review: `review-codex.md` (Codex, 2026-07-01, static/source review —
appropriate scope for a docs/artifact package). Verdict: **accepted**, one
non-blocking cleanup.

| # | Finding | Disposition | Action taken |
|---|---|---|---|
| C1 | F-A2's source-intent citation used current-tree line numbers only; the ADR-0024 anchor is the pinned baseline | **accepted** | Baseline lines independently re-verified (**Ran**) and cited in `sweep-notes.md` + `findings.md`: `wepp-forest_260430_baseline/src/watbal_hourly.for:361-363` (daily `fin`) and `:471-473` (hourly `xfin`). The `:471-473` read **strengthens the finding**: the hourly supply adds the upstream carry arrays `(ui_LfUrf + ui_SUrunf)` with area scaling — surface *and lateral* runon both re-infiltrate hourly in the baseline, so openWEPP's exclusion diverges at both granularities and both components. |

The six findings and dispositions stand as reviewed.
