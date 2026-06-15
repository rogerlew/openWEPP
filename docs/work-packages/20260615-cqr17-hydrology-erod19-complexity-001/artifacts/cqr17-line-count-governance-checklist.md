# CQR17 Line-Count Governance Checklist

Status: closed.

Ran: before line counts:

```text
612 crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod19.rs
837 crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/hydrology.rs
582 docs/work-packages/README.md
667 docs/work-packages/cqr-burndown-execplan.md
151 docs/work-packages/20260615-cqr17-hydrology-erod19-complexity-001/package.md
```

Ran: after line counts:

```text
784 crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod19.rs
918 crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/hydrology.rs
586 docs/work-packages/README.md
667 docs/work-packages/cqr-burndown-execplan.md
151 docs/work-packages/20260615-cqr17-hydrology-erod19-complexity-001/package.md
```

Static: no touched non-exempt Rust file is at or above `3000` lines.

Ran: after suppression census for `hydrology_phase_erod19.rs`:

```text
1:#[allow(clippy::wildcard_imports)]
300:#[allow(clippy::too_many_arguments)]
319:#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
445:#[allow(clippy::similar_names, clippy::too_many_lines)]
```

Static: the target function suppression at the original CQR17 row was removed.
Remaining suppressions are pre-existing and are tied to out-of-scope functions.
