[Parent contract](../SC-LANDSURFACEENERGY-001.md)

## Dependencies
| Target | Required when | Boundary/obligation | Reading extent |
|---|---|---|---|
| interface.md#interface | all receiver/support work | shared obligations | whole mechanism chapter |
| soil-coupling.md#soil-coupling | snow--soil receipt/closure | OFE/lane heat interface | whole mechanism chapter |
| map-custody.md#map-custody | native map identity or owner join | charged map custody | whole mechanism chapter |
| ../SC-SNOWENERGY-001.md#default-off-terminal-receiver-transaction-amendment | terminal transition/receiver | snow event and terminal parcel | named section plus destination scope; retain applicable single-file whole-contract/frozen-kickoff requirements |
| ../SC-COUPLEDTIME-001.md#algorithm-specification | support/event chronology | time admission and transaction owner | named section plus destination scope; retain applicable single-file whole-contract/frozen-kickoff requirements |
| ../SC-COUPLEDTIME-001.md#purpose-and-scientific-scope | cross-contract chronology scope | time scope | named section plus destination scope; retain applicable single-file whole-contract/frozen-kickoff requirements |

<a id="terminal-support"></a>
# Terminal Support
Current receiver/regime and positive-support admission rules. The released covered-forest policy is exactly 60000000000 ns; wire chronology at one nanosecond is not physical admission. Earlier floor-dependent evidence is superseded only as stated. V3 remains snow-free; represented snow uses one separate standard native covered map and retains inactive litter bytes.

<a id="terminal-receiver-remaining-support-amendment"></a>
## Terminal Receiver Remaining-Support Amendment

`INV-LANDSURFACEENERGY-114` admits LSE-V2 as the energy owner inside only the
default-off `terminal_receiver_v1` transaction. It receives exact
`dt_remaining`, actual vegetation/litter/mineral/frost/water owner state, and a
single 0 C terminal-liquid parcel. It must select the actual surface and
rebuild shortwave/albedo, longwave, aerodynamic geometry/roughness,
turbulence, evaporation/condensation, precipitation heat, soil heat, and
storage terms over `[wall_t*,wall_end)`. No snow temperature, albedo, roughness,
flux, residual, or terminal unallocated energy may enter any LSE operand.

`INV-LANDSURFACEENERGY-115` binds one candidate-only continuation: liquid
ingress is passed to hydrology exactly once; LSE neither repartitions it nor
relabels latent fusion as soil heat. `dt_remaining=0` suppresses the LSE solve.
Any invalid surface selection, support, receipt, owner join, or LSE solve
returns its typed error without mutation; the encompassing transaction restores
all beginning owners. CoE/default behavior and every turbulent-carrier,
efficacy, qualification, production, and cutover hold remain unchanged.

Vectors must distinguish litter/mineral/ponded and frozen/thawing receivers,
make snow and receiver fluxes unequal, cover rain and cross-midnight support,
and poison stale snow operands, full-interval LSE execution, liquid duplication,
and partial commit.

| Canonical surface | INV-114/115 binding |
|---|---|
| Algorithm | select actual receiver from beginning owners; rebuild every forcing/flux on half-open receiver support; solve LSE-V2; join hydrology/soil receipts |
| Branch/guard | zero duration skips solve; invalid/ambiguous receiver or any snow operand rejects before mutation |
| Alias/unit | absolute wall support is distinct from transaction order; 0 C parcel sensible enthalpy is zero and fusion energy is not `G` |
| Tolerance | existing LSE closure tolerances apply after exact support/identity validation; none repairs stale snow operands |
| Tests/gap | unequal snow/receiver fluxes, endpoint rain, actual surface matrix, zero remainder, rollback; carrier/efficacy/production remain held |

<a id="child-2c-shared-carrier-and-successor-support-amendment"></a>
## Child 2C shared-carrier and successor-support amendment

The covered-forest snow-free receiver remains a default-off physical adopter.
For any positive successor segment it consumes the coupled-time
`LseSupportAdmissibilityReceiptV1` and admits only the declared physical
support domain `dt >= 60000000000 ns`. A structural one-nanosecond clock interval
is not a valid constitutive LSE solve. The guard runs before Newton and leaves
the LSE, snow, surface-liquid, hydrology, soil-thermal, BGC, V11, and coupled
time owners unchanged on rejection.

<a id="snow-free-successor-chronology"></a>
### Snow-free successor chronology

