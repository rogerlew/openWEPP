---
contract_id: SC-ROOTZONEHYDRAULICS-001
title: Root-Zone Hydraulic Boundary Contract
status: open
maturity: draft
owner: openWEPP maintainers + soil/plant hydraulics reviewers
contract_version: 0.1.0-hold
producer_scope:
  - Prospective per-layer retention and root-path owner
consumer_scope:
  - Default-off V10 vegetation and LSE-V2 real-consumer shadow
evidence_level: static
last_reviewed: 2026-08-19
supersedes: []
superseded_by: []
---

# Root-Zone Hydraulic Boundary Contract

Status: `open / draft / HOLD`

Authority identity: `OPENWEPP_ROOT_ZONE_HYDRAULIC_OWNER_V1`

## Scope

This contract reserves the narrow owner boundary needed to replace caller-built
`SoilLayerForcing` hydraulic operands. It does not release an executable model.
The boundary owns per-OFE/layer retention configuration, live-state projection,
and per-occupancy root-tissue paths. V10 equations remain unchanged.

## Candidate V1 equations

For an unfrozen layer, in the written operation order:

```text
theta_liq = live_liquid_water_depth_m / layer_thickness_m
raw_relative_saturation = theta_liq / porosity
relative_saturation = min(1, max(0.01, raw_relative_saturation))
matric_potential_mm =
    max(psi_sat_mm * relative_saturation^(-B), -1e8)
```

Layer-node gravity is:

```text
top_depth_m[0] = 0
top_depth_m[i] = sum(thickness_m[j], j < i)
node_depth_m[i] = top_depth_m[i] + 0.5 * thickness_m[i]
gravity_root_mm[i] = 1000 * node_depth_m[i]
```

Required immutable domains are `psi_sat_mm < 0`, `B > 0`, and an explicit
per-occupancy/per-layer `root_path_length_mm > 0` satisfying
`root_path_length_mm >= gravity_root_mm`. No default or geometry alias is
admitted. In particular, WB14 wetting-front suction and
`RootLayer.lateral_root_length_m` are forbidden substitutes.

## Ownership boundary

The prospective configuration binds ordered OFE/layer identity, production
lane identity, vegetation/LSE/hydrology configuration identities, retention
parameters, and explicit occupancy root paths. A private interval receipt must
bind current hydrology state, soil thermal state, transaction/day/interval,
configuration digests, and independently reconstructed operands before V10 may
consume it.

## Provenance

| Source | Version / locator | Exact use | Evidence |
|---|---|---|---|
| Community Terrestrial Systems Model, *CLM5.0 Technical Note: Plant Hydraulics* | release-clm5.0, section 2.11.2.1.3, equations 2.11.14--2.11.18; https://escomp.github.io/CTSM/release-clm5.0/tech_note/Plant_Hydraulics/CLM50_Tech_Note_Plant_Hydraulics.html | Parallel soil-to-root flow, two series resistances, distinct `z3` and `dxroot`, gravity and soil/root potential gradient | Retrieved 2026-08-19; HTML SHA-256 `4228822c94293f6673adf12b0fbb7d4e3a78f72e5c268eecb9cefef75ba36cee` |
| Community Terrestrial Systems Model, *CLM5.0 Technical Note: Hydrology* | release-clm5.0, section 2.7.3.1, equations 2.7.49--2.7.55 | Porosity, B, node-depth matric potential, relative-saturation bounds and `-1e8 mm` floor | Retrieved 2026-08-19; HTML SHA-256 `fff5080f4b9285bfa19bca4f7913b17e93c341138249f70d515c6706b5cced09` |
| Clapp, R. B. and Hornberger, G. M. (1978), *Empirical equations for some soil hydraulic properties* | Water Resources Research 14(4), 601--604; DOI `10.1029/WR014i004p00601` | Retention and conductivity power functions; wetting-front suction is a derived Green-Ampt quantity, not matric-potential identity | Citation/DOI metadata only; copyrighted source bytes are not vendored |

The claim ceiling is boundary and equation lineage only. It is not calibration,
validation, transferability, or parameter-value authority for openWEPP sites.

## Blocking authority contradiction

`RootLayer.lateral_root_length_m` is already the soil-interface distance
`dxroot`. No repository input, immutable vegetation configuration field,
hydrology configuration field, or admitted builder supplies the distinct
root-tissue path `z3` for required rooted scenarios. Existing positive
`root_path_length_mm`/`z3_m` values are caller-owned templates or tests.

Additionally, `DirectSubsurfaceLayerState.conductivity_m_s` is copied from the
immutable subsurface layer input and used as a saturated/base conductivity;
the runtime applies a moisture-dependent factor separately. It is not an
admitted current unsaturated soil-root conductivity.

Therefore the owner cannot be released without inventing at least one material
operand. The first lift action is to admit a real input/configuration source for
explicit per-occupancy/per-layer root-tissue path values and independently
admit the unsaturated conductivity relation and its parameters. Production
implementation is forbidden while this HOLD remains.

## Change log

| Date | Version | Change |
|---|---|---|
| 2026-08-19 | `0.1.0-hold` | Reserved the narrow owner and recorded the exact root-path and conductivity authority blockers; no executable authority released. |
