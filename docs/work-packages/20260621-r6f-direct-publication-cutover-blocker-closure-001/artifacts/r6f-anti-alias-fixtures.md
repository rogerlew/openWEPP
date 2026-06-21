# R6F Anti-Alias Fixtures

Status: executed-held.

## Fixture Ledger

| Fixture/test | Output family | Alias rejected | Direct operand varied | Expected failure if aliased | Status |
|---|---|---|---|---|---|
| `r6f_publication_capture_accepts_typed_process_inputs_and_carries_layers` | WAT direct runtime operands | zero/default skeleton operands; no layer carry | PMET `soil_evaporation_m`, layer `theta_m`, profile projection operands | Would not publish `Es=10 mm`, storage `190 mm`, profile fields, or carried layer `theta_m=0.190`. | Passed. |
| `r6f_cutover_candidate_hbp_identity_exposes_wat_producer_gap` | HBP/WAT | HBP default zero `peakro`; WAT false parity from accepted fields only | near-zero runoff HBP operands; WAT accepted vs missing fields | Would fail HBP identity or fail to expose the WAT reduced mismatch. | Passed. |
| `r6_direct_publication_cutover_cli_flag_reaches_hbp_identity_then_fails_wat_producer_authority` | Public writes | compatibility output write-through under direct flag | direct cutover marker and output absence | Would write public files before WAT producer authority exists. | Passed. |

## Remaining Fixture Gaps

- PASS Parquet fixture coverage remains blocked behind WAT producer authority.
- Nonzero `Ep`/`Er`, snow/frost storage, and profile anti-tautology fixtures
  remain R6G/R6 continuation work.
- Manifest checksum/provenance anti-alias fixture remains blocked behind WAT
  and later output-family parity.
