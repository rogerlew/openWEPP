# Typed Direct Setup Symbol-Map Carrier Deletion

Status: EXECUTED-HOLD-STAGE1-TYPED-SEED-AUTHORITY-MISSING

Package id: `20260630-typed-direct-setup-symbol-map-carrier-deletion-001`

## Objective

Make the typed `DirectRunFrame` the sole production runtime representation, then
delete the symbol-map compatibility runtime carriers that are no longer
production-load-bearing.

## Required Reading

- `docs/decisions/0030-r7-terminal-contract-and-compatibility-runtime-deletion.md`.
- `docs/work-packages/20260630-compatibility-runtime-deletion-001/package.md`.
- `docs/work-packages/20260630-compatibility-runtime-deletion-001/artifacts/static-callgraph-audit.md`.
- `docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/artifacts/no-compatibility-proof-plan.md`.
- `docs/work-packages/20260630-typed-direct-setup-symbol-map-elimination-001/package.md`.
- `docs/architecture/array-native-runtime-specification.md` sections 2, 4.11,
  4.12, 5.1, 5.4, and 6.2.
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`.
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`.
- `crates/openwepp-runner/src/hillslope/direct_publication/`.
- `crates/openwepp-input-contract/`.
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`.
- `crates/openwepp-hillslope-orchestrator/src/day_frame.rs`.

## Scope

Stage 1 replaces production direct setup/seeding with typed input authority,
incremented by input family: climate, soil/layers, management/plant, slope and
topology, snow/frost init, and publication/coupling metadata.

Stage 2 deletes the now-orphaned symbol-map compatibility runtime production
executor, day frame, carrier types, and symbol-map kernel entry points. The
explicit `--compatibility-runtime` replay/comparator seam remains as the
deprecated ADR-0030 adapter.

Stage 3 proves no production path constructs or invokes compatibility carriers:
direct-executor allowlist, static call-graph audit, and zero forbidden API
counters on production fixtures.

## Non-Scope

- Do not delete the explicit `--compatibility-runtime` replay/comparator seam.
- No physics change.
- No output-schema column change.
- No watershed CLI change.
- No new perf/RSS work beyond ensuring the completed publication RSS arc does
  not regress.

## Gates

- Per-stage identity on H2637, a multi-OFE fixture, and a Wave-2 fixture:
  HBP/loss/manifest byte identity; WAT/PASS schema, row-count, and value
  identity.
- Production direct must not construct or read a symbol-map runtime surface
  after Stage 1.
- Stage 3 no-compatibility proof: zero compatibility-edge/symbol-map
  invocations on production fixtures and static call-graph exclusion from
  production entrypoints.
- RSS no-regression versus the completed streaming-sink baseline: H2637 remains
  near the recorded `112652 KiB` full-output / `52228 KiB` required-output
  envelope.
- Full closure loop: `cargo fmt --check`, `cargo clippy --workspace
  --all-targets -- -D warnings`, `cargo nextest run --workspace --profile full`,
  `cargo deny check`, authority anti-evasion, required-suite obligation guard,
  and scoped Markdown lint/validate.

## Execution Notes

Initial static inventory found the remaining setup carrier is larger than the
registry toggle: production direct still constructs `HillslopeWritebackSurface`
seed authorities and derives roughly `208` direct-publication seed reads from
legacy symbol names. Runtime-input helpers still emit roughly `266` symbol-map
insertions.

Stage 1A removed the direct-production `SymbolRegistry` / `HotSymbolTables`
construction and indexed/lane-dense writeback activation. H2637 remained
byte-identical and RSS dropped from the clean `5b139058` baseline's
`110916 KiB` to `91796 KiB`. The package stops before Stage 2 because the
primary Stage 1 gate still fails: production direct setup still reads
`HillslopeWritebackSurface` seed authorities.

## Disposition

Held. See `artifacts/disposition.md`.
