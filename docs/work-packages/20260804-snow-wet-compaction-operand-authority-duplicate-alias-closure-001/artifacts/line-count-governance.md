# Line-Count Governance

Status: complete / post-review snapshot

Evidence mode: Ran

`wc -l` was run from `/home/workdir/openWEPP` for every touched Rust file.

| File/class | Lines | Disposition |
| --- | ---: | --- |
| `runoff_reconciliation.rs` | 2,723 | WARN. The correction removes one caller argument and replaces one expression; no new control-flow block belongs here. This file is already a member of `support_helpers_mod`; further responsibility splitting is a distinct maintainability package and would increase Critical closure risk here. |
| `infiltration_reconciliation.rs` | 2,579 | WARN. The operand helper stays adjacent to exact hourly melt/rain authority and its typed guard; extraction also keeps the active finalizer below the warnings-denied Clippy limit. Further module decomposition is a distinct maintainability package. |
| `snowbench_coe_melt.rs` | 1,209 | Pass. |
| `03_kernel_support_00_support_helpers.rs` | 927 | Pass. |
| `snowbench_coe_density.rs` | 969 | Pass. |
| New wet-compaction integration test | 428 | Pass. |
| Existing integration tests | 78-449 | Pass; version-pin edits only except the owned CoE CSV header expectation. |

No touched Rust file reaches the mandatory `3000+` refactor threshold. The two
WARN files were already large support-helper modules; this package neither
creates a new monolith nor authorizes a cross-cutting mechanical refactor.
