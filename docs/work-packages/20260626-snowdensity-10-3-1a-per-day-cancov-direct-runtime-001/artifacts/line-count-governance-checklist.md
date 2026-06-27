# Line-Count Governance Checklist

Evidence class: Static + Ran.

- Root `AGENTS.md` was not edited.
- No nested `AGENTS.md` files were edited.
- `export_pysnobal_inputs` initially exceeded clippy's local function line
  threshold after adding daily canopy export. The lane export loop was extracted
  into `write_pysnobal_lane_exports`, and clippy passed.
- Work-package artifacts are package-scoped under
  `docs/work-packages/20260626-snowdensity-10-3-1a-per-day-cancov-direct-runtime-001/`.
- No long tutorial or process guidance was added to root docs.

Verdict: compliant.
