# INIMPL30 Merge Conflict Log

Evidence mode: `Ran` + `Static`

## Intake Summary

Wave 4 worker outputs were integrated in canonical order:
1. `INIMPL24` (`chan.inp`)
2. `INIMPL25` (`tc.txt`)
3. `INIMPL26` (`gwcoeff.txt`)
4. `INIMPL27` (`tcr.txt`)
5. `INIMPL28` (`phosphorus.txt`)
6. `INIMPL29` (`lcwb.txt`)

## Textual Merge Conflicts

`Ran: [DIRECT]` No textual merge conflicts were encountered during intake.

## Integration-Level Conflict Resolutions

`Ran: [DIRECT]` One integration-level quality-gate conflict class was resolved
post-intake:

| conflict_id | surface | symptom | resolution |
| --- | --- | --- | --- |
| `INIMPL30-C-001` | Wave 4 integration tests | `clippy -D warnings` failed because parser files were test-included via `#[path=...]`, producing `unreachable_pub` and related warnings in test targets. | Migrated Wave 4 tests to consume crate exports (`openwepp_input_contract::parsers::*`) and normalized float/cast assertions to satisfy pedantic lints. |

`Static: [DIRECT]` Shared-file follow-up requests from worker handoffs were
closed in integration-owned files:
- `crates/openwepp-input-contract/src/parsers/mod.rs`
- `Cargo.toml`
