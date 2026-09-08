[Parent contract](../SC-LANDSURFACEENERGY-001.md)

## Dependencies
| Target | Required when | Boundary/obligation | Reading extent |
|---|---|---|---|
| audit-details.md#sources | source provenance adjudication | complete source anchors and pinned-source paragraph | section |
| audit-details.md#enforcement | enforcement mapping or promotion-path evidence | complete guard map and parent mapping duty | section |
| [purpose](audit-details.md#purpose), [scope](audit-details.md#scope), [gaps](audit-details.md#gaps) | retained model scope, authority gaps, their supersession or promotability adjudication | complete scope and gaps, including successor GAP007/008 | section |
| replay-evidence.md#qualification | historical/current experiment retention, executable identity or capture limits | complete experiment requirements | whole chapter |
| audit-details.md#current-schema-reference | API/schema/registry or original tabular-conformance audit | complete retained original inventories and step table; current operative rules remain here | section |


<a id="common-details"></a>
# Common physical rules

Read this complete chapter for physical rules, solver correctness and accepted-primitive reconstruction. Interface and the selected physical mechanism retain current regime/owner exclusions and effective supersession. Missing coefficients, constitutive authority or run operands remain missing evidence; no default, numerical closure or production qualification follows from this directory. Applying those current physical, owner and closure rules and retaining their no-promotion limits does not itself adjudicate authority gaps, source provenance or implementation enforcement. Actual source, gap/supersession or enforcement/promotion adjudication requires the corresponding complete audit sections. Historical/current experiment retention and executable-evidence capture limits, including experiment HOLD, require replay-evidence. Physical owner/receipt identity is governed by its mechanism.


<a id="variables"></a>
<a id="variables-and-units-using-canonical-symbols-first"></a>
## Variables and Units Using Canonical Symbols First

Energy/mass is positive inward; named outgoing water nonnegative. Geometry A>0: horizontal m²; scheduler dt>0: exact s. Future identified LSE state: T_s K; beginning/ending E_s,0/E_s,1 J m⁻²; M_l,0/M_l,1 kg m⁻². Future exact-one flux ledger, W m⁻²: net R_sw/R_lw shortwave/longwave, sensible H, latent LE (evaporation negative), incoming precipitation/runon Q_p/Q_runon, nonnegative outgoing infiltration/runoff Q_inf/Q_runoff, surface-inward G (soil exactly −G). Water kg m⁻²: climate/hydrology admitted m_p/m_runon; actual surface-water EVAP debit m_evap; hydrology m_inf/m_runoff. Schema-v8 terminal mass/heat/duration remain prohibited v1 inputs. Independent epsilon_E/epsilon_M are closure guards in J m⁻²/kg m⁻²; nonnegative dimensionless rho_E/rho_M remain v1 gaps.

Soil V2, J m⁻² OFE-ground: finite binary64 H_hi,k; normalized signed-dyadic R_k; exact E_k=exact(H_hi,k)+R_k; typed interval operands Q_soil,k/Q_top,k/Q_inf,k for accepted internal/top/infiltration energy. LSE successor, J m⁻² tile-ground: finite U_hi,t, normalized R_U,t, authoritative U_t=exact(U_hi,t)+R_U,t; frozen V3/V2 highs mirror only on V16. Typed interval Q_surface,t,j is exact decode of finite accepted phase-free/fusion/retained-ingress energy. Full field/schema inventories remain normative in audit-details; actual owner/receipt reconstruction requires its complete mechanism.

<a id="state"></a>
<a id="algorithm-state-surfaces"></a>
## Algorithm State Surfaces

Future inputs: one immutable state identity, area/interval/forcing lineage, beginning energy/liquid and authoritative component/water records. Outputs: ending state, sealed ledger, independent residuals/tolerances, branch and operand provenance. Only E_s/M_l mutate here; adjacent soil/frost/ET/infiltration/runoff owners mutate themselves. The original-v1 missing-authority record states no current DirectDayFrame or production LSE-result consumer supplied this surface; stateless flux sums prove neither. Zero components/liquid require defined closure. Reject nonfinite/nonpositive A/dt, negative storage beyond tolerance, ambiguous lineage, snow or terminal payload. V1 requires authoritative zero beginning snow, no terminal event/payload, each lineage exactly once and valid domains. End state keeps input state/interval identity, both independent residuals pass their own tolerances, and adjacent handoffs occur once with no uncommitted mutation.

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

The ordered steps above and guard table below retain all step-local admission and
atomicity rules. Each delta, integrated sum and residual is independently finite in
its declared energy/mass units. delta_E/delta_M are the storage changes on the left
of steps4/5; sum_E/sum_M are their respective right-hand sides.
epsilon_E=delta_E-sum_E; epsilon_M=delta_M-sum_M.
Lineage counts are integers exactly one; invalid validation leaves byte-identical
state. Latent Q_LE=LE*dt and Q_evap=-L_v(T_s)*m_evap are finite J m^-2 with identical
lineage; require the admitted latent tolerance or LSEB-E-013 (v1 constitutive/tolerance
authority remains missing). Every advection pair has finite J m^-2 energy and retains its same interval, reference state, temperature/
enthalpy and lineage. Acquisition requires sealed component/water records with
matching state AND interval IDs. The surface G/consumer -G pair is finite W m^-2
and shares the same interval AND lineage. Commit adds no numerical intermediate. The original detailed
table remains normative for tabular-conformance/interpretation in audit-details.

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

T_s has no API alias. Celsius surtmp(hour)/Thra remains the pinned SNOWFREEZE frost/tmpadj boundary, not T_s: future replacement/supply needs named Celsius↔Kelvin conversion and atomic two-owner cutover; until then NON_PROMOTABLE. SurfaceEnergyBalanceTerms is mechanics-only meteorology, not runtime flux aliases. EVAP Es/Er/Ep are not interchangeable m_evap; require named m or kg m⁻² conversion. WATBAL/RUNOFFPART infiltration/runoff require named depth-to-area-mass conversion. Schema-v8 terminal aliases are prohibited snow-trace fields. Exact-owner high/carry/operand API bindings retain the declared bases and complete custody chapters: nearest-even finite persistent high, normalized signed-dyadic carry and candidate finite-binary64 dyadic receipts with source/chronology under LSE/SURFACELIQUID. Frozen V3/V2 surface fields are bit-identical successor-high mirrors. API/schema conformance additionally reads the complete normative alias inventory in audit-details.

<a id="constants"></a>
<a id="constants-and-parameters-with-provenance-anchors"></a>
## Constants and Parameters with Provenance Anchors

rho_w=1000 kg m⁻³ is fixed by unit/adjacent-water authority. Required L_v(T_s), J kg⁻¹, lacks exact v1 authority (GAP-003). Roughness/resistance/conductivity/heat-capacity/emissivity/albedo retain owning-contract units/authority; no new v1 values. No tmpadj.for/evap.for/evappm.for fallback/clamp is silently admitted. surtmp/Thra remains unconsumed, unsuperseded and NON_PROMOTABLE pending the named conversion/atomic cutover.

<a id="units"></a>
<a id="unit-governance-map"></a>
## Unit-Governance Map

Future typed temperature: named Celsius/Kelvin helper, no scalar exception. Future typed flux: named flux-duration integration, no final scalar exception. Future typed area energy/residual: named integration only, internal guarded-scalar candidate. Future typed area mass: named depth/area-mass conversion, no final scalar exception. Future typed duration: named cadence conversion, no scalar exception. Exact soil/surface amounts: successor registry entries before promotion, binary64-to-dyadic decode, arbitrary-precision integer aggregation, internal typed amounts, no floating-residual exception. Soil has no conversion; surface none after accepted named OFE-to-tile credit. No publication/metadata authorized. Update machine registry and pass unit guards before dimensional runtime surfaces land. Registry/API conformance additionally reads the complete normative unit inventory in audit-details.

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
