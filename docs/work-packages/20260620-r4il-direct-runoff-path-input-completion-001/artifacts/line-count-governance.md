# R4I-L Line-Count Governance

Status: complete.

Evidence class: Ran.

Command:

```text
wc -l crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs crates/openwepp-runner/src/hillslope/03_tests.rs
```

Baseline:

```text
1996 crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs
 940 crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs
1998 crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs
 650 crates/openwepp-runner/src/hillslope/03_tests.rs
5584 total
```

`direct_runtime.rs` and the direct-runtime test file are one small change away
from the 2000-line warning threshold. R4I-L should split runoff-specific source
before or during implementation rather than growing those files in place.

## Closure Recheck

Command:

```text
wc -l crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r4il.rs crates/openwepp-runner/src/hillslope/03_tests.rs
```

Result:

```text
1764 crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs
 803 crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs
 940 crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs
1857 crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs
 477 crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r4il.rs
 656 crates/openwepp-runner/src/hillslope/03_tests.rs
6497 total
```

Verdict: PASS. The R4I-L source split moved the root direct-runtime file and
the legacy direct-runtime test file away from the 2000-line warning threshold.
