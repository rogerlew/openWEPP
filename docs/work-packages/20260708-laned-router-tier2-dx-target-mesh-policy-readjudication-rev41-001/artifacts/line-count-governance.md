# Line-Count Governance

Status: `PASS`
Evidence mode: Ran.

Command:

```text
wc -l crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs crates/openwepp-runner/src/hillslope/laned_active.rs crates/openwepp-runner/src/hillslope/laned_shadow.rs
```

Result:

```text
  1819 crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs
  1288 crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs
   116 crates/openwepp-runner/src/hillslope/laned_active.rs
   706 crates/openwepp-runner/src/hillslope/laned_shadow.rs
  3929 total
```

No Rust production code changed in this package. This is an advisory scan of
the implementation surface that would have been touched if a production mesh
policy had been promoted. No scanned Rust file crosses the 2000-line warning
threshold, and no 3000-line closure blocker exists in that surface.
