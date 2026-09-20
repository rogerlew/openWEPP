[Parent contract](../SC-LANDSURFACEENERGY-001.md)

## Dependencies
| Target | Required when | Boundary/obligation | Reading extent |
|---|---|---|---|
| common-details.md#common-details | physical rules, solver correctness or accepted-primitive closure | shared physical symbols, state, algorithm, guards, units, tolerances and tests | whole chapter |
| replay-evidence.md#qualification | executable identity capture, experiment or replay-retention qualification claims | frozen protocols and separate scientific/production limits | whole chapter |

<a id="interface"></a>
# Shared interface

Energy/mass inward positive; outgoing water nonnegative. Preserve units, interval,
lineage, tile/OFE basis; physical binary64 conversion precedes exact decode.
Owners: surface thermal=LSE; water=hydrology; soil state=soil thermal; canopy
physiology=vegetation. Immutable-beginning/current-ingress order, exactly-once
opposite-sign transfers, typed failure and all-owner atomicity bind.

V1=snow-free; V2 imports V1 with V10 specialization; V3 adds admitted snow-free litter phase.
Represented snow: separate native map, inactive litter. Regime precedes iteration.
Covered physical floor=60000000000 ns; 1 ns structural chronology is not physical
admission. Apply all support rules.

Contract revision3 is not modelV3. Only named V1 missing/future/ownership labels are
superseded; conservation/failure/owner rules survive. Litter spill adds its negative exact
operand; V27 finalizes its own pending physical prefix; V30 validation reuse stays at original
positions. All mechanism qualifiers bind.

Authority never activates/qualifies production. Retain FAIL/HOLD, expected-red seams,
frozen identities, paused EXP-R/PC1/SG1. Identity proves no science result.
Uncertainty expands reading: EVERY selected external single-file contract is read
WHOLE, even for bounded claims. No silent defaults, unauthorized normalization,
replay fallback or duplicate owners.

Provenance shorthand `v31:Lx-Ly` means those exact original lines at commit
`b932db101cce07d0860b45b5ecaa8ddb7f455b58`, path
`docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`.
Citations remain binding; logical IDs resolve via binding-index.

