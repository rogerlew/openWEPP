# CRAP Classification Ledger

Baseline evidence is the predecessor's immutable
[`gate-results.md`](../../20260717-testgate-plan-shadow-planner-001/artifacts/gate-results.md),
which records the historical 14 raw / 2 adjudicated / 12 actionable report and
its SHA-256. All twelve rows are production paths (`E-PRODUCTION`); no exclusion
or adjudication is requested. The mutable `target/adjudicated-crap/` path now
contains the cleanup report and is not used as baseline identity.

| Source | Function | Baseline CC | Baseline coverage | Baseline CRAP | Treatment |
|---|---|---:|---:|---:|---|
| `ledger.rs` | `verify_predecessor` | 8 | 24.00% | 36.094 | Decompose predecessor identity, campaign, and CAS checks; retain fail-closed cases. |
| `ledger.rs` | `verify_authorizations` | 9 | 25.00% | 43.172 | Isolate exact authorization binding from the unauthenticated-authority rejection. |
| `ledger.rs` | `verify_certification_references` | 7 | 21.875% | 30.365 | Isolate reference lookup and binding predicate. |
| `ledger.rs` | `verify_assurance_replacements` | 14 | 40.00% | 56.336 | Separate replacement compatibility from cycle traversal. |
| `main.rs` | `write_plan_confined` | 13 | 0.00% | 182.000 | Add direct confinement/atomic-write contract tests and split parent validation, reservation, and persistence. |
| `planner.rs` | `reconcile_semantics` | 15 | 58.73% | 30.815 | Extract changed-path parsing and authorization checks. |
| `planner.rs` | `manifest_object_identity` | 8 | 29.03% | 30.875 | Split symlink identity from metadata dispatch. |
| `planner.rs` | `cargo_configuration_manifest` | 11 | 38.46% | 39.198 | Split candidate discovery and per-file record validation. |
| `verifier.rs` | `authority_outcome_accepted` | 9 | 31.25% | 35.321 | Separate blocking and advisory scientific outcome predicates. |
| `verifier.rs` | `verify_envelope` | 34 | 78.72% | 45.134 | Mandatory decomposition into subject, provenance, bundle, identity, and issuer-authority checks. |
| `verifier.rs` | `verify_reuse` | 24 | 39.47% | 151.719 | Separate identity, currency, and per-node verification. |
| `verifier.rs` | `verify_node_reuse` | 14 | 0.00% | 210.000 | Add direct reuse-class contract coverage and split trust from class dispatch. |

Tests are limited to externally meaningful branch contracts: confined output,
exact reuse identity/class behavior, and retained fail-closed ledger/planner
semantics. Helper extraction is preferred for bookkeeping branches.
