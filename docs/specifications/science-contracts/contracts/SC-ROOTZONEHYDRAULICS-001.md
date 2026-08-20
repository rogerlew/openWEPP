---
contract_id: SC-ROOTZONEHYDRAULICS-001
title: Root-Zone Hydraulic Owner Contract
status: approved
maturity: active
owner: openWEPP maintainers + soil/plant hydraulics reviewers
contract_version: 1.0.0
producer_scope: [OPENWEPP_ROOT_ZONE_HYDRAULIC_OWNER_V1]
consumer_scope: [Default-off V10 vegetation and LSE-V2 real-consumer shadow]
evidence_level: primary-source constitutive authority + contract vectors
last_reviewed: 2026-08-19
supersedes: []
superseded_by: []
---

# Root-Zone Hydraulic Owner Contract

Status: `approved / active / implemented in default-off V10 shadow`

Authority identity: `OPENWEPP_ROOT_ZONE_HYDRAULIC_OWNER_V1`

## Purpose and scientific scope

This contract defines the immutable configuration and interval-local sealed
receipt that replace caller-built root-zone hydraulic operands in the
default-off V10 consumer. It does not change V10 plant-hydraulic equations,
activate a selector, or authorize calibration. V1 is snow-free and rejects a
frozen rooted layer.

## Authority anchors

| Source | Binding use | Evidence |
|---|---|---|
| CTSM CLM5 Plant Hydraulics, release-clm5.0, 2.11.2.1.3, eqs. 2.11.14--18 | distinct `z3`/`dxroot`, series resistances, gravity, layer-local current K | `[DIRECT]` retrieved 2026-08-19; HTML SHA-256 `4228822c94293f6673adf12b0fbb7d4e3a78f72e5c268eecb9cefef75ba36cee` |
| CTSM CLM5 Hydrology, release-clm5.0, 2.7.3.1, eqs. 2.7.47 and 2.7.49--55 | Brooks--Corey intrinsic `2B+3` factor, node geometry, retention, and `-1e8 mm` floor; the vertical-interface averaging/ice operator is not imported | `[DIRECT]` retrieved 2026-08-19; HTML SHA-256 `fff5080f4b9285bfa19bca4f7913b17e93c341138249f70d515c6706b5cced09` |
| ESCOMP/CTSM `fdfc03a0abec838ec0b66663a6a2f5e619cd2611`, `PhotosynthesisMod.F90` lines 2671, 2680, 2835, 2841, 2847, 4644--4663, 4732--4771 | layer-local current `hk_l`, distinct root lateral path plus node depth, positive-down node depth, and signed `smp-root-1000*z` gradient | `[DIRECT]` immutable source inspected 2026-08-19 |
| Clapp & Hornberger (1978), WRR 14(4), DOI `10.1029/WR014i004p00601` | retention/conductivity power-law lineage | `[DIRECT]` citation metadata; source bytes not vendored |

CLM PFT/root defaults, hardcoded coarse-root length, WB14 suction/K, and
vertical-interface K averaging are explicitly not used.

## Variables and units

`theta_liq` (m3/m3) is live volumetric liquid; `S_raw`, `S`, `S_psi` are
dimensionless saturation; `psi_sat` and `psi` are mm; `B` is dimensionless;
`Ksat`/`Ksoil` are m/s; `z_node` and required stratum `z_lateral` are m;
`gravity_root` and root-tissue path `z3` are mm; soil-interface `dxroot` is m.

## Algorithm state surfaces

Configuration binds schema/model/configuration/owner identities; hydrology,
vegetation and LSE configuration identities; strictly ordered OFE/lane/layer
`(psi_sat,B)` records; and exactly one required
`root_tissue_lateral_path_m >= 0` for every rooted stratum, including explicit
zero. There is no default.

Canonical configuration bytes are UTF-8 JSON with object keys sorted
lexicographically, arrays retained in authority order, no insignificant
whitespace, and one terminal LF. `configuration_sha256` is empty while hashing
and is then filled with lowercase SHA-256. Validation requires unique strictly
increasing OFE/lane/layer tuples and unique strictly increasing stratum IDs.
The rooted stratum set must equal the bound vegetation configuration; layer
order must equal bound hydrology and LSE topology.

