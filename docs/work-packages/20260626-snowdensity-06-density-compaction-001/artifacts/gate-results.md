# Gate Results

Evidence mode: Ran.

## Required Gates

- `cargo fmt --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `cargo deny check` passed.
- `git diff --check` passed.
- `rg -n "qwet|frzftp" crates` returned no matches.

## Focused Gates

- `cargo test -p openwepp-runner snowbench_physics_bulk -- --nocapture`
  passed.
- `cargo test --test snowdensity03_physics_bulk_offline_contract -- --nocapture`
  passed.
- `cargo test --test snowdensity06_density_compaction -- --nocapture`
  passed.
- `cargo build -p openwepp-runner --bin openwepp-snowbench` passed.

## Adjudication Gate

Command:

```bash
.venv/bin/python tools/snowfreeze_observed/physics_bulk_adjudication.py --output-dir target/snowdensity06_adjudication_density_only --variant density_compaction_v1
```

Result: passed and produced five-site SNOTEL profile artifacts.

Density-cell result:

- Legacy/as-built density cells: fail `9`, score `16`.
- `density_compaction_v1` density cells: fail `7`, score `22`.

Whole-rubric result:

- Legacy/as-built robust cells: fail `9`, score `84`.
- `density_compaction_v1` robust cells: fail `18`, score `46`.

Disposition: density-cell gate complete; no runtime promotion, default
activation, parser/runfile/CLI selector, output schema change, or frost
attribution authorized by this package.
