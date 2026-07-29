# CAL-07B Terminal Verification Agent B

Evidence class: `Static`

Final verification result: `GO`

## Static checklist

| Check | Result | Evidence |
| --- | --- | --- |
| Package remains diagnostic-only and excludes production physics, CAL-07 input edits, clipping, normalization, and operator replacement. | PASS | `package.md`, `final-disposition.md` |
| Frozen cases, POWER grid point, LST time standard, hourly parameters, and one fixed hourly URL per case are declared. | PASS | `package.md`, `source-manifest.csv` |
| Source custody records retained hourly responses and POWER documentation with path, SHA-256, byte count, retrieval timestamp, and URL. | PASS | `source-manifest.csv`, `gate-evidence.md` |
| CAL-07 daily source is retained by reference and digest rather than reacquired or replaced. | PASS | `source-manifest.csv`, `gate-evidence.md` |
| Daily decomposition has the three frozen dates, 24 hours per case, no hourly negative VPD rows, negative reconstructed contract-daily VPD, and negative CAL-07 contract-daily VPD. | PASS | `daily-decomposition.csv`, `science-summary.md` |
| Daily operand reconstruction stays within the frozen `<= 0.01 C` serialized-resolution tolerance. | PASS | `daily-decomposition.csv`, `science-summary.md` |
| Additive decomposition closes within the frozen `1e-9 Pa` gate. | PASS | `daily-decomposition.csv`, `science-summary.md` |
| Attribution table publishes required primitive predicates before assigning labels. | PASS | `attribution.csv`, `package.md` |
| All three cases are classified as `DAILY_SUMMARY_OPERATOR_MISMATCH` with `AGGREGATE_OVERLAP_ONLY` lineage, avoiding identical-processing overclaim. | PASS | `attribution.csv`, `science-summary.md` |
| Figure Markdown sidecars bind their source CSVs by SHA-256 and the digests match `result-manifest.csv`. | PASS | `artifacts/figures/*.md`, `result-manifest.csv` |
| Figure sidecars state methods, limitations, and accessibility notes for the hourly, decomposition, and source-reconstruction figures. | PASS | `artifacts/figures/*.md` |
| Roadmap and work-package catalog retain CAL-07/Order 7 hold language and do not treat CAL-07B as production readiness or CAL-07 resumption. | PASS | `docs/planning/canopy-phenology-assurance-roadmap.md`, `docs/work-packages/README.md` |
| Gate evidence reports independent validation, XML/SVG rendering checks, Markdown lint, and diff hygiene. | PASS | `gate-evidence.md` |

## Verification disposition

`GO`: Static verification found no PASS/FAIL checklist failure for Agent B's
terminal review scope. This verification covers the retained evidence and
claims; it does not rerun the package tools or replace the independent peer
terminal review/verification.
