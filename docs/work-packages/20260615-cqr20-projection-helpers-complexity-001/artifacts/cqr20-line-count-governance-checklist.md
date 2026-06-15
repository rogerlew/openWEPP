# CQR20 Line-Count Governance Checklist

Status: complete.

Ran: before line counts captured before production edits:

```text
crates/openwepp-hillslope-orchestrator/src/runtime_inputs/05_projection_helpers.rs  964
docs/work-packages/README.md                                                        597
docs/work-packages/cqr-burndown-execplan.md                                         685
```

Ran: after line counts:

```text
crates/openwepp-hillslope-orchestrator/src/runtime_inputs/05_projection_helpers.rs  999
crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests/management.rs    1449
docs/work-packages/README.md                                                        601
docs/work-packages/cqr-burndown-execplan.md                                         685
```

Static: no touched non-exempt Rust file is at or above `3000` lines.

Ran: suppression census:

```text
before target-file suppressions:
05_projection_helpers.rs:273: #[allow(clippy::too_many_lines)]
05_projection_helpers.rs:584: #[allow(clippy::too_many_lines)]

after target-file suppressions:
05_projection_helpers.rs:273: #[allow(clippy::too_many_lines)]
```

Static: the remaining suppression belongs to the pre-existing
`growth_equation_parameter_values` helper, not the CQR20 annual-extension target.
