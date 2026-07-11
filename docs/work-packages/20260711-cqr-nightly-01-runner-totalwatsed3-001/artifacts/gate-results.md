# Gate Results

| Gate | Status | Evidence |
|---|---|---|
| `cargo check -p openwepp-runner` | PASS | Attempt exit `0`. |
| Focused nextest | PASS | Attempt `6/6`; `characterization.md`. |
| Focused LCOV/CRAP | FAIL | Attempt metrics improved, but cover-first and eligible retained-row gates failed review. |
| Focused all-target clippy | PASS | Attempt exit `0`. |
| Delegated `cargo fmt --check` | PASS | Exit `0`, `1.940s`; `/tmp/openwepp-cqr-20260711-t01-closure/01-*`. |
| Delegated workspace clippy | PASS | Exit `0`, `7.736s`; `/tmp/openwepp-cqr-20260711-t01-closure/02-*`. |
| Delegated full workspace nextest | NOT RUN TO COMPLETION | Deliberately terminated after local-hold decision: `1670` passed, one SIGTERM, `23` not run; not closure evidence. |
| `cargo deny check` | NOT RUN | Not required after implementation rollback/local hold. |
| `git diff --check` | PASS | Exit `0` after rollback/docs updates. |
| Package/catalog docs lint | PASS | Canonical scoped command, exit `0`: `23` files, `0` errors/warnings after final hold reconciliation. |

No external-authority suite posture, cohort fixture, or required-case binding
was edited; anti-evasion gates are not applicable.
