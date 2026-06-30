# Verification

Evidence mode: Static/Ran.

## Stage 1B Focused Verification

Ran:

```text
cargo fmt --check
cargo check -p openwepp-runner
cargo build --release -p openwepp-runner --bin openwepp-cli-hill
```

Result: passed.

Ran H2637 direct production with Stage 1B output directory:

```text
target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file /tmp/stage1-seed-authority/after-1b/h2637.run \
  --output-dir /tmp/stage1-seed-authority/after-1b/output \
  --manifest-path /tmp/stage1-seed-authority/after-1b/output/manifest.json
```

Result: passed. Max RSS: `95280 KiB`.

Ran H2637 output comparison against
`/tmp/typed-direct-carrier-identity/base/output`:

- `H2637.hbp`: byte-identical.
- `H2637.loss.json`: byte-identical.
- `H2637.plot.parquet`: byte-identical.
- `H2637.wat.parquet`: byte-identical.
- `H2637.pass.parquet`: byte-identical.

Manifest counters:

- `selected`: `direct-production-executor`.
- `compatibility_edge_invocations`: `0`.
- `day_frame_constructions`: `235961`.
- `erod14_wave2_kernel_status_seen`: `true`.
- `scheduler_kernel_executed`: `false`.

Ran focused multi-OFE/Wave-2 fixture:

```text
cargo test -p openwepp --test cli03_runner_contract_derived_tests \
  cli03_mofe03_multiofe_runfile_executes_wave2_without_manual_symbol_injection \
  -- --nocapture
```

Result: passed.

## Static Inventory

Ran:

```text
rg -n "require_runtime_surface_scalar|runtime_surface_symbol_value|direct_publication_optional|direct_publication_required|DirectProduction.*from_seed|from_seed\\(|direct_publication_layer_states|direct_publication_profile_inputs|direct_publication_percolation_inputs|direct_publication_subsurface_inputs" crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs | wc -l
```

Result: `207`.

Ran:

```text
rg -n "HillslopeWritebackSurface" crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers | wc -l
```

Result: `58`.

## HOLD Verification

Static:

- `seed_direct_production_lane_constructor_inputs` still constructs a
  day-zero seed surface and reads constructor state from it.
- `DirectProductionDayInputBuilder::new` still builds lane authority from
  day-zero seed surfaces.
- `build_direct_production_coupling_vector_provenance` still reads coupling
  metadata from a day-zero runtime surface.

Conclusion: Stage 1 is not complete and Stage 2 deletion is not authorized.

## Final Hygiene

Ran:

```text
cargo fmt --check
git diff --check
```

Result: passed.

Ran scoped Markdown validation:

```text
wctl doc-lint --path docs/work-packages/README.md
find docs/work-packages/20260630-stage1-seed-authority-migration-001 -name '*.md' -print |
  sort |
  xargs -r -n1 sh -c 'markdown-doc lint --path "$1"' sh
```

Result: README and all package Markdown files passed. The directory-level
`wctl doc-lint --path docs/work-packages/20260630-stage1-seed-authority-migration-001`
validated `0` files because the package files are still untracked; direct
per-file `markdown-doc lint --path` validated them.
