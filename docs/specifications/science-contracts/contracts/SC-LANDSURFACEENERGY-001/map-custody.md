[Parent contract](../SC-LANDSURFACEENERGY-001.md)

## Dependencies
| Target | Required when | Boundary/obligation | Reading extent |
|---|---|---|---|
| interface.md#interface | every task | universal scope, owners, failure and qualification | whole mechanism chapter |
| surface-custody.md#surface-custody | exact owner/receipt/restart implementation or reconstruction | high/carry and parent chronology | whole mechanism chapter |
| terminal-support.md#terminal-support | represented-snow map or transition | inactive litter/native regime | whole mechanism chapter |
| soil-custody.md#soil-custody | unpublished soil continuation implementation or audit | non-owner versus promotion | whole mechanism chapter |
| ../SC-SNOWENERGY-001.md#adr-0044-nonfinal-physical-only-covered-map-companion | native pending map or publication | same-map custody and physical-prefix owner | named section plus destination scope; retain applicable single-file whole-contract/frozen-kickoff requirements |
| ../SC-COUPLEDTIME-001.md#algorithm-specification | parent commit/restart or map custody | transaction owner | named section plus destination scope; retain applicable single-file whole-contract/frozen-kickoff requirements |

<a id="map-custody"></a>
# Map Custody
Current validated handoff, final reseal and pending-map custody. V27 consumes the converged pending map’s own physical prefix; the v26 separately charged final physical-map description in historical Change Log is not current authority. V30 extends INV-159 only at the original validation positions and retains full trust-boundary validation.

<a id="accepted-map"></a>
<a id="canonical-stage-3-accepted-map-boundary-amendment"></a>
## Canonical Stage-3 Accepted-Map Boundary Amendment



<a id="profile-integration"></a>
### Profile integration

| Profile surface | Binding |
| --- | --- |
| algorithm step | After native covered-column closure, return one authentic payload to the Stage-3 map; do not run an outer-algorithm-selected LSE successor. |
| branch/guard | Same exact LSE branch/topology/owner envelope or typed `LSEB-E-049/050`; continuous outer comparison uses only `TOL-SNOWENERGY-007`. |
| invariant guard map | `INV-LANDSURFACEENERGY-152` -> Stage-3 accepted-map boundary validator and exact snow--soil debit/credit join. |
| test vector | `OBL-LANDSURFACEENERGY-C-007`: boundary/next-above, branch poison, duplicate heat, uncharged map, rollback. |
| binding exposure | `LSE-STAGE3-CANONICAL-ACCEPTED-MAP`, active, `new-INV`, IDs `152/C-007`, dual review/verification. |
| change log | 2026-09-01, contract 17: bound the canonical Stage-3 accepted-map boundary without changing native LSE physics or solver. |

<a id="handoff"></a>
<a id="validated-in-memory-lse-custody-handoff-amendment"></a>
## Validated In-Memory LSE Custody Handoff Amendment

`INV-LANDSURFACEENERGY-159` permits a trusted in-process consumer to receive
the exact immutable object whose complete semantic validation has already
succeeded. The authority carrier is private typed state, not serialized
authority. It binds model/schema and authenticated configuration identity, the
complete state or envelope digest, transaction, predecessor and support, and,
for a retained frozen-litter publication history, the exact prefix count,
first receipt, last receipt, and chain digest. It owns or immutably borrows the
value, has no public or unchecked constructor and no mutable dereference, and
cannot be persisted or reconstructed from a digest alone.

Installing an unchanged `FrozenLitterV3Resident` may therefore move this typed
validated resident directly rather than serialize and restore its complete
publication history. An append operation validates the new support and its
predecessor against the bound tail and produces a new validated revision; it
does not revalidate the immutable prefix. Likewise, one fully validated V2
surface-resource candidate may be consumed by its trusted ingress chain
without repeatedly serializing the same three nested owner envelopes. Every
mutation or replacement consumes the proof and requires full semantic
validation of the resulting revision.

