# PERFDEEP09 Artifacts

Status: executed - `READY-FOR-R2`.
Evidence class: Static + Ran.

PERFDEEP09 closed `PERFDEEP09-DISABLED-PATH-R2-BLOCKER` by retaining the
one-pass perennial decomposition indexed-overflow guard.

Key evidence:

- No-edit same-machine control: `682.65 s`, RSS `228924 KB`.
- Retained final reps: `634.61 s`, `635.65 s`, `636.58 s`; median
  `635.65 s` (`<= 676.67 s`).
- Rejected candidate: private `SymbolRegistry` reverse lookup `HashMap`;
  `689.30 s`, RSS `229352 KB`, and PASS parquet raw checksum drift.
- Protected identity: HBP, loss, WAT, and plot byte checks passed; PASS parquet
  passed Arrow/DuckDB row equivalence (`left_minus_right=0`,
  `right_minus_left=0`).
- Full gates passed: `cargo fmt --check`, `cargo clippy --workspace
  --all-targets -- -D warnings`, `cargo test --workspace`, `cargo deny check`,
  and `git diff --check`.

Artifact details are split by topic in this directory.
