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
