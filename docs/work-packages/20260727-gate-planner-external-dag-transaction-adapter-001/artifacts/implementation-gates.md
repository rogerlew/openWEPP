# Implementation Gates

Status: `CORRECTION PASS / DUAL RE-REVIEW REQUIRED`

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

Independent review at `08205b57` rejected the implementation despite passing
focused tests. The reviewed consumer paths exposed unauthenticated synthetic
READY construction, insufficient source/toolchain identity, nonverifiable
historical manifests, absent Generation B, incompatible custody capabilities,
dead CLI options, empty-environment Cargo failure, simulated rather than real
science-equivalence tests, unauthenticated publication, premature Harvard token
creation, unhandled external orphan STARTED records, and filesystem TOCTOU.
These are open implementation defects; no heavy work is authorized until
correction and dual re-review.

Correction gates on the integrated worktree:

| Gate | Result |
|---|---|
| gate-planner Nextest | PASS, 199/199; 14 skipped; run `62860f41-5a98-415f-8a00-b550fedc25f8` |
| gate-planner all-target/all-feature Clippy with `-D warnings` | PASS |
| CAL Python discovery | PASS, 26/26 |
| CAL executor Rust tests | PASS, 22/22 |
| CAL executor warnings-denied Clippy | PASS |
| exact-base and worktree diff hygiene | PASS |
| Rust formatting and all gate-policy JSON parsing | PASS |
| exact dual-CSV reconstruction | PASS, plan `64a95a1e878bfa2bd71436159de6874b13074c5f71be1b116f5a3c281bcde9a4` |
| real LIGHT -> evaluated READY -> HEAVY subprocess fixture | PASS |
| descriptor-relative publication/race fixtures | PASS |
| real reconstruct/verify/readiness dual-root equivalence | PASS |

All `GED-R01` through `GED-R16` have implemented corrections and passing
focused consumer/adversarial evidence. Independent re-review remains mandatory
before the prerequisite can close or any CAL population work can begin.

Implemented controls include canonical same-process external transitions,
STARTED-before-validation with balanced terminal lifecycle, exhaustive confined
output manifests, independent verification, journaled publication/recovery,
two-generation custody admission, external execution-root injection, and
path-only CAL executor changes.

The Generation-A plan intentionally has empty holdout custody bindings.
Execution must reject `holdout-v1` until post-freeze Generation B binds the
exact calibration receipt, freeze receipt, two distinct verifier attestations,
and capability identities.
