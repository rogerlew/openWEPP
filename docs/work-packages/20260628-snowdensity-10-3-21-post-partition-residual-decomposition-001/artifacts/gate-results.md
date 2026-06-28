# Gate Results

Evidence class: Static + Ran.

| Gate | Result | Evidence |
|---|---|---|
| Consume `INV-SNOWFREEZE-050` without new gate authority | PASS | Package and tool authority sections list `SC-SNOWFREEZE-001 INV-SNOWFREEZE-050`; no SC contract file edited. |
| Decompose residual robust fails by signature and climate | PASS | `post-partition-residual-decomposition.json` `robust_fail_rows` and `residual_clusters`. |
| Decompose mass/SWE, depth, and density directly | PASS | `mass_density_depth_decomposition.site_rows` records site-level residual directions from corpus quantities. |
| Split over- vs under-persistence | PASS | `over_under_persistence_split`: under `4`, over `0`, density-structure `11`. |
| Classify residual clusters by new mechanism class vs irreducible/forcing-limited | PASS | `mechanism_class_read` and cluster records classify density, cancov geometry, and mountain timing clusters. |
| Produce frost-threshold input without deciding threshold or unblocking frost | PASS | `frost_attribution_threshold_input.not_a_decision = true`; summary disposition is no-promotion/no-frost-decision. |
| Preserve protected boundaries | PASS | Report `protected_boundaries` all false for production/default/selector/cap/schema/fixture/frost/site-calibration changes. |

## Validation Commands

| Command | Result |
|---|---|
| `.venv/bin/python -m py_compile tools/snowfreeze_observed/post_partition_residual_decomposition.py` | PASS |
| `.venv/bin/python tools/snowfreeze_observed/post_partition_residual_decomposition.py` | PASS |
| `cargo fmt --check` | PASS |
| `cargo test --test snowdensity10_3_21_post_partition_residual_decomposition` | PASS |
| `cargo test --test snowdensity03_physics_bulk_offline_contract` | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS |
| `cargo deny check` | PASS |
| `git diff --check` | PASS |