The snow-free receiver may be called only after a consumed
`EventBoundaryCoalescingReceiptV1` proves that the accepted event tick is a
valid successor boundary. It reconstructs all fluxes on
`[accepted_event_tick,parent_end_tick)` using the successor forcing and owner
set. It must not consume snow albedo, snow temperature, snow roughness, snow
vapor, snow sensible heat, snow longwave, or terminal snow liquid after the
accepted tick. If the remaining support is zero, the receiver records a
zero-duration custody transition and does not execute Newton. If it is
positive but below the support receipt domain, it returns `LSEB-E-041` before
Newton; it never scales a longer result or freezes a state.

<a id="child-2c-guards"></a>
### Child 2C guards

| ID | Binding rule | Guard/failure |
|---|---|---|
| `INV-LANDSURFACEENERGY-121` | A positive snow-free successor consumes the accepted event receipt and a support-admission receipt whose active adopter is the covered-forest LSE policy. | typed receipt join / `LSEB-E-042` |
| `INV-LANDSURFACEENERGY-122` | The successor uses only post-event snow-free operands and its exact accepted support. | chronology/operand lineage / `LSEB-E-043` |
| `INV-LANDSURFACEENERGY-123` | Below-domain positive support rejects before Newton with exact owner rollback; zero support performs no physical solve. | preflight / `LSEB-E-041` |

`LSEB-E-043` is the wrong-regime operand failure. The existing exact energy,
liquid, and equal/opposite `G` ledgers remain binding; Child 2C adds no new
storage arithmetic and does not admit compensated or sub-ULP increments.

<a id="version-9-positive-support-admission-owner-amendment"></a>
## Version 9 positive-support admission owner amendment

`OPENWEPP_SNOW_FREE_LSE_V1` retains nanosecond chronology at the coupled-time
wire boundary, but its physical nonlinear solver has a deterministic declared
positive-support domain. The released domain policy is
`OPENWEPP_SNOW_FREE_LSE_V1_SUPPORT_POLICY_V1` with
`minimum_support_ns = 60000000000` (60 seconds). This is the exact Stage-3
adaptive temporal floor selected by the 2026-08-27 owner amendment. It is a
conservative model-specific numerical-domain boundary; open-mineral, litter,
and other surface profiles remain prospective/non-admitted until separately
profiled. It is not
a hidden duration floor, a changed V10 cadence, or a promise that every larger
support is globally convergent under arbitrary forcing.

The amendment changes temporal admission only. It does not change any LSE
constitutive equation, conservation or liquid/energy custody, phase ownership,
active-set/topology predicate, receipt identity, rollback, or fail-closed
behavior. Stable ordinary supports must accept steps substantially larger than
60 seconds. Every earlier admission, trace, or performance result that depended
on the provisional 0.6-second floor is superseded and requires a fresh
amended-floor run; none is claimed here.

For `OPENWEPP_STAGE3_ADAPTIVE_OWNER_TOLERANCE_V1` only, direct-versus-composed
LSE energy uses `1e-6 J m^-2 + 5e-3*max_abs`, soil-thermal energy uses
`1e-6 J m^-2 + 1.5e-2*max_abs`, and LSE/soil temperature uses
`1e-2 K + 1e-8*max_abs`. These are truncation-error controller bounds, not
constitutive, residual, mass, or energy-closure tolerances; all exact owner,
topology, phase, and receipt predicates remain exact.

Every physical V11 LSE invocation must first admit a sealed
`LseSupportAdmissibilityReceiptV1` binding parent transaction, segment, slab,
absolute half-open support, requested support ticks, exact binary64 duration
bits, model/configuration identity, beginning LSE/soil state identity,
tolerance-policy identity, numerical-policy identity, the exact minimum, and
its digest. Requested support below the declared minimum rejects
before Newton/nonlinear execution with `LSEB-E-041`; the caller receives no
candidate and all owner state remains byte-identical. A support exactly at the
minimum is an ordinary positive-support solve and must pass the declared
fixture; one nanosecond below it is a typed domain rejection. Coupled-time
identity, ordering, event receipts, and restart chronology remain valid at one
nanosecond; they do not force the physical LSE solver to execute below its
admitted domain.

The admission receipt does not canonicalize, round, or silently replace the
requested duration. A support which is rejected is not retried at the minimum.
Terminal events may coalesce an otherwise below-domain LSE segment only at an
admitted event boundary and only when exact event-time, mass, and energy error
bounds in the event contract are satisfied; no flux from the wrong surface
regime may be applied and no time gap may be created. This amendment does not
change the frozen V10 configuration, V10 behavior, selectors, defaults,
coupled-time V2 bytes, or DirectV10 restart V1 bytes.

