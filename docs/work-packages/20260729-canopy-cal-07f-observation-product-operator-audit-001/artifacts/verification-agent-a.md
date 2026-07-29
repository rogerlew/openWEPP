# CAL-07F Terminal Verification A

Evidence class: `Ran + Static`

Verdict: `PASS / DO NOT CALIBRATE`

## Independent executed checks

| Check | Result | Evidence |
| --- | --- | --- |
| Dependency custody | `PASS` | Independently recomputed SHA-256 and byte size for all six dependencies; 6/6 match `dependency-manifest.csv`. |
| Daily-product inventory | `PASS` | Independently read the retained source: 731 consecutive rows from 1 January 2024 through 31 December 2025, 21 provider interpolation flags, and no date gap. |
| Transition reconstruction | `PASS` | Independently expanded the eight retained Data Record 5 rows to 24 transitions and linearly reconstructed same-direction daily crossings: 23 exact dates and one `+4.625`-day crossing inside its source CI. |
| Seasonal windows | `PASS` | Independently reconstructed eight product/year/direction windows from the midpoint between falling T10 and rising T10; every retained selected crossing is in its applicable window. |
| Relative comparisons | `PASS` | Independently indexed CAL-07D BASE crossings and reproduced all 888 rows, including candidate counts, selected ordinals, residuals, missing states, and CI classifications with maximum numeric difference `0`. |
| Absolute comparisons | `PASS` | Independently reproduced all 296 rows and the per-product result of 11 seasonal crossings; CI hits are one for `gcc_mean` and zero for `gcc_90`. |
| Member summaries | `PASS` | Independently reduced 74 product/member summaries; all counts, medians, signed direction residuals, penalties, and pass fields match with maximum numeric difference `0`. |
| Product ranking | `PASS` | Both independently sorted 37-member rank orders are identical; `GSI-4831` ranks first in each product. |
| CAL-07D parameter screen | `PASS` | Independently screened all four non-base scenarios. None has 148/148 matches and both direction medians within ±21 days. |
| Decision reduction | `PASS` | Independently obtained `PASS, FAIL, FAIL, FAIL, FAIL, PASS` and final `DO_NOT_RECOMMEND`. |
| Figures and sidecars | `PASS` | Three SVGs parse, render, and contain accessible title/description metadata; visual inspection agrees with the retained tables and each Markdown sidecar states caption, reading guidance, and ancillary limits. |
| Package validator | `PASS` | `.venv/bin/python .../tools/validate.py` reports 6 dependencies, 731 daily rows, 24 transitions, 888 relative rows, 296 absolute rows, 74 summaries, 3 figure/sidecar pairs, and `DO_NOT_RECOMMEND`. |
| Python syntax | `PASS` | `.venv/bin/python -m py_compile .../tools/*.py`. |
| Diff hygiene | `PASS` | `git diff --check` passed at this review snapshot. |
| Exact write set | `PASS` | Changes are limited to the CAL-07F package, the CAL-07E acquisition-deferral artifact, canopy roadmap, and work-package catalog. No production, contract, ADR, dependency, or predecessor result file changed. |

## Independently reproduced decisive values

```text
source hashes:                         6/6
daily rows / transitions:              731 / 24
relative / absolute inventories:       888 / 296
product-member summaries:              74
members complete in either product:    0
uncertainty-fit passes:                 0
direction-coherence passes:             0
best member:                            GSI-4831
best GCC mean score / CI hits:          59.124713 d / 1
best GCC 90 score / CI hits:            65.874713 d / 0
final decision:                         DO_NOT_RECOMMEND
```

Crossing availability independently reduces to 37/37 for every falling level
and both rising T10 events, 1/37 for 2024 rising T25, 8/37 for 2025 rising
T25, and 0/37 for both rising T50 events. These counts are identical under
both observation products.

The counterfactual scenario screen independently returned:

```text
PHOTOPERIOD_AND_VPD_UNCONSTRAINED   85/148 matches
PHOTOPERIOD_UNCONSTRAINED           93/148 matches
TEMPERATURE_UNCONSTRAINED           64/148 matches
VPD_UNCONSTRAINED                  148/148 matches;
                                   medians +44.491749 / -59.497229 d
```

Thus no scenario passes the package's completeness and bidirectional
21-day criterion.

## Closure statement

All independently executable CAL-07F science and artifact gates reviewed by
Verifier A pass. The no-calibration and ecosystem-model-limitation claims are
supported within their declared scope.

I reran the strengthened package validator after closure integration; it
passes with the same inventories and `DO_NOT_RECOMMEND` decision. Python
syntax, package and changed-document Markdown lint, unstaged and staged diff
hygiene, updated 785/270/511 line counts, prompt archival, exact-diff
reconciliation, finding disposition, dual review/verification, final
disposition, and roadmap/catalog closure status all pass. No final mismatch
remains.