An optional canonical byte/digest cache is derived only during validation,
keyed to the exact configuration and immutable revision, must equal fresh
canonical serialization, and is discarded on mutation. It is never wire,
restart, checkpoint, receipt, or publication authority. Restart/checkpoint
restore, external bytes, durable publication, and untrusted executor outputs
still perform the existing full parse, canonical reconstruction, digest and
semantic validation. In particular, the Stage 3 frozen-litter restart chain
retains full prefix replay under `SC-SNOWENERGY-001#INV-SNOWENERGY-083`; no
receipt, history row, owner join, or rollback check is removed.

`OBL-LANDSURFACEENERGY-C-014` — Prove O(1)-with-history direct install of an
unchanged validated resident, exact prefix retention, new-tail-only validation,
and full restart history replay; resource-candidate validation once per
revision and zero duplicate nested-owner serialization at trusted ingress;
mutation/configuration/transaction/support/chain/proof-transfer poison
rejection; byte-identical outputs; and complete rollback. Prove restart,
external/durable bytes, and untrusted executor returns still receive fresh
full validation.

| Profile surface | Binding |
| --- | --- |
| algorithm step | Mint a private handoff only after complete semantic validation; move the unchanged resident/candidate or validate one appended tail; validate every changed revision anew. |
| branch/guard | A cache is derived evidence only. Mutation invalidates it. Restart, checkpoint, external bytes, durable publication, and untrusted executor returns always take the full validator. |
| invariant guard map | `INV-LANDSURFACEENERGY-159` -> validated resident/resource typestate, publication-prefix/tail join, revision-bound cache, mandatory boundary validators, atomic rollback. |
| test vector | `OBL-LANDSURFACEENERGY-C-014`: 1/N-history O(1) install, tail append/poison, restart full replay, resource validation-once, zero repeat serialization, mutation/configuration/proof-transfer poisons, untrusted validation, equality, rollback. |
| binding exposure | `LSE-V24-VALIDATED-IN-MEMORY-HANDOFF`, active, `maps-to-existing-INV`, IDs `159/C-014`, dual review/verification. |
| change log | 2026-09-02, contract 24: admitted private immutable nonserializable validation-once handoffs while retaining complete validation at every restart, external, durable, and untrusted-executor boundary. |

<a id="reseal"></a>
<a id="snow-free-final-receipt-reseal-amendment"></a>
## Snow-Free Final-Receipt Reseal Amendment

`INV-LANDSURFACEENERGY-160` admits no new physical result. The provisional
snow-free V11 execution must first complete all existing validation and retain
its exact immutable LSE, surface, hydrology, BGC, vegetation, and soil ending.
A private move-only proof may then authorize the final accepted-slab pass to
reconstruct and reseal only identities whose preimage contains the accepted
slab receipt. Phase-free evaluation, litter phase, resource arbitration,
current ingress, WB14, soil thermal, and every constitutive or conservation
equation are forbidden in that final reseal.

The proof binds the live beginning revision, parent/segment/slab ordinal,
support and duration, configuration/topology, forcing and support receipt,
beginning complete owners, all non-slab inputs, provisional slab identity, and
the validated physical ending. Consumption requires an independently created
final slab whose only permitted binding difference is its ending-owner-derived
accepted-slab identity. The resealed final complete-owner bytes must equal the
provisional physical ending exactly. The provisional pass publishes nothing;
the final pass publishes exactly once. Missing, reused, stale, foreign,
mutated, or restored proofs fail before owner mutation; restart always executes
fresh physics and cannot serialize or recover the proof.

`OBL-LANDSURFACEENERGY-C-015` requires forced direct-versus-reuse byte equality,
one-call counters for every physical provider and adjacent owner operation,
zero provisional and one final publication, exhaustive non-slab identity and
physical-ending poisons, single-use rejection, pre/post-restart vectors, and
complete rollback.

| Profile surface | Binding |
| --- | --- |
| algorithm step | Fully validate one snow-free physical execution, consume one private proof to reseal final accepted-slab identities, compare exact ending owners, then publish once. |
| branch/guard | The final reseal cannot call any physical evaluator; a missing or invalid proof is a typed failure with no replay fallback. Restart invalidates the proof and requires fresh physical execution. |
| invariant guard map | `INV-LANDSURFACEENERGY-160` -> private physical-reuse typestate, accepted-slab-only delta validator, exact ending-owner comparison, provider/publication counters, restart and rollback gates. |
| test vector | `OBL-LANDSURFACEENERGY-C-015`: direct/reuse equality, physical call count one, publication 0/1, non-slab/ending poisons, duplicate use, restart refusal/fresh evaluation, rollback. |
| binding exposure | `LSE-V25-SNOW-FREE-PHYSICAL-REUSE`, active, `maps-to-existing-INV`, IDs `160/C-015`, dual review/verification. |
| change log | 2026-09-02, contract 25: admitted one private single-use snow-free physical ending for final accepted-slab identity resealing; physical equations, exact owners, publication, restart, and rollback are unchanged. |

