# Frozen forest-litter phase authority and implementation

Status: `ACTIVE — CONTRACT-FIRST RED`

Execution mode: `contract-first package-end-to-end`

## Objective

Close the valid `p61` and native-forest production failures by admitting and
implementing the selected ISBA-MEB snow-free forest-litter liquid/ice phase
model with exact mass, fusion-energy, vapor, owner, receipt, restart, rollback,
and real-consumer evidence, then return passing evidence to the workspace gate
hold-lift package.

## Rationale

Current `SC-LANDSURFACEENERGY-001` and `SC-SURFACELIQUID-001` explicitly reject
frozen/thawing surface liquid and mark frozen custody authority missing. The
retained peer-reviewed R-156 PDF supplies equations A1-A14; official SURFEX v8
supplies the exact `3300 s` litter phase timescale and executable bounded update
order. This is a genuine distinct-authority successor, not a solver clamp.

## Included scope

- retain and checksum the exact CeCILL-C official SURFEX v8 source/license;
- contract-first LSE and surface-liquid next-version authority;
- explicit adjudication of R-156 printed A4 sign inconsistency and paper/code
  constant differences;
- snow-free `forest_litter` liquid/ice state, phase transfer, vapor split,
  fusion energy, receipt, digest/restart, current-ingress ordering, and rollback;
- independent oracle/anti-tautology vectors and unchanged `p61`/native consumers;
- consume the parent-owned `WGHL-FULL-001I` soil-thermal owner V2 envelope in
  the new complete-owner projection and successor runtime/restart handoff;
- full applicable gates, dual review, dual verification, and handback.

## Excluded scope

- snow, bare-surface or soil frozen-state admission; soil mutation remains
  parent-owned by `WGHL-FULL-001I` and is not implemented by this child;
- SURFEX tiny-ice cleanup, soil compensation, or hidden tolerance;
- instantaneous equilibrium, bound lowering, temperature clamp, sub-60 stepping,
  fitted parameters, or surrogate phase physics;
- changed WB14 availability chronology, conservation thresholds, or event rules.

## Canonical selection

Use R-156 selected forest-litter equations and `T_ref=273.15 K`; use official
SURFEX v8 `ice_litter` for `tau_ice=3300 s`, bounded kinetic ordering, and the
conservation-resolved sign `phase=freeze-melt`; select `L_f=333700 J kg^-1`
from that named instantiation. Ice capacity is the source's liquid-water-
equivalent `0.85*rho_w*dz`, and both litter phases use the paper's liquid-water
saturation function. Record and test every discrepancy. Refuse `zertol`
cleanup, `xwgmin` frozen-fraction regularization, and later soil correction.
No surrogate physics is allowed.

## Version-16 retained-surface-enthalpy hold-lift amendment

The unchanged `p61` real consumer advanced to
`176400000000000..178200000000000 ns` and then correctly refused a positive
retained surface-energy credit whose checked binary64 addition left the
resident high-term bits unchanged. The retained run did not preserve the exact
beginning high bits or accepted retained tile-credit operands, so this package
must not invent a numeric fixture from the failure prose.

This amendment admits one minimal LSE-owned exact surface-enthalpy companion
under `SC-LANDSURFACEENERGY-001` v16 and `SC-SURFACELIQUID-001` v16. Frozen LSE
V1/V2/V3, surface-owner V1/V2, and complete-projection V1/V2/V3 bytes remain
unchanged. On the successor path their surface-enthalpy field is a
nonauthoritative high mirror joined to authoritative
`U=exact(U_hi)+R_U`. The implementation must aggregate exact named accepted
phase-free, fusion, and retained-ingress tile-credit operands, round once to
finite nearest-even high, retain the exact canonical dyadic remainder, and
bind schema/definition/owner/transaction/support/receipt/restart/checkpoint/
projection/rollback identity. It may not force an ULP, snap zero, discard a
credit, use a tolerance, or feed the carry into temperature, flux, phase, or
WB14 physics.

The exact 60-second minimum fallback, stable larger-support obligation, V14
liquid/ice/fusion behavior, event chronology, topology, custody, closure,
rollback, and fail-closed posture remain unchanged. The amendment reopens
contract-first red and package review closure until production, independent
vectors, split restart, unchanged `p61`, and unchanged native-forest evidence
pass.

### Covered support-receipt adoption-parity amendment

The retained 64 MiB-stack `p61` rerun advanced through the former exact-high
mirror refusal and failed at the same `176400000000000..178200000000000 ns`
support after `193.22 s` with `VEG-E-123`, invalid or mismatched LSE support
receipt. A temporary gated audit proved that receipt and staged native V3 LSE
and soil digests matched through the immediately preceding
`174600000000000..176400000000000 ns` support. The failing covered-segment
path did not enter the already-correct snow-free receipt selector: covered
owner finalization still admitted its receipt from the legacy inner LSE state,
which cannot match the staged native V3/V4 beginning owner.

