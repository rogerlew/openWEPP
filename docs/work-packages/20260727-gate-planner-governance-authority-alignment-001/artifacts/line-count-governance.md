# Line-count Governance

Evidence class: `Ran`.

The changed Rust sources are integration guards:

- `tests/integration/testgate_align_authority_contract.rs`: 258 lines, below
  the 2,000-line warning threshold.
- `tests/integration/snowdensity03_physics_bulk_offline_contract.rs`: 142
  lines; its obsolete deleted-guard allowlist entry was removed.
- `tests/integration/testgate_ci_executor_contract.rs`: deleted; it was 1,303
  lines.

No changed or added `.rs` file reaches the warning or required-refactor
threshold. Disposition: `PASS`, with no decomposition note or exception
required.
