# PERFMIG02 Line-Count Governance

Static: measured touched Rust file line counts after implementation and formatting.

Ran:

```text
wc -l crates/openwepp-kernel-contract/src/lib_mod/writeback.rs \
  crates/openwepp-kernel-contract/src/lib.rs \
  crates/openwepp-hillslope-orchestrator/src/scheduler.rs \
  crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs \
  crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/writeback.rs \
  crates/openwepp-hillslope-orchestrator/src/lib.rs
```

Result:

```text
   541 crates/openwepp-kernel-contract/src/lib_mod/writeback.rs
   839 crates/openwepp-kernel-contract/src/lib.rs
  2630 crates/openwepp-hillslope-orchestrator/src/scheduler.rs
  2197 crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs
  1250 crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/writeback.rs
    76 crates/openwepp-hillslope-orchestrator/src/lib.rs
  7533 total
```

Disposition:

- `scheduler.rs`: WARN, above 2000 and below the 3000 required-refactor threshold. PERFMIG02 adds small
  policy wiring and was also reformatted around the indexed execution closure; no blocking split required
  in this package.
- `state_access.rs`: WARN, above 2000 and below the 3000 required-refactor threshold. The dense-first helper
  migration is local to existing helper responsibilities; no blocking split required in this package.
- Other touched files: below WARN threshold.

No file crosses the 3000-line required-refactor threshold.
