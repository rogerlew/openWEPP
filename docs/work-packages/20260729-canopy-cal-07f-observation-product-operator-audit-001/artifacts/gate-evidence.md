# CAL-07F Gate Evidence

Evidence class: `Ran + Static`

| Requirement | Status | Evidence |
| --- | --- | --- |
| Dependency/source custody | `PASS` | Six predecessor files independently hash and size match `dependency-manifest.csv`. |
| Daily curve integrity | `PASS` | 731 consecutive 2024–2025 rows; finite curves; zero outlier flags; 21 provider interpolation flags. |
| Transition reconstruction | `PASS` | 24/24 transitions have daily crossings; 23 exact nominal dates; one 4.625-day difference remains inside its source CI. |
| Relative comparison inventory | `PASS` | 888 unique product/member/event/level rows across 37 members. |
| Absolute comparison inventory | `PASS` | 296 unique product/member/event rows. |
| Seasonal selection | `PASS` | Product/year windows prevent wrong-season intra-annual recovery crossings from counting. |
| Member reduction | `PASS` | 74 summaries reconstruct crossing, missing, CI-hit, residual, and direction fields with biconditional pass/fail checks. |
| Decision reduction | `PASS` | Validator independently reconstructs rank/overlap and CAL-07D scenario predicates; six criteria reduce to 2/6 passing and `DO_NOT_RECOMMEND`. |
| Figures and sidecars | `PASS` | Three SVGs parse, render, carry accessible title/description, and have complete Markdown sidecars. |
| Python syntax | `PASS` | `.venv/bin/python -m py_compile .../tools/*.py`. |
| Package validation | `PASS` | `.venv/bin/python .../tools/validate.py`. |
| Markdown and diff hygiene | `PASS` | Package, CAL-07E deferral, roadmap, and catalog lint with zero findings; `git diff --check` passes. |
| Dual terminal review/verification | `PASS` | Reviews A/B and Verifications A/B independently reproduce the no-calibration disposition. |

Rust, workspace, conservation, consumer-path, dependency-resolution, and
security gates are not applicable to this documentation and package-local
diagnostic increment.
