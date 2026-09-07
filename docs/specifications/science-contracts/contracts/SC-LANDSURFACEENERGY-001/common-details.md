[Parent contract](../SC-LANDSURFACEENERGY-001.md)

## Dependencies
| Target | Required when | Boundary/obligation | Reading extent |
|---|---|---|---|
| interface.md#interface | always | shared authority | whole chapter |
| audit-details.md#sources | source provenance adjudication | complete source anchors and pinned-source paragraph | section |
| audit-details.md#enforcement | enforcement mapping or promotion-path evidence | complete guard map and parent mapping duty | section |
| [purpose](audit-details.md#purpose), [scope](audit-details.md#scope), [gaps](audit-details.md#gaps) | retained model scope, authority gaps, their supersession or promotability | complete scope and gaps, including successor GAP007/008 | section |
| qualification.md#qualification | historical/current experiment retention, identity or capture limits | complete experiment requirements | whole chapter |

<a id="common-details"></a>
# Common physical rules

Read this complete chapter for physical rules, solver correctness and accepted-primitive reconstruction. Interface and the selected physical mechanism retain current regime/owner exclusions and effective supersession. Missing coefficients, constitutive authority or run operands remain missing evidence; no default, numerical closure or production qualification follows from this directory. Underlying source, enforcement and historical applicability claims require the corresponding normative audit sections. Experiment retention and capture limits, including historical predicates and existing HOLD, are governed by qualification; applying those limits alone does not adjudicate model gaps or enforcement evidence. Actual provenance, gap or enforcement claims expand the audit sections.


<a id="variables"></a>
<a id="variables-and-units-using-canonical-symbols-first"></a>
## Variables and Units Using Canonical Symbols First

Positive energy is into the surface control volume. Positive water mass is
into surface storage; named outgoing water terms remain non-negative.

| Symbol | Units | Meaning | Owner/status |
|---|---|---|---|
| `A` | `m^2` | represented horizontal area, strictly positive | geometry input |
| `dt` | `s` | exact interval duration, strictly positive | scheduler input |
| `T_s` | `K` | land-surface temperature at an identified state point | future LSE state |
| `E_s,0`, `E_s,1` | `J m^-2` | start/end surface-control-volume energy | future LSE state |
| `M_l,0`, `M_l,1` | `kg m^-2` | start/end liquid held at the surface | future LSE state |
| `R_sw`, `R_lw` | `W m^-2` | signed net shortwave and net longwave into surface | future exact-one ledger |
| `H` | `W m^-2` | signed sensible heat into surface | future exact-one ledger |
| `LE` | `W m^-2` | signed latent heat into surface; evaporation is negative | future exact-one ledger |
| `Q_p`, `Q_runon` | `W m^-2` | precipitation/runon advected heat into surface | future exact-one ledger |
| `Q_inf`, `Q_runoff` | `W m^-2` | non-negative energy advected out with infiltration/runoff | future exact-one ledger |
| `G` | `W m^-2` | ground heat into surface; soil sees exactly `-G` | shared boundary |
| `m_p`, `m_runon` | `kg m^-2` | liquid precipitation and runon admitted during interval | climate/hydrology inputs |
| `m_evap` | `kg m^-2` | actual surface-water evaporation debit | `SC-EVAP-001` handoff |
| `m_inf`, `m_runoff` | `kg m^-2` | infiltration and runoff debits | hydrology handoffs |
| `m_terminal`, `Q_terminal`, `dt_terminal` | contract units | censored schema-v8 snow-terminal payload | prohibited v1 input |
| `epsilon_E`, `epsilon_M` | `J m^-2`, `kg m^-2` | independently reconstructed closure residuals | guards only |
| `rho_E`, `rho_M` | `dimensionless` | non-negative relative closure coefficients | authority gap |
| `H_hi,k` | `J m^-2 OFE-ground` | finite binary64 high term of soil-layer enthalpy | soil-thermal V2 owner |
| `R_k` | `J m^-2 OFE-ground` | exact normalized signed-dyadic soil-layer enthalpy carry | soil-thermal V2 owner |
| `E_k` | `J m^-2 OFE-ground` | exact soil-layer enthalpy `exact(H_hi,k)+R_k` | soil-thermal V2 owner |
| `Q_soil,k`, `Q_top,k`, `Q_inf,k` | `J m^-2 OFE-ground interval` | exact accepted soil-internal, top-boundary, and infiltration energy operands | typed receipt inputs |
| `U_hi,t` | `J m^-2 tile-ground` | finite binary64 high term mirrored into frozen LSE V3 and surface-owner V2 fields | LSE exact-surface owner; mirror only on the V16 successor path |
| `R_U,t` | `J m^-2 tile-ground` | exact normalized signed-dyadic surface-enthalpy carry | LSE exact-surface owner |
| `U_t` | `J m^-2 tile-ground` | authoritative exact surface enthalpy `exact(U_hi,t)+R_U,t` | LSE exact-surface owner |
| `Q_surface,t,j` | `J m^-2 tile-ground interval` | exact dyadic decode of one finite accepted phase-free, fusion, or retained-ingress energy operand | typed surface-energy receipt input |

<a id="state"></a>
<a id="algorithm-state-surfaces"></a>
## Algorithm State Surfaces

Required future inputs are one state identity, area, interval, forcing lineage,
start energy and liquid storage, authoritative component records, and
authoritative water-transfer records. Required outputs are end state, a sealed
component ledger, closure residuals/tolerances, branch identity, and provenance
for every operand. Mutated state is limited to `E_s` and `M_l`; adjacent owners
mutate their own soil, frost, ET, infiltration, and runoff states.

No current `DirectDayFrame` field satisfies this state surface. No production
consumer currently reads a land-surface energy result. A stateless sum of
fluxes is diagnostic mechanics, not this state or consumer proof.

Allowed degenerate states include zero flux components and zero surface liquid
when every closure identity remains defined. Invalid states include non-finite
values, `A <= 0`, `dt <= 0`, negative storage beyond tolerance, ambiguous
lineage, snow presence, and any terminal schema-v8 payload.

Exact future precondition: authoritative upstream state reports zero represented
snow at the interval start, no snow-terminal event or payload exists, every
required lineage ID occurs once, and all domain guards pass. Exact future
postcondition: the sealed end state shares the input state/interval identity,
both independently reconstructed residuals pass their own tolerance, and every
adjacent-owner handoff is accounted once without an uncommitted mutation.

<a id="algorithm"></a>
<a id="algorithm-specification-with-step-sequence"></a>
## Algorithm Specification with Step Sequence

The future algorithm shall execute atomically for one interval:

1. Validate state identity, units, finiteness, `A`, `dt`, branch predicates,
   lineage uniqueness, and all required inputs before mutation.
2. Select exactly one branch. Version 1 only defines the `snow_free` authority
   boundary; `snow_present` delegates without evaluation and `snow_terminal`
   fails closed.
3. Obtain, without recomputation, one authoritative record for each admitted
   component. A missing constitutive family blocks implementation.
4. Reconstruct the signed energy identity independently:

   `E_s,1 - E_s,0 = dt * (R_sw + R_lw + H + LE + Q_p + Q_runon - Q_inf - Q_runoff + G)`.

5. Reconstruct surface-liquid custody independently:

   `M_l,1 - M_l,0 = m_p + m_runon - m_evap - m_inf - m_runoff`.

6. Bind latent mass and energy exactly once when phase is liquid evaporation:

   `LE * dt = -L_v(T_s) * m_evap`.

   Other vapor/phase branches require their own admitted authority and may not
   alias this identity.
7. Bind every liquid crossing to one advected-energy record using the same
   mass, interval, temperature/enthalpy reference state, and lineage:
   precipitation to `Q_p`, runon to `Q_runon`, infiltration to `Q_inf`, and
   runoff to `Q_runoff`. The constitutive enthalpy/reference-state authority is
   missing in v1; omitting these terms is not permitted closure.
8. Bind the shared ground boundary as equal and opposite: the surface records
   `G`, while the sole soil/frost consumer records `-G`; neither side computes
   or applies a second ground flux.
9. Commit both state ledgers only after every guard passes. Any failure leaves
   all producer and consumer state unchanged and returns a typed `LSEB-E-*`
   error.

These equations define conservation and custody, not the missing constitutive
equations for `T_s`, `H`, `LE`, `Q_p`, `G`, or energy storage.

<a id="step-local-preconditions-intermediates-and-postconditions"></a>
### Step-Local Preconditions, Intermediates, and Postconditions

| Step | Preconditions | Computed intermediate and domain | Postcondition |
|---|---|---|---|
| 1 validate | complete immutable request; no state mutation | finite `A > 0 m^2`, `dt > 0 s`; finite unit-typed operands; unique lineage counts are integers exactly one | validated snapshot or one typed error; state byte-identical on error |
| 2 branch | validated explicit snow-state and terminal-field presence | enumerated branch in `{snow_terminal, snow_present, snow_free}` | terminal rejected; snow delegated; only snow-free advances |
| 3 acquire | snow-free branch; named owner for every operand | sealed component/water records with matching state and interval IDs | exactly one record per required operand or `LSEB-E-010` |
| 4 energy reconstruct | sealed component records | `delta_E = E_s,1-E_s,0` and `sum_E = dt*(R_sw+R_lw+H+LE+Q_p+Q_runon-Q_inf-Q_runoff+G)`, both finite `J m^-2`; `epsilon_E=delta_E-sum_E`, finite `J m^-2` | `&#124;epsilon_E&#124;` satisfies `TOL-LANDSURFACEENERGY-001` or `LSEB-E-011` |
| 5 water reconstruct | sealed water records | `delta_M=M_l,1-M_l,0`, `sum_M=m_p+m_runon-m_evap-m_inf-m_runoff`, `epsilon_M=delta_M-sum_M`, all finite `kg m^-2` | mass tolerance passes and `M_l,1` is non-negative within tolerance, else `LSEB-E-012/015` |
| 6 latent join | passing ledgers; liquid-evaporation branch | `Q_LE=LE*dt` finite `J m^-2`; `Q_evap=-L_v(T_s)*m_evap` finite `J m^-2`; exact same lineage | admitted latent tolerance passes or `LSEB-E-013`; tolerance/constitutive authority currently missing |
| 7 advection join | passing ledgers; each liquid crossing identified | four mass/energy pairs with same interval, reference state, temperature/enthalpy, and lineage; energy finite `J m^-2` | every pair occurs exactly once or `LSEB-E-010/011`; constitutive authority currently missing |
| 8 ground join | passing ledger; one soil/frost consumer identified | surface `G` and consumer `-G`, finite `W m^-2`, same interval/lineage | equal/opposite pair occurs once or `LSEB-E-014` |
| 9 atomic commit | all preceding postconditions pass | sealed end state and ledger; no additional numeric intermediate | `E_s,1` and `M_l,1` commit together and consumer handoffs become visible together; otherwise no mutation |

The missing tolerances and constitutive values in steps 4-8 are precisely
`GAP-LANDSURFACEENERGY-001..004`; therefore the table makes the ledger
mechanics reproducible but does not make the runtime algorithm promotable.

<a id="guards"></a>
<a id="branch-and-guard-table"></a>
## Branch and Guard Table

| Condition | Required disposition | Failure |
|---|---|---|
| snow absent and no terminal payload | `snow_free`; evaluate only after all authority families are admitted | typed failure while gaps remain |
| snow present | delegate exclusively to snow owner; no LSE mutation | `LSEB-E-020` on attempted dual evaluation |
| snow terminal/censored payload present | reject; there is no v1 recipient | `LSEB-E-021` |
| non-finite/unit/domain failure | reject before mutation | `LSEB-E-001` |
| duplicate/missing component lineage | reject | `LSEB-E-010` |
| energy or water closure exceeds tolerance | reject atomically | `LSEB-E-011` / `LSEB-E-012` |
| latent mass-energy mismatch | reject | `LSEB-E-013` |
| ground-flux dual ownership | reject | `LSEB-E-014` |
| negative end storage beyond tolerance | reject; no clamp/default | `LSEB-E-015` |
| positive successor support is below the active adopter domain | reject before Newton and restore every owner/receipt byte | `LSEB-E-041` |
| support/adopter/event receipt join, duration, policy, or owner digest is invalid | reject before physical evaluation | `LSEB-E-042` |
| post-event operand or support chronology is invalid | reject before physical evaluation | `LSEB-E-043` |
| exact-carry schema is noncanonical, identity/receipt/restart/checkpoint join fails, an operand/high term is nonfinite, exact-total rounding overflows, or exact reconstruction fails | reject before installation and roll back the complete envelope | `LSEB-E-049` |
| exact surface-enthalpy successor is absent or noncanonical, a V2/V3 high mirror differs, an accepted surface operand is missing/duplicated/stale, exact rounding/reconstruction fails, or restart/rollback identity does not join | reject before installation and roll back every joined owner | `LSEB-E-050` |
| a post-phase litter-liquid candidate exceeds its configured liquid capacity and the exact typed spill mass/enthalpy cannot be constructed, joined to the phase receipt, admitted once to current ingress, or reconstructed by the exact-surface owner | reject before phase-adjusted owner sealing and roll back every joined owner | `LSEB-E-047/048/050` / `SURFACELIQUID-E-003/009..012` |

Branch priority is `snow_terminal` rejection, then `snow_present` delegation,
then `snow_free`. No temperature-only guess may override explicit snow state.

<a id="aliases"></a>
<a id="symbol-alias-map"></a>
## Symbol Alias Map

| Canonical symbol | Boundary/API name | Scope | Units check | Owner contract |
|---|---|---|---|---|
| `T_s` | none | future LSE state | gap | this contract |
| legacy `surtmp(hour)` / `Thra` | not an alias of `T_s` in v1 | pinned frost/tmpadj input in `degC` | future named `degC <-> K` conversion and atomic cutover required | `SC-SNOWFREEZE-001` |
| `R_sw`, `R_lw`, `H`, `LE`, `Q_p`, `Q_runon`, `Q_inf`, `Q_runoff`, `G` | `SurfaceEnergyBalanceTerms` members are mechanics-only candidates | meteorology helper | `W m^-2`; no runtime alias admitted | this contract |
| `m_evap` | `Es`/`Er`/`Ep` are not interchangeable aliases | ET handoff | named `m` or `kg m^-2` conversion required | `SC-EVAP-001` |
| `m_inf`, `m_runoff` | infiltration/runoff records | direct hydrology | named depth-to-area-mass conversion required | `SC-WATBAL-001`, `SC-RUNOFFPART-001` |
| `m_terminal`, `Q_terminal`, `dt_terminal` | schema-v8 terminal fields | snow trace only | prohibited | `SC-SNOWENERGY-001` |
| `H_hi` | `SoilThermalLayerStateV2.enthalpy_hi_j_m2_ofe_ground` | soil-thermal persistent state | `J m^-2 OFE-ground`; finite binary64 nearest-even from exact total | this contract |
| `R` | `SoilThermalLayerStateV2.enthalpy_carry` / `ExactDyadicEnthalpy` | soil-thermal persistent state | `J m^-2 OFE-ground`; canonical normalized signed dyadic | this contract |
| `Q_soil/Q_top/Q_inf` | `SoilThermalEnergyCreditReceiptV2.accepted_operands` | candidate credit receipt | `J m^-2 OFE-ground interval`; exact finite-binary64 dyadics | this contract + `SC-SURFACELIQUID-001` |
| `U_hi` | `LseSurfaceEnthalpyStateRecordV1.enthalpy_hi_j_m2_tile` and the bit-identical frozen V3/V2 `surface_enthalpy_j_m2_tile[_ground]` fields | successor persistent LSE high term; frozen fields are mirrors | `J m^-2 tile-ground`; finite binary64 nearest-even from exact total | this contract |
| `R_U` | `LseSurfaceEnthalpyStateRecordV1.enthalpy_carry` / `ExactDyadicEnthalpy` | successor persistent LSE carry | `J m^-2 tile-ground`; canonical normalized signed dyadic | this contract |
| `Q_surface` | `LseSurfaceEnthalpyEnergyCreditReceiptV1.accepted_operands` | candidate surface-energy receipt | `J m^-2 tile-ground interval`; exact finite-binary64 dyadics with typed chronology and source | this contract + `SC-SURFACELIQUID-001` |

<a id="constants"></a>
<a id="constants-and-parameters-with-provenance-anchors"></a>
## Constants and Parameters with Provenance Anchors

| Constant/parameter | Units | Status | Authority |
|---|---|---|---|
| water density `rho_w` | `1000 kg m^-3` | fixed conversion constant | unit governance / adjacent water contracts |
| latent heat `L_v(T_s)` | `J kg^-1` | required constitutive function; exact authority missing | `GAP-LANDSURFACEENERGY-003` |
| all roughness, resistance, conductivity, heat-capacity, emissivity, and albedo inputs | contract-declared | no new value admitted by v1 | owning contracts/future authority |

Legacy fallback constants and clamps in `tmpadj.for`, `evap.for`, or
`evappm.for` are not silently admitted as LSE parameters.
Legacy `surtmp`/`Thra` remains an `SC-SNOWFREEZE-001` boundary and is neither
consumed nor superseded here. A future package must define whether `T_s`
replaces or supplies it, prove the named Celsius/Kelvin conversion, and cut
over both owners atomically; until then the seam is `NON_PROMOTABLE`.

<a id="units"></a>
<a id="unit-governance-map"></a>
## Unit-Governance Map

| Symbol | Declared units | Boundary registry entry | Conversion helper | Scalar exception | Publication metadata |
|---|---|---|---|---|---|
| `T_s` | `K` | future typed temperature | named Celsius/Kelvin helper | none | none |
| energy fluxes | `W m^-2` | future typed flux | named flux-duration integration | no final scalar exception | none |
| `E_s`, energy residual | `J m^-2` | future typed area energy | named integration only | internal guarded scalar candidate | none |
| water stores/transfers | `kg m^-2` | future typed area mass | named depth/area-mass conversion | no final scalar exception | none |
| `dt` | `s` | future typed duration | named cadence conversion | none | none |
| `H_hi,R,E,Q_soil,Q_top,Q_inf` | `J m^-2 OFE-ground` | successor exact-carry registry entries required before promotion | no conversion; exact binary64-to-dyadic decode and arbitrary-precision integer aggregation | internal typed amount; no floating residual exception | none |
| `U_hi,R_U,U,Q_surface` | `J m^-2 tile-ground` | successor surface-enthalpy exact-carry registry entries required before promotion | no conversion after the named retained OFE-to-tile credit is accepted; exact binary64-to-dyadic decode and arbitrary-precision integer aggregation | internal typed amount; no floating residual exception | none |

No publication is authorized. A future implementation must update the machine
registry and pass its unit guards before a dimensional runtime surface lands.

<a id="tolerances"></a>
<a id="tolerance-and-numeric-notes"></a>
## Tolerance and Numeric Notes

For INV-114/115, `wall_t*`, `wall_end`, support identity, receipt cardinality,
and snow-operand absence are exact. Existing LSE energy/mass tolerances apply
only after those checks and cannot repair them.

- `TOL-LANDSURFACEENERGY-001`: future energy closure shall require
  `|epsilon_E| <= max(a_E, rho_E*sum_abs_energy_operands)`; `a_E` has units
  `J m^-2`, `rho_E` is dimensionless and non-negative, and both are authority
  gaps in v1.
- `TOL-LANDSURFACEENERGY-002`: future mass closure shall use a separately
  justified scale-aware predicate `|epsilon_M| <=
  max(a_M, rho_M*sum_abs_mass_operands)`; `a_M` has units `kg m^-2`, `rho_M`
  is dimensionless and non-negative, and both are authority gaps in v1.
- Zero snapping, storage clipping, denominator substitution, or rounding into
  plausibility is prohibited unless a later contract gives a threshold, unit,
  provenance, and tests.
- Closure, solver convergence, phase-event, and representation tolerances are
  distinct and cannot be substituted for one another.
- Version-15 carry arithmetic has no tolerance: aggregate exactly, round once
  to nearest-even, and require exact reconstruction. Signed-zero treatment of
  the existing binary64 high term is unchanged; only the dyadic carry has one
  schema-zero form. `nextafter`, forced ULPs, zero snapping, subnormal flush,
  producer residuals, and canonical-zero changes are prohibited.
- Version-16 surface carry arithmetic likewise has no tolerance. The frozen
  V2/V3 high mirror must equal the exact owner's `U_hi` bit-for-bit; no ULP
  forcing, zero snap, tolerance envelope, discarded credit, or carry-to-flux
  feedback is authorized.

<a id="calibration"></a>
<a id="calibration-and-identifiability"></a>
## Calibration and Identifiability

Disposition: `CALIBRATION_NOT_APPLICABLE`.

```text
science_implementation_status = NOT_IMPLEMENTED
calibration_evidence_status = NOT_APPLICABLE
identifiability_status = NOT_APPLICABLE
```

Version 1 admits no empirically estimated parameter. Fixed constants are
science authority, forcing/state is externally supplied, and missing
constitutive parameters remain gaps. Comparator agreement, synthetic recovery,
or an execution assumption is not empirical calibration.

<a id="tests"></a>
<a id="test-vector-obligations"></a>
## Test-Vector Obligations

| Vector family | Expected observable/result | Bound invariant/failure |
|---|---|---|
| bare dry snow-free; residue/canopy variants | `snow_free`; one sealed ledger, no implicit cover alias | `INV-001/020/030` |
| calm wind; night/zero shortwave | admitted zero component or typed constitutive-domain failure, never a hidden default | `INV-001/002/041` |
| freezing/thawing `T_s` | named branch/domain result with no implicit `surtmp` alias | `INV-002/031` |
| all-zero flux | unchanged energy and water state with positive `dt` | `INV-010/011` |
| rain, no snow handoff | one `m_p/Q_p` pair; terminal fields absent | `INV-014/020/021` |
| ponded/unponded | one named `M_l` state and no LSE water repartition | `INV-011/022` |
| ET latent versus vapor | one `m_evap/LE` pair; no second vapor debit | `INV-012/031` |
| ground sign reversal | surface `G` equals soil/frost `-G` | `INV-013`, `LSEB-E-014` |
| all-distinct operands | independently reconstructed `epsilon_E/epsilon_M` pass | `INV-010/011` |
| snow present | delegate with no LSE mutation | `INV-020`, `LSEB-E-020` |
| terminal schema-v8 payload | reject with no mutation | `INV-021`, `LSEB-E-021` |
| non-finite/domain/negative storage | reject with no clamp | `INV-002/015`, `LSEB-E-001/015` |
| missing/duplicate lineage | reject before mutation | `INV-010/011/014`, `LSEB-E-010/011/012` |
| exact soil-energy credit below high-term ULP | unchanged or nearest-even `H_hi`, exact nonzero `R`, and exact reconstructed `E` | `INV-150`, `LSEB-E-049` |
| exact retained surface-energy credit below high-term ULP | unchanged or nearest-even `U_hi`, exact nonzero `R_U`, and exact reconstructed `U` | `INV-151`, `LSEB-E-050` |

Poison vectors shall independently omit and duplicate precipitation water and
heat, runon water and heat, infiltration water and heat, runoff water and heat,
latent, sensible, shortwave, longwave, ground heat, storage change, and
evaporation. Additional vectors reject non-finite inputs,
invalid Kelvin/log/roughness/resistance domains, negative storage, duplicate
lineage, wrong branch priority, and every censored schema-v8 terminal payload.
Real-consumer proof remains intentionally unsatisfied in version 1.

<a id="child-2c-invariant-ids"></a>
### Child 2C invariant IDs

| ID | Binding rule | Guard/failure |
|---|---|---|
