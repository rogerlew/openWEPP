# Progress

Static: package executed for row #5 hydrology WB kernel CQR closure.

Ran:

- Reused final post-row-9 full-workspace LCOV + CRAP JSON:
  `/tmp/openwepp-crap-row9-full-after.json`.

Result:

- Row #5 CRAP-before extraction found 11 unique production offender entries
  above 30. `cargo crap` currently reports each row twice in this workspace
  build, giving 22 duplicated report rows.

## Work Log

- Package scaffolded after row #9 commit `15c3f11e`.
- Added focused row #5 unit coverage in hydrology modules for guard error
  code/display branches, snow-albedo formatting variants, multilayer snow
  density boundary-mass operations, R7G JSON escaping, frozen-soil k-factor
  resolution, snow-density guard error mapping, SIMIMPL29 melt branches, and
  active-snow coupling edge paths.
- Refactored behavior-preserving high-complexity hydrology helpers:
  SIMIMPL29 hourly melt calculation, active-snow coupling state advancement,
  TMPADJ aerodynamic roughness/depth derivation, and boundary-mass update
  helpers.
- Full-workspace CRAP-after reduced row #5 owned offenders from `11` unique
  entries (`22` duplicated report rows) to `0` entries above CRAP 30.
- H2637 protected outputs remained byte-identical against
  `/tmp/typed-direct-carrier-identity/base/output`, with
  `compatibility_edge_invocations=0`.
- Full gates passed: focused `cqr_row5` tests, `cargo fmt --check`, `cargo
  clippy --workspace --all-targets -- -D warnings`, `cargo nextest run
  --workspace --profile full`, `cargo deny check`, authority anti-evasion,
  Auth11 required-suite obligation guard, markdown-doc lint/validate, and
  line-count governance.

## Disposition

Result: `EXECUTED-COMPLETE-ROW5-CQR`.

Row #5 is complete without ADR-0021 warnings.