<a id="pending"></a>
<a id="covered-nonfinal-physical-only-map-amendment"></a>
## Covered Nonfinal Physical-Only Map Amendment

`INV-LANDSURFACEENERGY-161` — The canonical covered-map evaluator retains its
physical result in a private typed posture appropriate to the preselected
regime. Role and regime dispatch are orthogonal. Every ordinary
posture performs the unchanged projection, covered LSE, snow/soil lower-
boundary, hydrology, surface/WB14-custody, and soil-candidate calculations.
Every native represented-snow posture instead executes the existing native
snow/LSE/soil branch, performs zero snow-free surface or WB14 physics, and
validates byte-retained inactive litter/WB14 custody. Both validate their
complete physical, identity, and discrete-custody relations. `Initial` stops at that validated
endpoint. Every later charged role yields a non-Clone pending adjudication
value only after those validations. Until that value is consumed into the final disposition, it does not
project a V8 receipt, build vegetation
persistent/material candidates, build a BGC candidate or ending joint, or
construct a complete owner envelope. Outer nonclosure consumes the pending
value into iteration history; dependent-only nonclosure consumes it into typed
adaptive rejection. Full closure consumes the same pending value once as the
`FinalAccepted` disposition and continues its own already-executed physical
prefix through those custody constructors, producing a private publishable
envelope but no enqueue, exposure, live-owner install, or publication.

The represented-snow native V3/V4 branch remains distinct from the ordinary
inner envelope: native physical-only evaluation requires both residents,
nonempty Stage-3 native tiles, empty active litter tiles, exact optical and
lower-boundary receipts, and byte-retained inactive litter/WB14 custody. A
physical-only result is private, move-only, non-wire, non-owner, and has no
installation or publication path. Wrong role, ordinal, regime, native posture,
support, transaction, topology, forcing, soil predecessor, surface custody, or
physical endpoint fails typed with exact rollback; duplicate or cross-
disposition consumption and final construction failure cannot retry through a
physical-only, history, or alternate-envelope path.

| Amendment trigger | Typed failure and precedence |
| --- | --- |
| role, ordinal, or charged-map order mismatch | `DirectV11RealConsumerError::AdaptiveRefinement`, before LSE execution |
| outer nonclosure | consume the pending map into history; no error |
| dependent-only nonclosure | consume the pending map into `DirectV11RealConsumerError::AdaptiveRefinement`; no history or constructor |
| support, transaction, topology, forcing, regime, native posture, predecessor, custody, promotion, or state-leak mismatch | `DirectV11RealConsumerError::Identity`, before pending minting or the affected constructor/exposure |
| ordinary/native physical-prefix failure | retain the unchanged specific `LSEB-E-020/040/047/048/049/050` or nested hydrology/surface/soil error |
| V8, vegetation, BGC, joint, serialization, or envelope construction failure | retain its unchanged typed downstream error and construct no complete envelope |
| map-level publication attempt | coupled-time `ERR-CT-018 PublicationState`; expose nothing |

Validation follows table order; downstream LSE error precedence is unchanged.

`OBL-LANDSURFACEENERGY-C-016` — Compare every initial, history, rejected, and
final-disposition physical prefix against a test-only forced-complete path
exactly for ordinary and native represented-snow
regimes, including precipitation, LSE, soil, surface/WB14, Stage-3, branch,
topology, and receipt-custody fields. Prove final-only V8/vegetation/BGC/joint/
envelope constructor counts, absence of those calls for history or rejection
dispositions, one complete final envelope, one-ULP and identity poisons,
unpublishability, and byte-identical rollback. Apply the SnowEnergy charge,
physical-endpoint, exclusive-disposition, physical-failure, dependent-
rejection, and final-constructor failure matrix exactly. Map-level
publication is always zero; the selected composed parent publishes once only
at atomic parent commit.

