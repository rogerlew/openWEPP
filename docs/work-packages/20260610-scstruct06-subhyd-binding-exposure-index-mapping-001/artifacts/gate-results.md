# SCSTRUCT06 Gate Results

Evidence: Static + Ran
Date: 2026-06-11

| Gate | Result | Evidence |
|---|---|---|
| Binding Exposure Index present over every addendum section. | pass | 22 top-level addenda; 22 BEI rows. |
| Default binding-exposure lint. | pass-deferred | `PASS-DEFERRED`; exit `0`; 22 rows and 15 deferred rows. |
| Strict binding-exposure lint. | pass-deferred/strict-fail | `PASS-DEFERRED`; strict exit `1` by lint contract because rows remain deferred. |
| No malformed rows / no gamed gate flips. | pass | Custom count: 0 `Canonical binding IDs = none` with `Review gate = none`. |
| Contract diff boundary. | pass | `SC-SUBHYD-001` diff adds BEI section only. |
| Runtime/kernel boundary. | pass | No `.rs` or runtime files changed. |

No full Rust closure loop was run because SCSTRUCT06 is docs-only index/triage
work with no kernel, runtime, or mechanical Rust refactor edits.
