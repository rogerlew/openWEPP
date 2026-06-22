# Line-Count Governance

Evidence class: Static and Ran.

## Pre-Refactor Counts

- `direct_runtime.rs`: `2922` lines at `HEAD`.
- `tests/tests_mod/direct_runtime.rs`: `2890` lines at `HEAD`.

The root module was below the `3000` hard block but in WARN-band. The test file
was also WARN-band before this package.

## Post-Refactor Counts

Ran:

```text
wc -l crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/02_state_reports.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/04_audit_error_helpers.rs \
  crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs
```

Observed:

```text
   210 crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs
  1001 crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs
   454 crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs
   433 crates/openwepp-hillslope-orchestrator/src/direct_runtime/02_state_reports.rs
   391 crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs
   434 crates/openwepp-hillslope-orchestrator/src/direct_runtime/04_audit_error_helpers.rs
  2844 crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs
  5767 total
```

## Direct Runtime Module Directory

Ran `find crates/openwepp-hillslope-orchestrator/src/direct_runtime -maxdepth 1
-type f -name '*.rs' -print0 | xargs -0 wc -l | sort -n`.

Largest production direct-runtime module after the split:

- `subsurface.rs`: `1655` lines.
- `evapotranspiration.rs`: `1114` lines.
- `growth.rs`: `1095` lines.
- `00_core_frames.rs`: `1001` lines.

All production direct-runtime `.rs` files are below `2000` lines.

## Disposition

- The package objective is satisfied: `direct_runtime.rs` is below `2000`
  lines, and every new included file is below `2000` lines.
- No touched `.rs` file is at or above `3000` lines.
- The direct-runtime test file remains WARN-band at `2844` lines, but this
  package reduced it from `2890` lines while updating the source scan. Splitting
  that test file is a separate mechanical-refactor candidate, not required for
  this package closure.
