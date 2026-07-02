# Line-Count Governance

Evidence class: Ran (`wc -l` on touched `.rs` files at closure).

| Touched file | Before (dab1e031) | After | Δ |
|---|---:|---:|---:|
| `runoff.rs` | 2740 | 2268 | -472 |
| `00_core_frames.rs` | 2535 | 2542 | 7 |
| `01_publication.rs` | 595 | 599 | 4 |
| `03_executor.rs` | 736 | 742 | 6 |
| `direct_runtime.rs` | 768 | 745 | -23 |
| `direct_runtime_r7g_frost.rs` | 1238 | 1376 | 138 |
| `00_builders_and_authority.rs` | 4141 | 4143 | 2 |
| `00a_snow_frost_authority_impl.rs` | 667 | 668 | 1 |

The pre-existing 3000+ file (`00_builders_and_authority.rs`, 4,141 → 4,148,
+7 for the outcome carry) keeps the WP-1 `follow-up` disposition
(mechanical-refactor split queued); `runoff.rs` shrank materially with the
second-solve deletion. No file crossed a governance band in this package.
