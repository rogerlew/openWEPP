[Parent contract](../SC-LANDSURFACEENERGY-001.md)

## Dependencies
| Target | Required when | Boundary/obligation | Reading extent |
|---|---|---|---|
| surface-custody.md#surface-custody | exact owner/receipt/restart implementation or reconstruction | high/carry and parent chronology | whole chapter |
| terminal-support.md#terminal-support | represented-snow map or transition | inactive litter/native regime | whole chapter |
| soil-custody.md#soil-custody | unpublished soil continuation implementation or audit | non-owner versus promotion | whole chapter |
| dependency-replay.md#dependency-replay | validation-once optimization implementation or error-order audit | V30 detail and C019; complete INV159 remains here | whole chapter |
| ../SC-SNOWENERGY-001.md#adr-0044-nonfinal-physical-only-covered-map-companion | native pending map or publication | same-map custody and physical-prefix owner | whole external contract; frozen protocol scope |
| ../SC-COUPLEDTIME-001.md#algorithm-specification | parent commit/restart or map custody | transaction owner | whole external contract; frozen protocol scope |

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

<a id="canonical-invariants"></a>
## Canonical invariants
| Invariant ID | Statement | Authority | Evidence | Guard | Failure posture |
|---|---|---|---|---|---|
| <a id="INV-LANDSURFACEENERGY-159"></a> `INV-LANDSURFACEENERGY-159` | Trusted in-process LSE custody may move an already fully validated immutable resident/candidate revision through a private nonserializable typed handoff instead of serializing and reparsing it. The proof binds schema/model/configuration, complete state/envelope digest, transaction/predecessor/support, and—where retained publication history exists—the exact prefix count, head, tail, and chain digest. Within one admitted terminal parent, the same invariant permits: one private generation-bound plan for validated immutable configuration/topology/index facts; one per-map proof from the existing first exact forcing validation to the later validation of that pointer-identical forcing object; and one per-map borrowed proof sourced from the existing fully validated `FrozenLitterV3Resident` revision for the exact native V3 LSE and V2 surface objects. The resident proof is consumed at the existing native-validation position after V8 and ingress-schedule derivation; V8 does not attest to those distinct resident objects. Every map still validates support, duration, transaction, joint, all map-dynamic LSE/surface/vegetation/BGC/soil/hydrology/lower-boundary state, residuals, solver results, and outputs afresh. No ephemeral plan or per-map proof is digest-only, wire, cloneable, mutable, transferable, or restart-capable. The existing private validated revision may clone only inseparably with its exact immutable whole resident; it is never independently exposed as map authority. Every resident mutation fully validates the successor before atomically advancing its validated revision. Restart/checkpoint, external bytes, durable publication, and untrusted-executor returns always receive fresh full semantic validation and canonical reconstruction. | `INV-LANDSURFACEENERGY-151/153/154/156/157/158/161` + `SC-SURFACELIQUID-001#INV-SURFACELIQUID-031` + `SC-SNOWENERGY-001#INV-SNOWENERGY-083/086` + `SC-COUPLEDTIME-001#INV-COUPLEDTIME-030` | `[INFERENCE][Static]` | private validated resident/resource typestate, parent-static structural plan, per-map forcing proof, resident-revision-sourced native proof, append-only tail validator, mandatory boundary validators | `LSEB-E-040/047/048/050` with complete rollback |
| <a id="INV-LANDSURFACEENERGY-161"></a> `INV-LANDSURFACEENERGY-161` | Every charged covered map validates its ordinary or native physical LSE/hydrology/soil prefix and exact custody. Role and regime dispatch are orthogonal: ordinary maps execute the admitted surface/WB14 physical branch; native represented-snow maps execute the native snow/LSE/soil branch and validate byte-retained inactive litter/WB14 custody. `Initial` returns a physical-only endpoint; every later charge returns a non-Clone pending adjudication map only after custody validation. Outer nonclosure consumes it into history without error, dependent-only nonclosure into typed rejection, or full closure consumes the same physical prefix once into V8/vegetation/BGC/joint and complete-owner construction as the `FinalAccepted` disposition. No additional final physical map exists, and no completed nonfinal endpoint can be promoted. No map publishes. | `SC-SNOWENERGY-001#INV-SNOWENERGY-086` + `INV-LANDSURFACEENERGY-154/155/156/157/159` | `[INFERENCE][Static]` | custody-before-pending physical typestate, orthogonal role/regime dispatch, exclusive disposition and final-only constructors, exact differential and rollback guards | outer nonclosure -> history, no error; dependent-only nonclosure/wrong role/ordinal -> `DirectV11RealConsumerError::AdaptiveRefinement`; wrong identity/regime/topology/custody/disposition/leak -> `DirectV11RealConsumerError::Identity`; physical and final-constructor failures retain `LSEB-E-020/040/047/048/049/050`; complete rollback, no promotion/fallback |
| <a id="INV-LANDSURFACEENERGY-152"></a> `INV-LANDSURFACEENERGY-152` | When the canonical Stage-3 covered solver calls LSE as an authentic physical map, LSE retains its own `INV-138/139` covered column algorithm and `TOL-LSE-001/002` closure. The outer Stage-3 admission compares continuous LSE boundary payload under `TOL-SNOWENERGY-007`; it does not replace, relax, or select the LSE solver. The accepted payload is sealed once. Owner/receipt envelope, exact high-plus-carry representation, exact snow--soil debit/credit identity, topology, branch, transaction, and rollback remain exact under `INV-LANDSURFACEENERGY-150/151`. | v31:L2085-L2092 | [INFERENCE][Static] | [Guards/errors](water-vapor.md#errors) | [Guards/errors](water-vapor.md#errors) |

<a id="canonical-obligations"></a>
## Canonical obligations
| Obligation ID | Statement | Applicability | Authority | Enforcement/failure | Test bindings |
|---|---|---|---|---|---|
| <a id="OBL-LANDSURFACEENERGY-C-014"></a> `OBL-LANDSURFACEENERGY-C-014` | prove a private immutable validated handoff moves an unchanged frozen-litter resident and surface-resource candidate without serialize/parse roundtrip or duplicate owner validation; binds the exact configuration, revision, transaction, predecessor, support, and publication-history prefix; validates only an authenticated appended tail; invalidates on mutation; and retains fresh full validation for restart, external/durable bytes, and untrusted-executor returns with full rollback. | Validated in-memory Stage-3 handoff consumers | v31:L366-L372 | [Guards/errors](water-vapor.md#errors) | [Tests](common-details.md#tests); [Detail](#handoff); named fixtures/tests/real consumers |
| <a id="OBL-LANDSURFACEENERGY-C-016"></a> `OBL-LANDSURFACEENERGY-C-016` | requires ordinary/native physical-prefix equality against a test-only forced-complete reference, zero nonfinal and one final V8/transaction/owner construction, exact provider counts, role/ordinal/ identity/regime/topology/one-ULP poisons, unpublishability, and complete rollback without fallback. | Covered nonfinal physical-map companion consumers | v31:L523-L527 | [Guards/errors](water-vapor.md#errors) | [Tests](common-details.md#tests); [Detail](#pending); named fixtures/tests/real consumers |
| <a id="OBL-LANDSURFACEENERGY-C-007"></a> `OBL-LANDSURFACEENERGY-C-007` | The real covered consumer must reject a tolerance-equivalent payload with a changed branch/topology/owner envelope, reject any unequal or duplicated snow--soil transfer, and prove continuous boundary sensitivity through the actual Stage-3 admission seam without a second LSE algorithm or uncharged physical evaluation. | Canonical Stage-3 accepted-map consumers | v31:L2094-L2098 | [Guards/errors](water-vapor.md#errors) | [Tests](common-details.md#tests); [Detail](#accepted-map); named fixtures/tests/real consumers |
