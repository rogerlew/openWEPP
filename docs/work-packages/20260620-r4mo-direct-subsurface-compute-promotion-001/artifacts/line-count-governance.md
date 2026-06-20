# Line-Count Governance

Status: baseline.

Static: R4M/O will add `direct_runtime/subsurface.rs` so WB18/WB19 compute does
not inflate `direct_runtime.rs`.

Baseline command:

```text
wc -l crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs \
  crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs \
  crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r4il.rs \
  crates/openwepp-runner/src/hillslope/03_tests.rs
```

Ran:

```text
1764 crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs
 803 crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs
 940 crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs
1857 crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs
 477 crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r4il.rs
 656 crates/openwepp-runner/src/hillslope/03_tests.rs
6497 total
```

Disposition: baseline has no touched Rust file in the 2000-line WARN band.

## Closure Recheck

Command:

```text
wc -l crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/subsurface.rs \
  crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs \
  crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r4il.rs \
  crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r4mo.rs \
  crates/openwepp-runner/src/hillslope/03_tests.rs
```

Result:

```text
1809 crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs
 803 crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs
 940 crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs
1625 crates/openwepp-hillslope-orchestrator/src/direct_runtime/subsurface.rs
1954 crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs
 477 crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r4il.rs
 522 crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r4mo.rs
 656 crates/openwepp-runner/src/hillslope/03_tests.rs
8786 total
```

Verdict: PASS. No touched production `.rs` file is in the 2000-line WARN band;
the largest production files are `direct_runtime.rs` at 1809 lines and
`direct_runtime/subsurface.rs` at 1625 lines. The touched direct-runtime test
file is also below 2000 lines.