<a id="canonical-invariants"></a>
## Canonical invariants
| Invariant ID | Statement | Authority | Evidence | Guard | Failure posture |
|---|---|---|---|---|---|
| <a id="INV-LANDSURFACEENERGY-001"></a> `INV-LANDSURFACEENERGY-001` | Every dimensional operand has explicit units, interval, area basis, finite domain, and provenance. | REF-009, unit governance | `[INFERENCE][Static]` | profile/test | hard `HOLD` |
| <a id="INV-LANDSURFACEENERGY-002"></a> `INV-LANDSURFACEENERGY-002` | `A > 0`, `dt > 0`, temperatures satisfy their declared absolute/log domains, and resistances/conductivities satisfy admitted domains. | REF-009 | `[INFERENCE][Static]` | future runtime/test | typed failure |
| <a id="INV-LANDSURFACEENERGY-010"></a> `INV-LANDSURFACEENERGY-010` | The energy identity is independently reconstructible with every signed radiative, turbulent, advective, ground, and storage component exactly once. | REF-009 | `[INFERENCE][Static]` | future runtime/test | `LSEB-E-011` |
| <a id="INV-LANDSURFACEENERGY-011"></a> `INV-LANDSURFACEENERGY-011` | Surface-liquid start, inputs, debits, and end storage close independently. | REF-001/004/009 | `[DIRECT][Static] + [INFERENCE][Static]` | future runtime/test | `LSEB-E-012` |
| <a id="INV-LANDSURFACEENERGY-012"></a> `INV-LANDSURFACEENERGY-012` | Actual liquid evaporation has one shared mass/latent-energy identity and one state debit. | REF-005/009 | `[DIRECT][Static] + [INFERENCE][Static]` | future runtime/test | `LSEB-E-013` |
| <a id="INV-LANDSURFACEENERGY-013"></a> `INV-LANDSURFACEENERGY-013` | Surface `G` and soil/frost `-G` are one interface transfer, never two production fluxes. | REF-006/009 | `[DIRECT][Static] + [INFERENCE][Static]` | future runtime/test | `LSEB-E-014` |
| <a id="INV-LANDSURFACEENERGY-014"></a> `INV-LANDSURFACEENERGY-014` | Each precipitation, runon, infiltration, and runoff mass crossing has exactly one linked advected-energy term with the same lineage/reference state. | REF-004/009 | `[DIRECT][Static] + [INFERENCE][Static]` | future runtime/test | `LSEB-E-010/011` |
| <a id="INV-LANDSURFACEENERGY-015"></a> `INV-LANDSURFACEENERGY-015` | End liquid storage is non-negative within its admitted mass tolerance; material negative storage is never clamped. | REF-009 | `[INFERENCE][Static]` | future runtime/test | `LSEB-E-015` |
| <a id="INV-LANDSURFACEENERGY-020"></a> `INV-LANDSURFACEENERGY-020` | Snow-present, snow-terminal, and snow-free branches are mutually exclusive with the declared priority. | REF-008 | `[DIRECT][Static]` | future runtime/test | `LSEB-E-020/021` |
| <a id="INV-LANDSURFACEENERGY-021"></a> `INV-LANDSURFACEENERGY-021` | Schema-v8 terminal liquid, energy, and time are censored and cannot enter this contract. | REF-008 | `[DIRECT][Static]` | contract/runtime/test | `LSEB-E-021` |
| <a id="INV-LANDSURFACEENERGY-022"></a> `INV-LANDSURFACEENERGY-022` | LSE cannot reorder or duplicate hydrology/ET mutations; it consumes sealed handoffs. | REF-004/005/007 | `[DIRECT][Static]` | scheduler/consumer gate | hard `HOLD` |
| <a id="INV-LANDSURFACEENERGY-030"></a> `INV-LANDSURFACEENERGY-030` | Climate owns forcing and phase; LSE owns surface flux evaluation, not forcing correction. | REF-007 | `[DIRECT][Static]` | ownership test | hard `HOLD` |
| <a id="INV-LANDSURFACEENERGY-031"></a> `INV-LANDSURFACEENERGY-031` | Constitutive equations remain with their named owner; conservation orchestration does not confer duplicate ownership. | REF-007/009 | `[INFERENCE][Static]` | review/test | hard `HOLD` |
| <a id="INV-LANDSURFACEENERGY-032"></a> `INV-LANDSURFACEENERGY-032` | Producer-only, skeleton-only, or serialized ledger evidence cannot prove a runtime path; a real downstream consumer must read and act on the result. | governance | `[DIRECT][Static]` | consumer gate | hard `HOLD` |
| <a id="INV-LANDSURFACEENERGY-040"></a> `INV-LANDSURFACEENERGY-040` | No production implementation is promotable while any required family is `AUTHORITY_MISSING`. | REF-003-010 | `[DIRECT][Static] + [INFERENCE][Static]` | governance gate | `NON_PROMOTABLE` |
| <a id="INV-LANDSURFACEENERGY-041"></a> `INV-LANDSURFACEENERGY-041` | Comparator agreement, current helper code, or silent legacy clamps cannot substitute for canonical physics authority and typed guards. | ADR-0017, REF-010 | `[DIRECT][Static]` | review/gate | hard `HOLD` |
| <a id="INV-LANDSURFACEENERGY-042"></a> `INV-LANDSURFACEENERGY-042` | Future vegetation radiation receipts remain recipient-specific across canopy strata, ground, litter, snow, soil, ponded water, and atmosphere; no recipient is an alias or residual bucket for another. | SC-VEGETATION-001#INV-VEGETATION-021 | `[INFERENCE][Static]` | future integration/test | hard `HOLD` |
| <a id="INV-LANDSURFACEENERGY-043"></a> `INV-LANDSURFACEENERGY-043` | Interval-integrated Stage C transpiration mass and its latent-energy debit share one transaction, stratum, area, interval, lineage, and authority-tagged `h_v`, satisfying `Q_T,s=-h_v*T_s` exactly once. | SC-VEGETATION-001#INV-VEGETATION-014 | `[INFERENCE][Static]` | future integration/test | hard `HOLD` |

