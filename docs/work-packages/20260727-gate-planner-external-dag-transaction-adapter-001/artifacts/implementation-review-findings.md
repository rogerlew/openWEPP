# Implementation Review Findings

Status: `HOLD / CORRECTION IN PROGRESS`

Evidence class: `Ran + Static`

Exact reviewed commit: `08205b57`.

| ID | Severity | Finding | Required correction |
|---|---|---|---|
| GED-R01 | Critical | External READY uses ten literal PASS rows | Execute canonical independent audit checks; no second policy implementation |
| GED-R02 | Critical | Clean commits share one status-only identity | Bind exact HEAD/tree/diff, sources, executables, toolchains, environment |
| GED-R03 | Critical | Final-root verification invalidates earlier manifests | Verify bound historical scope/bytes while allowing declared downstream outputs |
| GED-R04 | Critical | No executable Generation B | Deterministically bind calibration/freeze/two verifier receipts before holdout |
| GED-R05 | Critical | Capability protocol conflicts and attestations are self-referential | One compatible single-use protocol; verify actual receipt/script/argv/freeze bytes |
| GED-R06 | High | Real custody CLI options rejected | Admit and test exact options |
| GED-R07 | High | `env_clear` makes Cargo undiscoverable | Bind and install exact toolchain environment or absolute executable |
| GED-R08 | High | Science-equivalence gate uses fabricated CSV | Dual-run real reconstruct/verify/readiness fixtures |
| GED-R09 | High | Publication trusts caller IDs/hashes | Read and independently verify the exact producing receipt |
| GED-R10 | Critical | Harvard token precedes pre-spawn fallible work | Create nofollow/fsynced token immediately before first Harvard open |
| GED-R11 | Critical | External STARTED has no orphan reconciliation | Balance crash-orphan lifecycle with typed terminal record |
| GED-R12 | Critical | Root/publication checks are TOCTOU-prone | Descriptor-relative nofollow operations and immediate pre-rename baseline check |
| GED-R13 | Medium | Exact diff hygiene claim was false | Remove trailing EOF blank lines and rerun exact-base diff check |
| GED-R14 | Critical | Self-ID JSON plan is not reconstructed from frozen CSV authority and may come from `/tmp` | Require committed repo plan and independently prove exact 18-row CSV/contract parity |
| GED-R15 | Critical | Transaction verifier trusts receipt-selected argv, roots, claims, and manifests | Reconstruct plan expansion and verify prerequisite IDs, claims, attempt/root/ledger, live identities, and confined manifests |
| GED-R16 | High | Exact reconstruction exposed prose/incomplete CSV output authority, then edits began outside declared scope | Stop edits, prospectively authorize only the two exact CSVs for producer-proven output canonicalization, obtain dual review |

The existing focused PASS results demonstrate regression stability only. They
do not override these semantic consumer-path failures.
