# Measurement Review A

Evidence class: Static / Ran.

Reviewer: independent read-only measurement reviewer A.

Initial disposition: `BLOCK`.

Ran:

- Python compilation: `PASS`.
- collector self-test: `PASS`.
- focused Nextest contract: `5/5 PASS`.
- independent inventory partition: disjoint union `PASS`.

Initial closure-blocking findings:

1. execution used mutable source across admission and collection;
2. the pre-heavy report asserted checks without complete mechanical evidence;
3. raw profile sets were indexed and later re-globbed without hash
   revalidation;
4. zero science coverage was mislabeled legitimately uncovered;
5. terminal verification trusted producer inventories;
6. the new ledger test name entered `science-manual`;
7. the pre-existing roadmap edit escaped the Order 3 write set.

Corrections were requested before heavy execution. The roadmap update was
preserved in predecessor commit `f0da06bb`; the implementation was then revised
for read-only execution snapshots, evidence-backed admission, exact raw-set and
row identity, independent enumeration, and profile-safe test naming.

Terminal rereview: `PASS`.

Final Static / Ran assessment:

- admitted executable hashes are revalidated after `full`, after
  `science-manual`, and after LCOV derivation;
- LIGHT, audit, and HEAVY admission remain in one `transition` process;
  standalone `collect` is rejected;
- prior snapshot, raw-input, snowbench, independent-inventory, write-set, and
  profile-routing corrections remain intact;
- Python compilation, collector self-test, focused Nextest `5/5`,
  `git diff --check`, and exact write-set validation passed.

Heavy-readiness: `PASS`. No remaining implementation blocker.

Tooling-defect-01 retry rereview: `PASS` at attempt-2 admission; superseded by
tooling defect 02.

That intermediate correction kept control and source paths read-only while
fixture files and their temporary copies remained writable. Attempt 2 later
proved its directory-freeze premise incomplete.

Tooling-defect-02 rereview: `PASS`.

Attempt 2 proved directory freezing incompatible with valid repository-relative
scratch. The current writable no-hardlink clone retains exact tracked,
nonignored-untracked, index, source-manifest, evaluator-link, and executable
identity checks at every profile and LCOV boundary. A fresh attempt is
authorized after zero-open finding disposition.
