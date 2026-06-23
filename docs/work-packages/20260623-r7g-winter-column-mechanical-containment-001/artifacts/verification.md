# Verification

Status: COMPLETE.

Ran:

```bash
cargo check -p openwepp-hillslope-orchestrator
```

Result: PASS, exit code 0.

Ran:

```bash
cargo fmt --check
```

Result: PASS on final tree, exit code 0.

Ran:

```bash
cargo clippy --workspace --all-targets -- -D warnings
```

Result: PASS on final tree, exit code 0.

Ran:

```bash
cargo test -p openwepp-hillslope-orchestrator --lib winter_column -- --nocapture
```

Result: PASS, exit code 0. Both zero-state containment tests passed.

Ran:

```bash
cargo test -p openwepp-hillslope-orchestrator --lib r7b_constructor_type_size_layout_is_bounded -- --nocapture
```

Result: PASS, exit code 0. Output included `DirectLaneFrame=1184`,
preserving the bounded layout guard after boxing `DirectWinterColumnState`.

Ran:

```bash
cargo test -p openwepp --test clim06_frost_frozen_soil_kernel_contract fdhp01_de_lower_front_heat_suppresses_marginal_autumn_freeze_onset -- --nocapture
```

Result: PASS after active-frost no-freeze hourly diagnostic remediation, exit
code 0.

Ran:

```bash
cargo test -p openwepp-runner r7f_typed_day_input_hot_loop_excludes_runtime_surface_reads -- --nocapture
```

Result: PASS, exit code 0.

Ran:

```bash
cargo test -p openwepp-runner r7g_direct_production -- --nocapture
```

Result: PASS, exit code 0.

Ran:

```bash
cargo test --workspace
```

Result: PASS on final tree, including doc tests, exit code 0.

Ran:

```bash
cargo deny check
```

Result: PASS, exit code 0: `advisories ok, bans ok, licenses ok, sources ok`.

Ran:

```bash
rg -n "DirectFrostRunoffSurface|HillslopeKernelRequest|HillslopeWritebackSurface|BoundarySymbol|BoundaryValue|WB13|BTreeMap|HashMap|Symbol" crates/openwepp-hillslope-orchestrator/src/winter_column.rs
```

Result: PASS by no matches, exit code 1.

Ran:

```bash
markdown-doc lint --path docs/work-packages/20260623-r7g-winter-column-mechanical-containment-001 --no-ignore
markdown-doc lint --path docs/work-packages/README.md --no-ignore
git diff --check
```

Result: PASS, exit code 0 for all three commands.

Gate non-deferral check:

- Every current package exit criterion has direct evidence in this artifact or
  `boundary-proof.md`.
- The package does not claim solver migration, publication parity, performance,
  default direct activation, or R7G closure.
- The active-frost diagnostic fix is recorded as validation blocker remediation,
  not as winter-column ownership cutover.
