# CAL-07E Gate Evidence

Evidence class: `Ran + Static`

| Requirement | Status | Evidence |
| --- | --- | --- |
| Search and admission protocol | `PASS` | Four lanes, full-text rule, evidence hierarchy, and stop rule are explicit. |
| Source register | `PASS` | 15 sources or acquisition leads; stable locators, geography, full-text status, and claim ceilings validate. |
| Claim calibration | `PASS` | 15 claims bind to source IDs and allowed statuses; direct-site, regional, and analogue ceilings remain distinct. |
| PhenoCam source custody | `PASS` | Eight exact provisional rows retained; archive, member, and subset hashes plus processing/retrieval dates recorded. |
| Transition-product audit | `PASS` | Validator reconstructs all 12 nominal dates and normalized confidence intervals from retained rows; deltas pass. |
| Figure and sidecar | `PASS` | SVG XML and 1200×720 render pass; accessible title/description, legible marks, caption, and ancillary limits retained. |
| Python syntax | `PASS` | `.venv/bin/python -m py_compile .../tools/validate.py`. |
| Package validation | `PASS` | 15 sources, 15 claims, 12 comparisons, and one figure. |
| Markdown lint | `PASS` | Package, roadmap, and catalog have zero errors or warnings. |
| Diff hygiene | `PASS` | `git diff --check`; only the declared package, roadmap, and catalog changed. |
| Dual terminal review and verification | `PASS` | Reviews A/B and Verifications A/B authorize method audit only and retain the science hold. |

No production, contract, forcing, or parameter test is applicable because the
package changes documentation and package-local validation only.
