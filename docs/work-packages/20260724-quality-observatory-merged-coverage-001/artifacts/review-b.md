# Measurement Review B

Evidence class: Static / Ran.

Reviewer: independent read-only measurement reviewer B.

Initial disposition: `BLOCK`.

Ran:

- collector self-test: `PASS`;
- `git diff --check`: `PASS`;
- downloaded run `30113946779` and independently reconstructed the historical
  snowbench rows: exact report SHA-256 and all seven fields for all 18 rows
  matched the package ledger.

Initial closure-blocking findings:

1. admitted Nextest config and build inputs were incompletely revalidated;
2. terminal verification did not independently enumerate inventories;
3. snowbench matching omitted crate/line/complexity and asserted unsupported
   legitimacy;
4. compact CRAP rows and identity fields were under-validated;
5. verification mutated published evidence and followed symlinks;
6. envelope/run-status policy fields were under-validated;
7. the ledger test name contaminated `science-manual`;
8. the roadmap edit was outside the package write set.

Corrections were requested before heavy execution. The current implementation
adds exact admission/config/collector/ledger/executable bindings, read-only
snapshot execution and terminal verification, independent profile enumeration,
exact compact row reconstruction, no-follow publication checks, and profile-safe
test naming.

Terminal rereview: `PASS`.

Final Static / Ran assessment:

- source/config/toolchain/ledger/collector/inventory/build drift rejects before
  HEAVY;
- one-process transition and post-profile/post-LCOV executable checks pass;
- exact raw duplicate handling, full admission binding, independent compact
  CRAP adjudication reconstruction, and exact snowbench proof pass review;
- publication verification is canonical, no-follow, read-only, and exact-shape;
- Python compilation, collector self-test, focused Nextest `5/5`,
  warnings-denied focused Clippy, and `git diff --check` passed.

Heavy-readiness: `PASS`. No remaining implementation blocker.

Tooling-defect-01 retry rereview: `PASS` at attempt-2 admission; superseded by
tooling defect 02.

The intermediate implementation hashed index-listed worktree bytes and exact
untracked regular files directly, explicitly required and bound the `.venv`
symlink target, and checked identity after every profile and around every LCOV
derivation. Attempt 2 later proved its directory-freeze premise incomplete.

Tooling-defect-02 rereview: `PASS`.

The complete attempt-2 inventory established that directory permissions, not
test correctness, caused the 26 failures. The current clone has no chmod path,
rejects non-owner-writable directories before admission, and preserves exact
identity guards across execution and coverage derivation. A fresh attempt is
authorized after zero-open finding disposition.
