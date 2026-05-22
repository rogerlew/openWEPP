# ARCH21 Open Blockers and Follow-Ons

Status: `complete`
Evidence mode: `Static + Ran`

| blocker_id | source_finding | severity | blocker_statement | owner | required_follow_on_wp | unblock_criteria |
|---|---|---|---|---|---|---|
| `BLK-CRF006-GATE-FMT` | `CRF-006` | `high` | Required ARCH21 ratification gate set is not all green because `cargo fmt --check` fails on `tests/integration/infile_hbp_parser_contract.rs`. | `ARCH/KERNEL + HBP owners` | `ARCH18 hold-lift replay` | File formatted and all four required ARCH21 gates pass in one replay. |
| `BLK-CRF007-RUN-PRQ` | `CRF-007` | `medium` | `.run` and parquet boundaries remain `complete-with-hold` with open `RUN-HOLD-*` and `PRQ-HOLD-*` items. | `CONTRACT + runtime/output owners` | `ARCH19-F01..F05` | Close or explicitly risk-accept `RUN-HOLD-001..003` and `PRQ-HOLD-001..003` with evidence. |

## Non-Blocking Follow-Ons

| follow_on_id | source_finding | severity | statement | owner | target |
|---|---|---|---|---|---|
| `FO-CRF004-PURITY` | `CRF-004` | `medium` | Trait mutability/purity contract alignment remains queued as amendment scope. | `ARCH/KERNEL owners` | follow-on contract/ADR package |
| `FO-CRF010-COVERAGE` | `CRF-010` | `medium` | ARCH17 closure is representative; additional parser-family seam coverage is still open. | `INPUT + ORCHESTRATOR owners` | seam coverage follow-on package |
