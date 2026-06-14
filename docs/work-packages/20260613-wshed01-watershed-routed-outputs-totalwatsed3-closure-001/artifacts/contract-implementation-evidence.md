# Contract Implementation Evidence

Status: T-B2 executed

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

W-D contract/metadata implementation:

- `crates/openwepp-sim-contract/src/units_mod/output_catalog.rs` now registers
  `watershed_totalwatsed3.Interception` as a `mm` boundary-registry-backed
  publication column.
- `crates/openwepp-watershed-output/src/writers.rs` now aligns exact
  totalwatsed3 volume columns with their `m^3` metadata while preserving
  explicit depth aliases as mm.
- `crates/openwepp-runner/src/watershed_wat.rs` follows the MOFE outlet
  lateral aggregation rule for WAT-backed watershed publication and carries
  optional WAT profile/interception fields into row seeds.

Contract implications carried into the T-arc:

- W-D did not amend process-physics contracts because it corrected
  publication lineage and output unit metadata only.
- The remaining closure blocker requires canonical daily PASS `runvol`
  authority. T-A scopes this as dedicated `openwepp-cli-totalwatsed3`
  implementation work, not watershed-CLI work.
- If T-B exposes new HBP/PASS payload fields, adds a PASS parquet output
  contract, or changes PASS publication semantics, update the relevant
  contract/metadata authority before implementation.
- `totalwatsed3-cli-scope.md` is a design artifact only; it does not replace
  canonical `SC-*` authority for T-B production behavior.

T-B contract/metadata implementation:

- T-B did not change process physics or require a new PASS writer contract.
  It consumes the already published openWEPP-native hillslope interchange
  `H.pass.parquet` and `H.wat.parquet` files.
- `crates/openwepp-sim-contract/src/units_mod/output_catalog.rs` now records
  `watershed_totalwatsed3.Runoff` as a publication-only depth derived from
  independent PASS runoff volume over aggregate area, with authority
  `SC-WATBAL-001#INV-WATBAL-054`.
- `crates/openwepp-watershed-output/src/writers.rs` now carries separate
  diagnostic WAT `Q` and PASS-derived `Runoff` operands so the published
  schema no longer aliases runoff depth to WAT `Q`.
- The dedicated CLI keeps the wepppy producer as semantic/audit reference
  only; it does not depend on the wepppyo3 `wepp_interchange` crate.

T-B2 contract/metadata implementation:

- `crates/openwepp-hillslope-output/src/contracts.rs` adds optional
  `pass_parquet` to the hillslope output contract. The binary HBP `pass`
  output remains required and unchanged.
- `crates/openwepp-hillslope-output/src/hillslope_pass.rs` defines an
  openWEPP-owned runoff-delivery parquet schema with row-level unit metadata.
- `crates/openwepp-sim-contract/src/units_mod/output_catalog.rs` registers
  `hillslope_pass.runvol`, `sbrunv`, `peakro`, `tdet`, `tdep`, and
  `sedcon_1` through `sedcon_5`.
- `tests/integration/sim_contract_boundary_unit_registry.rs` now includes
  `hillslope_pass_schema` in the output unit metadata coverage gate.
- No process-physics contract was amended. T-B2 publishes existing outlet
  MOFE transfer state as an output surface; it does not change hydrology
  execution.

Contract implications carried into T-C:

- The producer surface is now present and independently test-pinned.
- The native openWEPP PASS surface is now present and independently
  test-pinned.
- T-C must close the remaining residual as a water-balance identity issue,
  not by weakening the unit lineage or substituting WAT `Q` for PASS `runvol`.
