# CQR20 Implementation and Test Evidence

Status: complete.

Static: implementation summary:

- Added focused characterization tests in
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests/management.rs`.
- Removed the CQR20 target's production `#[allow(clippy::too_many_lines)]`.
- Split `project_annual_extension_controls` into private per-branch helpers for
  herbicide, burn, silage, cut, remove, and no-extension branches.
- Kept unsupported `resmgt=7` and out-of-range `resmgt` errors in the target
  dispatcher with unchanged fields and strings.

Ran: focused tests before production refactor:

```text
running 3 tests
test runtime_inputs::tests::cqr20_project_annual_extension_controls_characterizes_domain_errors ... ok
test runtime_inputs::tests::cqr20_project_annual_extension_controls_characterizes_valid_branches ... ok
test runtime_inputs::tests::cqr20_project_annual_extension_controls_characterizes_error_branches ... ok
```

Ran: focused tests after production refactor:

```text
running 3 tests
test runtime_inputs::tests::cqr20_project_annual_extension_controls_characterizes_domain_errors ... ok
test runtime_inputs::tests::cqr20_project_annual_extension_controls_characterizes_error_branches ... ok
test runtime_inputs::tests::cqr20_project_annual_extension_controls_characterizes_valid_branches ... ok
```
