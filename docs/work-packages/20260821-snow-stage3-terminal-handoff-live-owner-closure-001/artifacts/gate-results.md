Status: complete
Evidence mode: Ran/Static

Ran: `cargo fmt --all -- --check` — PASS.
Ran: affected crate `cargo check` — PASS.
Ran: affected-crate full nextest — 994 passed, 24 skipped.
Ran: focused Stage-3/auth tests — 12 passed.
Ran: authority anti-evasion — PASS.

Static/Ran: Workspace quick and exact-head full profiles also exercised the
repository's pre-existing assurance authority drift. They fail before or
alongside this package because the tracked `assurance/v2/identity.lock.json`
hashes do not match the checked-in SC-SNOWENERGY/SC-SNOWFREEZE files, and
several retained authority tests assert older contract versions. No authority
file or identity lock was changed by this package; the affected-crate full gate
is the valid package-scoped heavy gate.
