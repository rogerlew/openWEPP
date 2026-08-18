# Comparator Heavy Run v6

Verdict: HOLD

Canonical per-command evidence is the nine `commands/*.meta` files and their
matching raw logs. Eight commands passed and one failed.

- Seven required nonzero benchmark surfaces: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo nextest run --workspace --profile full`: FAIL; 2,990 tests ran,
  2,974 passed, 16 failed.

The earlier generated aggregate summary duplicated rows and counters. This
reconciled summary replaces that corrupt aggregation; raw logs and individual
metadata are unchanged.
