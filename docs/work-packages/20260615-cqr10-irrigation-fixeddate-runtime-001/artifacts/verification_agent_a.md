# Verification Agent A

Status: complete-with-warnings.

Verification stance: metric and gate verification.

Ran: before and after LCOV/CRAP artifacts exist:

- `artifacts/lcov_before.info`
- `artifacts/crap_before.json`
- `artifacts/lcov_after.info`
- `artifacts/crap_after.json`

Ran: after CRAP proves:

- `seed_hillslope_runtime_surface_from_irrigation_fixeddate` CRAP `4.0`.
- maximum newly extracted fixed-date helper CRAP
  `14.218480996665143`.
- out-of-scope depletion row remains CRAP `1122.0`.

Ran: required closure gates passed:

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
- `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr10-irrigation-fixeddate-runtime-001 --format json`
- `git diff --check`

Gate Evidence Non-Deferral: satisfied. No gate was deferred or substituted.

Result: PASS with WARN holds for coverage threshold and out-of-scope depletion
CRAP debt.
