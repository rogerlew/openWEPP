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