This amendment authorizes exactly
`crates/openwepp-hillslope-orchestrator/src/v11_covered/owner_finalization.rs`
at the LSE support-receipt beginning-state selection and focused tests. The
covered path must reuse the snow-free legacy-versus-native-V3 staged-byte
selection and fail closed on any mismatch. This is receipt/adoption parity for
the unchanged exact owner, not new physics or a tolerance, support-floor,
closure, publication, or owner-finalization expansion. The temporary audit is
not production behavior and must be absent from the terminal diff.

## Intended write set

- this package tree and `docs/work-packages/README.md`;
- exact retained authority under `references/vendorable/surfex-v8/`;
- `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`;
- `docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md`;
- `docs/specifications/science-contracts/index.md`;
- exact LSE production/test paths prospectively listed in `artifacts/owned-file-manifest.md`;
- exact orchestrator surface-liquid/runtime/restart/test paths prospectively
  listed in that manifest;
- `tests/integration/land_surface_energy_balance_authority_contract.rs` and
  applicable surface-liquid authority/real-consumer tests;
- `docs/sim-contract-boundary-units.md` and the exact boundary-unit registry
  contract binding listed in the owned-file manifest;
- `tools/release/authority-policy/impact-map.json` exact bindings only.

The v16 exact-surface extension additionally authorizes only these prospective
production seams after the retained red:

- `crates/openwepp-land-surface-energy/src/exact_dyadic_enthalpy.rs` (reuse of
  the canonical dyadic wire/arithmetic only);
- new
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner/v3_exact_enthalpy.rs`
  and its included focused test module;
- new
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner/v4_projection.rs`
  and its included focused test module;
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs` (module/export
  wiring only);
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/real_hydrology_execution.rs`
  at `credit_retained_receipt_group` and complete-owner join only;
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/v3_multitile_adoption.rs`
  and the package-owned frozen-litter real-consumer modules for authoritative
  exact-total consumption;
- `crates/openwepp-hillslope-orchestrator/src/v11_covered/owner_finalization.rs`
  only for covered LSE support-receipt legacy/native-V3 staged-owner selection
  and focused legacy/native/mismatch/rollback/no-publication tests;
- new
  `crates/openwepp-persisted-restart-v1/src/frozen_litter_v4_exact_enthalpy.rs`
  and its included focused test module, plus
  `crates/openwepp-persisted-restart-v1/src/lib.rs` wiring only;
- new additive
  `crates/openwepp-persisted-restart-v1/src/snow_stage3_v11_v4_exact_enthalpy.rs`,
  `crates/openwepp-persisted-restart-v1/src/snow_stage3_v11.rs`, and
  `crates/openwepp-persisted-restart-v1/src/hydrology_restart.rs` only for
  atomic preservation/reconstruction of unchanged nested Stage-3/V3 bytes and
  the authoritative V4 exact-enthalpy supplement across the real production
  checkpoint/reload call sites;
- `crates/openwepp-persisted-restart-v1/src/projection.rs` and
  `crates/openwepp-persisted-restart-v1/src/transaction.rs` only to select,
  retain, and reload `DirectHydrologyExactEnthalpyRestartV2` whenever the live
  Stage-3 consumer owns a frozen-litter V4 resident; V1 inputs and V1 wire
  bytes remain unchanged and no unrelated transaction behavior is authorized;
- unchanged `p61` and native-forest fixture drivers only for successor identity
  binding and independent real-consumer assertions.

Any different module path requires a prospective owned-file-manifest amendment
before creation. This amendment does not authorize edits to phase equations,
solver tolerances, temporal floors, WB14 physics, or unrelated runtime paths.

The reopened V16 custody review found that the standalone V4 restart schema
had no production checkpoint/reload caller. The additive Stage-3 V4 restart
seams above are therefore required to prevent a live V16 consumer from losing
both native frozen-litter residents and silently selecting a legacy path after
reload. Existing V1/V2/V3 wire bytes remain nested unchanged; missing,
substituted, stale, or noncanonical V4 supplements fail before host mutation.

The owner-authorized V16 restart-callsite correction additionally permits
`crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_attachment.rs` and
its existing runtime-accessor seam, plus
`crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/frozen_litter_v4_adoption.rs`,
only for (a) read-only detection that any retained Stage-3 consumer owns a V4
resident so V1 projection refuses omission and (b) atomic, fully validated
replacement of the hydrology frame reconstructed by
`DirectHydrologyExactEnthalpyRestartV2`. This is restart-only custody wiring;
it creates no general mutation API, acceptance bypass, or unrelated attachment
or shadow behavior.

Canonical run r96b then proved that initial and covered-finalized V11 staged LSE
bytes still selected the legacy/V3 mirror even though the real consumer owned
the mandatory V4 envelope. The correction may additionally edit exactly
`crates/openwepp-hillslope-orchestrator/src/canonical_owner_bytes.rs` and the
already-owned `v11_covered/owner_finalization.rs` solely to select the existing
augmented V4 LSE bytes for initial/staged complete-owner publication whenever a
V4 resident exists. Legacy/V3 behavior is unchanged when V4 is absent; no
physics, tolerance, receipt, or owner-topology change is authorized.

The accepted-credit restart proof may add exactly one
`#[cfg(feature = "restart-authority-evidence")]` helper module below
`crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/`, plus
module wiring only. It must reuse the authentic frozen-litter V3/V4 executor
and negative-zero no-op fixture and expose only public accepted owner,
projection, receipt, and seal constituents needed by persisted-restart tests.
It is forbidden from the default/production build and may not change physics
or production APIs. The persisted test must prove V2 checkpoint/reload and
next-support continuity, V1 omission refusal, failed-restore rollback, and
signed-zero preservation.

