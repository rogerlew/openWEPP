# Gate Results

Status: `PASS`

Evidence mode: `Static + Ran`

## Contract, Implementation, And Consumer Gates

| Gate | Result |
|---|---|
| Pre-implementation contract target | `7 passed / 2 expected failed` because schema v4 and the new fields were absent. |
| Focused contract/runtime suite | `cargo nextest run --no-fail-fast --test snow_surface_eb03_contract --test snow_surface_eb04v_density_process_diagnostics_contract --test snow_surface_eb04w_accumulation_melt_diagnostics_contract --test paradigm2_stage3_liquid_routing_meltwater_temperature --test paradigm2_stage3_decouple_water_temperature --test hphys0296_snow_rm_acceptance_authority_contract` — `34/34 PASS`. |
| Terminal EB-04W contract target | `cargo nextest run --no-fail-fast --test snow_surface_eb04w_accumulation_melt_diagnostics_contract` — `9/9 PASS`. |
| Formatter unit tests | `cargo test -p openwepp-runner formatter_preserves -- --nocapture` — `2/2 PASS`. |
| Independent real consumer | `.venv/bin/python .../tools/trace_closure.py verify ...` — `PASS`; `14245` rows, maximum closure error `1.2272e-17 m`, all four aliases rejected, WAT and HBP/PASS identical. |
| Release runner build | `cargo build --release -p openwepp-runner --bins` — `PASS`; terminal CLI SHA-256 `464c87e1...`. |
| Assurance source adoption | Typed `adopt-report-source --check`, `--apply`, and idempotent `--check` — `PASS`; generation `9e64c4c7... -> 12bddac7...`, receipt `ac9ae76f...`, no invalidated authority. |
| Assurance validation | `target/release/openwepp-assurance validate --all` — `PASS`, three draft reports and zero public reports. |

## Direct Quality Gates

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | `PASS` |
| `cargo clippy --workspace --all-targets -- -D warnings` | `PASS` |
| `cargo test --workspace --doc` | `PASS` |
| `git diff --check` | `PASS` |
| In-memory `compile()` of `tools/trace_closure.py` | `PASS`; no bytecode written. |
| Package/contract/index/catalog/roadmap `markdown-doc lint` and `validate` | `PASS` |

## Broad Exact-Current Campaign

The authorized comparator runner executed all three profiles against scaffold
commit `48d89081` plus the terminal dirty package diff:

| Profile | Tests | Result | Wall time | Receipt SHA-256 |
|---|---:|---|---:|---|
| quick | `2160` | `2160 passed; 0 failed/error/skipped` | `2298.793 s` | `52a85ec6...` |
| frost | `345` | `345 passed; 0 failed/error/skipped` | `532.448 s` | `5bce87b6...` |
| full / Critical | `2209` | `2209 passed; 0 failed/error/skipped` | `2323.118 s` | `8c922518...` |

Receipts and logs are under
`target/snow_stage3_liquid_signed_hour_trace_closure/gates/`. The first quick
attempt correctly failed on stale generated assurance identity after the
contract edit; it was superseded only after the typed source-adoption
transaction and the exact-current `2160/2160` rerun above.

## Terminal Diff Disposition

The diff remains behavior-neutral diagnostic publication: five Rust modules,
the v123 contract/index, contract-derived tests and mechanical v123 pins,
generated assurance locks/receipt, and package documentation/tooling. No
equation, branch, selector, default, parameter, fixture, observation, WAT,
HBP/PASS, or public schema changed. Line-count governance passes with three
sub-3000-line `WARN` files and no 3000-line refactor blocker.
