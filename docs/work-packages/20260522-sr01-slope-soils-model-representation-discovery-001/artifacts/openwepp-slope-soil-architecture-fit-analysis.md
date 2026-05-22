# openWEPP Slope/Soil Architecture-Fit Analysis

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- openWEPP architecture requires immutable parsed inputs, typed boundaries, and explicit alias continuity.
- slope and soil parser contracts are already authored and implemented.

Ran:
- Inspected openWEPP parser/orchestrator/symbol-registry/test surfaces:
  - `crates/openwepp-input-contract/src/parsers/slope.rs`
  - `crates/openwepp-input-contract/src/parsers/soil.rs`
  - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
  - `tests/integration/infile_slope_parser_contract.rs`
  - `tests/integration/infile_soil_parser_contract.rs`
  - `tests/integration/parser_runtime_seam_integration.rs`
  - `crates/openwepp-sim-contract/src/symbols.rs`

## Typed State and Seam Fit

1. Parser layer fit: strong.
- Slope parser implements strict/compat datver policy, distance-mode resolution, endpoint closure, and cross-OFE boundary checks.
- Soil parser implements datver-variant arity/policy parsing, monotone depth checks, restrictive-layer parsing, and typed error codes.

2. Runtime seam fit: partial.
- Hillslope runtime seam exists for soil only, projecting `solthk`, `dg`, `thetdr`, `thetfc` from first OFE/top-layer + profile depth.
- No corresponding slope runtime seam exists in `runtime_inputs.rs`.

3. Integration test fit: partial.
- Parser-to-runtime seam integration asserts soil state symbols in scheduler execution.
- There is no analogous slope runtime-surface integration test path.

4. Symbol continuity fit: partial.
- Canonical alias registry currently includes `thetdr`, `thetfc`, `dg`, `solthk`, but does not yet include slope geometry/profile symbols (`nslpts`, `slplen`, `xinput`, `slpinp`, `avgslp`, `a`, `b`, `xu`, `xl`).

## Constraint and Gap Analysis

| constraint | current state | impact |
|---|---|---|
| Canonical symbol continuity for slope runtime surfaces | missing alias coverage | blocks contract-complete slope boundary publication |
| Soil seam coverage depth | top-layer + profile depth projection only | insufficient for full parity of layer-distributed consumers (`soil`, `infpar`, `watbal`, `perc`) |
| Slope runtime surface projection | absent | prevents typed handoff to runoff/routing kernels |
| Cross-subsystem ownership closure | incomplete for slope/soil beyond parsers | boundary cannot be closed as implementation-complete |

## Recommended Integration Pattern

1. Keep parser contracts as source-of-truth for file semantics.
2. Add a dedicated slope runtime surface builder in hillslope orchestrator, preserving canonical symbol continuity.
3. Expand soil runtime boundary from minimal seed symbols to full layer/profile surfaces needed by `SC-SOIL-001`, `SC-WATBAL-001`, `SC-SUBHYD-001` consumers.
4. Expand `openwepp-sim-contract` alias registry to include full slope + soil runtime symbols.
5. Add parser-to-runtime integration tests for slope and expanded soil surfaces before kernel-coupled implementation.

## Evidence Links

- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md:20`
- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md:82`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/slope.rs:277`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/slope.rs:434`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/soil.rs:291`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/soil.rs:642`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:388`
- `/home/workdir/openWEPP/tests/integration/infile_slope_parser_contract.rs:15`
- `/home/workdir/openWEPP/tests/integration/infile_soil_parser_contract.rs:19`
- `/home/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs:50`
- `/home/workdir/openWEPP/crates/openwepp-sim-contract/src/symbols.rs:209`
