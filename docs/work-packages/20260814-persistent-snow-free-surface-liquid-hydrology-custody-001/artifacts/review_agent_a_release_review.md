# Review Agent A — Release Rust Correctness Review

Evidence class: `Static exact-commit + Ran exact-commit`

Reviewed commit: `9ab4b1f1786d2f559bbcf54677dfe59b83c94610`

Verdict: `HOLD / NO-GO`.

Static inspection and every command reported below used an isolated
`git archive` of the exact reviewed commit. Later shared-worktree commits and
uncommitted remediation bytes are excluded.

## Findings

### High — B-RELEASE-HIGH-001: unsupported frozen, thawing, and retained-snow-liquid states bypass E004 preflight

`validate_native_shadow_domain()` checks only
`lane.winter_column.snow.has_runtime_state()` and
`lane.snow_runtime_carry.is_some()` in
`crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/mod.rs:820-835`.
It does not reject `winter_column.frost.has_runtime_state()`, a present
`frost_runtime_carry`, or production subsurface layers with positive
`frozen_depth_m`/`frozen_water_m`. In addition,
`DirectSnowLaneState::has_runtime_state()` omits
`liquid_water_retained_m` in
`crates/openwepp-hillslope-orchestrator/src/winter_column.rs:233-242`, so a
retained-snow-liquid-only state with no carry also passes.

These are public, representable production-frame states. They pass the E004
guard before the fixed-cap callback and can continue through surface
authorization, ingress, and the shared same-pass infiltration transition. In
the frozen-layer case, the bridge can therefore apply liquid infiltration to
a domain that `SC-SURFACELIQUID-001` and `SC-LANDSURFACEENERGY-001` require to
reject before candidate work. This is a science-domain and error-precedence
violation, not merely incomplete diagnostics.

Required correction: make the unified-entry preflight reject every snow,
terminal-snow, retained-snow-liquid, frost/thaw runtime/carry, and positive
production-layer frozen-state representation with contextual E004 before the
callback. Add public-bridge poisons for retained-snow-liquid-only,
frost-state-only, frost-carry-only, positive frozen depth, and positive frozen
water; assert callback non-execution, exact lane/OFE/tile context, rollback
hashes, and byte-identical production state.

### High — aggregate same-store demand can overflow and silently replace proportional authorization

`authorize_surface_liquid_withdrawals_inner()` validates each request as
finite, then accumulates `demand_by_store` with unchecked `f64` addition at
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner.rs:1407`.
The aggregate is not checked before division at lines 1425-1441. Two distinct,
finite same-store requests can therefore overflow `D_sum` to positive infinity.
For finite positive supply, every non-final authorization becomes exact zero
because `D_i*S/D_sum == 0`, and the final canonical row receives the entire
floating remainder. All produced amounts remain finite and satisfy the local
`0 <= A <= D` check, while `validate_arbitration()` reproduces the same
overflowed arithmetic, so this non-proportional allocation is accepted.

This silently violates the same-snapshot proportional equation and
`INV-SURFACELIQUID-003/004`; it can proceed through finalized-use debit and
state closure without another guard detecting the allocation drift.

Required correction: fail closed when any demand accumulation or other
authorization intermediate becomes nonfinite, before proportional allocation,
with the canonical contextual domain/candidate error required by the guard
precedence. Add a same-store, distinct-requester overflow vector that proves no
authorization batch is returned, plus a large-finite non-overflow control.

## Confirmed prior-finding disposition

Static exact-commit review confirms that the previously accepted findings are
otherwise materially closed:

- the pre-callback later thermal-expectation mismatch preserves the actual
  soil-thermal owner and first mismatching OFE/tile;
- finalization and final receiver topology checks reject missing, extra,
  reordered, duplicate, and nonfinite thermal receivers with contextual E011;
- the exact three-row LSE/Hydrology/SoilThermal rollback sequence rejects
  missing non-terminal rows by expected absent owner and retains actual owner
  context for malformed, extra, and reordered rows;
- production soil closure reconstructs ordered layer changes and aggregate
  water using `theta_m + residual_theta * max(depth_m-frozen_depth_m,0)`;
- arbitration and candidates are sealed, proportional authorization is
  re-derived, `0 <= F <= A <= D` is checked, and only finalized use is debited,
  subject to the aggregate-overflow finding above;
- signed condensation is credited before overflow, current ingress cannot
  satisfy same-interval authorization, WB14 continuation/restart state is
  digest-bound, and parcel mass/enthalpy/routing joins are independently
  reconstructed;
- canonical configuration/state parsing, emission, digest sensitivity,
  restart combinations, and candidate validation fail closed;
- candidate execution remains clone-only, and no runner selector, production
  default, scheduler dispatch, publication, or activation consumer was added;
  and
- no affected Rust file reaches 3,000 lines. The exact package line-governance
  artifact dispositions remain applicable to every 2,000-line WARN file.

No duplicated constitutive or WB14 transition arithmetic was found. The daily
wrapper and persistent continuation use the shared interval transition.

## Ran at the exact reviewed commit

Working directory: isolated `git archive` of
`9ab4b1f1786d2f559bbcf54677dfe59b83c94610`.

- `cargo nextest run --profile quick --test surface_liquid_hydrology_custody_authority_contract --test land_surface_energy_real_hydrology_shadow_contract`
  — PASS, 27/27.
- `cargo nextest run -p openwepp-hillslope-orchestrator -E 'test(/surface_liquid/)' --profile quick`
  — PASS, 30/30; 507 skipped by the filter.
- `cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings`
  — PASS.

The passing suites do not contain the unsupported-state or aggregate-demand
overflow vectors above and therefore do not clear the HOLD.

## Residual risk and missing tests

- Add the five E004 public-entry poisons named above and retain the existing
  E007/E011 exact-context and callback-nonexecution assertions.
- Add finite-input aggregate-overflow and adjacent large-finite proportional
  authorization vectors.
- Retain all current thermal topology/finiteness, rollback sequence,
  nonzero-residual/frozen-depth aggregate, D/A/F, signed-condensation,
  ingress/restart, serialization/sealing, byte-identical rollback, clone-only,
  and non-activation vectors after remediation.
- This bounded review did not run full-workspace nextest, workspace doctests,
  dependency policy, or a comparator. The package still lacks a passing
  exact-head full-workspace critical-boundary run after its retained historical
  workspace failure.

## Approval statement

`NO-GO`: exact commit `9ab4b1f17` is not acceptable for dependency-package
closure. The two high-severity findings permit unsupported frozen/snow-liquid
candidate work and silent non-proportional authorization. Both require
in-package correction, focused regression evidence, and a fresh exact-byte
release review.