| Profile surface | Binding |
| --- | --- |
| algorithm step | Execute and validate one physical prefix per charged map; consume each post-initial pending value into history, rejection, or same-map final custody. |
| branch/guard | Typed ordinary/native pending endpoints cannot cross regimes, be consumed twice, or enter owner/restart/publication APIs except through the successful final disposition. |
| invariant guard map | `INV-LANDSURFACEENERGY-161` -> orthogonal role/regime dispatcher, ordinary/native physical-prefix evaluator, native V3/V4 custody-before-pending validator, pending disposition gates, final-only V8/vegetation/BGC/envelope constructors, separate envelope/parent-publication counters, rollback gate; outer nonclosure -> history/no error, dependent-only nonclosure/role -> `AdaptiveRefinement`, identity/regime/custody/disposition/leak -> `Identity`, downstream failures retain existing typed variants. |
| test vector | `OBL-LANDSURFACEENERGY-C-016`: exact differential prefix, ordinary/native regime matrix, exclusive dispositions, success/failure constructor counts, role/identity/ULP poisons, zero map publication, parent-only publication, unpublishability, rollback. |
| binding exposure | `LSE-V27-PENDING-ADJUDICATION`, active, `new-INV`, IDs `161/C-016`, dual review/verification. |
| change log | 2026-09-03, contract 27: the converged pending adjudication map continues from its own physical prefix into complete owner custody; no final physical replay is authorized. |

<a id="validation"></a>
<a id="carrier-parent-static-and-same-map-validation-once-amendment"></a>
## Carrier Parent-Static and Same-Map Validation-Once Amendment

This version extends the already admitted private validation-once custody of
`INV-LANDSURFACEENERGY-159`; it creates no new invariant and no solver version.
Within one already admitted terminal parent, one private non-Clone, non-wire,
generation-bound structural plan may retain only successful semantic
validation and deterministic indexes for immutable LSE and surface
configuration plus authenticated OFE/tile/occupancy topology. It never attests
to structural V8 state or to the distinct native resident's V3 LSE
configuration/state or V2 surface configuration/owner. It owns no mutable
runtime state, cannot be constructed from digests alone, and is absent from
restart, checkpoint, serialization, receipt, publication, and external APIs.

Plan construction is lazy at the first structural validation that an admitted
charged map reaches. If the parent has no charged map, no plan is minted and no
new validation occurs. Each plan join occurs at the exact configuration,
topology, or index check it replaces, after every existing carrier guard that
precedes that check; no plan join is hoisted ahead of support, duration,
transaction, joint, or forcing errors. On first use, the canonical full
validation at that position executes in its unchanged order. Failure returns
the same first typed error and leaves the parent and every owner byte-identical.
A successful plan binds the exact live parent generation, configuration,
topology, and index-source objects. Later maps may omit only those immutable
checks while every binding remains exact. Generation change, replacement,
equal-digest substitution, mutation, or transfer rejects at the original check
position without reconstructing a plan or falling back.

Every charged map retains the source-real order:

1. Run every existing carrier child/joint, support, duration, transaction,
   vegetation, receipt, boundary, prepared-input, and soil-read guard in its
   current position. Join the structural plan only when the first replaceable
   immutable structural check is reached.
2. At the existing forcing-validation position before V8, validate and
   canonically normalize that map's exact forcing. That first validation may
   mint a private move-only map proof bound to the live forcing allocation,
   transaction, support, generation, complete semantic digest, and normalized
   values. Equal digest with a different allocation is not authority.
3. Execute V8 projection in its current position. It freshly validates all
   current-map structural state and dynamic vegetation, LSE, surface, BGC,
   soil, hydrology, lower-boundary, and join surfaces. At V8's later validation
   of the pointer-identical forcing, consume the forcing proof instead of
   repeating only that validation. V8 neither receives nor attests to the
   distinct native resident's V3 LSE or V2 surface objects.
4. Derive the ingress schedule in its current fallible position after V8 and
   before native projection.
