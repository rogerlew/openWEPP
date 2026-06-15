# CQR15 Line-Count Governance Checklist

Status: complete with WARN.

Static: line counts:

- Before production edit:
  `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs`
  had `2124` lines.
- After production edit:
  `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs`
  has `2371` lines.
- `docs/work-packages/README.md` has `562` lines after CQR15 registration.

Static: WARN. The touched Rust target file remains above the ADR-0021 `2000`
line advisory threshold and grew by `247` lines due helper extraction plus
focused tests elsewhere. This package did not split the file because the CQR
scope is behavior-preserving CRAP burn-down for one function and broad file
reorganization was explicitly out of scope.

Static: no touched non-exempt Rust file is at or above the `3000` line hard
threshold.

Static: the target `too_many_lines` suppression was removed. A pre-existing
out-of-scope `too_many_lines` suppression remains later in the same file.
