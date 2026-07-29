# CAL-07C Finding Disposition

Evidence class: `Static + Ran`

## Prospective review A

| Finding | Disposition | Evidence |
| --- | --- | --- |
| Authority-boundary wording overstated CAL-07B if read as full-period positivity. | `accepted/fixed` | `package.md` now limits CAL-07B positivity to the three CAL-07 blocker dates and records 349 full-period negative hourly components. |
| Negative hourly rows must remain visible. | `accepted/fixed` | `artifacts/admission-table.csv`, `artifacts/hourly-vpd-reconstruction.csv`, `cal07c-vpd-reconstruction-audit.svg`, and all sidecars retain the negative-hour claim ceiling. |

## Prospective review B

| Finding | Disposition | Evidence |
| --- | --- | --- |
| CAL07C-PRB-001: prove Alerce consumes admitted `vpd_pa` and not rejected daily-summary VPD. | `accepted/fixed` | `artifacts/executor-path-proof.md`; `tools/validate.py`; validation PASS with max VPD residual `0.000e+00 Pa` and explicit rejected-date assertion. |
| CAL07C-PRB-002: complete source/admission custody before execution. | `accepted/fixed` | `artifacts/source-manifest.csv`, `artifacts/dependency-manifest.csv`, `artifacts/admission-table.csv`, and `artifacts/source-admission.md`. |
| CAL07C-PRB-003: do not imply full-period hourly products are positive. | `accepted/fixed` | `package.md`, `artifacts/science-summary.md`, roadmap/catalog updates, and figure sidecars retain the 349 negative-hour statement. |
| CAL07C-PRB-004: figures and sidecars must expose negative hourly evidence and authority boundary. | `accepted/fixed` | Four SVG figures and Markdown sidecars in `artifacts/figures/`; sidecars bind source/admission hashes, operators, LST units, no-clipping rule, POWER grid limitation, and no-OBL-replacement boundary. |

No accepted prospective finding remains open.
