# Typed Day-Zero Seed Computation

Status: EXECUTED-CHECKPOINT-GATE1-PASSED-PHASE3-PRODUCTION-SEED-SURFACE-DELETED

Package id: `20260630-typed-day-zero-seed-computation-001`

Continuation of:
`20260630-typed-seed-authority-carrier-rearchitecture-001/`.

## Objective

Reimplement the computed day-zero direct-production seed authority as typed
projection code so the typed `DirectRunFrame` can become the sole production
runtime representation from parse to output. The typed computation must be the
authority; symbol-map surfaces are allowed only as transient shadow comparators
and as adapters for the explicit deprecated `--compatibility-runtime` replay
seam.

## Required Reading

- `docs/work-packages/20260630-typed-seed-authority-carrier-rearchitecture-001/artifacts/seed-pipeline-map.md`.
- `docs/work-packages/20260630-typed-seed-authority-carrier-rearchitecture-001/artifacts/disposition.md`.
- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime/00_wb11_runtime_seed.rs`.
- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime/01_wb12_wb16_wb19_seed.rs`.
- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime/02_mofe03_wave2_seed.rs`.
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`.
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`.
- `crates/openwepp-runner/src/hillslope/direct_publication/`.
- `crates/openwepp-input-contract/`.
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs`.
- `docs/decisions/0025-array-native-hillslope-day-frame.md`.
- `docs/work-packages/20260618-perfarch03-full-array-native-floor-prototype-001/`.

## Scope

Phase 1: implement typed day-zero seed computations, sub-computation by
sub-computation, over typed inputs and typed day-one climate:

- typed static input projection for soil/slope/management/PMET/snow/frost seed
  state;
- typed WB18/WB19 lane substep controls and multi-OFE hourly carry;
- typed rainfall/hyetograph normalization;
- typed initial layer water stores and WB11 totals;
- typed fine-frost frozen-depth refresh;
- typed residue and `Ws` defaults;
- typed WB12 reconciliation seed values;
- typed ET-demand seed;
- typed `efflen`, `m`, WB16 `ealpha`, and MOFE03/Wave-2 seed values.

Each sub-computation must be shadow-proven value-identical against the existing
day-zero `HillslopeWritebackSurface` on H2637, one multi-OFE fixture, and one
Wave-2 fixture. The shadow proof is transient migration evidence, not a
retained runtime dual representation.

Phase 2: add typed consumers and cut over all seed consumers at once:

- lane constructor;
- `DirectProductionDayInputBuilder`;
- coupling metadata;
- Wave-2 flag;
- winter hourly geometry.

Phase 3: delete orphaned symbol-map seed authority and production runtime
machinery while keeping only the explicit deprecated `--compatibility-runtime`
replay seam.

Phase 4: prove no compatibility production path remains and re-measure H2637
time/RSS.

## Non-Scope

- Do not delete the explicit `--compatibility-runtime` replay seam.
- No physics change; typed computation must reproduce current day-zero seed
  values exactly.
- No output-schema column change.
- No watershed CLI work.

## Gates

- Phase 1 seed identity: typed sub-computation values match corresponding
  day-zero surface values on H2637, multi-OFE, and Wave-2 fixtures.
- Phase 2 output identity: H2637, multi-OFE, and Wave-2 outputs remain
  identical before vs after cutover. HBP/loss/manifest byte-identical; WAT/PASS
  parquet schema, row-count, and value identity.
- Phase 4 no-compatibility proof: zero compatibility-edge/symbol-map
  invocations on production fixtures; static call graph reaches symbol-map
  runtime only through explicit `--compatibility-runtime`.
- Perf: H2637 remains `<=10x` legacy, RSS remains run-length-flat, and a new
  legacy ratio toward `<=5x` is recorded.
- Full gates: `cargo fmt --check`, `cargo clippy --workspace --all-targets --
  -D warnings`, `cargo nextest run --workspace --profile full`, `cargo deny
  check`, authority anti-evasion, required-suite obligation guard, and
  Markdown lint/validate.

## Disposition

Result:
`EXECUTED-CHECKPOINT-GATE1-PASSED-PHASE3-PRODUCTION-SEED-SURFACE-DELETED`.

Phase 1 expanded the typed seed-computation slice. The existing surface seeder
now delegates these sub-computations to typed projection cores:

- WB18/WB19 lane substep controls and multi-OFE hourly-carry activation;
- rainfall/hyetograph normalization, including breakpoint cardinality and
  zero-cardinality synthesized events;
- WB11 initial layer water stores and storage totals;
- fine-frost frozen-depth refresh;
- residue interception and `Ws` defaults;
- WB12 reconciliation seed defaults;
- ET-demand seed, including Priestley-Taylor and EVAPPM/PMET branches;
- `efflen` and default `m`;
- WB16 `ealpha` compatibility-default decision.

Each landed sub-computation has a focused shadow test comparing typed projection
values to the existing day-zero `HillslopeWritebackSurface` outputs by exact
`f64::to_bits()` where applicable.

The autonomous completion resume then assembled and cut over the production
typed seed authority. Production direct execution and the snowbench diagnostic
now construct `DirectProductionSeedAuthority` from typed parsed inputs,
sidecars, and day-one climate rather than from a day-zero
`HillslopeWritebackSurface`. The five production consumer groups now read the
typed carrier: lane constructor, `DirectProductionDayInputBuilder`, coupling
metadata, Wave-2 flag, and winter hourly geometry.

Gate 1 is passed. H2637 protected outputs are byte-identical against clean
`5b139058`, cli01 protected outputs are byte-identical, the focused
multi-OFE/Wave-2 fixture passes, full workspace gates pass, and H2637 RSS
improves from `113268 KiB` to `91692 KiB`.

Phase 3 production seed-surface deletion is partly complete. The direct
production setup branch no longer builds static `HillslopeWritebackSurface`
seed parts, persistent lane symbol-map state, symbol registry, or hot symbol
tables. The obsolete direct-publication day-zero seed-surface bridge
(`from_day_zero_seed_surfaces`, `direct_publication_day_zero_seed_surface`, and
lane seed-surface helpers) and the dead
`DirectProductionDayInputBuilder::build_lane_authority` surface reader have
been removed. Snowbench/PySnobal export diagnostics now source initial canopy,
winter context, and snow-density bridge values from the typed seed carrier
instead of the deleted setup surface. H2637 protected outputs remain
byte-identical and the direct-runtime counter reports
`compatibility_edge_invocations=0`; final H2637 RSS is `84776 KiB`.

Remaining Phase 3 work is the broader symbol-map runtime/code deletion that can
be removed without breaking the explicit deprecated `--compatibility-runtime`
replay seam.
