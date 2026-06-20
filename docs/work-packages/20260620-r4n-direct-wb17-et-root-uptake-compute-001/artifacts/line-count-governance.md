# Line-Count Governance

Status: pre-implementation.

Static: R4N will add `direct_runtime/evapotranspiration.rs` rather than expand
the root direct-runtime file with WB17 process compute.

Baseline command:

```text
wc -l crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/subsurface.rs \
  crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs
```

Ran:

```text
1809 crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs
 940 crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs
1625 crates/openwepp-hillslope-orchestrator/src/direct_runtime/subsurface.rs
1954 crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs
6328 total
```

Final line-count evidence:

```text
1849 crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs
 940 crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs
1636 crates/openwepp-hillslope-orchestrator/src/direct_runtime/subsurface.rs
1095 crates/openwepp-hillslope-orchestrator/src/direct_runtime/evapotranspiration.rs
2003 crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs
 447 crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r4n.rs
7970 total
```

Disposition:

- The root direct-runtime file remains below the 2000-line WARN band.
- `direct_runtime/evapotranspiration.rs` is a new R4N process module and is
  below the 2000-line WARN band.
- `tests/tests_mod/direct_runtime.rs` is at 2003 lines. This is a test module
  with existing aggregate direct-runtime fixtures; it is a WARN-band file but
  below the 3000-line blocker. Follow-on R4P/Q/Z should avoid expanding it
  further and prefer package-specific test modules.
