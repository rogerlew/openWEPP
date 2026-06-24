# Review Disposition

Evidence class: Static/Ran.

Status: complete.

Required reviews:

1. Data-provenance and licensing review.
2. Harness/contract review.

| Finding | Reviewer | Severity | Disposition | Resolution |
| --- | --- | --- | --- | --- |
| Reynolds Creek was incorrectly left blocked through the HydroShare metadata path even though the package identified the USDA-ARS/Data.gov route. | data provenance | High | accepted | Harness now fetches Data.gov/Figshare `soiltemperature.zip`, normalizes station 127, and records 4,356 site5 rows. |
| Normalized outputs were not checksum-locked. | data provenance | Medium | accepted | Manifest and provenance now include normalized CSV byte counts and SHA-256 checksums; `validate` checks them offline. |
| License/terms provenance was too generic. | data provenance | Medium | accepted | Source records now carry source-specific terms; Reynolds cites the included public-domain `license.txt`. |
| Newly generated corpus was untracked at review time. | data provenance | Medium | accepted | Corpus remains under `tests/fixtures/snowfreeze_observed/observations/` for commit inclusion. |
| Compare harness hardwired `--compatibility-runtime`. | harness/contract | High | accepted | `compare` defaults to `--direct-production-executor`; compatibility is only explicit `--runtime compatibility`. |
| Soil-temperature isotherm rows were reduced to direct residuals. | harness/contract | High | accepted | Metrics now separate frost-tube residuals from isotherm upper-bound checks. |
| Censoring was recorded but not honored. | harness/contract | High | accepted | Non-`none` censoring rows are counted and excluded from magnitude/upper-bound residual metrics. |
| Required timing metrics were incomplete. | harness/contract | Medium | accepted | Reports now include observation-date seasonal onset, thaw, and frozen-duration summaries. |
| WAT date alignment lacked duplicate guard coverage. | harness/contract | Medium | accepted | WAT loader now rejects duplicate modeled calendar dates instead of silently overwriting them. |

Residual follow-up:

- Direct-runtime storage-reconciliation failures block comparison for sites 3 and 4.
- Modeled snow depth remains unavailable, so observation disagreements stay
  `UNRESOLVED`, not `OPENWEPP-DEFECTIVE`.