Every interval independently projects ordered current staged hydrology layers
(`DirectSubsurfaceLayerState.theta_m`, `depth_m`, `porosity`, and
`conductivity_m_s`) and vegetation root bindings keyed by
occupancy/stratum/OFE/lane/layer (`RootLayer.lateral_root_length_m` and
accessibility). Their canonical projections have separate SHA-256 identities;
multiple occupancy/stratum bindings may reference one hydrology layer. A
private immutable receipt binds both source digests, transaction, day,
interval, occupancy/stratum/OFE/lane/layer, and vegetation/LSE/configuration
identities. No scientific state is mutated.

Required domains are: finite `liquid_water_depth_m >= 0`, finite
`layer_thickness_m > 0`, finite `0 < porosity <= 1`, finite `Ksat > 0`, finite
`psi_sat < 0`, finite `B > 0`, finite `layer_top_depth_m >= 0`, finite
`root_tissue_lateral_path_m >= 0`, and finite `dxroot > 0`.

## Algorithm specification

In exact written grouping/order for each exact ordered OFE/layer:

```text
theta_liq = liquid_water_depth_m / layer_thickness_m
S_raw = theta_liq / porosity
S = min(1.0, max(0.0, S_raw))
S_psi = max(0.01, S)
psi = max(psi_sat * libm::pow(S_psi, -B), -1.0e8)
conductivity_exponent = 2.0 * B + 3.0
raw_Ksoil = Ksat * libm::pow(S, conductivity_exponent)
Ksoil = min(Ksat, raw_Ksoil)
soil_conductivity_mm_s = 1000.0 * Ksoil
layer_top[0] = 0.0
layer_top[i] = ordered_sum(thickness[j], j < i)
z_node = layer_top[i] + 0.5 * thickness[i]
gravity_root = -1000.0 * z_node
z3 = 1000.0 * (z_node + root_tissue_lateral_path_m[stratum])
dxroot = RootLayer.lateral_root_length_m
```

Precondition priority is canonical configuration/digest, cross-configuration
topology, transaction/cadence, current hydrology digest, scalar domains,
frozen/accessibility posture, pore capacity, equations, then receipt digest.
The postcondition is one fully validated immutable receipt or a typed error
with no owner mutation. Inaccessible/unrooted layers emit no receipt; frozen
accessible rooted layers fail.

At exact `S=0`, current K is positive zero. `S_psi` is never used for K.
Before division, compute `capacity = porosity * layer_thickness_m` in that
order and `capacity_one_bit = f64::from_bits(capacity.to_bits() + 1)` (capacity
is finite and positive). Reject when `liquid_water_depth_m > capacity_one_bit`.
Thus exact capacity and its immediate next representable f64 are admitted; any
larger value rejects before the upper clamp.

## Branch and guard table

| Trigger | Typed failure |
|---|---|
| missing/duplicate/out-of-order identity or rooted stratum | `ConfigurationIdentity` |
| nonfinite or invalid thickness/porosity/Ksat/psi_sat/B/path | `Domain` |
| material liquid water above pore capacity | `WaterAbovePoreCapacity` |
| accessible rooted frozen layer | `FrozenRootedLayerUnsupported` |
| attempted receipt for inaccessible/unrooted binding | `InaccessibleRootedLayer` |
| source/configuration/transaction/cadence mismatch | `OwnerJoin` |
| digest mismatch or caller receipt construction | `ReceiptDigest` / impossible API |

## Invariants and invariant guard map

- `INV-RZH-001`: `0<=S<=1`, `0.01<=S_psi<=1`.
- `INV-RZH-002`: `-1e8<=psi<0`, `0<=Ksoil<=Ksat`.
- `INV-RZH-003`: `gravity_root<=0`, `z3>0`, and
  `z3>=abs(gravity_root)`. Positive-down node depth is converted to the signed
  CLM gravitational-potential change because existing V10 adds this operand.
- `INV-RZH-004`: `z3` uses required `z_lateral`; `dxroot` uses only existing
  `lateral_root_length_m`; they are never aliased.
- `INV-RZH-005`: WB14 suction/K never enter the owner.
- `INV-RZH-006`: receipts bind current staged hydrology and are rebuilt every interval.
- `INV-RZH-007`: ordering and explicit canonical fields bind digests; no `Debug` bytes.

Each invariant maps to the typed table, exact-bit vectors, and poison matrix;
any missing executable mapping blocks release.

## Symbol alias and unit-governance map

