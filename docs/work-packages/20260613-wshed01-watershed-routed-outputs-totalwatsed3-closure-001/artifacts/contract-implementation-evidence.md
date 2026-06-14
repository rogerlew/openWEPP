# Contract Implementation Evidence

Status: W-C executed

Evidence mode: Static

W-B modified the watershed runfile contract and impoundment parser contract
behavior.

W-B contract implementation:

- `docs/contracts/openwepp-watershed-runfile-contract.md` now pins no-pond
  semantics:
  - `inputs.pw0_imp` remains required in schema v1,
  - supported `.imp` files with `jpond=0` are valid typed empty sets only when
    structure declares zero impoundments,
  - positive structural counts fail as a typed count mismatch,
  - bare parser use without structural count context does not relax `jpond=0`.
- `crates/openwepp-input-contract/src/parsers/watershed_impoundment.rs` now
  reconciles `declared_count` against `expected_structural_count` before the
  bare `jpond >= 1` domain guard. This preserves fail-closed behavior for
  non-numeric/negative/malformed counts and for mismatches.

Contract implications carried into W-C:

- W-C must preserve the watershed runfile output contract:
  `openwepp-watershed-runfile-contract.md:141-163` requires all 14 parquet
  outputs, including `totalwatsed3`.
- The totalwatsed3 schema in `openwepp-watershed-output/src/writers.rs` already
  mirrors the wepppy schema names, but current publication defaults most
  unmapped fields to `0.0`; W-C must replace placeholder publication with
  real routed operands.

W-C contract implementation:

- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md` now pins
  zero-sediment contributor semantics: complete HBP sediment payloads with zero
  mass and zero concentration are valid without positive particle-flow
  fractions; positive mass or concentration support still requires positive
  particle-fraction support.
- `SC-ROUTE-001` also pins `nchnum=0` as an output-disabled channel detail
  state, not a routing domain violation.
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/helpers.rs`
  preserves fail-closed required-field checks while distinguishing zero-mass
  payloads from positive sediment payloads.
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/diagnostics.rs`
  skips fraction normalization only for zero-mass hillslope sediment payloads.
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/validation.rs`
  accepts `nchnum >= 0`.
- `crates/openwepp-runner/src/watershed_wat.rs` builds daily watershed row
  seeds from sibling WAT parquet files and fails closed when only a partial WAT
  sibling set exists.
- `crates/openwepp-watershed-output/src/writers.rs` publishes multiple
  watershed rows and maps water-balance fields from row seeds instead of
  writer defaults.

Contract implications carried into W-D:

- W-C proves routed output publication and anti-placeholder WAT-backed fields.
- W-D must still run the totalwatsed3 audit and close the water-balance
  identity with independent operands.