<a id="canonical-obligations"></a>
## Canonical obligations
| Obligation ID | Statement | Applicability | Authority | Enforcement/failure | Test bindings |
|---|---|---|---|---|---|
| <a id="OBL-LANDSURFACEENERGY-C-004"></a> `OBL-LANDSURFACEENERGY-C-004` | a real scheduler consumer must prove that the new state and ledger affect the intended direct path before runtime closure. | All scheduler/direct-path consumers claiming runtime closure | v31:L332-L333 | [Guards/errors](water-vapor.md#errors) | [Tests](common-details.md#tests); [Detail](common-details.md#tests); named fixtures/tests/real consumers |

## Cold-canopy M1 prospective interface

For review-pending `OPENWEPP_C3_WOODY_COLD_M1_V1`, vegetation supplies one
authoritative `(M,H)` reservoir per occupancy, phase-selected external vapor
operands and post-solve liquid-only release. LSE assembles the full shared-air,
radiation, sensible, vapor, ground and soil residuals plus both reservoir
balances; it never copies a vegetation residual. Only diagnosed liquid with
mass, liquid enthalpy, temperature and exact occupancy/tile/support/area lineage
may enter the existing `DirectSurfaceLiquidParcelReceipt` receiver path. Canopy
ice never enters ground snow, surface liquid or hydrology. Any phase, owner,
support, enthalpy, duplicate or late receiver mismatch rejects atomically.

<a id="m1-diagnostic-envelope"></a>
### M1 diagnostic implementation envelope

The first detached M1 entry admits exactly two distinct occupancies in their
configured upper-to-lower order and exactly six distinct ordered soil nodes.
Caller-supplied ordered occupancy and soil identities bind the prepared column,
phase records and per-occupancy supplied conditions to that topology; identities
are not hardcoded to a fixture and may differ between OFEs. Its lower boundary
is explicitly `V11SnowCovered` with a validated represented-snow boundary and
optical receipt, and with the native constitutive exchange bundle absent.
It retains the supplied snow sensible/vapor operands, the existing shared-air
heat/vapor equations, and the existing Stage-3 ground-temperature and six
soil-temperature identity rows. These rows preserve the already selected
represented-snow boundary; they neither solve nor stub a new ground/litter
process and do not introduce a second snow owner.

Before residual evaluation or owner mutation, unsupported topology, identity
ordering or lower-boundary mode rejects through the named typed
`LandSurfaceEnergyError::UnsupportedDomain("m1_diagnostic_envelope")`,
`LSEB-E-030` family. Structural envelope validation precedes M1 physical
condition and numerical-domain checks; inside that envelope the declared
`VEG-E-140`, `VEG-E-141`, `VEG-E-142` ordering remains unchanged. There is no
fallback, inferred native bundle or reinterpretation of another model's
boundary. Required controls bind the admitted envelope and wrong occupancy
count/order, soil count/identity, lower-boundary mode and native-bundle-presence
rejections, including structural refusal combined with invalid pressure.
This restriction governs only the detached diagnostic entry and its local
consumer; it does not change existing selectors or qualify broader M1,
native-snow, bare/litter, all-season or production execution. It adds no new
physical equation to `INV-LANDSURFACEENERGY-167` or
`OBL-LANDSURFACEENERGY-C-023`.

The ordered topology binding is a separate input from physical supplied-condition
records. Missing, duplicate or unknown supplied-condition occupancy keys in an
otherwise valid topology remain `VEG-E-140`; they are not relabeled structural
envelope failures.
