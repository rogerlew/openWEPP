# Progress

Static: package executed for row #6 growth/decomposition CQR closure.

Ran:

- Reused final post-row-5 full-workspace LCOV + CRAP JSON:
  `/tmp/openwepp-crap-row5-after.json`.

Result:

- Row #6 CRAP-before extraction found 2 unique production offender entries
  above 30. `cargo crap` currently reports each row twice in this workspace
  build, giving 4 duplicated report rows.

## Work Log

- Package scaffolded after row #5 commit `bce07c6e`.
- Added focused row #6 unit coverage in
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs` for
  annual schedule action/window branches, perennial schedule optional-day and
  action branches, and growth equation guard families.
- Refactored behavior-preserving high-complexity growth validators:
  `validate_schedule_domain` now delegates to annual/perennial schedule helpers,
  and `validate_equation_inputs` now delegates to weather/thermal, shape, root,
  and monthly GDD helper groups.
- Full-workspace CRAP-after reduced row #6 owned offenders from `2` unique
  entries (`4` duplicated report rows) to `0` entries above CRAP 30.
- H2637 protected outputs remained byte-identical against
  `/tmp/typed-direct-carrier-identity/base/output`, with
  `compatibility_edge_invocations=0`.
- Full gates passed: focused `cqr_row6` tests, `cargo fmt --check`, `cargo
  clippy --workspace --all-targets -- -D warnings`, `cargo nextest run
  --workspace --profile full`, `cargo deny check`, authority anti-evasion,
  Auth11 required-suite obligation guard, markdown-doc lint/validate, and
  line-count governance.

## Disposition

Result: `EXECUTED-COMPLETE-ROW6-CQR`.

Row #6 is complete without ADR-0021 warnings.
