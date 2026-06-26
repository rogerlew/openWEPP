# SNOWDENSITY-05A Melt Contract + Sign Reconciliation

Status: complete.

Package type: contract/sign-reconciliation package.

Primary contract: `SC-SNOWFREEZE-001`.

Closure: COMPLETE-05A-CONTRACT-SIGN-GATE.

Objective: install the contract authority required before any melt
modernization code: opt-in melt selector shape, shortwave/albedo operand
placeholders, no-radiation-tuning guard, negative-benchmark disposition for
degree-day snowbench variants, and explicit reconciliation of WEPP Chapter 3
`amelt - bmelt + cmelt + dmelt` prose with openWEPP's signed
`melt_bmelt_in` trace convention.

No production runtime code, constants, parser surfaces, output schemas, or
defaults are changed by SNOWDENSITY-05A.

## Decision Boundaries

- `snow_melt_model = legacy_coe | coe_shortwave_albedo_v1` is the accepted
  selector shape. `legacy_coe` remains default and rollback. The opt-in path is
  not implemented in this package.
- `melt_bmelt_in` is an already-signed contribution. Current executable raw
  melt remains `hrmelt_raw = 0.0254 * (amelt + melt_bmelt_in + cmelt + dmelt)`.
  A silent sign flip or double subtraction is invalid.
- `dense_slow_melt_v1` is retained only as a negative benchmark. It must not be
  promoted into production melt physics.
- Shared radiation forcing is governed by `SC-CLIMATE-001#INV-CLIMATE-013` and
  must not be tuned, rescaled, clipped, or reinterpreted to fit snowmelt.
- Shortwave source/provenance is deferred to SNOWDENSITY-05B. Albedo state and
  constants are deferred to SNOWDENSITY-05C. Opt-in production implementation is
  deferred to SNOWDENSITY-05D.

## Required Reading Completed

Static:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/planning/snow-frost-fidelity-strategy.md` sections 2, 4, 5, 7, and 10
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `tests/AGENTS.md`
- Existing melt-term source/test lineage in
  `crates/openwepp-hillslope-orchestrator/src/hydrology/` and
  `tests/integration/clim05_snow_runtime_kernel_contract.rs`

## Write Set

- `Cargo.toml`: registered the new integration test target.
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`:
  bumped to v76 and added the 05A melt-modernization/sign authority.
- `tests/integration/snowdensity05a_melt_contract_guard.rs`: added the
  contract/package guard.
- `docs/work-packages/README.md`: updated active/completed package status.
- Package artifacts under this directory.

## Evidence

Ran:

- Before amendment, `cargo test --test snowdensity05a_melt_contract_guard`
  failed as expected because the contract was still v75, the signed-`bmelt`
  language was absent, and the package was not closed.

Static:

- `SC-SNOWFREEZE-001` v76 contains `INV-SNOWFREEZE-052`,
  `OBL-SNOWFREEZE-P-027`, the selector
  `snow_melt_model = legacy_coe | coe_shortwave_albedo_v1`, the signed
  `melt_bmelt_in` convention, and the no-radiation-tuning guard.
- Production implementation remains deferred. The pre-implementation contract
  gate for 05A is green; the 05D production gate is not claimed.

## Exit Criteria Disposition

- Contract authority exists: satisfied by `SC-SNOWFREEZE-001` v76.
- Contract-derived test exists: satisfied by
  `snowdensity05a_melt_contract_guard`.
- Default behavior unchanged: satisfied by no production runtime edits.
- Opt-in implementation not claimed: deferred to 05D.
- Conservation/routed-melt reconstruction not claimed: deferred to 05D.
- SNOTEL/rubric adjudication not claimed: deferred to 05E.

## HOLD Boundaries Checked

No HOLD was required. The sign/alias convention reconciles: current openWEPP
stores `bmelt` as a signed term and the contract now binds that trace identity.
`SC-CLIMATE-001#INV-CLIMATE-013` is compatible with the no-radiation-tuning
guard. Shortwave source and albedo details are intentionally deferred to 05B
and 05C rather than smuggled into 05A.

## Subagent Authorization

Subagent authorization: not used. This package performed local static review
and verification only because the 05A write set is contract/test/artifact
limited and no production code was changed.

## Security Impact

No external network access was used. No copyrighted PDF content was copied into
the repository. No secrets, tokens, or generated credentials were added.
