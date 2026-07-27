# Line-Count Governance

Status: `PASS WITH WARN`

Evidence class: `Ran + Static`

Terminal implementation counts:

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-gate-planner/src/policy.rs` | 1,000 | PASS |
| `crates/openwepp-gate-planner/src/executor.rs` | 2,986 | WARN, below the 3,000-line required-refactor threshold |
| `tests/integration/testgate_assure_campaign_currency_contract.rs` | 360 | PASS |
| `gate-policy/v1/README.md` | 81 | PASS |

The `executor.rs` change is confined to the existing isolated policy fixture:
it copies the two canonical assurance schemas and constructs a complete
identity-bound generated lock. Refactoring the executor during this gate-policy
repair would broaden production risk. Follow-on owner: TESTGATE maintainers;
split the next independently changing fixture/executor domain before this file
crosses 3,000 lines. This is a warning, not an exemption.
