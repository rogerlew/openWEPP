# Eligibility Classification

Static: source SHA-256 is
`476b78dc855e9626a170406b85fc1fec3563aae6b00cab8a97003521554c67bc` for every
row. All are hand-authored admission, validation, ordering, identity, or
fail-closed guard logic. Under ADR-0021 they are `E-PRODUCTION`; each retains
aggregate, 75%-floor, and CRAP treatment. No exception is proposed.

| Symbol | Line | CRAP | CC | Coverage | Class | Evidence/disposition |
| --- | ---: | ---: | ---: | ---: | --- | --- |
| `build_audit` | 80 | 506 | 22 | 0% | E-PRODUCTION | audit construction; actionable |
| `failure_check_index` | 295 | 38.9581 | 19 | 61.9048% | E-PRODUCTION | error-to-check precedence; actionable |
| `validate_audit` | 326 | 650 | 25 | 0% | E-PRODUCTION | READY admission validation; actionable |
| `validate_audit_for_execution` | 397 | 42 | 6 | 0% | E-PRODUCTION | executor binding; actionable |
| `light_stage_passed` | 742 | 42 | 6 | 0% | E-PRODUCTION | LIGHT pass guard; actionable |
| `cheap_prerequisites` | 757 | 182 | 13 | 0% | E-PRODUCTION | hygiene/prompt/line guards; actionable |
| `inventory_and_arguments_are_exact` | 821 | 72 | 8 | 0% | E-PRODUCTION | inventory reconstruction; actionable |
| `execution_identities` | 945 | 110 | 10 | 0% | E-PRODUCTION | identity/claim guards; actionable |
| `light_attempt_isolated` | 998 | 182 | 13 | 0% | E-PRODUCTION | checkpoint/artifact guards; actionable |
| `valid_stage_order` | 1053 | 90 | 9 | 0% | E-PRODUCTION | dependency order guard; actionable |
| `no_open_tooling_defect` | 1176 | 51.9708 | 9 | 19.0476% | E-PRODUCTION | durable defect guard; actionable |
| `validate_combined_decision` | 1213 | 210 | 14 | 0% | E-PRODUCTION | quality DAG guard; actionable |
| `validate_stage_receipt` | 1247 | 240 | 15 | 0% | E-PRODUCTION | receipt binding guard; actionable |

Static: independent selection reviews accepted this exact ledger after the
initial CC transcription correction; see the two target-selection review files.
