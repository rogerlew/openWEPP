# HPHYS0257 Disposition

Status: completed/HOLD

Evidence mode: mixed

## Decision

- Static: package execution is complete.
- Static: disposition is `HOLD` for overall hillslope water-balance semantic
  parity.

## Basis

- Static: HPHYS0257 identified and corrected a real hourly WB19 authority gap:
  modern hourly lateral lanes now consume required `wb19_lateral_ssh_####`
  projected from layer `ui_anisrt`/`ui_ssh` lineage.
- Ran: contract-derived red tests failed before production edits and passed
  after implementation.
- Ran: `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, `cargo deny check`, and required authority guards
  passed.
- Ran: H1/H7/H39 and full H1..H39 diagnostics completed.
- Ran: full H1..H39 semantic pass remains `0/39`.
- Ran: semantic summary improves versus HPHYS0256 for `latqcc`, `Dp`, and
  aggregate storage, but does not close parity.

## Continuation

- Static: do not reopen horizontal-conductivity lineage unless new evidence
  contradicts the HPHYS0257 contract vectors.
- Static: next work-package should focus on the hourly WB19 cap and
  publication lineage: `tdvv`, frozen-adjusted thresholds, top-down
  withdrawal, `latqcc` accumulation, `Qd`/WB13 publication, and aggregate
  storage reconciliation.
