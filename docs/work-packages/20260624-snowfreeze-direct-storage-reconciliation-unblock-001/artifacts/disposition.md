# Disposition

Status: complete

Evidence mode: Static + Ran.

Final disposition: COMPLETE.

Closed defects:

- `SNOWFREEZE-DRSTOR-001`: `site3_scan_mandan_nd` no longer fails at
  `storage_reconciliation.frost_storage_projection_theta_m`; it exits `0` and
  emits a metric-bearing `UNRESOLVED` report with `10643` matched rows.
- `SNOWFREEZE-DRSTOR-002`: `site4_ggd498_morris_mn` no longer fails at
  `storage_reconciliation.frost_storage_projection_theta_m`; it exits `0` and
  emits a metric-bearing `UNRESOLVED` report with `83` matched frost-depth
  residual rows.

The production fix stayed inside the declared R4B explicit frost storage
projection envelope. It does not change frost physics, observation thresholds,
direct activation policy, compatibility runtime, or rollback/shadow paths.

Final gates passed:

- focused R4B regression tests;
- site3/site4 pre-fix reproduction and post-fix observed comparisons;
- `cargo build -p openwepp-runner --bin openwepp-cli-hill`;
- `cargo test --test snowfreeze_observed_frost_depth_contract`;
- `cargo fmt --check`;
- `git diff --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `cargo deny check`;
- dual delegated review with accepted findings fixed.
