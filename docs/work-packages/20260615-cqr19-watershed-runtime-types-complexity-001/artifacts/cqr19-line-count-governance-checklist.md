# CQR19 Line-Count Governance Checklist

Status: complete.

Ran: before line counts from `git show HEAD:<path> | wc -l`:

```text
crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/types.rs 513
crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/tests.rs 1192
docs/work-packages/README.md 592
docs/work-packages/cqr-burndown-execplan.md 679
```

Ran: after line counts from `wc -l`:

```text
586 crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/types.rs
1489 crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/tests.rs
596 docs/work-packages/README.md
679 docs/work-packages/cqr-burndown-execplan.md
```

Static: target file remains below the `3000` line non-exempt limit.

Ran: suppression census after refactor:

```text
crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/tests.rs:142:#[allow(clippy::too_many_lines)]
crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/tests.rs:247:#[allow(clippy::too_many_lines)]
crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/tests.rs:439:#[allow(clippy::similar_names, clippy::too_many_lines)]
```

Static: no production `#[allow(clippy::too_many_lines)]` remains in
`types.rs`. New suppressions are restricted to focused test characterization
and existing long fixture tests.