5. Only in a native regime, at the existing native-validation position, join
   the exact `FrozenLitterV3Resident` to its private
   `ValidatedFrozenLitterV3ResidentRevisionV1`. The revision must still match
   the resident's complete validated configuration/state/envelope digest,
   topology, transaction/predecessor/support, publication-prefix count/head/
   tail/chain, and exact V3-LSE/V2-surface references. That successful join may
   mint one borrowed, pointer-, revision-, parent-generation-, and map-bound
   proof, consumed immediately to omit only the repeated
   `lse_beginning.validate(lse_configuration)` and
   `surface_beginning.canonical_bytes(surface_configuration)` calls.
6. Continue every remaining native solver-ready, topology, rebinding,
   lower-boundary, residual, solver, output, and owner validation and every
   physical operation exactly as before. Ordinary maps mint and consume no
   resident proof.
7. Consume the final map through existing finalization and atomic parent
   commit. Rejected, history, or failed maps expose no plan/proof and mutate no
   owner. Restart discards ephemeral authority and reconstructs a fully
   validated resident revision through the canonical restore path.

The role order remains Initial, zero or more history candidates, and one final
candidate; each adaptive attempt retains direct before composed, and composed
retains Half1 before Half2 with Half2 beginning from the authenticated Half1
ending. Lazy plan creation, plan joins, forcing proof consumption, and resident
revision joins occur only at the checks they replace. Thus support, duration,
transaction, joint, forcing, V8, ingress-schedule, native-resident, subsequent
dynamic/solver, and output failures retain their present relative order, and
only the first error is returned. A stale plan paired with an earlier support,
duration, transaction, or joint poison returns that earlier error; a native
resident poison paired with an ingress-schedule poison returns the ingress
error. Malformed restart input still fails at its existing boundary.

The plan or proof must not contain or cache a
`ValidatedV8RuntimeInputProjection`, projected column, solver-ready tile,
hydrology snapshot, physical result, or dynamic owner candidate. It must not
use `Arc<DirectV10...>` or another shared owning handle to extend the lifetime
of a dynamic or complete DirectV10 input. A canonical digest may accompany
pointer/generation identity as evidence but can never independently admit an
object. The persistent resident revision is private validated custody and may
remain with an unchanged resident across maps; it is not the ephemeral map
proof. Its existing `Clone` implementation is authorized only as an inseparable
private clone of the exact whole immutable resident and never as independently
transferable admission. Every accepted resident successor is fully validated
before its revision advances atomically, and the resident may not mutate while
a borrowed proof exists. The plan, forcing proof, and resident map proof have no `Clone`,
serde, wire, public or unchecked constructor, cross-map/cross-parent transfer,
persistence, or restart restore. Second consumption and transfer are rejected;
alternate solver selection and silent full-validation fallback are prohibited.

`OBL-LANDSURFACEENERGY-C-019` requires an executable forced-full-validation
oracle against the admitted path. On the retained authentic terminal-parent
workload whose carrier performs 52 maps, audit evidence must report exactly one
parent-static validation, 52 exact normalized-forcing validations, and 52 fresh
dynamic-map validations. For each applicable regime independently, the oracle
must enumerate and compare every required Initial/history/final and
direct/Half1/Half2 role/path, with byte-for-byte physical and final-owner parity
and exact call order. Native and native-multilane maps exercise the real native
consumer; ordinary maps prove zero resident-proof mint/consume and zero native
physical execution.

Independent poisons distinguish structural versus native LSE configuration
and state, structural versus native surface configuration and owner,
generation, topology, index, support, duration, transaction, joint, forcing
pointer, same digest/different allocation, ingress schedule, resident revision,
proof second-use, cross-map, cross-parent, restart restoration, dynamic
vegetation/surface/soil-hydrology state, native solver/residual, and output
validation. Competing-poison vectors cross each ordered boundary through
dynamic validation, solver/residual, and output validation and require the same
first typed error on full and admitted paths. Every rejection has zero fallback and
publication plus byte-exact rollback. Counters and order records must originate
at the real carrier, first forcing validator, V8 projection, ingress scheduler,
resident-revision join, native V3 consumer, dynamic validators, and final owner;
fabricated outcomes, manually incremented fixture counters, or source scanning
alone cannot satisfy the obligation.

