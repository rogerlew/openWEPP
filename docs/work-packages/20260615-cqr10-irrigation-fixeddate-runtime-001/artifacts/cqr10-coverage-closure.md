# CQR10 Coverage Closure

Status: complete-with-warnings.

Static: target file:
`crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs`.

Ran: before LCOV was generated with:

```bash
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr10-irrigation-fixeddate-runtime-001/artifacts/lcov_before.info
```

Ran: after LCOV was generated with:

```bash
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr10-irrigation-fixeddate-runtime-001/artifacts/lcov_after.info
```

Evidence:

- Before target-file coverage: `194/686` lines, `5/10` functions.
- After target-file coverage: `423/747` lines, `17/20` functions.
- Target function coverage after refactor:
  `seed_hillslope_runtime_surface_from_irrigation_fixeddate` = `100.0%`.
- New fixed-date helper coverage after refactor:
  `validate_fixeddate_irrigation_header` = `77.77777777777779%`;
  `seed_fixeddate_irrigation_header_symbols` = `87.87878787878788%`;
  `seed_fixeddate_irrigation_events` = `100.0%`;
  `seed_fixeddate_irrigation_event` = `100.0%`;
  `seed_fixeddate_irrigation_event_schedule` = `83.78378378378379%`;
  `seed_fixeddate_irrigation_sprinkler_event` = `67.3076923076923%`;
  `seed_fixeddate_irrigation_furrow_event` = `70.1492537313433%`;
  `fixeddate_event_next_record` = `100.0%`;
  `FixedDateProjectionState::new` = `100.0%`;
  `FixedDateProjectionState::advance` = `100.0%`.

Disposition:

- PASS: target-file line and function coverage improved relative to the
  package baseline.
- WARN: target-file coverage remains below the science-tier threshold in
  `docs/decisions/0021-module-coverage-closure-thresholds.md`.
- HOLD: no additional CQR10 edits are authorized for broad coverage expansion;
  the remaining low-coverage depletion path is out of scope for this package.