`theta_m -> liquid depth`; `depth_m -> thickness`; `conductivity_m_s -> Ksat`;
`root_tissue_lateral_path_m -> z_lateral`; `root_path_length_mm -> z3`;
`RootLayer.lateral_root_length_m -> dxroot`; `matric_potential_mm -> psi`;
`hydraulic_conductivity_mm_s -> current Ksoil`. Metre-to-mm conversions use
explicit `1000.0`. No publication surface is added.

## Constants, tolerances, and numerical identity

`0.01`, `-1e8 mm`, and `2B+3` follow cited CLM5. Power is pinned to
`libm 0.2.16` `libm::pow`. Inputs normalize admitted signed zero to positive
zero; NaN/inf reject. Vectors compare every f64 with exact `to_bits`; the
independent calculator emits 16-digit hexadecimal bits. The pore-capacity
tolerance admits exactly the specified one-bit-above-capacity vector and no
larger excess. Python's independent semantic calculator is not the exact-bit
oracle: emitted bits must also pass the Rust `libm` evaluator for every vector.

| Constant/parameter | Domain/provenance | Custody |
|---|---|---|
| `0.01`, `-1e8 mm`, `2B+3` | cited CLM5 constitutive relations | model definition |
| `1000 mm/m` | exact SI conversion | model definition |
| `psi_sat < 0`, `B > 0` | external site/OFE/layer input | configuration |
| `root_tissue_lateral_path_m >= 0` | external stratum geometry; no default | configuration |
| `Ksat > 0`, liquid/thickness/porosity | current hydrology | interval source |

## Calibration and identifiability posture

`science_implementation_status=IMPLEMENTED`;
`calibration_evidence_status=NOT_CALIBRATION_READY`;
`identifiability_status=NOT_ASSESSED`. External `psi_sat`, `B`, and path values
have no universal default or calibrated/validated/transferable claim.

| Readiness obligation | Disposition | Evidence/rationale |
|---|---|---|
| constitutive equations | PASS | anchors and exact algorithm |
| typed/enumerable parameters | PASS | closed schema/configuration vector |
| deterministic execution | PASS | Python semantic + Rust exact-bit evaluators |
| observation operator | NOT_APPLICABLE | no empirical-calibration intent |
| empirical calibration/validation | NOT_APPLICABLE | explicitly excluded |
| identifiability/synthetic recovery | NOT_APPLICABLE | not a readiness package |

## Test-vector obligations

Accepted and rejected vectors cover saturation/clamps/dryness/pore roundoff,
multiple depths/OFE parameters/strata, explicit zero/positive paths, z3/gravity
and z3/dxroot separation, K below/equal Ksat, signed zero, one-bit boundaries,
frozen/inaccessible posture, WB14 substitution, Ksat-as-current-K, S_psi-for-K,
wrong exponent/order/domains/identities/digests, missing/defaulted/aliased path,
wrong geometry order, and caller-created receipts.

## Gap register and promotability

The former contradictions are resolved by the required non-defaulted stratum
path and cited layer-local Brooks--Corey operator. Independent calculators,
contract-derived tests, and three independent reviews passed exact authority
commit `b30f42de67136bca37f888fa62e8f1145537a230`. Terminal authority release
subsequently authorized the default-off implementation, which completed at
`3ea08d81d966ccbf163ee64377aa741308e2665a` with dual terminal PASS.

## Binding Exposure Index

Historical HOLD artifacts are `superseded` as blockers but retained as
provenance. The active binding set is `INV-RZH-001..007`, the typed guard table,
numeric identity, and test-vector obligations above. No package-local addendum
adds hidden executable authority.

## Change log

| Date | Version | Change |
|---|---|---|
| 2026-08-19 | `0.1.0-hold` | Recorded missing root-path/current-K authority. |
| 2026-08-19 | `1.0.0-rc1` | Admitted required stratum geometry, live Brooks--Corey relations, exact operation order and sealed receipt custody; candidate pending vectors/review. |
| 2026-08-19 | `1.0.0` | Approved after exact-bit vectors, executable owner/poison authority, and three independent PASS reviews on `b30f42de67136bca37f888fa62e8f1145537a230`. |
| 2026-08-19 | `1.0.0` | Recorded completed default-off implementation at `3ea08d81d966ccbf163ee64377aa741308e2665a`; no equation or authority version change. |
