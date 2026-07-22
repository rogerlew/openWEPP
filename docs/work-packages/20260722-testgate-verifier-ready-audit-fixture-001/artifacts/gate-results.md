# Gate Results

Ran: `cargo fmt --all -- --check` passed.

Ran: `cargo nextest run -p openwepp-gate-planner
ready_audit_verification_preserves_order_and_exact_verdict --no-fail-fast`
passed 1/1 with 144 skipped in 207.468 seconds. The command compiled the
corrected test, constructed the complete isolated staged fixture, and exited 0.

Ran: post-test inspection found no
`openwepp-gate-executor-verifier-ready-audit-*` directory and no
`target/verifier-ready-audit-ledger-*` file. The unrelated pre-existing
`/tmp/owg-crap-8KgRIN` path was observed and was not modified.

Static: `git diff --check` passed. Repository-wide `markdown-doc lint` checked
17,730 files and reported only 17 pre-existing broken-link errors outside both
active packages; it reported no warning or error in their files.

Ran: warnings-denied all-target Clippy exposed two correction-owned
`needless_borrow` warnings and one pre-existing `too_many_lines` warning in an
unchanged verifier test. The two owned warnings were corrected. Ran:
warnings-denied library Clippy then passed. The unchanged historical test lint
is not attributed to RTR-029 and is not hidden with a new allow.

Ran: package validation from the original scaffold base and the first write-set
correction base both failed closed with `BASE_WRITE_SET_SCHEMA_INVALID`. The
first package authority omitted `executor.rs`; the second still encoded the
external durable ledger as a repository path. These are retained RTR-030
evidence, not passing audits. A new valid package base and code-only head are
required before closure.
