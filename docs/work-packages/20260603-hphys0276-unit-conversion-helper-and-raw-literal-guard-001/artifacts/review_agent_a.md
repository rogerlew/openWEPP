# Review Agent A

Status: complete
Evidence mode: Static + Ran

Static: reviewed AGENTS.md, package.md, unit-governance authority, unit-safe
boundary contract/architecture docs, current worktree diff, and requested
production/tool/test paths.

Ran:
- `cargo fmt --check` (pass)
- `cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings` (pass)
- `cargo clippy --manifest-path crates/openwepp-unit-boundary/Cargo.toml --all-targets -- -D warnings` (pass)
- `cargo test --manifest-path crates/openwepp-unit-boundary/Cargo.toml` (pass)
- `cargo test -p openwepp-hillslope-orchestrator` (pass)
- `cargo test --test hphys0275_boundary_value_dimensional_typing_contract` (pass)
- `cargo test --test hphys0276_raw_unit_conversion_guard_contract` (pass)
- `tools/release/check_raw_unit_conversions.sh` (pass)
- `tools/release/check_raw_unit_conversions.sh --inventory-all-production` (ran; reports remaining inventory outside first-wave enforcement)

## Findings

### A1. Medium - first-wave SIMIMPL28 seconds-to-hours conversion remains raw and unguarded

Required disposition: accepted before package closure, unless rejected with an
explicit contract/package scope amendment that moves seconds-to-hours out of
HPHYS0276 first-wave enforcement and links a follow-up package.

Evidence:
- `docs/specifications/unit-governance.md:137` to
  `docs/specifications/unit-governance.md:139` defines first-wave
  time/rate helpers as `h <-> s`.
- `tools/release/check_raw_unit_conversions.py:29` to
  `tools/release/check_raw_unit_conversions.py:34` only catches `3600.0`,
  `3_600.0`, and `3.6e6` for hour/second conversions.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs:607`
  still computes `stmdur_s * 0.000_277_78`, a seconds-to-hours conversion in a
  default guard-enforced production file.
- `tools/release/check_raw_unit_conversions.sh` passed, so the guard currently
  misses this unauthorized reciprocal literal.
- `crates/openwepp-unit-boundary/src/lib.rs:134` to
  `crates/openwepp-unit-boundary/src/lib.rs:137` adds `hours_to_seconds`, but
  there is no matching `seconds_to_hours` helper despite the governance wording.

Risk:
This leaves a raw directional time conversion in the exact SIMIMPL28 climate
path HPHYS0276 claims to guard. The runtime value is behavior-preserving today,
but the guard does not enforce the package objective and would not catch the
same wrong-direction defect class that motivated the work.

Required fix:
Add canonical `seconds_to_hours` authority or explicitly narrow the contract,
replace the SIMIMPL28 literal with the named helper, add direction tests, and
extend the raw-literal guard/test fixtures so reciprocal hour/second literals
such as `0.000_277_78` cannot bypass first-wave enforcement.

## Residual Risk And Missing Tests

- Static: the converted radiation, snow-density/depth, WB19 drainage, and
  process-rate replacements preserve the reviewed arithmetic direction and
  route helper failures to typed domain errors.
- Static: `docs/architecture/unit-safe-boundary-types.md:60` to
  `docs/architecture/unit-safe-boundary-types.md:71` lists only part of the
  helper surface; align it when disposing A1 so architecture/docs do not lag the
  canonical contract.
- Ran: focused crates/tests and guard checks passed.
- Not run: full `cargo test --workspace`, `cargo deny check`, or legacy
  comparator harnesses.

## Initial Approval Statement

Superseded by the disposition update below. Initial review did not approve
HPHYS0276 closure while A1 remained open. No additional correctness blockers
were found in the reviewed helper directions or typed error mapping.

## Disposition Update

Static: A1 accepted and fixed after review. The implementation now includes
canonical `seconds_to_hours` and `seconds_to_legacy_stmtim_hours` helpers,
replaces the SIMIMPL28 raw seconds-to-hours literal, extends the guard to catch
reciprocal hour/second literal spellings, and updates helper/guard tests.

Ran:
- `tools/release/check_raw_unit_conversions.sh`: pass.
- `cargo test -p openwepp-unit-boundary`: pass, 24 tests.
- `cargo test --test hphys0276_raw_unit_conversion_guard_contract`: pass, 5 tests.

Disposition: accepted/fixed; no open A-findings remain.