The independent representation analysis records adjacent binary64 temperature
spacing, finite-capacity energy quanta, configured absolute/relative energy
tolerances, and the necessary storage-resolution support for every declared
finite-capacity V11 actual-stack profile. The prior 0.6-second profiling and
execution evidence is historical and does not validate the amended floor. The
declared 60-second policy remains fail-closed; fresh fixture execution is
required before any conformance or performance claim. A future smaller or additional
surface/state-qualified domain
requires a new contract cycle; it may not be inferred from a successful
individual run.

The receipt digest preimage is the compact canonical JSON object with the
`receipt_sha256` field replaced by the empty string, prefixed by the exact
domain tag `OPENWEPP_LSE_SUPPORT_ADMISSION_V1\\0`. Parent, segment, slab, and
owner chronology use the 64-lowercase-hex coupled-time identity projection;
slab ordinal and tick bounds use canonical decimal strings with no leading
zero. The baseline receipt and 12 independent identity/digest/rollback cases
are frozen in the package artifact set.

| New guard | Required result |
|---|---|

| Error | Meaning |
|---|---|
| `LSEB-E-041` | Requested positive support is below `60000000000` ns for the declared LSE policy; typed pre-Newton rejection. |
| `LSEB-E-042` | Support receipt parent/segment/slab, absolute support, duration bits, configuration/state, tolerance, or numerical-policy binding is invalid. |

| Date | Version | Author | Change |
|---|---:|---|---|
| 2026-08-27 | 9 owner amendment | Codex | Raised the Stage-3 adaptive temporal admission floor from the provisional 0.6 seconds to exactly 60 seconds (`60_000_000_000 ns`). Constitutive equations, energy/liquid custody, phase ownership, topology, receipt, rollback, and fail-closed obligations are unchanged; stable ordinary supports must accept substantially larger steps. Prior floor-dependent evidence is superseded and awaits rerun. |
| 2026-08-20 | 6 | Codex | Prospective deterministic positive-support admission; nanosecond chronology remains coupled-time-valid, below-domain support rejects before Newton, and event-boundary coalescing is deferred to the reviewed snow/event contract. |
| 2026-08-20 | 7 | Codex | Bound the covered-forest snow-free successor to the accepted event receipt, post-event-only operands, and pre-Newton support admission without scaling, freezing, or sub-ULP storage treatment. |
<a id="represented-snow-native-lse-cross-regime-amendment"></a>
## Represented-Snow Native-LSE Cross-Regime Amendment

`INV-LANDSURFACEENERGY-154` — Regime classification uses the immutable
represented-snow state before any physical evaluation. `OPENWEPP_SNOW_FREE_LSE_V3`
remains exclusively snow-free, and its V3 covered-column evaluator continues to
reject a Stage-3 snow lower boundary. When represented Stage-3 snow owns the
destination surface, the standard `INV-LANDSURFACEENERGY-138/139` Stage-3
covered-column physics executes exactly once under the native vegetation and
LSE owner identities. That charged map carries its own exact typed optical
receipt and `CoveredLowerBoundaryEnergyOperands::Stage3SnowCovered` lower-
boundary receipt; neither may be copied from, inferred from, or replaced by an
independent envelope.

During that represented-snow map, snow is the sole atmospheric ground surface.
Frozen-litter V3/V4 vapor exchange, liquid/ice phase work, surface-storage
arithmetic, current-ingress adoption, and WB14 are inactive. The frozen-litter
V3 physical owner, V4 exact-energy companion, their predecessor/receipt chains,
and their canonical bytes are retained unchanged. A standard covered tile
result may be embedded in the same heterogeneous native batch only as output of
that one charged native-identity map; constructing or charging a second inner
legacy LSE/hydrology envelope is forbidden.

At the earliest accepted Stage-3 terminal event the parent is split exactly.
The represented-snow child retains inactive litter custody; only a positive
snow-free successor may invoke the snow-free V3/V4 litter path, exactly once on
post-event operands. Missing or mismatched native identity, optical or lower-
boundary receipt, any under-snow litter-phase/WB14 call or byte mutation, a
second physical envelope, or failed terminal transition rejects atomically with
the existing typed LSE/surface-custody error and byte-exact rollback. This
amendment changes no equation, continuous tolerance, eight-map budget, temporal
floor, or exact receipt comparison.