This is validation/custody architecture only. It adds no dimensional symbol,
conversion, scalar exception, constant, empirical parameter, tolerance,
equation, physical branch, solver, residual, output, publication field, or wire
format. All existing units, aliases, numeric guards, closure thresholds,
calibration posture, and constitutive-suite obligations remain unchanged.
Calibration and identifiability are therefore `CALIBRATION_NOT_APPLICABLE` for
this amendment; the contract-level fields remain unchanged.

| Profile surface | Binding |
| --- | --- |
| state surface | Private non-Clone/non-wire parent structural plan, per-map exact-forcing proof, existing resident validated revision, and borrowed non-Clone resident map proof; no cached dynamic state, result, owner candidate, restart, or publication representation. |
| algorithm step | Retain existing early carrier guards; join the lazy structural plan only at each replaced immutable check; validate forcing once before V8 and consume its proof at V8's duplicate forcing check; run V8 and fallible ingress; then join the exact resident revision and consume its proof only for the two repeated native V3/V2 validations. |
| branch/guard | Exact pointer, revision, parent generation, map, transaction, support, configuration, topology, index, and semantic identity are mandatory as applicable. Changed, reused, or transferred authority rejects at its original validation position with no fallback; restart and every trust boundary perform canonical full validation. |
| invariant guard map | `INV-LANDSURFACEENERGY-159` -> parent-static plan, exact-forcing proof, resident-revision-sourced native proof, authentic call-site audit, forced-full oracle, paired poison/error-order matrix, and rollback gate; `INV-LANDSURFACEENERGY-161` and `SC-COUPLEDTIME-001#INV-COUPLEDTIME-030` retain role/disposition custody. |
| alias/unit/constant/tolerance | No new aliases, dimensional values, conversions, constants, parameters, tolerances, or numeric normalization. Existing contract tables remain authoritative. |
| calibration | `CALIBRATION_NOT_APPLICABLE`: no parameter, observation, objective, calibration evidence, or identifiability claim changes. |
| test vector | `OBL-LANDSURFACEENERGY-C-019`: authentic 1/52/52 audit, per-applicable-regime bitwise role/path parity, ordinary zero-native proof, exact order, structural/native identity and proof-custody poisons, paired precedence, no cache/Arc/wire/fallback surface, and byte-exact rollback. |
| binding exposure | `LSE-V30-CARRIER-PARENT-STATIC-VALIDATION-ONCE`, active, `maps-to-existing-INV`, IDs `159/C-019`, dual review/verification. |
| change log | 2026-09-04, contract 30: admitted parent-static, source-ordered forcing, and resident-revision-sourced native validation-once custody only; structural V8 and resident V3/V2 objects remain distinct; no process physics, solver, tolerance, output, publication, or wire change. |