The feature-only helper may change the existing crate-private
`FrozenLitterV3PhaseFreeInput::from_authority_operands_for_test` gate in
`land_surface_energy_shadow/v3_input_projection.rs` from `cfg(test)` to
`cfg(any(test, feature = "restart-authority-evidence"))`. This one-line
prerequisite exposes no public/default API and authorizes no other projection
change.

The real V16 exact transaction checkpoint exposed one pre-existing tagged-
parent wire defect: a live Stage-3 checkpoint nests `V8CoupledOwnedState`,
whose raw `u128` lineage cannot be decoded through serde's internally tagged
checkpoint phase. The child may additionally edit exactly
`crates/openwepp-vegetation/src/v8_state.rs` and its existing inline focused
test module to add an explicit canonical u128 codec. Existing values through
`u64::MAX` must retain their numeric JSON bytes; larger values use canonical
decimal strings. Negative, fractional, leading-zero, plus-prefixed, overflow,
and otherwise noncanonical forms fail closed. No vegetation physics, state
identity, or unrelated serialization change is authorized.

The child may bind and consume the parent-owned `SoilThermalOwnerEnvelopeV2`
and successor restart identity in `SurfaceLiquidCompleteOwnerProjectionV3` and
its V3 runtime/restart integration. It may not define or mutate the soil carry,
soil owner, soil receipt, or soil restart schema. The parent 001I contract-first
handoff must be stable before those child integration edits begin.

## Mandatory sequence

1. Freeze authority bytes, checksums, license, and discrepancy decision.
2. Amend canonical contracts and contract-derived tests.
3. Record the pre-implementation failing gate on unchanged production.
4. Implement production state/physics/custody/restart atomically.
5. Run focused, authority, anti-evasion, A0/A1/A3, closure, real-consumer,
   workspace-facing, line-count, and terminal gates.
6. Complete dual reviews, disposition, dual verification, and handback.

## Acceptance

Independent reconstruction must prove equal liquid debit/ice credit and
`L_f*m_phase` energy; phase-specific vapor cannot double-debit; rejected work
must preserve exact bytes. Anti-tautology vectors must distinguish wrong sign,
wrong `T_ref/rho_i/L_f/tau`, instantaneous projection, freeze-only, hidden
cleanup, ice-as-WB14 supply, current-ingress donation, and producer residuals.

Risk is `CRITICAL`. Parameter posture is implementation/calibration-not-
applicable: fixed published constants only, no empirical calibration claim.

`SC-EVAP-001` remains unchanged: its daily WB17 soil/residue/canopy ET scope
does not own this subdaily, pre-WB14 surface-liquid/ice vapor transaction.
SC-LSE v14 owns that transaction and cross-binds SC-SURFACELIQUID v14 and
SC-WATBAL-001 without admitting litter ice as WB14 supply or soil `frozwt`.

Subagent authorization: REQUIRED. Standing user authority explicitly permits
workers for bounded implementation ownership, comparator runners for heavy
gates, and two independent reviewers/verifiers. Workers may edit only assigned
prospective paths and must preserve concurrent work.

Exit requires both real consumers passing, independent mass/energy closure,
exact rollback/restart evidence, no unreviewed findings, and a stable increment
committed before the parent workspace full-profile rerun.
