# CAL-07E Terminal Verification A

Evidence class: `Ran + Static`

Verdict: `PASS`

## Independent checks

| Check | Result | Evidence |
| --- | --- | --- |
| Package validator | `PASS` | `.venv/bin/python .../tools/validate.py`: 15 sources, 15 claims, 12 transition comparisons, one figure. |
| Retained source reconstruction | `PASS` | All 12 dates and confidence intervals independently reconstructed from the eight retained Data Record 5 rows. |
| Date-delta arithmetic | `PASS` | Recomputed `gcc_90 - gcc_mean` for every 2024--2025 direction/threshold row; all signed values match. |
| Source identity | `PASS` | Downloaded archive, complete member, and retained subset SHA-256 values match `inputs/README.md`; subset is a lossless eight-row source selection. |
| Retained predecessor identity | `PASS` | CAL-07 simplified rows exactly carry `gcc_mean` dates; CAL-07/CAL-07D observation files and support artifacts explicitly carry `smooth_gcc_90`. |
| Citation fidelity | `PASS` | DOI metadata independently checked for S02 and S04--S10; S01 and S03 full-text headers checked; corrected names/titles match. |
| Claim calibration | `PASS` | All 15 claims bind known source IDs and preserve site, regional, analogue, method, unresolved, and acquisition ceilings. |
| Four solution routes | `PASS` | Observation semantics advances only to audit; forcing, ecotype transfer, and missing-process routes remain held for named evidence. |
| Figure and sidecar | `PASS` | SVG parses, rendered successfully at 1200 by 720, labels are legible, assistive title/description are present, and sidecar states caption and ancillary limits. |
| Markdown lint | `PASS` | `markdown-doc lint` passes for the package, roadmap, and catalog. |
| Diff hygiene | `PASS` | `git diff --check`, staged diff check, and write-set inventory pass. |
| Line-count governance | `PASS` | No Rust file changed; package validator is below the 2,000-line warning threshold. |

## Requirement verification

- The search, inclusion, exclusion, and scientific stop rules are explicit.
- Every claim-bearing source is recorded as full text or full data; discovery
  and acquisition leads do not authorize scientific conclusions.
- Direct-site findings are separated from Madagascar regional evidence and
  outside-Madagascar analogues.
- The provisional PhenoCam source is checksum-retained, reproducible, and not
  represented as a curated V3 release.
- The 2014 thesis, post-2011 monitoring, and camera/field correspondence
  requests are specific enough for human acquisition.
- Roadmap and catalog state the same bounded conclusion and retain Order 7.
- The exact write set contains only the CAL-07E package, canopy roadmap, and
  work-package catalog.
- No production code, science contract, forcing, parameter, or predecessor
  package was changed.

## Closure statement

All independently verifiable CAL-07E exit criteria pass. Final package-status,
prompt-archive, dual-review integration, gate-evidence, finding-disposition,
exact-diff, and final-disposition bookkeeping may be completed after both
reviewers return; those administrative updates must not broaden this verdict.
