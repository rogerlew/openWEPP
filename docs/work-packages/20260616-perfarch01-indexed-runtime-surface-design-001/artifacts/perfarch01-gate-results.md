# PERFARCH01 Gate Results

Status: COMPLETE 2026-06-16
Evidence mode: **Ran**

## Prototype

Ran:

```text
rustfmt docs/work-packages/20260616-perfarch01-indexed-runtime-surface-design-001/artifacts/prototypes/indexed_surface_microbench.rs
rustc -O docs/work-packages/20260616-perfarch01-indexed-runtime-surface-design-001/artifacts/prototypes/indexed_surface_microbench.rs -o /tmp/perfarch01_indexed_surface_microbench
/tmp/perfarch01_indexed_surface_microbench
```

Result:

```text
symbols=6396
lookup_ops=1944
sorted_id_order_matches_string_sort=true
clone_btreemap_ns_per=464296.16
clone_dense_ns_per=4226.56
clone_speedup=109.85x
lookup_btreemap_format_ns_per_op=303.65
lookup_dense_precomputed_ns_per_op=1.39
lookup_speedup=219.16x
update_btreemap_clone_insert_ns_per_batch=815942.34
update_dense_clone_set_ns_per_batch=7048.14
update_batch_speedup=115.77x
```

## Repository Gates

Ran:

```text
cargo fmt --check
```

Result: passed.

Ran:

```text
git diff --check
```

Result: passed.

Advisory docs-maintainer checks:

```text
wctl doc-lint --path docs/work-packages/20260616-perfarch01-indexed-runtime-surface-design-001
wctl doc-lint --path docs/ROADMAP.md
wctl doc-lint --path docs/work-packages/README.md
wctl doc-lint --path docs/decisions/README.md
```

Results: no errors or warnings reported. The package path, roadmap, and
decisions README invocations reported `0 files validated` under this repo's
`wctl` configuration; `docs/work-packages/README.md` reported `1 files
validated, 0 errors, 0 warnings`.

Full `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test
--workspace`, and `cargo deny check` were not run because PERFARCH01 edited only
documentation and a standalone prototype artifact, not production Rust.