<a id="canonical-invariants"></a>
## Canonical invariants
| Invariant ID | Statement | Authority | Evidence | Guard | Failure posture |
|---|---|---|---|---|---|
| <a id="INV-LANDSURFACEENERGY-159"></a> `INV-LANDSURFACEENERGY-159` | Trusted in-process LSE custody may move an already fully validated immutable resident/candidate revision through a private nonserializable typed handoff instead of serializing and reparsing it. The proof binds schema/model/configuration, complete state/envelope digest, transaction/predecessor/support, and—where retained publication history exists—the exact prefix count, head, tail, and chain digest. Within one admitted terminal parent, the same invariant permits: one private generation-bound plan for validated immutable configuration/topology/index facts; one per-map proof from the existing first exact forcing validation to the later validation of that pointer-identical forcing object; and one per-map borrowed proof sourced from the existing fully validated `FrozenLitterV3Resident` revision for the exact native V3 LSE and V2 surface objects. The resident proof is consumed at the existing native-validation position after V8 and ingress-schedule derivation; V8 does not attest to those distinct resident objects. Every map still validates support, duration, transaction, joint, all map-dynamic LSE/surface/vegetation/BGC/soil/hydrology/lower-boundary state, residuals, solver results, and outputs afresh. No ephemeral plan or per-map proof is digest-only, wire, cloneable, mutable, transferable, or restart-capable. The existing private validated revision may clone only inseparably with its exact immutable whole resident; it is never independently exposed as map authority. Every resident mutation fully validates the successor before atomically advancing its validated revision. Restart/checkpoint, external bytes, durable publication, and untrusted-executor returns always receive fresh full semantic validation and canonical reconstruction. | `INV-LANDSURFACEENERGY-151/153/154/156/157/158/161` + `SC-SURFACELIQUID-001#INV-SURFACELIQUID-031` + `SC-SNOWENERGY-001#INV-SNOWENERGY-083/086` + `SC-COUPLEDTIME-001#INV-COUPLEDTIME-030` | `[INFERENCE][Static]` | private validated resident/resource typestate, parent-static structural plan, per-map forcing proof, resident-revision-sourced native proof, append-only tail validator, mandatory boundary validators | `LSEB-E-040/047/048/050` with complete rollback |
| <a id="INV-LANDSURFACEENERGY-160"></a> `INV-LANDSURFACEENERGY-160` | A fully validated snow-free V11 provisional execution may transfer its immutable LSE physical ending through one private move-only proof to the final accepted-slab transaction. The final path may reseal only slab/receipt-dependent identities and must not repeat phase-free evaluation, litter phase, current ingress, WB14, soil evaluation, or any energy equation. Exact physical operands, owner bytes, support, topology, configuration, predecessor, and non-slab lineage must match; the final LSE owner and complete owner set must equal the provisional physical ending byte-for-byte. The proof is single-use and absent from restart, checkpoint, receipt, publication, and wire. | `INV-LANDSURFACEENERGY-151/153/155/156/157/159` + `SC-COUPLEDTIME-001#INV-COUPLEDTIME-029` | `[INFERENCE][Static]` | private snow-free physical-reuse typestate, exact final reseal/reconstruction, physical-provider call counter, restart and poison gates | `LSEB-E-040/047/048/050` with complete rollback |
| <a id="INV-LANDSURFACEENERGY-161"></a> `INV-LANDSURFACEENERGY-161` | Every charged covered map validates its ordinary or native physical LSE/hydrology/soil prefix and exact custody. Role and regime dispatch are orthogonal: ordinary maps execute the admitted surface/WB14 physical branch; native represented-snow maps execute the native snow/LSE/soil branch and validate byte-retained inactive litter/WB14 custody. `Initial` returns a physical-only endpoint; every later charge returns a non-Clone pending adjudication map only after custody validation. Outer nonclosure consumes it into history without error, dependent-only nonclosure into typed rejection, or full closure consumes the same physical prefix once into V8/vegetation/BGC/joint and complete-owner construction as the `FinalAccepted` disposition. No additional final physical map exists, and no completed nonfinal endpoint can be promoted. No map publishes. | `SC-SNOWENERGY-001#INV-SNOWENERGY-086` + `INV-LANDSURFACEENERGY-154/155/156/157/159` | `[INFERENCE][Static]` | custody-before-pending physical typestate, orthogonal role/regime dispatch, exclusive disposition and final-only constructors, exact differential and rollback guards | outer nonclosure -> history, no error; dependent-only nonclosure/wrong role/ordinal -> `DirectV11RealConsumerError::AdaptiveRefinement`; wrong identity/regime/topology/custody/disposition/leak -> `DirectV11RealConsumerError::Identity`; physical and final-constructor failures retain `LSEB-E-020/040/047/048/049/050`; complete rollback, no promotion/fallback |
| <a id="INV-LANDSURFACEENERGY-152"></a> `INV-LANDSURFACEENERGY-152` | When the canonical Stage-3 covered solver calls LSE as an authentic physical map, LSE retains its own `INV-138/139` covered column algorithm and `TOL-LSE-001/002` closure. The outer Stage-3 admission compares continuous LSE boundary payload under `TOL-SNOWENERGY-007`; it does not replace, relax, or select the LSE solver. The accepted payload is sealed once. Owner/receipt envelope, exact high-plus-carry representation, exact snow--soil debit/credit identity, topology, branch, transaction, and rollback remain exact under `INV-LANDSURFACEENERGY-150/151`. | v31:L2085-L2092 | [INFERENCE][Static] | Local guards; [errors](water-vapor.md#errors) | Local guards; [errors](water-vapor.md#errors) |

<a id="canonical-obligations"></a>
## Canonical obligations
| Obligation ID | Statement | Applicability | Authority | Enforcement/failure | Test bindings |
|---|---|---|---|---|---|
| <a id="OBL-LANDSURFACEENERGY-C-014"></a> `OBL-LANDSURFACEENERGY-C-014` | prove a private immutable validated handoff moves an unchanged frozen-litter resident and surface-resource candidate without serialize/parse roundtrip or duplicate owner validation; binds the exact configuration, revision, transaction, predecessor, support, and publication-history prefix; validates only an authenticated appended tail; invalidates on mutation; and retains fresh full validation for restart, external/durable bytes, and untrusted-executor returns with full rollback. | Validated in-memory Stage-3 handoff consumers | v31:L366-L372 | Local guards; [errors](water-vapor.md#errors) | [Shared tests](common-details.md#tests) and [Mechanism tests](map-custody.md#handoff); named fixtures/tests and real consumers |
| <a id="OBL-LANDSURFACEENERGY-C-015"></a> `OBL-LANDSURFACEENERGY-C-015` | prove snow-free provisional/final execution evaluates every LSE, litter-phase, surface-ingress/WB14, and soil physical operator exactly once; the final accepted-slab path reseals identities only, yields byte-identical final owners and one publication, rejects every changed non-slab operand and any reused or post-restart proof, and rolls back exactly. | Snow-free final receipt reseal consumers | v31:L373-L377 | Local guards; [errors](water-vapor.md#errors) | [Shared tests](common-details.md#tests) and [Mechanism tests](map-custody.md#reseal); named fixtures/tests and real consumers |
| <a id="OBL-LANDSURFACEENERGY-C-019"></a> `OBL-LANDSURFACEENERGY-C-019` | on the authentic 52-map terminal-parent workload, prove exactly one parent-static configuration/topology/index validation, exactly 52 exact normalized-forcing validations, and exactly 52 fresh dynamic-map validations. Prove full-versus-admitted bitwise physical and final-owner parity for every applicable Initial/history/final and direct/Half1/Half2 role/path in ordinary, native, and multilane regimes; ordinary maps must mint and consume zero native-resident proofs. Preserve exact source call and first-error order. Independently poison structural and native LSE configurations/states, structural and native surface configurations/owners, generation, topology, index, support, duration, transaction, joint, forcing pointer, same-digest/different-allocation, resident revision, proof second-use, cross-map, cross-parent, and restart; add competing-poison vectors across all ordered boundaries. Every rejection has zero fallback/publication and byte-exact rollback. Executable evidence must exercise the real carrier, first forcing validator, V8 structural seam, ingress schedule, resident revision, and native-V3 consumer; fabricated counters or source scanning alone cannot discharge this obligation. | Carrier parent-static/same-map validation reuse consumers | v31:L378-L394 | Local guards; [errors](water-vapor.md#errors) | [Shared tests](common-details.md#tests) and [Mechanism tests](map-custody.md#validation); named fixtures/tests and real consumers |
| <a id="OBL-LANDSURFACEENERGY-C-016"></a> `OBL-LANDSURFACEENERGY-C-016` | requires ordinary/native physical-prefix equality against a test-only forced-complete reference, zero nonfinal and one final V8/transaction/owner construction, exact provider counts, role/ordinal/ identity/regime/topology/one-ULP poisons, unpublishability, and complete rollback without fallback. | Covered nonfinal physical-map companion consumers | v31:L523-L527 | Local guards; [errors](water-vapor.md#errors) | [Shared tests](common-details.md#tests) and [Mechanism tests](map-custody.md#pending); named fixtures/tests and real consumers |
| <a id="OBL-LANDSURFACEENERGY-C-007"></a> `OBL-LANDSURFACEENERGY-C-007` | The real covered consumer must reject a tolerance-equivalent payload with a changed branch/topology/owner envelope, reject any unequal or duplicated snow--soil transfer, and prove continuous boundary sensitivity through the actual Stage-3 admission seam without a second LSE algorithm or uncharged physical evaluation. | Canonical Stage-3 accepted-map consumers | v31:L2094-L2098 | Local guards; [errors](water-vapor.md#errors) | [Shared tests](common-details.md#tests) and [Mechanism tests](map-custody.md#accepted-map); named fixtures/tests and real consumers |