`OBL-LANDSURFACEENERGY-C-009` — Prove typed represented-snow classification,
one charged standard covered map, zero under-snow V3/V4 phase/storage/ingress/
WB14 calls, exact optical and lower-boundary receipt retention, absence of a
second constructor or charge, unchanged V3/V4 owner bytes, exact terminal
transition to one snow-free successor call, poison rejection, and full rollback.

| Profile surface | Binding |
| --- | --- |
| algorithm step | Classify represented snow before evaluation; run one standard Stage-3 covered map under native identities, or the snow-free V3/V4 path after an exact terminal split. |
| branch/guard | Represented snow keeps litter V3/V4 inactive and byte-retained; V3 rejects Stage-3 lower-boundary input; duplicate physical construction rejects. |
| invariant guard map | `INV-LANDSURFACEENERGY-154` -> regime classifier, charged-map identity/receipt join, inactive-owner validator, and terminal split. |
| test vector | `OBL-LANDSURFACEENERGY-C-009`: classification, zero phase calls, exact optical/lower receipt, no second envelope, unchanged bytes, transition, poison, rollback. |
| binding exposure | `LSE-STAGE3-NATIVE-CROSS-REGIME`, active, `new-INV`, IDs `154/C-009`, dual review/verification. |
| change log | 2026-09-01, contract 19: one native-identity Stage-3 covered map with inactive frozen-litter custody; unchanged physics/tolerances/budget. |


