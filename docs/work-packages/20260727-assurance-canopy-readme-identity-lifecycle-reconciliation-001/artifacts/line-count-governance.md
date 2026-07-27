# Line-Count Governance

Status: `PASS WITH WARN`

Evidence class: `Ran + Static`

Terminal counts:

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-assurance/src/v2/amendment.rs` | 2,592 | WARN, below 3,000 required-refactor threshold |
| `crates/openwepp-assurance/src/cli.rs` | 1,413 | PASS |
| `tests/integration/assurance_v2_amendment_contract.rs` | 1,012 | PASS |

`amendment.rs` remains the transaction/lifecycle coordinator for a tightly
coupled confined-generation protocol. This package added one bounded operation
and reused its shared transaction machinery. Splitting that machinery during
the assurance repair would increase rollback and identity-regeneration risk.
Follow-on split intent: extract report-source adoption and its validation
helpers into a dedicated sibling module when the amendment protocol next gains
an operation or the file approaches 3,000 lines. Owner: openWEPP assurance
maintainers. The file is not exempt from the 3,000-line closure threshold.
