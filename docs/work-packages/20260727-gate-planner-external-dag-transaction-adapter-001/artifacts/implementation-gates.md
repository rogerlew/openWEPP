# Implementation Gates

Status: `FOCUSED PASS / IMPLEMENTATION REVIEW REQUIRED`

Evidence class: `Ran + Static`

Integrated worktree gates:

| Gate | Result |
|---|---|
| `git diff --check` | PASS |
| `cargo fmt --all -- --check` | PASS after canonical formatting |
| `cargo nextest run -p openwepp-gate-planner` | PASS, 190/190; 14 skipped; run `98fe4470-9d07-46bb-a838-e4dc1e196785` |
| gate-planner all-target/all-feature Clippy with `-D warnings` | PASS |
| CAL-04B Python `test_*.py` discovery | PASS, 23/23 |
| CAL-04B executor `cargo test` | PASS, 22/22 |
| `cargo deny check` | PASS |
| authority-suite source anti-evasion | PASS |
| `auth11_required_suite_obligation_guards_contract` | PASS, 3/3; run `5d18a4c4-bc80-4bda-9ef0-8ae2ff102a13` |

No population command or Harvard read occurred.

Implemented controls include canonical same-process external transitions,
STARTED-before-validation with balanced terminal lifecycle, exhaustive confined
output manifests, independent verification, journaled publication/recovery,
two-generation custody admission, external execution-root injection, and
path-only CAL executor changes.

The Generation-A plan intentionally has empty holdout custody bindings.
Execution must reject `holdout-v1` until post-freeze Generation B binds the
exact calibration receipt, freeze receipt, two distinct verifier attestations,
and capability identities.

