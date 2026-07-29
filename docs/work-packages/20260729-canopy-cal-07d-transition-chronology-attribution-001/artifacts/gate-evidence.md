# CAL-07D Gate Evidence

Evidence class: `Ran`

| Requirement | Status | Evidence |
| --- | --- | --- |
| Dependency/source custody | `PASS` | `dependency-manifest.csv`; validator verified ten hashes and commit binding. |
| BASE CP-GSI01 reconstruction | `PASS` | 61,642 rows; maximum equation residual `0.000e+00`. |
| CAL-07C reproduction | `PASS` | 148 rows, 11 matches, residuals and crossing counts reproduced. |
| Declared inventories and keys | `PASS` | 1,628 model-level, 444 source-level, 1,488 scenario rows; validator PASS. |
| Fixed event-year thresholds | `PASS` | Validator proved equation and no per-crossing/year-boundary threshold change. |
| Scenario isolation | `PASS` | Full-history recomputation; five ensemble scenarios plus one default trajectory. |
| Observation support | `PASS` | Twelve event/level rows retain raw support, smoothing, confidence width, and source order. |
| Decision predicates | `PASS` | Seven machine-readable rows independently reduced. |
| Figures and sidecars | `PASS` | Four SVG/Markdown pairs; XML, render, source binding, assumptions, and evidence ceilings verified. |
| Python syntax | `PASS` | `.venv/bin/python -m py_compile .../tools/*.py`. |
| Package validation | `PASS` | `.venv/bin/python .../tools/validate.py`. |
| Markdown lint | `PASS` | Package: 26 files; roadmap and catalog: one file each; zero errors and warnings. |
| Exact diff hygiene | `PASS` | `git diff --check`; `exact-diff-reconciliation.md`; only declared package, roadmap, and catalog surfaces changed. |
| Dual terminal review/verification | `PASS` | Reviews A/B and verifications A/B pass; both retain the Order 7 scientific hold. |
