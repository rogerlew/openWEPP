# PERFDEEP06 Follow-On Package Sequence

Status: complete 2026-06-19.
Evidence class: Static.

## Recommended Sequence

### PERFDEEP07 - Zero-Cost Disabled Path and Direct-Frame Hydrology Fast Path

Objective: first remove or bypass the default-disabled dense-first tax, then
implement a bounded direct-frame hydrology daily OFE chain over typed
`HillslopeDayFrame`/view APIs. The migrated success path must not use symbol
maps, writeback payloads, registry lookup, dense/logical refresh, or logical
fallback reads.

P0 disabled-path gate: PERFDEEP05 final-code default-disabled H2637 measured
`701.95 s` versus the `669.97 s` reference, and PERFDEEP03 default-disabled
measured in the `697-708 s` band. PERFDEEP07 must prove that all dense-first
resolution, compact dense view construction, indexed shadow setup, direct-frame
shadow setup, and symbol-table work are bypassed when PERFDEEP opt-ins are off.
The timing protocol is predeclared: at least three clean H2637 no-UI runs with
all PERFDEEP opt-ins disabled, min/median/max/RSS recorded, and median
`<= 676.67 s` (`669.97 s + 1%`). Candidate and control should run on the same
machine in the same harness/session where feasible. If the historical
`669.97 s` reference cannot be rerun, PERFDEEP07 must report a same-machine
pre-cleanup control and cannot pass above `676.67 s` without hard attribution
to an external environment change.

Write set:

- `crates/openwepp-hillslope-orchestrator/src/day_frame.rs`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/**`
- `crates/openwepp-hillslope-orchestrator/src/tests/**`
- `crates/openwepp-kernel-contract/src/lib_mod/core_types/**` only for bounded
  new typed frame/view types if ownership requires contract exposure
- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs` only
  for typed publication projection shadowing
- `docs/work-packages/<PERFDEEP07>/**`
- `docs/ROADMAP.md`, `docs/work-packages/README.md`, and
  `docs/architecture/array-native-runtime-specification.md` for disposition

Required gates:

- default-disabled H2637 identity plus endpoint/RSS after the zero-cost-disabled
  cleanup, compared against `701.95 s`, `669.97 s`, and the `676.67 s`
  disabled-path pass threshold;
- same-machine control or explicit hard attribution for any timing environment
  difference;
- static proof that no dense-first/request-view/direct-frame shadow work runs
  when all PERFDEEP opt-ins are disabled;
- direct-frame seed fixture from current logical state;
- shadow `to_bits()` identity for migrated frame outputs and arrays;
- H2637 HBP/WAT byte identity and PASS Arrow equivalence;
- endpoint and RSS versus `669.97 s` and final default-disabled comparison;
- layout/type-size and allocation evidence;
- static no-hot-loop-map proof using this package's absence checklist;
- full Rust gates: `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo deny check`;
- markdown lint and package review/verification closure.

Stop criteria:

- `HOLD` if identity diverges or an authority/contract issue appears.
- `HOLD` if the default-disabled median remains above `676.67 s` or the static
  disabled-path bypass proof is incomplete before adding more fast-path
  plumbing.
- `NO-GO` if map/writeback absence is proven but endpoint remains flat/negative
  against the final default-disabled path.
- No default activation unless the opt-in endpoint beats `669.97 s` and all
  identity/full gates pass.

### PERFDEEP08 - Typed Publication Projection Cutover

Objective: if PERFDEEP07 proves the frame path, move WB13/HBP/PASS construction
to `HillslopeDayPublicationProjection` and keep logical surfaces only for
replay/diagnostics.

Gate: HBP byte identity, WAT byte/Arrow identity, PASS Arrow identity, manifest
provenance equivalence, and the publication ledger's anti-alias fixtures.

### PERFDEEP09 - Complete OFE-Day Frame Path

Objective: port remaining transitions, growth/decomposition, erosion, and
closure diagnostics so all 14 phases mutate one frame.

Gate: full H2637 identity and endpoint/RSS; no symbol/logical surfaces in phase
execution.

### PERFDEEP10 - Delete Logical Hot-Path Plumbing

Objective: remove `HillslopeWritebackSurface`, indexed mirrors, writeback
payloads, and registry build from the per-OFE-day loop, leaving symbol surfaces
only at intake, replay, diagnostics, and serialization.

Gate: full H2637 identity and <=10x / <=5x viability check.

## Gate

PASS. PERFDEEP07 can be scaffolded directly from this sequence.
