# Validation Execution Provenance

Evidence mode: **Ran**.

## Source And Dirty Identity

All terminal direct gates below ran from `/home/workdir/openWEPP` at Git HEAD
`045cac9475738b0306a89a934702c479803f0935` on the final EB-04W source.
The expected dirty-tree status snapshot hashed to
`8092092471b8520dce2e213a7702e188d181b1261679c7b0233a01ff2e24681c`;
the tracked binary diff against HEAD hashed to
`aaa2d4fede9aba5e1c4eafbb27b596e997e9737ad5613fa6ff54f41b68315f8a`.
Subsequent package-local review, disposition, and suite evidence may change the
status snapshot without changing the scoped source identity below.

The final release binary SHA-256 is
`b50dd71cb00f24806193b98d73fc5444e836efac84ad5a4e0465d1e67c81fec9`.
`execution-receipt.json` has SHA-256
`6f6bfe361c5b0aa155de1cfba61306e6d20fd570e68f67521eed12a3154dfbf7`
and independently binds all ten production source files, the release binary,
the exact build/execution commands, sanitized environment, and all 16 per-cell
provenance files. A post-run check found zero production-source hash mismatches.
Supplemental terminal inputs are:

- `Cargo.toml`: `163009d90f7659d5a95590bb21a1b0dd58b38150527d4e3a86b0deaa879ec2ff`;
- `Cargo.lock`: `33986f95b2d62927687bc3c17539ee3f87fd61c5a8bc2876b4f32e524e9d52b5`;
- `SC-SNOWFREEZE-001.md`: `53ef8d5f8e771c5eef13e04d7fca71e9a9f4b06ecca2248270b81d8dcc0ee9b4`;
- focused integration target: `481b935f40965a5dd0fc874852d255adf69b598a91cd28b8e1219e53a9821ba5`;
- snowbench adapter with non-100-density regression:
  `a65e9a2c93ae0e040583f9409cf37aeb78bfd666a09b32277a77bfa7bd948e2e`.

## Direct Terminal Gates

Each row records the exact argv, exit status, elapsed wall time, durable stdout/
stderr log, timing receipt, and requirement closed. All commands used the
working directory and source identity above.

| Gate / exact argv | Result | Elapsed | Evidence | Requirement |
|---|---:|---:|---|---|
| `cargo fmt --all -- --check` | 0 | 2.93 s | `logs/final-fmt.log`; `logs/final-fmt.time` | Rust formatting |
| `git diff --check` | 0 | 0.05 s | `logs/final-diff-check.log`; `logs/final-diff-check.time` | patch integrity |
| `cargo nextest run --test snow_surface_eb04w_accumulation_melt_diagnostics_contract` | 0; 2/2 | 4.53 s | `logs/final-focused.log`; `logs/final-focused.time` | contract and real-consumer bindings |
| `cargo test -p openwepp-runner real_trace_formatter_preserves_phase_depth_swe_and_distinct_melt_operands` | 0; 1/1 | 0.25 s | `logs/final-runner-formatter.log`; `logs/final-runner-formatter.time` | numeric v3 real formatter / anti-alias proof |
| `cargo test -p openwepp-hillslope-orchestrator simimpl29_melt_hour_covers_zero_wind_rain_and_cap_paths` | 0; 1/1 | 0.16 s | `logs/final-melt-ledger.log`; `logs/final-melt-ledger.time` | four-term/cap closure and typed guard |
| `cargo test -p openwepp-hillslope-orchestrator phase_fraction_closure_distinguishes_dry_and_active_hours` | 0; 1/1 | 0.16 s | `logs/final-phase-semantics.log`; `logs/final-phase-semantics.time` | dry/wet phase semantics |
| `cargo test -p openwepp-runner noncanonical_new_snow_density_preserves_runtime_phase_closure` | 0; 1/1 | 1.16 s | `logs/final-noncanonical-density.log`; `logs/final-noncanonical-density.time` | snowbench/runtime phase-operand compatibility |
| `cargo clippy -p openwepp-hillslope-orchestrator -p openwepp-runner --all-targets -- -D warnings` | 0 | 1.21 s | `logs/final-clippy.log`; `logs/final-clippy.time` | warnings-denied affected crates |
| `bash tools/release/check_unit_registry.sh` | 0; 21/21 | 8.35 s | `logs/final-unit-registry.log`; `logs/final-unit-registry.time` | canonical unit registry |
| `cargo deny check` | 0 | 1.39 s | `logs/final-deny.log`; `logs/final-deny.time` | dependency advisories/bans/licenses/sources |
| `cargo run --quiet -p openwepp-assurance -- validate --all` | 0; 3 DRAFT / 0 public | 9.08 s | `logs/final-assurance-validate.log`; `logs/final-assurance-validate.time` | governed assurance catalog |
| `.venv/bin/python tools/local_ci/render_assurance_review_drafts.py --root . --binary target/debug/openwepp-assurance --check` | 0; 92 current | 105.53 s | `logs/final-assurance-render.log`; `logs/final-assurance-render.time` | real rendered-review freshness |
| `markdown-doc lint --path docs/work-packages/20260801-snow-surface-eb-04w-accumulation-under-persistence-001` | 0; 34 files | 0.01 s | `logs/final-markdown-package.log`; `logs/final-markdown-package.time` | package Markdown |
| `markdown-doc lint --path docs/ROADMAP.md` | 0; 1 file | 0.00 s | `logs/final-markdown-root-roadmap.log`; `logs/final-markdown-root-roadmap.time` | root roadmap Markdown |
| `markdown-doc lint --path docs/planning/snow-surface-energy-balance-roadmap.md` | 0; 1 file | 0.00 s | `logs/final-markdown-campaign-roadmap.log`; `logs/final-markdown-campaign-roadmap.time` | campaign roadmap Markdown |
| `markdown-doc lint --path docs/work-packages/README.md` | 0; 1 file | 0.01 s | `logs/final-markdown-work-package-catalog.log`; `logs/final-markdown-work-package-catalog.time` | work-package catalog Markdown |

A full repository docs scan reported 15 pre-existing broken links outside the
EB-04W write set; it is not represented as a clean global gate.

## Exact Empirical Workflows

| Workflow / exact argv | Result | Elapsed | Identity / evidence |
|---|---:|---:|---|
| `.venv/bin/python docs/work-packages/20260801-snow-surface-eb-04w-accumulation-under-persistence-001/tools/run_accumulation_diagnostics.py --execute --workers 4` under `/usr/bin/time` | 0; 16/16 cells, 5/5 operators | 273.07 s | receipt `6f6bfe36…`; result `a44c3561cfea5bec64cc7514c4e3701d95111d8368ab96d8f0cb4784fcae6816`; `logs/exact-cohort-terminal.time` |
| `.venv/bin/python docs/work-packages/20260801-snow-surface-eb-04w-accumulation-under-persistence-001/tools/compare_terminal_behavior.py` under `/usr/bin/time` | 0; exact-zero retained outputs | 254.85 s | result `b896b53ecb3787dd85fe46732a7154b22788c942af3f6c8093a96113859e7d0e`; comparator `1b3f331d…`; `logs/behavior-neutrality-final.time` |

Earlier exact cohorts and interrupted broad/analysis attempts are explicitly
invalidated in their named artifacts and logs. They are chronology, not reused
terminal evidence. Quick/frost/full source identity, argv, durations, exits,
run IDs, and logs are recorded separately by the terminal suite runner.
