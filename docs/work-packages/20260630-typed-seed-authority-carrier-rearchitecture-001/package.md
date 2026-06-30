# Typed Seed-Authority Carrier Re-Architecture

Status: EXECUTED-HOLD-PHASE1-TYPED-PROJECTION-APIS-MISSING

Package id: `20260630-typed-seed-authority-carrier-rearchitecture-001`

## Objective

Replace the production direct setup-time symbol-map seed authority with a typed
per-lane seed carrier, prove seed-value identity against the existing computed
day-zero `HillslopeWritebackSurface`, cut over production seeding to the typed
carrier, and then delete the now-orphaned symbol-map runtime production paths.

This package supersedes the per-read burn-down approach from
`20260630-stage1-seed-authority-migration-001/`.

## Required Reading

- `docs/work-packages/20260630-typed-direct-setup-symbol-map-elimination-001/`.
- `docs/work-packages/20260630-typed-direct-setup-symbol-map-carrier-deletion-001/`.
- `docs/work-packages/20260630-stage1-seed-authority-migration-001/`.
- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime/00_wb11_runtime_seed.rs`.
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`.
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`.
- `crates/openwepp-runner/src/hillslope/direct_publication/`.
- `crates/openwepp-input-contract/`.
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs`.
- `docs/architecture/array-native-runtime-specification.md` section 4.11.
- `docs/decisions/0025-array-native-hillslope-day-frame.md`.
- `docs/decisions/0030-r7-terminal-contract-and-compatibility-runtime-deletion.md`.
- `docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/artifacts/no-compatibility-proof-plan.md`.
- `docs/work-packages/20260618-perfarch03-full-array-native-floor-prototype-001/`.

## Finding This Package Acts On

The remaining setup bridge is an ordered computed pipeline:

1. Build static symbol-map surfaces from typed parse results.
2. Merge lane static seed authority.
3. Merge day-one climate.
4. Run `seed_wb11_runtime_surface_inputs`.
5. Read the resulting day-zero surface into `DirectLaneConstructorInputs`,
   `DirectProductionDayInputBuilder`, and coupling/publication metadata.

The 208-to-207 burn-down proved the seed authority is not independent reads.
The replacement must be one typed carrier built for the whole pipeline.

## Scope

Phase 1: build a typed per-lane seed-authority carrier from parsed
input-contract data plus day-one climate. The carrier must include:

- typed WB11 day-zero projection;
- constructor seed state for water, layers, ET stage, plant growth, plant
  stress, and snow/frost;
- direct day-input authority state currently built by
  `DirectProductionDayInputBuilder`;
- coupling/publication metadata currently read from
  `execution_runtime_surface`.

Phase 2: shadow the typed carrier alongside the existing day-zero symbol-map
surface and compare value-by-value on H2637, a multi-OFE fixture, and a Wave-2
fixture. This shadow is transient setup-time migration evidence only.

Phase 3: cut over production seeding to the typed carrier in one change. The
symbol-map seed surface must become unused in production direct setup.

Phase 4: delete the orphaned symbol-map production runtime: scheduler executor,
day frame, production carrier types, production kernel entry signatures, and
dead tests/fixtures. Keep only the explicit `--compatibility-runtime`
replay/comparator seam.

Phase 5: prove no compatibility production path remains and re-measure H2637
time/RSS.

## Non-Scope

- Do not delete the explicit `--compatibility-runtime` replay seam.
- No physics change.
- No output-schema column change.
- No watershed CLI change.

## Gates

- Phase 2 seed identity: typed carrier seed values match the symbol-map
  day-zero surface on H2637, multi-OFE, and Wave-2.
- Phase 3 output identity: H2637, multi-OFE, and Wave-2 outputs are identical
  before vs after cutover. HBP/loss/manifest byte-identical; WAT/PASS parquet
  schema, row-count, and value identity.
- Phase 5 no-compatibility proof: zero compatibility-edge/symbol-map
  invocations on production fixtures; static call graph reaches symbol-map
  runtime only through explicit `--compatibility-runtime`.
- Perf: H2637 remains `<=10x` legacy, RSS remains run-length-flat, and the
  measured legacy ratio is recorded.
- Full gates: `cargo fmt --check`, `cargo clippy --workspace --all-targets --
  -D warnings`, `cargo nextest run --workspace --profile full`, `cargo deny
  check`, authority anti-evasion, required-suite obligation guard, and
  Markdown lint.

## Disposition

Result:
`EXECUTED-HOLD-PHASE1-TYPED-PROJECTION-APIS-MISSING`.

The scaffold and static execution completed. Phase 1 cannot honestly build the
typed per-lane carrier yet because the computed seed pipeline is exposed only as
symbol-map surface builders and surface-mutating seed functions. A carrier built
from `direct_publication_day_zero_seed_surface` or the `from_seed` helpers would
be a typed wrapper around the same `HillslopeWritebackSurface` authority, not the
single-authority typed parse-to-frame representation required by this package.

First actionable follow-on: factor typed projection APIs for the ordered seed
pipeline, with surface-writer adapters retained only for the deprecated
compatibility seam and transition comparison. The minimum projection units are
the static per-lane parsed-input projection, typed day-one climate projection,
typed `Wb11DayZeroProjection`, direct lane constructor seed state,
`DirectProductionLaneDayInputAuthority`, coupling/publication metadata, and
Wave-2 flag authority. Once those APIs exist, resume this package at Phase 1 and
run the Phase 2 seed-identity gate before cutover.
