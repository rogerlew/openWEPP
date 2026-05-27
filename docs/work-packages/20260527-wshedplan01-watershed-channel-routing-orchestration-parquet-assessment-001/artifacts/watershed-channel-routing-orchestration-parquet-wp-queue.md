# Watershed Channel Routing, Orchestration, and Parquet Work-Package Queue

Status: complete
Evidence mode: static
Date: 2026-05-27

## Static
Dependency-ordered follow-on queue to close watershed routing/orchestration and
non-placeholder parquet publication gaps.

## Queue

### 1. WSHED02 - Watershed contract authority closure and gap normalization
- Objective: align canonical contracts with baseline-authoritative watershed
  routing/impoundment/sediment process scope and explicit unresolved gap rows.
- Depends on: WSHEDPLAN01.
- Primary write set:
  - `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
  - `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
  - `docs/specifications/science-contracts/contracts/SC-SED-001.md`
  - `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
  - `docs/specifications/science-contracts/index.md`
- Exit criteria:
  - WS11/WS12 authority language matches required baseline lineages,
  - new watershed-channel-sediment migration gaps are explicit,
  - provenance map references corrected `detach.for` lineage.

### 2. WSHED03 - Watershed contract-derived tests and pre-implementation gate
- Objective: add contract-derived vectors that fail under current partial
  runtime, then record pre-implementation gate evidence.
- Depends on: WSHED02.
- Primary write set:
  - `tests/integration/ws11_*`
  - `tests/integration/ws12_*`
  - new watershed CLI end-to-end fixture tests
  - package artifacts for WSHED03 gate evidence
- Exit criteria:
  - vectors exist for KW/MC branch lineage, impoundment RK4/regime-transition,
    channel sediment routing entry, and watershed parquet emission non-stub.

### 3. WSHED04 - Runtime seam closure for channel/impoundment state families
- Objective: project parser-authoritative channel/impoundment families into
  runtime surfaces required by WS11/WS12 production physics.
- Depends on: WSHED03.
- Primary write set:
  - `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
  - `crates/openwepp-input-contract/src/parsers/watershed_channel.rs`
  - `crates/openwepp-input-contract/src/parsers/watershed_impoundment.rs`
  - `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- Exit criteria:
  - no manual coefficient seeding in tests,
  - typed guards for missing/non-finite/out-of-domain seam symbols,
  - no silent defaults for required parity surfaces.

### 4. WSHED05 - Channel hydrology routing migration (`wshcqi/wshirs/wshrun/wshpek/wshchr/chrqin`)
- Objective: port baseline-authoritative channel hydrology routing branches,
  including `ipeak` method families and `ipeak>2` wave routing arrays.
- Depends on: WSHED04.
- Primary write set:
  - `crates/openwepp-watershed-orchestrator/src/lib.rs`
  - supporting modules for wave-array state and branch helpers
  - watershed routing comparator fixtures/harness artifacts
- Exit criteria:
  - WS11 branch lineage and closure invariants pass contract-derived vectors,
  - no surrogate fallback replaces required wave-routing methods.

### 5. WSHED06 - Channel sediment routing migration (`chnero/chnrt/detach`)
- Objective: migrate baseline channel sediment routing and detachment/deposition
  process families with per-class transport closure.
- Depends on: WSHED05.
- Primary write set:
  - `crates/openwepp-watershed-orchestrator/src/lib.rs` (or extracted modules)
  - potential new crate/module for channel sediment process kernels
  - `docs/specifications/science-contracts/contracts/SC-SED-001.md` (if new canonical symbols required)
- Exit criteria:
  - watershed channel sediment process executes without heuristic proxy math,
  - typed guard continuity retained,
  - contract-derived sediment vectors pass.

### 6. WSHED07 - Impoundment continuity migration (`wshiqi/wshimp` + `imphnw/impflo/impmai`)
- Objective: port baseline-authoritative impoundment continuity integration,
  outflow composition, adaptive timestep retry, and regime-transition logic.
- Depends on: WSHED04 (can run in parallel with WSHED05/06 until integration point).
- Primary write set:
  - `crates/openwepp-watershed-orchestrator/src/lib.rs`
  - optional extracted impoundment physics module(s)
  - WS12 contract-derived test vectors
- Exit criteria:
  - WS12 authority replacement requirements are runtime-true,
  - surrogate-deauthorization vector remains satisfied,
  - typed guard family preserved.

### 7. WSHED08 - Watershed output row model and parquet writer activation
- Objective: replace `OWSOUT-E-004` placeholder with real row-model-to-parquet
  emission for all required watershed outputs.
- Depends on: WSHED05, WSHED06, WSHED07.
- Primary write set:
  - `crates/openwepp-watershed-output/src/writers.rs`
  - supporting row-builder sources in runner/orchestrator
  - watershed parquet integration tests
- Exit criteria:
  - all required parquet files emit non-empty schema-compatible outputs,
  - writer no longer blocks with `OWSOUT-E-004` for valid execution paths,
  - failure modes remain typed and fail-closed.

### 8. WSHED09 - End-to-end validation, comparator rerun, and hold-lift disposition
- Objective: run required validation lanes and baseline comparators, then issue
  explicit GO/HOLD disposition for watershed routing/orchestration/parquet
  closure.
- Depends on: WSHED08.
- Primary write set:
  - package evidence artifacts and comparator outputs
  - any residual contract/queue updates required by findings
- Exit criteria:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
  - comparator evidence classified by confidence tier with explicit disposition.

## Recommended execution order
`WSHED02 -> WSHED03 -> WSHED04 -> (WSHED05 + WSHED07) -> WSHED06 -> WSHED08 -> WSHED09`

## Ran
- not run