<a id="canonical-invariants"></a>
## Canonical invariants
| Invariant ID | Statement | Authority | Evidence | Guard | Failure posture |
|---|---|---|---|---|---|
| <a id="INV-LANDSURFACEENERGY-114"></a> `INV-LANDSURFACEENERGY-114` | Default-off LSE-V2 selects the actual receiver and rebuilds every flux only on `[wall_t*,wall_end)` without snow operands. | terminal receiver authority | `[INFERENCE][Static]` | runtime/test | typed receiver failure |
| <a id="INV-LANDSURFACEENERGY-115"></a> `INV-LANDSURFACEENERGY-115` | The 0 C parcel enters hydrology once; fusion energy is not soil heat, zero remaining support skips LSE, and any failure rolls back all owners. | conservation/transaction authority | `[INFERENCE][Static]` | runtime/test | typed join/rollback failure |
| <a id="INV-LANDSURFACEENERGY-154"></a> `INV-LANDSURFACEENERGY-154` | Represented Stage-3 snow is a distinct native covered-LSE regime. It executes the standard Stage-3 covered-column map exactly once under native vegetation/LSE identities and retains that same map's exact optical and snow--soil lower-boundary receipts. Snow-free V3 remains snow-free only. While snow is represented, frozen-litter V3/V4 vapor, phase, storage, current-ingress, and WB14 work are inactive and their physical/exact owner bytes are retained. A second inner legacy LSE/hydrology envelope is forbidden. | `SC-SNOWENERGY-001#INV-SNOWENERGY-083` + `SC-SURFACELIQUID-001#INV-SURFACELIQUID-026` + ADR-0044 | `[INFERENCE][Static]` | typed regime/runtime/receipt/test | `LSEB-E-020/049/050` with complete rollback |
| <a id="INV-LANDSURFACEENERGY-121"></a> `INV-LANDSURFACEENERGY-121` | A positive snow-free successor consumes the accepted event receipt and a covered-forest support-admission receipt. | [Original authority](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L519-L519) | [INFERENCE][Static]; [original rule/context](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L519-L519) | typed receipt join / `LSEB-E-042` | typed receipt join / `LSEB-E-042` |
| <a id="INV-LANDSURFACEENERGY-122"></a> `INV-LANDSURFACEENERGY-122` | The successor uses only post-event snow-free operands and its exact accepted support. | [Original authority](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L520-L520) | [INFERENCE][Static]; [original rule/context](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L520-L520) | chronology/operand lineage / `LSEB-E-043` | chronology/operand lineage / `LSEB-E-043` |
| <a id="INV-LANDSURFACEENERGY-123"></a> `INV-LANDSURFACEENERGY-123` | Below-domain positive support rejects before Newton with exact owner rollback; zero support performs no physical solve. | [Original authority](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L521-L521) | [INFERENCE][Static]; [original rule/context](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L521-L521) | preflight / `LSEB-E-041` | preflight / `LSEB-E-041` |
| <a id="INV-LANDSURFACEENERGY-116"></a> `INV-LANDSURFACEENERGY-116` | Coupled-time nanosecond identity is independent of the LSE physical support domain. | [Original authority](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L2017-L2017) | [INFERENCE][Static]; [original rule/context](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L2017-L2017) | Existing coupled domain/closure guards in [ordered solve](nonlinear-solve.md#ordered-numerical-algorithm-active-branches-and-error-precedence) and [typed errors](water-vapor.md#independent-closure-and-errors); [source](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L2017-L2017) | Existing typed domain/convergence/closure failure: [error map](water-vapor.md#independent-closure-and-errors); no partial state or promotion |
| <a id="INV-LANDSURFACEENERGY-117"></a> `INV-LANDSURFACEENERGY-117` | Every physical solve carries one sealed support-admission receipt with exact identity and policy joins. | [Original authority](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L2018-L2018) | [INFERENCE][Static]; [original rule/context](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L2018-L2018) | Existing coupled domain/closure guards in [ordered solve](nonlinear-solve.md#ordered-numerical-algorithm-active-branches-and-error-precedence) and [typed errors](water-vapor.md#independent-closure-and-errors); [source](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L2018-L2018) | Existing typed domain/convergence/closure failure: [error map](water-vapor.md#independent-closure-and-errors); no partial state or promotion |
| <a id="INV-LANDSURFACEENERGY-118"></a> `INV-LANDSURFACEENERGY-118` | Below-minimum support rejects before Newton and leaves all staged/committed owners unchanged. | [Original authority](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L2019-L2019) | [INFERENCE][Static]; [original rule/context](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L2019-L2019) | Existing coupled domain/closure guards in [ordered solve](nonlinear-solve.md#ordered-numerical-algorithm-active-branches-and-error-precedence) and [typed errors](water-vapor.md#independent-closure-and-errors); [source](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L2019-L2019) | Existing typed domain/convergence/closure failure: [error map](water-vapor.md#independent-closure-and-errors); no partial state or promotion |
| <a id="INV-LANDSURFACEENERGY-119"></a> `INV-LANDSURFACEENERGY-119` | Support exactly at the declared minimum uses the unchanged constitutive equations and existing tolerances. | [Original authority](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L2020-L2020) | [INFERENCE][Static]; [original rule/context](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L2020-L2020) | Existing coupled domain/closure guards in [ordered solve](nonlinear-solve.md#ordered-numerical-algorithm-active-branches-and-error-precedence) and [typed errors](water-vapor.md#independent-closure-and-errors); [source](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L2020-L2020) | Existing typed domain/convergence/closure failure: [error map](water-vapor.md#independent-closure-and-errors); no partial state or promotion |
| <a id="INV-LANDSURFACEENERGY-120"></a> `INV-LANDSURFACEENERGY-120` | No hidden floor, longer-step scaling, tolerance relaxation, frozen state, or V10 mutation is admitted. | [Original authority](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L2021-L2021) | [INFERENCE][Static]; [original rule/context](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L2021-L2021) | Existing coupled domain/closure guards in [ordered solve](nonlinear-solve.md#ordered-numerical-algorithm-active-branches-and-error-precedence) and [typed errors](water-vapor.md#independent-closure-and-errors); [source](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L2021-L2021) | Existing typed domain/convergence/closure failure: [error map](water-vapor.md#independent-closure-and-errors); no partial state or promotion |

<a id="canonical-obligations"></a>
## Canonical obligations
| Obligation ID | Statement | Applicability | Authority | Enforcement/failure | Test bindings |
|---|---|---|---|---|---|
| <a id="OBL-LANDSURFACEENERGY-C-009"></a> `OBL-LANDSURFACEENERGY-C-009` | prove represented-snow classification invokes one standard Stage-3 covered-column map under native identities, retains its exact optical/lower-boundary receipts, invokes no frozen-litter V3/V4 phase, storage, ingress, or WB14 work, constructs no second inner envelope, retains V3/V4 owner bytes, transitions to snow-free V3/V4 only after the exact terminal split, and rolls back every owner on failure. | Represented-snow consumers with inactive litter; retain the Statement and linked source model/regime, failure and qualification limits | [Original clause](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L339-L344) | Existing local mechanism guards and [coupled error map](water-vapor.md#independent-closure-and-errors); [original obligation](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L339-L344) | [Shared test-vector obligations](interface.md#test-vector-obligations) and [owning detailed requirements](terminal-support.md#represented-snow-native-lse-cross-regime-amendment), including their named fixture/test and real-consumer requirements; [original source](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L339-L344) |
