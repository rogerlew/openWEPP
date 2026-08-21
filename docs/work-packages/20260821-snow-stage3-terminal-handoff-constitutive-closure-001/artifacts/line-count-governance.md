# Line-count governance

Status: `MEASURED / NO THRESHOLD VIOLATION IDENTIFIED`.

`Ran:` terminal measurement uses `wc -l` over changed Rust files. The package
threshold is `WARN` at 2,000 lines and requires refactor at 3,000 lines for
non-exempt files. The two newly expanded core files are below the hard
threshold after extraction:

| File | Lines |
|---|---:|
| `stage3_solver.rs` | 2,982 |
| `v9_real_consumer_shadow.rs` | 2,966 |
| `snow_stage3_v11_attachment.rs` | 845 |
| `canonical_owner_bytes.rs` | 100 |

Other changed files were measured and no file exceeds 3,000 lines after the
terminal refactors. The full exact changed-file count is preserved by the
terminal diff reconciliation.
