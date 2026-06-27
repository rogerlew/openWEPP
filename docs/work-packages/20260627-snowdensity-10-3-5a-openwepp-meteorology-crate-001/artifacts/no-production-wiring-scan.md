# No Production Wiring Scan

Status: queued
Evidence mode: not-run

This package is crate-only. Record evidence that production snow/frost behavior,
selectors, schemas, and defaults did not change.

Suggested checks:

- Static: `git diff --name-only HEAD -- crates/openwepp-hillslope-orchestrator crates/openwepp-runner crates/openwepp-climate-runtime-adapter crates/openwepp-input-contract`
- Static: `rg -n "Harder|Pomeroy|psychrometric|openwepp_meteorology|openwepp-meteorology" crates/openwepp-hillslope-orchestrator crates/openwepp-runner crates/openwepp-climate-runtime-adapter crates/openwepp-input-contract`
- Static: review `Cargo.toml` dependency edges so production crates do not consume the new crate in this package.

Gate table:

| Surface | Status | Evidence |
|---|---|---|
| Production runtime wiring unchanged | NOT RUN | Pending execution. |
| Runtime/default selectors unchanged | NOT RUN | Pending execution. |
| Public output schemas unchanged | NOT RUN | Pending execution. |
| `RST` path unchanged | NOT RUN | Pending execution. |
