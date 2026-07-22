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

Ran: corrected-base validation at exact code-only head `219ec924` returned
READY, no reason codes, no unauthorized paths, and package audit ID
`33039a84f9d90052eac88f1e01dbc181e4164d42f69b3b6fffd29c1f07529c7a`.
The retained audit is `/tmp/rtr029-ready-package-audit.ezphjM.json`, SHA-256
`e4ce913677564906e35312aeaae3f8c8d33d776d9f2517ffdcb485017d8a1f04`.

Ran: the one focused regression at exact code-only head `219ec924` passed 1/1
with 144 skipped in 205.964 seconds. Post-run inspection found no fixture
directory, ledger, or child process leak.

Ran: durable closure records for RTR-030 and RTR-029 have canonical entry
SHA-256 values `944fccfa9fd41353296842218de5b7cb1b34fea683494610b206dcd00456d150`
and `79e86283316748d48a02f9bf85a9ada8c4c74a07996d56ff8dae960ddfc68380`.
The ledger fold contains zero open tooling defects.
