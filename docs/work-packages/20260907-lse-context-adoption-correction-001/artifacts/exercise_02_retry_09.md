# Exercise 02 retry 09 — represented-snow component-temperature Jacobian reuse

Static: requirements review at `c2dd02bc33aa5dace9013970d1001cdcd6e8d72a` (confirmed by `git rev-parse HEAD`). No production execution, build, numerical experiment, source implementation review, or correctness-gate execution was performed. The answer establishes what a conforming implementation and its evidence must do; it does not establish that any implementation currently does it. Candidate v32 adoption is pending, and the prospective mechanism package is paused by owner request. Neither this reading exercise nor the routed older kickoff resumes it.

The permitted optimization is exact, sweep-local reuse of successful unaffected evaluator nodes while recomputing every dependency of one canonical signed component-temperature probe. It is not derivative redesign or cross-evaluation caching. Eligibility, exact custody, complete graph dependencies, canonical operation/error order, and independent full-result evidence are simultaneous requirements. A timing improvement cannot compensate for a missing one.

## Reading route and scope

The current [entry](../../../specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md) and complete `interface.md` are the authority entry, not a selective list of invariant labels. I read complete routed chapters for dependency replay, retention/experiments, current numerical methods, physical solve boundary, accepted-solve admission, common rules, surface energy, water/vapor, soil coupling, terminal support and map custody. This includes introductions, dependency sections, applicable definitions, guards, tests and all subordinate section extents, rather than isolated search hits. I also read the complete mandatory external `SC-VEGETATION-001`, `SC-SNOWENERGY-001`, and `SC-COUPLEDTIME-001` contracts. The full external scope is not waived because most of their bytes are outside the optimized node.

Expansion was necessary because graph independence depends on actual gas/wet-routing/radiation/hydraulic laws; perturbation reuse depends on stencil, cap and admission rules; unchanged outcomes include liquid enthalpy, soil receipts, native regimes and accepted-map chronology; and R's actual eligibility/evidence differs from historical retention through PC1 and SG1. The frozen experiment protocol, reproduction instructions, active kickoff and current worker handoff were therefore read completely. Governance adds repository/work-package/science instructions, science-obligation and role-review instructions, reviewer validation strategy sections, correctness-authority model, numerical-solver standard/ADR-0044 and context-measurement guidance. These are accounted separately below.

I did not follow historical evidence-tree references out of the required current sources: the assignment explicitly excludes old LSE, Git history, archives, intake/predictions/rubrics/reviews, other answers and parent evidence. Historical paragraphs appearing inside mandatory current contracts were read as source content and reconciled to their explicit supersession clauses; their linked records were not opened. No requirement to reconstruct a historical executable is introduced. No experiment input, observation or counter is manufactured from those paragraphs. This assignment audits evaluator reuse, not provider-rule implementation, a new water-owner closure algorithm, or production cutover; conditional expansion to complete SNOWFREEZE/WATBAL/SURFACELIQUID provider/owner work is not triggered merely by a reference to their parcel boundary.

## Exact replay mechanism and dependency closure

Anchors: `dependency-replay.md:20–282`, `INV-LANDSURFACEENERGY-164`, `OBL-LANDSURFACEENERGY-C-020`; current qualifications in `replay-evidence.md:65–255`.

1. **One immutable successful base, one real sweep.** The `ValidatedCoveredComponentReplaySweepBase` belongs to one already validated represented-snow solve and current Jacobian sweep. It binds validated-input generation and every referenced input, exact potential/fixed-final caps and posture, frozen branch values, current trial bits, authentic map/solve/Newton-iteration/sweep identities and graph identity. It may be borrowed by that sweep's probes, then is dropped before the next sweep/iteration/solve/map/retry. Dropping has no physical mutation or publication.
2. **One canonical signed-probe capability.** The existing constructor proves exactly one changed coordinate, sign, perturbation bits, complete probe bits, actual stencil and bound relation, then mints a fresh non-Clone `ValidatedCoveredComponentProbeReplay`. It is consumed once on success or error, never transferred, restored, serialized or reused. A later full-vector comparison is not the constructor proof.
3. **Exact custody without expensive proxies.** Immutable borrows, typed generations and compact canonical seals are allowed only at full validation and with every bound identity/value bit preserved and joined. Debug strings, digest length, approximate/reconstructed values, allocation-independent hash proxies, whole-probe clones and repeated full-vector scans do not meet custody. The static graph descriptor is established once per authenticated topology/configuration generation; strings, ordered maps, closure and whole-input hashes must not be rebuilt per sweep/probe. Its reusable structural descriptor does not authorize retention of evaluation/branch/residual/solver/map state.
4. **Closed graph, explicit direct edges.** Schema is `covered-component-temperature-dependency-v1`. Stable node/direct-edge records expand by authenticated occupancy rank and soil rank; lexical order, uniqueness, length framing, schema/N/S and SHA-256 are governed. Inclusive closure uses these exact directed edges in stable order. Compact graph representations need an exact established equivalence. Unknown nodes/reads or missing dependencies conservatively disable replay for the coordinate. Extra conservative edges must be explicit, versioned and independently compared; reachability tests cannot substitute for direct-edge completeness.

The complete required graph generator remains binding, including these easily missed propagation paths:

| Changed or computed family | Mandatory dependents and distinctions |
| --- | --- |
| Any sun/shade/wet/stem temperature | Its longwave layer, the reciprocal column, **all** component energy/tolerance/output nodes, ground-facing longwave and final output. Radiation is not local to the perturbed occupancy. |
| Sun/shade temperature | Matching current and maximum leaf, vapor, hydraulic/root, sensible, energy/tolerance and occupancy output. Current leaf also feeds maximum/vapor/hydraulic/energy/tolerance/output; maximum feeds vapor/hydraulic/output; vapor feeds shared vapor. |
| Wet temperature | First `route.wet` and first finalization, second `occ.wet`, wet sensible/energy/tolerance, second liquid and output. First finalization changes incident rain and stemflow prefixes for **every lower occupancy**, including final ground release/stemflow. Adjacent-only propagation is incomplete. |
| Incident rain / route preparation | `route.incident[o] -> route.prepare[o]`; prepare feeds wet/finalize, longwave layer, second wet, and every vapor/hydraulic/sensible/energy/tolerance/liquid/output reader of area, wet fraction, store or branch. Existing stemflow plus finalized stemflow propagate the ordered prefix. |
| Dry stem | Matching sensible/energy/tolerance/output plus the global longwave dependencies; no invented physiological dependency. |
| Second wet / sensible / hydraulic / liquid | Wet feeds hydraulic, wet energy/tolerance/liquid/output/shared vapor. Sensible feeds energy/tolerance/output/shared heat. Hydraulic/energy/tolerance/liquid feed occupancy output. Both liquid finalizations feed route-match, which must return a mismatch before later occupancies/shared work. |
| Occupancy output / lower ground output / shared reductions | Occupancy-order raw/tolerance rows and shared/final reductions; lower feeds shared heat/vapor/tolerance, corresponding rows and final output. Heat and vapor feed shared tolerance and matching rows; tolerance feeds normalized rows. Every residual node and terminal route feeds final output. |

Complete and replay evaluators must call **one shared canonical implementation** for every common node and tail. They may expose typed intermediates but may not duplicate, translate, mirror or reorder physics, tolerances, residuals, branches or output arithmetic. Source order is: trial admission; top rain; top-to-bottom prepare → first wet → first finalization → incident/stemflow; reciprocal longwave; per occupancy sun current, shade current, sun maximum, shade maximum, vapor, second wet, hydraulic/root, sensible, energy/tolerance, second liquid, route-match, output; lower/ground; shared heat then vapor and tolerances; ground/soil rows; normalization; result. Only unreachable successful base nodes copy. The two wet calls and two finalizations remain distinct source positions even when values coincide.

## Canonical numerics and related reuse are not interchangeable

Anchors: `solve-boundary.md:22–105`; `numerical-methods.md:27–107` (`INV-138`, `INV-112`); `nonlinear-solve.md:21–94`; `dependency-replay.md:285–516` (`INV-162/163`, `C-018/019`).

The unknown order is ten values per ordered occupancy: four hydraulic potentials, two betas, then sun, shade, wet and stem temperatures; shared canopy-air temperature/humidity; ground temperature; configured soil temperatures. OFEs' tiles are in typed-ID order, occupancies top-to-bottom rank then typed ID. Residuals retain matching occupancy gas/energy/hydraulic blocks, shared heat/vapor, ground energy and ordered soil rows. Open-tile equations omit shared-air coordinates and use reference air; that is not represented-snow replay eligibility.

For every covered potential and fixed-final solve:

- `delta_i = sqrt(epsilon) * max(abs(x_i), unit_scale_i)`; unit scales remain `1 K`, `0.001 kg/kg`, `1 Pa`, `1000 mm`, and `1` for beta. Minus precedes plus, with unchanged normalized residuals and frozen branches.
- Two admissible canonical probes use `(R_plus-R_minus)/(2*delta)`. A valid current iterate with exactly one inadmissible probe uses its unique inward derivative: lower `(R_plus-R_current)/delta`, upper `(R_current-R_minus)/delta`. An invalid current iterate rejects before Jacobian construction; neither admissible probe rejects typed `covered_jacobian_bound`. Do not shrink delta, clamp a probe, infer a derivative or continue.
- Exact beta 0/1 and exact 273.15 K active **or zero-area** liquid-vapor components, plus liquid-bearing ground, are ordinary one-sided cases. Liquid-vapor temperatures remain 273.15–350 K, other numerical temperatures 200–350 K, shared humidity 0–0.1 kg/kg, and unchanged physiological/potential domains remain binding. Physical/configuration validation precedes numerical bounds.
- Preserve deterministic dense partial-pivot LU; equal magnitude pivots select the lowest row; a pivot below `64*epsilon*matrix_inf_norm` is singular. At most 50 completed Newton updates and ordered `2^-b`, `b=0..20`, strict normalized-infinity-norm decrease apply. No sparse LU/Jacobian, graph coloring, simultaneous perturbations, analytic/automatic derivative substitution, altered pivoting, memoization, hardcoded 2-occupancy/6-soil logic or alternate solver is authorized by replay.
- Energy thresholds remain `1e-6 W/m² + 1e-10*max(1,sum_abs_operands)`; water/vapor `1e-12 kg/m²/s + 1e-9*scale`. Governed steps remain temperature `1e-8 K`, humidity `1e-12 kg/kg`, hydraulics `1e-7 mm`, beta `1e-10`. The constitutive law is evaluated before the cap; negative law errors are not hidden by `min`; equality `cap<=law` is cap-active. Freeze the selected branch for that Jacobian, reselect on a new iterate/backtracking evaluation.

The no-update witness is specific. First examine the existing domain-valid full `b=0` trial's prospective steps against the current complete passing residual vector. Only when that witness fails because of domain or a governed step, while current residuals pass, examine `b=1..20` until the **first domain-valid** halved trial. Completely evaluate it; all governed applied step norms must pass, and derived ci remains diagnostic with no independent threshold. Accept the unchanged current solution/evaluation/branch/ledger/owner only, not the prospective state. Retain the actual exponent contribution in the existing cumulative backtracking diagnostic, not a new public/persisted field. Do not skip a failing first admissible witness, accept a failing current residual, or use incomplete prospective evaluation. Otherwise ordinary strict-decrease line search governs.

The V10 `x=D*y`, `J_y=J_x*D`, canonical pivot test, solve and exact `delta_x=D*delta_y` scaling applies only to the uncapped active nonpositive-assimilation **potential** solve. It is forbidden for V1, V8/V9-derived, positive-PAR V10 or fixed-final solves. Inward stencils grant no extra scaling authority; pivot relaxation, regularization, larger iteration/trust limits or physiological floors are not equivalents.

FullSupply fixed-final admission may seed coordinates from potential, then freshly evaluate immutable beginnings with actual fixed caps and every residual/domain/branch and exact `F<=A<=D` check. It cannot copy potential fluxes, branches, candidates, receipts or diagnostics. Iteration-zero acceptance needs its specified zero-step/backtracking posture and no Jacobian. Partial authorization of positive root use on the nonpositive-assimilation path remains unsupported. Inactive sun/shade/wet temperatures anchor to `max(T_canopy,273.15)` and stem to canopy temperature; this is zero-area representation, not added physics. The narrow near-inactive wet exception needs the admitted V10 nonpositive potential/store-limited branch, preliminary rate within the water threshold **and** physical wet-energy closure; it must not discard a small positive mass/energy flux or spread to condensation/other regimes.

The three earlier optimizations have separate proof scopes:

| Authority | Allowed work removal | Still required |
| --- | --- | --- |
| `INV-162` identity anchors | Represented-snow ground `(T_ground-T_snow)/1e-9` and soil `(T_i-T_i,begin)/1e-9` rows, using exact subtraction then division. Copy current normalized residual vector and replace the one row, using unchanged signed stencil. | Same-solve proof; zero constitutive calls only for these anchor probes. Ordinary bases/backtracking/prospective trials still evaluate completely unless independently eligible under INV-164. Forced-complete multi-iterate, centered/inward and poison tests. No use of ground unknown instead of actual snow boundary temperature in physics. |
| `INV-163` leaf maximum reuse | Same occupancy, same evaluator stack: successful current leaf may supply maximum only at bit-exact beta one, or private `Inactive`/`ExactZeroPar` classification proving beta unread. | Sun current and shade current both precede maxima. `RespirationDominated`, positive assimilation and unclassified cases remain complete unless beta is exactly one. Private Copy result, no cross-evaluation cache; full fields/call counts, next-binary64-below-one and positive-PAR/branch boundary vectors. |
| V30 / `C-019` validation-once | One lazy parent-static plan at first charged map; same-map exact validated forcing/resident custody removes only named duplicate validations. | No work on zero-charge parents; no hoisted dynamic checks. Parent static contains only immutable structure/config/topology, never mutable V8 or resident owner physics. Full validation creates every proof; changed successor is freshly validated/atomically revised, not normalized-and-trusted. |

V30 order is part of correctness: run each original guard and join the structural plan exactly when its first replaceable immutable check is reached, never hoisting it ahead of earlier support/duration/transaction/joint/forcing errors. The map then retains its original forcing validation before V8 → fresh V8 structural/dynamic validation and later pointer-identical forcing-proof consumption → fallible ingress schedule → native resident revision proof at its original later location → remaining dynamic/solver/output work → finalization/commit. Ingress can fail before resident validation. Neither a support-duration/transaction/joint failure nor forcing/resident failure may be hoisted or reordered. Resident proof may remove only the exact V3-LSE/V2-surface duplicate validations; ordinary paths retain zero native proof uses. Authentic 52-map evidence requires one static validation, 52 forcing validations and 52 dynamic validations, with native/multilane and direct-before-Half1-before-Half2 sources, competing poisons and genuine counter provenance. A proof is private exact immutable custody, not a digest-only fallback or cross-map mutable cache.

## Error order and proof obligations

`solve-boundary.md:94–105` orders malformed serialization; model/config/state/transaction identity; missing/duplicate topology/owner; nonfinite; unsupported regime/domain; constitutive domain; request/authorization; singular; backtracking; iteration; accepted-step/residual; component closure; control-volume closure; cross-owner join. Return only the first error with diagnostics up to that point, identity/residual/iteration/step/bounds/cap/bracket/pivot/matrix evidence and rollback hashes. A failed iterate or partial owner is unusable. Replay preserves this order rather than imposing a new generic replay error on physical failures.

| Condition | Required selection or error |
| --- | --- |
| Non-Stage-3, noncomponent, malformed/multicoordinate, unproved/unknown-edge coordinate | Canonical complete evaluation **before** capability creation; this is ordinary selection, not recovery. Hydraulic-potential, beta and shared-air columns stay complete; ground/soil anchors are exclusively INV-162. |
| Graph/version/topology mismatch discovered before mint, with identical complete-evaluator operands | Complete evaluation before replay, preserving its first error. |
| Stale/foreign base, transfer, wrong coordinate/sign/perturbation/probe/stencil join or second consumption | Direct `LandSurfaceEnergyError::ConstitutiveDomain("covered_component_dependency_replay_integrity")`; no complete call and no mutation. |
| Reachable fallible node or route-match fails after replay begins | Its existing first source-real typed error. Never call complete evaluation or another solver afterward. |
| Success | Assemble existing result once, consume/drop probe, zero publication. |

The fallibility matrix (`dependency-replay.md:159–189,269–282`) is not satisfied by artificial fault hooks:

- **Current leaf is canonically crossable.** For each authentic existing crossable leaf error, both applicable components/occupancy positions need a successful physical base plus an unchanged canonical signed probe, paired replay/forced-complete first-error/location and byte-exact rollback, with no later node/fallback.
- **Maximum leaf is fallible but not generally established crossable.** Historical C-020 needs a guard-by-guard successful-base/admitted-probe implication plus authentic exact-beta/branch successes and full field parity. Only an authentic successful-base canonical counterexample may reclassify it, then paired source-real error evidence is required. PC1 below closes a narrower prospective eligibility domain; it does not retroactively prove the historical assertion.
- **Longwave, hydraulics, route-match and lower output** require reviewable implications over every actual guard **and** authentic boundary successes with every field bitwise equal. Longwave uses finite bounded temperatures/areas; hydraulics uses successful leaf/wet predecessors and immutable root/soil/caps/branches; route-match follows identical immutable inputs through the shared finalizer; lower-domain validity uses unchanged lower-boundary/caps/frozen operands. Test reciprocal multiple occupancies/zero areas, active/inactive/limiting roots, all six hydraulic residuals/tolerances, upper-to-every-lower routing/drainage/terminal routes and represented-snow resistance/ground/soil boundaries. A forged route mismatch is private integrity evidence, not a natural physical poison.
- **The five wet/routing calls** historically had a successful-base implication obligation; prospective SG1 changes it only for actually recomputed calls, as specified below. It is not permission to assert pressure-margin validity from temperature bounds.
- **Infallible assembly nodes** receive exact source-ordered node/value/branch/residual/tolerance/output parity, not invented failures. Pre-admission malformed/boundary selections and private capability failures are separate populations. Use authentic constructors/lifetime transitions, never mutable private-field injection, impossible branches, alternate perturbations/tolerances, synthetic error paths or test-only physics entrypoints.
- The unmodified differential corpus remains a catch-all: every naturally occurring error in either evaluator must match its first typed counterpart, with no fallback and exact beginning/custody rollback. A generic statement that the base succeeded once is insufficient proof for a skipped fallible guard.

## Owner, physical-regime and chronology boundaries

Relevant anchors: `interface.md` P-001–P-004; `common-details.md`; `surface-energy.md:47–190`; `water-vapor.md:171–206`; `soil-coupling.md:18–28,136–219`; `terminal-support.md`; `map-custody.md`; whole external vegetation/snow/time contracts.

Hydrology owns surface/litter/top-soil water authorization, vegetation canopy water and physiology, LSE its tile thermal state, soil the authoritative temperature/enthalpy profile, climate sealed forcing and precipitation phase. LSE does not invent another water, snow or soil owner. Fixed beginning snapshots precede current ingress; hydrology authorizes once; current rain/runon/final canopy release cannot be donated into withdrawal capacity or shrink/rewrite fixed authorization. Tile/OFE conversions apply `f_t` exactly once; no whole-hillslope or covered-subset rescaling. Preserve request/authorized/final identities and exact inequalities, typed missing/duplicate topology/unit/owner/band/direction checks, validation-before-atomic-commit and complete operand lineage. Existing censored schema-v8 terminal evidence is not joint production cutover authority.

All leaves, wet surface, stem and actual snow/ground surface participate once in the tile's one shared canopy-air heat/humidity residual. Use current component temperatures in reciprocal arbitrary-rank longwave; `tau=exp[-0.8*Omega*(LAI+SAI)]` retains transparent zero-area behavior. Bulk canopy radiation, a stale shared-air temperature used as canopy emission, prescribed stale upward ground longwave, omitted ground exchange, independently split canopy-air nodes or reordered reductions are invalid. Neutral transfer retains finite positive admitted wind/geometry; no hidden floor, fixed forest attenuation or raw nominal 10 m wind substituted for sealed exposure-projected forcing. Canopy-intercepted snow is outside this carrier.

The complete vegetation contract is necessary to know dependencies, not to import obsolete branch behavior. V10 imports prior current physics and supersedes only its admitted low-light branches: signed exact-zero PAR yields `Ag=0`, `An=-Rd`, `gs=g0`, beta one with its non-daytime residual; positive PAR first uses the historical `[Gamma*,ca]` solve, with the explicitly admitted `[ca,ci_dark]` nonpositive extension only on its unbracketed `F(ca)<0` case. Exact compensation is not an epsilon bucket. Leaf/root capacities, gas variables, nitrogen/carbon-area basis, humidity saturation denominator and hydraulic warm-start/cap branches remain actual inputs. No extra pressure lower bound is inferred from a successful finite corpus. Sun/shade activity and all six coupled hydraulic equations must remain identical.

Signed vapor water and energy remain paired: `h_l(T)=4218*(T-273.15) J/kg`, `Lv(T)=2.501e6-2369*(T-273.15) J/kg`, and the signed vapor energy includes both `h_l+Lv`. Evaporation requests/debits water; condensation is negative flux and credits water. No zero clipping, latent-only energy, PMET donation or vapor/liquid alias. Positive liquid parcels require authoritative source temperature/enthalpy; zero mass does not fabricate heat. Throughfall and initial/second drainage route down the same tile; stemflow bypasses lower foliage to ground. Each release uses accepted wet-surface temperature, retains distinct source/receipt/order and enters mass/advection exactly once. `INV-130` permits **only** `0x4071126666666667`, the first upward binary64 neighbor of 273.15 K, to become exact `0x4071126666666666` before covered-canopy ledger/release or terminal-liquid publication; below reference retains rejection, second neighbor and other values are unchanged. This cannot normalize solver residuals, probe temperatures, liquid mass or energy storage.

Inside represented-snow LSE, the real snow boundary temperature drives longwave and shared-air snow exchange; ground/soil numerical coordinates are identity anchors. Ground vapor/storage/soil conduction are not secretly restored there. The outer snow–soil interface is one OFE/lane bottom-snow-to-first-soil-node Crank–Nicolson exchange, with two half-layer series resistances and both endpoint temperatures inside the coupled map. It is not per-tile heat averaged or weighted again. Snow debit and soil credit are exact equal/opposite and occur once. Resealing may retain consumed heat only with finite reconstructed energy residual `<=1e-9 J/m²` and both ending-temperature residuals `<=1e-8 K`; this does not loosen the independent physical energy ledger (`1e-6 J/m²`), signs, nodes, support, phase, identities or receipt operands.

Regime selection is before iteration from immutable physical snow and sealed forcing. No snow invokes no covered solver. Represented snow at/below `1 kg/m²` uses the sole V22 terminal one-volume process; above it uses the canonical covered water/enthalpy/LSE/top-soil solver (`SNOWENERGY INV-082`). Frozen/mixed/thaw/refreeze/density/layer/event states inside it are not fallback eligibility. ADR-0044 supersedes the historical v23/v25–57 numerical dispatch, the 96-map recovery chains, receipt-cycle/Q-lattice searches and Picard fallback. Historical metadata does not create a new solver version. The inner LSE canonical finite differences reviewed here are distinct from the forbidden outer-coordinate finite-difference physical-map scheme.

The current outer solver uses at most seven authentic maps within its unchanged eight-map ceiling: `Initial@0`, `FixedPointAdjudication@1`, then `MultisecantAdjudication(n)@(n+1)`, `0<=N<=5`, `M=N+2`. Initial is physical-only. Each later map validates physical/identity/discrete custody before minting one non-Clone/non-wire pending value. Candidate-versus-**own-output** outer nonclosure consumes it into history; after outer closure, dependent instability against the preceding authentic map consumes it into adaptive rejection with no history; full closure consumes that same physical prefix into `FinalAccepted` and its sole envelope, without another map. Constructor failure never becomes history. Wrong role/budget/support refuses before physics; identity/regime/custody refuses at its proper join; physical and constructor errors preserve existing typed variants. All maps, direct/rejected/unselected candidates publish zero; only selected composed-parent atomic commit publishes once (`SNOWENERGY INV-086/C-054`, `COUPLEDTIME INV-030/C-013`, LSE pending amendment).

Outer continuous convergence uses `TOL-SNOWENERGY-007`, not historical exact-density or inner-LSE stopping: canopy/snow T `1e-5 K+1e-9*maxabs`; top-soil T `1e-8 K`; heat `1e-5 W/m²+1e-8*maxabs`; vapor `1e-10 kg/m²/s+1e-6*maxabs`; snow water `1e-6 kg/m²+1e-9*maxabs`; energy `1e-6 J/m²+1e-10*maxabs`; density `1e-6 kg/m³`; thickness `1e-9 m+1e-9*maxabs`; q `1e-12 kg/kg+1e-8*maxabs`. Exact branch/topology/settling/support/receipt/carry remains separate. Dimensionless depth-one multisecant scales include both charged inputs/outputs; `alpha=max(-0.75,min(alpha_raw,1))`, and zero/nonfinite/degenerate/repeated endpoints refine without a new charge. Derived depth reconstructs from selected ice/density; positive cold-content proposals use total water as ice and exact zero liquid. No independent depth interpolation or substituted earlier-output convergence. Exhaustion above floor retries the same solver on smaller exact support; at floor typed failure and rollback.

Native represented-snow maps keep exact optical/lower receipts and frozen-litter V3/V4 vapor/phase/storage/ingress/WB14 owners byte-identical, with zero inactive work and no second legacy inner envelope. Candidate-only soil continuation is an authenticated read-only physical beginning under the original prepared soil target and exact next-child lineage, not an accepted owner/restart/checkpoint. Final authentic acceptance alone replays complete original prepared owner, accumulated operands/credit chain and selected ending into one sealed atomic owner install. Pending parent-finalization soil-close authority is one-use, exact parent/clock/owner/vegetation-successor/soil-custody permission for the same-tick soil-only mutation; it grants no owner rebasing or private publication. The first positive snow-free child authenticates the complete represented-snow prefix as chronology only, with unchanged WB14 cumulatives and first physical ordinal zero, exact restart and rollback.

The exact LSE positive-support floor is `60_000_000_000 ns`, checked before Newton under a receipt binding support/duration bits, models/configuration, owners and policies. Equality is ordinary; one tick below is rejected. Structural 1 ns clock identity is not 1 ns constitutive authority. Active physical minima aggregate by maximum over active participants only. No undersized receiver solve, frozen remainder, scaled longer result or dropped support. Earlier 600 ms floor-dependent traces/timings require fresh execution. Coupled time owns checked u128 half-open support, one correctly rounded duration shared bit-identically, deterministic constraint/event ordering, complete-owner atomic acceptance and accepted-only reductions. Attempts cannot advance accepted clock/owner/receipt/publication state; direct and composed children use their own immutable beginnings, the composed result is selected, and a floor decision has one physical trial with no fabricated split. Scheduled-once work remains once, events integrate no rate, and physical transitions preserve exact support without gap/overlap.

V22 terminal physics pairs bounded sublimation mass with latent truncation and retains deposition exactly; `W=I0+L0+D-S+Lin`, `E=-C0+Lf*(L0+Lin)+Q`, with cold/mixed/all-liquid phase projection and exact equality at fusion capacity. Independently close ice, liquid and energy, never tolerance-delete positive solid or double-credit vapor latent heat. An accepted terminal endpoint transfers its actual liquid once at the 0 C reference; fusion energy is not liquid sensible heat/soil credit. The actual positive snow-free successor recomputes every flux only on its support; no post-event snow albedo/T/roughness/radiation/turbulence/evaporation/precipitation/soil heat survives. Zero remainder performs no physics. Where event-boundary coalescing applies, both neighbor minima and independently reconstructed time/snow-mass/liquid/energy errors must pass; selection minimizes displacement, then the ordered normalized error, then tick. No candidate is atomic retry/failure, not permission to alter an owner or minimum.

Private validated handoffs are local exact revision capabilities, not wire authority. External/untrusted/restart inputs receive independent complete validation and fresh process-local incarnation; no digest-only reconstruction of required owner/controller/publication bytes. Covered-map proof does not imply snow-free physical-reuse authority. Accepted publication/outbox is parent-only, idempotent by receipt identity, accepted-only and entirely rolled back on failure. Completed-day archive rotation requires durable exact acknowledgement before resident data drop; neither archiving nor receipt resealing is another physical replay.

## Required evidence and prospective limitations

The independent graph oracle must enumerate every direct record for at least `N=1,S=1` and authentic `N=2,S=6`, compare schema golden hashes and reject removal/change of **every required edge**. It must not derive expectations by copying the replay producer graph/counters. Both potential and fixed-final forced-complete modes compare every node/evaluation field, raw/tolerance/normalized residual bits, full dense Jacobian, pivots/matrix norm, branch, iteration/backtracking trajectory, diagnostic, accepted owner, output and first typed error bitwise. Demonstrate one shared canonical node/tail and actual real dispatcher/evidence consumption. Seven unconditional declaration names turning a structural expected-red classifier green, empty skeletons/dead code, cargo check or successful fixture construction are not runtime proof.

For a full interior sweep, logical=`2*(10*N+3+S)`, anchors=`2*(1+S)`, component replay=`8*N`, complete=`12*N+4`; authentic `N=2,S=6` is `58=14+16+28`. The complete 28 correspond to eight hydraulic, four beta and two shared-air columns. This is a **named single completed fully centered sweep**, never a run total. Reset each sweep before its first column and seal `Completed` only when complete or `Failed` at the actual first error. `ShortCircuited` exists only if a genuine canonical non-error early exit exists; do not fabricate an always-zero lifecycle. `RejectedBeforeProbe` belongs to a column, not a sweep short-circuit. Record potential/fixed-final, N/S, each centered/inward-lower/inward-upper/rejected stencil, admitted signs and disjoint logical=anchor+replay+complete actual work. Map/solve/iteration/sweep IDs derive independently from real nested lifecycles, not a copied ordinal, invented label or address/hash proxy. An unaudited library seam needs typed authenticated caller context rather than a guessed map.

Release aggregation resets separately, retains every sealed record without merging mode/stencil/status identities, reports actual lifecycle histograms and sums, reconciles every solve/map and failed/inward path, and exposes dropped records. Disabled ordinary audit need not allocate records. Historical retention additionally requires a real fully centered completed 58/14/16/28 sweep; a declarations test or whole-run aggregate does not satisfy it.

Historical `replay-evidence.md:13–63` retention is conjunctive: three unchanged CPU-0 release-binary runs, exact outputs and control counts, total median at most `4,803,570 us` and potential median at most `253,431 us` (at least `100,000 us` below their frozen baselines), each RSS at most `65,536 KiB`, complete parity/count/closure/real-consumer proof and the qualifying sweep. Fixed scientific outputs are source `0.8488061229561478`, outlet `0.8471105124736579`, storage `0.0016956104824910018`, clamp zero; workload counts are 48 parents/56 publications/20 direct/32 children/4 microsteps. Failure of any conjunct requires the entire production increment reverted, not a partial retained optimization. Its exact historical baseline/candidate command is:

```text
timeout 1800 taskset -c 0 env RUST_MIN_STACK=67108864 CARGO_PROFILE_RELEASE_LTO=false nix develop -c cargo test --release -p openwepp-runner --lib hillslope::tests::stage3_laned_release_one_ofe_positive_baseline_profile -- --ignored --exact --nocapture --test-threads=1
```

The baseline source manifest is `78d756be1fa11ed85ee92b7d19e6c04427b01b122efaf7804d1b55d60536bbbe`, binary SHA-256 `9a91c82f1799382014c3a561e79130b5f5b665bef0667a4bdff613c91d8e573f`. Historical tuples `(run_wall_us,potential_us,rss_kib)` are `(4926758,354838,70696)`, `(4903570,353374,54624)`, `(4896095,353431,59364)`; sort each timing field separately and select its middle integer. Candidate source and binary must each remain identical before/after three repetitions. These are source-reported historical inputs, not inputs recovered or runs performed in this exercise. This is historical retention authority, not the new experiment's admission criterion or evidence that those runs occurred here.

The prospective R qualifications are narrower and separately identified:

- **EXP-STAGE3-20260906-R:** freeze coherent current baseline A plus common observation-only harness, then independent R=A+replay and F=A+feed-forward source cuts. No R-on-F, combined arm or addition of speedups. Historical failed/HOLD records and production HOLD remain unchanged. Required science, ownership, error order, restart, exact-one transfer, stopping, perturbations and arithmetic remain binding. The new independent stencil oracle derives actual signed probes/classifications from topology, current bounds/base/graph: centered two, inward one, attempted failure counts exact. Require nonzero **executed/completed real** replay and full lifecycle/drop reconciliation; eligibility alone is not execution. Neither an existential fully centered runner case nor hardcoded 54/14/16/24 applies to this new experiment. Do not change state to manufacture centered coverage.
- **PC1 (`replay-evidence.md:134–193`):** prospective sun/shade eligibility requires the affected beta be bit-exact one before capability minting. The current signed leaf call still executes, and its successful state goes through the shared same-state maximum helper. Other beta values select complete evaluation, not physical rejection or normalization. For wet/stem probes, retained leaf operands must be bit-identical with every routed dependency covered. Finite test-corpus success, including 198 observed trials, is not a general maximum-domain implication. Prove source-real current/maximum, each component/occupancy, adjacent-beta, wet/stem, pre-capability, error/rollback and independent-classifier behavior under dual prospective review.
- **SG1 (`replay-evidence.md:194–255`):** for only the five **recomputed** `route.prepare`, `route.wet`, `route.finalize`, `occ.wet`, `occ.liquid` nodes (including lower routing), replace an unsupported guarantee of success with same-function, same-source-order **result-or-error** equality to complete evaluation. Finite bounded wet temperature and a successful base do not prove saturation/pressure-margin safety. Copied/skipped nodes still need exact successful input/branch/full-dependency proof including earlier guards. The first/second wet and finalization roles remain separate. PC1, current-leaf crossability and other noncrossable implication/boundary obligations survive. Unknown eligibility goes complete before mint; a post-start error returns directly. SG1 does not mark historical C-020 passed, create synthetic error hooks or authorize a global search.

Detailed forced-complete oracle work **and the detailed audit** must be outside **both timing and memory trials**, not merely excluded from `run_wall_us`. Both arms match optional-audit posture. Measured runs retain only approved bounded compact counters. Correctness/parity/admission precedes comparative samples. The authentic workload remains the complete-owner one-OFE, 100 m², one-day real `execute_hillslope_run_with_runtime_policy` consumer; 10/19 OFEs are separate authenticated bounded scale checks with identical per-comparison inputs. Confirm actual A control counts and document source evolution rather than forcing history; only explicitly intended lower-level work counts may differ.

The complete frozen prospective protocol still governs any later owner-authorized measurements:

| Surface | Frozen requirement |
| --- | --- |
| Reproduction | Full exact-base workspace plus all runtime additions/deletions, manifests/Cargo.lock/Nix/compiler/flags, compiler-read JSON, arm kit and matching canonical adjunct; content/path hashes and patches verified against named base; Cargo JSON discovers executable SHA-256. A04 base e89 uses current SG1, F05/R05 base 2b use common/SG1 under `composition-manifest-05.json` current recipes. Historical A labels remain distinct. Narrow source archives are not full integration workspaces; shared `.venv` symlinks are local tooling, not portable committed dependencies. No historical cache/index reconstruction or binary custody guess. |
| Environment | Linux; same permitted logical CPU, CPU 0 if allowed else documented lowest allowed performance core; `RUST_MIN_STACK=67108864`, release LTO false, one libtest thread, matched allocator/output/counters. Offline Nix build outside observations; frozen executable invoked directly in captured environment. Any executable/harness/environment change starts a new retained series; no network fetching or concurrent heavy measurements/builds. |
| Timing | F comparison first, R independent. Two fresh-process warmups per executable; 12 pairs `AB BA BA AB AB BA BA AB AB BA BA AB`. Timeouts 600 s for 1/10/19 OFEs, 1800 s for ten teardown iterations. Whole consumer wall includes actual publication; fixture authoring/validation/oracle/digests/teardown outside. CLOCK_MONOTONIC brackets and /proc CPU ticks slightly enclose runner wall, with CLK_TCK resolution stated. |
| Statistics/stop | Retain all samples, paired B-A seconds and B/A, median seconds saved and improvement, 10,000 paired bootstrap resamples seed 20260906 and descriptive percentile 95% intervals. Standalone threshold ≥5% median wall gain, lower improvement interval >0 and ≤2% median CPU regression. One decision-changing extension to 24 **total** pairs, combined analysis; otherwise bounded inconclusive. One planned performance refinement per mechanism, correctness repaired before admission. Timeout/exit/parity failure stops that arm's admission and is not speedup. Slow/high-RSS successful samples are valid; actual documented infrastructure invalidity allows one retained infrastructure-only rerun. |
| Memory | Separate six fresh-process pairs `AB BA BA AB AB BA`; /proc child status every 100 ms, executable readiness/PID lifetime/monotonic sample times/actual gaps. Pre-fixture, pre-run, end-run, post-validation, post-drop VmRSS/VmHWM and selected outside-run smaps boundaries; wait4 lifetime ru_maxrss in Linux KiB separate from sampled active maximum and endpoint RSS. Private_Clean/Dirty, Anonymous, Pss_Anon/Pss_File and RssFile retain their actual meanings, not live requested heap ownership. Explicitly drop report/snapshot/audit/telemetry/parsed-output/JSON buffers before post-drop, then delete directory. |
| Lifetime/scale | Three fresh processes per arm, ten complete one-day iterations with full teardown; no ten-day continuity/long-term boundedness claim. Competitive or preregistered useful building-block scaling gets three pairs at 10 and 19 OFEs. Report engineering `128 MiB+16 MiB*OFE` and return `8 MiB+1 MiB*OFE` separately including A failures; old 64 MiB is not this gate. Unresolved meaningful increase makes recommendation provisional; at most one decision-changing targeted lifetime experiment, no global allocator/stack/ASLR/host manipulation. |
| Science/decision | Per-field source/units/operand map before implementation; independent WAT5 source/HBP outlet/storage/clamp closure and exact deterministic output, only enumerated volatile allowlist. Exact critical-cut correctness/A0/A1/applicable A3, typed guards, consumer/restart, format/Clippy/anti-evasion/line-count and dual independent terminal verification. No inherited expected-red is relabeled PASS. Required architecture handoff gives actual counts, exclusive costs versus nested bounds, absolute savings/feasibility gap, ≤3 directions and one decisive prototype/kill criterion. Each arm gets a supported salvage/building-block/rejected/defer/inconclusive decision, never production qualification. |

The protocol's later F-only day-frame construction qualification is not a broad volatile-counter exception or an R allowance: compare only the named leaf by `D-2*N*P`, with primary A D=1205/P=400/N=1 and F D=805/P=200/N=1, both residue 405; keep raw values and balanced actual Provider=Carrier calls, no errors/drops and checked arithmetic. N=10/19 require fresh cardinality/branch/population/invocation/output evidence and equal residues, not extrapolated 405. Every other counter/scientific leaf remains exact.

Current routed handoff (`20260906.../artifacts/worker-handoff.md:28–36`) retains optimized A/F/R full-run FAIL, including R 4,206 tests / 4,028 passed / 178 failed / 77 skipped, broad Clippy and release-golden FAIL. It reports focused R checks as prior recorded evidence, not my execution or full-workspace PASS. Controlled timing, memory, teardown and scale series are NOT RUN; current-cut checks, executable custody and terminal verifications remain unresolved. This exercise leaves them that way. A0/A1/applicable A3 are binding implementation authority; legacy comparison is investigation evidence, never the sole acceptance oracle. Calibration is not applicable to these exact engineering mechanisms; no empirical transferability or production science claim follows.

## Inputs still needed for an implementation verdict

A real verdict requires the actual reviewed runtime source cut/call graph; complete validated beginning owners and receipt bytes; forcing/exposure/support/model/configuration/topology including N/S; all current unknown bits, caps/frozen branches, leaf/root/soil/liquid/radiative inputs; authenticated map/solve/iteration/sweep identities; canonical signed stencils/graph records; exact candidate/probe results and every natural failure; raw sealed counters and dropped/lifecycle records; independently enumerated expected graphs/stencils; forced-complete node/Jacobian/trajectory/owner/error/output corpus; independent mass/energy/liquid/phase/receipt operands; rollback/restart/publication evidence; current gate outputs; and coherent source/toolchain/binary/environment/workload manifests. Performance decisions additionally require the frozen paired sample and lifecycle datasets. None was supplied as actual run input to this exercise, and I did not execute or reconstruct it. Static equations, source kits named in a handoff and prior test counts cannot supply these missing observations.

Before this report's first final write I compared it against my own private 55-item source-derived inventory, including exclusions, exceptions, error order, chronological supersession and evidence duties; I separately reread the core replay/error and stencil/solve passages to check written claims. The inventory is a derived audit aid, not authority and not permission to omit routed reading. This is the first completed report and will not be repaired after delivery.

## Reading and measurement record

These are UTF-8 source-byte measurements, not delivered model tokens, inference tokens, runtime telemetry or end-to-end workflow cost. Inclusive source ranges include original newline bytes. Each request records the full-file SHA-256 and requested range SHA-256. “New” is first-request union; “repeat” includes deliberate rechecks and truncation recovery. A requested range is not claimed to have been fully displayed on the first attempt.

| Category | Files | Requests | Requested bytes | Unique source bytes | Repeat bytes |
| --- | ---: | ---: | ---: | ---: | ---: |
| Governance/current package | 14 | 17 | 81179 | 74289 | 6890 |
| LSE entry/chapters | 13 | 24 | 277811 | 203861 | 73950 |
| Mandatory external contracts | 3 | 34 | 1055390 | 839166 | 216224 |
| Frozen experiment sources | 4 | 4 | 45937 | 45937 | 0 |
| Repository measurement helper | 1 | 1 | 3544 | 3544 | 0 |
| **Total direct source requests** | **35** | **80** | **1463861** | **1166797** | **297064** |

The always-required LSE entry/interface union is 12,021 bytes (4,090+7,931), within the package's 12,288-byte ceiling. This exercise's finite selected LSE union is 203,861 bytes, 23.0315% below the package's stated 264,863-byte predecessor comparison (that old source was not read). The union meets the stated ≥20% structural reduction; this does **not** claim the actual session requested 20% fewer bytes. Direct LSE requests were 277,811 bytes, 4.8886% above that comparison because recoveries and claim checks repeat source. Including separately recorded LSE heading-search source-equivalent exposure, actual requested LSE exposure was 287,021 bytes, 8.3658% above it. External/governance reading is not hidden in this metric.

`tools/agents/context_report.py` was read and run locally without `--revision`, so it read current files and made no Git-history request. The generated structural selection/report retain all requests, while mapping the one out-of-range request to its actual available bytes for this helper only; the original request remains unchanged below. Its bootstrap/expansion fields are declared grouping, not runtime phases or hidden context measurements. Source hashes were rechecked at final assembly. No numerical tests were run.

Truncation recovery is retained explicitly: dependency-replay request 11 recovered by 12–13; vegetation 32/34/36 by 33/35/37; snow 42/45/47 by 43/46/48–51; snow 53/57 by 54/58; coupled-time 61 by 62. These cover full missed interiors, including long single-line table rows. Water request 23 asked for 209–229 although the file ends at 227: its slice/ledger/hash covers 209–227 (4,328 bytes), then line-display construction failed with IndexError before source output. Request 25 re-read the complete actual range. Both requested byte exposures remain counted; no nonexistent lines are counted as bytes. An initial `python` command was unavailable (exit 127); `.venv/bin/python` was used thereafter. No failed read/command is represented as passed production validation.

Search and automatic exposure are separate from the direct-request table:

- Instruction discovery: `pwd` and `tools/agents/find-agents --for` the owned report returned repository/root-work-package paths; no extra file content. `git rev-parse HEAD` returned only the current identity, not history. Directory/file listing and `wc -l` output were metadata, not source extents.
- Search 1: `rg -n '^#{1,6} |^<a id='` over LSE water-vapor/map-custody plus testing-and-gate-strategy/prompt-wording-guidance; retained `/tmp/lse-solver09-search1.txt`, SHA-256 `ead84f2d3549a0c40474db91deef6d9470b1190f075976a2734b9b82c315b163`. The search and repeated saved display are counted as two exposures: 7,443 output bytes each, 2,312 underlying source-line bytes each, 4,624 source-equivalent requested bytes total. Those headings add 591 unique governance bytes beyond selected direct ranges. The source-equivalent LSE portion is 2,802 bytes over the two exposures.
- Search 2: `rg --files docs/decisions docs/standards tools/agents | rg '(0044|numerical-solver-architecture|context_report)'` returned only three file paths; zero source-content bytes.
- Search 3: the same heading pattern over eleven already-read LSE chapters, redirected once then displayed from `/tmp/lse-solver09-search3.txt`: 22,406 output bytes, 6,408 source-equivalent bytes, zero new source bytes; SHA-256 `fb3b8d4a92494eb34d1f1d8d460e21595ed3d1801ba0b88fbcf64204c8db5bcd`.
- The user-provided automatic root AGENTS body maps to one complete 9,508-byte repository source-equivalent exposure, recorded separately from its explicit tool read. This is a source-equivalent count, not a byte count of the entire automatic message including wrappers. Other automatically supplied system/developer/skill catalogs, task messages and the compaction summary have no measured delivered-byte/token ledger here: **UNOBSERVED**, not zero. The compaction summary is derived private continuity, not independent authority or additional raw source reading.

Thus direct requests plus measured searches total 1,474,893 requested source-equivalent bytes; adding the known automatic root-AGENTS equivalent gives 1,484,401. The union including search-only headings is 1,167,388 bytes. These are explicitly scoped source-equivalent arithmetic, not complete-session telemetry. Search manifests retain every hit path/line/byte count. Generated inventory, drafts, private helper source, accounting outputs and self-audit are derived material, not added authority bytes; their contents/hashes are tracked below. Metadata displays of the ledger and scalar totals are derived exposure. Exact raw transcript size and model/runtime cost remain unobserved.

### Source identities

The request table uses these IDs solely as path abbreviations; hashes bind complete current files.

| ID | Path | Full-file SHA-256 |
| --- | --- | --- |
| S01 | `AGENTS.md` | `9bd7887bcae731debc64689a4774ac222d529bcd527ebea2293e3ffd600289f6` |
| S02 | `docs/work-packages/AGENTS.md` | `29e2f48a0b69fd9f43eb8d3a6dc2b72d30792337f90a2141b89df105726282a5` |
| S03 | `docs/work-packages/20260907-lse-context-adoption-correction-001/package.md` | `0cb2505989026d5e911d696eb7eda8d96c5ebd5498da421e55a2f7682290929b` |
| S04 | `docs/work-packages/20260907-lse-context-adoption-correction-001/artifacts/worker-handoff.md` | `00f80e082f41b65a25b6864340cd89b6daffbf0d16ba54d6df5612e31d25d6fc` |
| S05 | `docs/work-packages/role-review.md` | `2193e9538ad22dad5678fc5bb59080f686a543f697fec4983208214a73cc5eb1` |
| S06 | `docs/specifications/science-contracts/AGENTS.md` | `bace4ab3e33119e0239f700f92c53707d884f297e92d2454a86c477758c930c7` |
| S07 | `docs/work-packages/science-obligations.md` | `1a0377a3362a6c34d783796b6756854f8630e1f05e994dc3d963eaafd16c542e` |
| S08 | `docs/standards/AGENTS.md` | `b62a33d9a70a3d3f45e7eccdcf05ca3a194486fcdbfc7bd439371cf8f0ae3015` |
| S09 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md` | `391772bfc9345357a654552ae84203809a91e0280f5579693ae73da68119cc53` |
| S10 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/interface.md` | `758873659d360c8edfc7bcec7aa6cb58d065d44e221e890166390252810ed481` |
| S11 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/dependency-replay.md` | `6ce8ffe21317027f5d1171579a9059c64c7ae3bd9ba9181ceea91323254b6b25` |
| S12 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/numerical-methods.md` | `172438a4196f13e3b9260845680abc81b98c55d3af044dbda744f5a1ca6932f8` |
| S13 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/common-details.md` | `b930bbd0f0ce2c16d826be6a6d1d0bb4e6fe0155e611232047e0b4205e2dbfc4` |
| S14 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/surface-energy.md` | `2c3afa3f25827508ab9bd4826249d6557d08aa44b8f4ba7f0ecd5c988a900bb8` |
| S15 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/soil-coupling.md` | `a85b0aa76a404143b049a56a25225250b4eddeabdf8ac2fc895c1aee27a4434d` |
| S16 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/solve-boundary.md` | `62e13854fe0bdddb71a4e5aae874aa13c89431bef3d47526b9af973ec8014b54` |
| S17 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/nonlinear-solve.md` | `bb75c9248557ae704992c1f5629c1b0ab563d764cef193f6a8e220907671dc43` |
| S18 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/terminal-support.md` | `2222800b0e5fa73d281ba68ec21961a6fcc8b38401f9c59d045139e2671dd709` |
| S19 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/replay-evidence.md` | `d29eeca69cf60d1a0953f6fb0cc2ac3841c100159961f01394fdbdf17d5403b7` |
| S20 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/water-vapor.md` | `fda2350a1d4bf826dd3ad6f622e4ee0f41d708860aef6005e8948c27c3d729f8` |
| S21 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/map-custody.md` | `8508b3ceaa49f081ce8c2df1d7cce39249d8f3875fea5a017404c0816cdae2c1` |
| S22 | `docs/standards/testing-and-gate-strategy.md` | `56277d9848d80b0672ebeaaa8277ac2ad9888c90e54b4f31aa7c5b8ca9b04230` |
| S23 | `docs/standards/prompt-wording-guidance.md` | `294385612b565fe8be3e9e530caa0792c901406ecf53abc9402649e5568ffe3f` |
| S24 | `docs/prompt_templates/assurance-findings-template.md` | `95373d3c7df1f0a7e3ad7640fa22c0e137095e6cf5c4d92e3803846eb3f6ee83` |
| S25 | `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md` | `3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa` |
| S26 | `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md` | `94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d` |
| S27 | `docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md` | `1d10611b5143be201381f283ec5594d9f65c99f8252e944dfae57076a8a699f7` |
| S28 | `docs/work-packages/20260906-stage3-prospective-mechanism-experiments-001/artifacts/experiment-protocol.md` | `40033ea2e536984ae5a7455e99509e7e760624b78cdb08ee347597fe2f5f115f` |
| S29 | `docs/work-packages/20260906-stage3-prospective-mechanism-experiments-001/artifacts/authority-input-reproduction.md` | `1c813fd08e34763808d3ac8a033d18d8c5176857aff76378aa126762166c8883` |
| S30 | `docs/work-packages/20260906-stage3-prospective-mechanism-experiments-001/prompts/active/kickoff.md` | `6f3f0852afbfae138c4e40c62e15bf727c569611d00dec08bc8338e05ebdd9b3` |
| S31 | `docs/work-packages/20260906-stage3-prospective-mechanism-experiments-001/artifacts/worker-handoff.md` | `b3ca642a6b4fd94f6e1459f07058cd9fc18fab747368421fc0dc6e591d143183` |
| S32 | `docs/standards/numerical-solver-architecture.md` | `46089ca55bb565b7dc67ea1b2075bbcc16db6902eeba1952572b072abadd0738` |
| S33 | `docs/decisions/0044-prohibit-accretive-production-solver-dispatch.md` | `a597aaf802917e1ed9c21db1f4453c571dde09ad48684c0f7d0ff9062b6645e6` |
| S34 | `docs/specifications/correctness-authority-model.md` | `64275bad708f353c2c3b66f5b1475c070254591e26278d0a4ec442cad6e6eb0f` |
| S35 | `tools/agents/context_report.py` | `2aa217325da4b279926aa0780d1aa9427fb21d1e80a533314cbe277474d2bbde` |

### Every direct request

| # | Source | Inclusive requested lines | Phase | Bytes | New | Repeat | Requested-range SHA-256 |
| ---: | --- | --- | --- | ---: | ---: | ---: | --- |
| 1 | S01 | 1–121 | instructions | 9508 | 9508 | 0 | `9bd7887bcae731debc64689a4774ac222d529bcd527ebea2293e3ffd600289f6` |
| 2 | S02 | 1–105 | instructions | 6642 | 6642 | 0 | `29e2f48a0b69fd9f43eb8d3a6dc2b72d30792337f90a2141b89df105726282a5` |
| 3 | S03 | 1–75 | bootstrap | 5304 | 5304 | 0 | `0cb2505989026d5e911d696eb7eda8d96c5ebd5498da421e55a2f7682290929b` |
| 4 | S04 | 1–22 | bootstrap | 1586 | 1586 | 0 | `00f80e082f41b65a25b6864340cd89b6daffbf0d16ba54d6df5612e31d25d6fc` |
| 5 | S05 | 1–7 | instructions | 1400 | 1400 | 0 | `2193e9538ad22dad5678fc5bb59080f686a543f697fec4983208214a73cc5eb1` |
| 6 | S06 | 1–84 | governance | 6532 | 6532 | 0 | `bace4ab3e33119e0239f700f92c53707d884f297e92d2454a86c477758c930c7` |
| 7 | S07 | 1–98 | governance | 6149 | 6149 | 0 | `1a0377a3362a6c34d783796b6756854f8630e1f05e994dc3d963eaafd16c542e` |
| 8 | S08 | 1–58 | governance | 4052 | 4052 | 0 | `b62a33d9a70a3d3f45e7eccdcf05ca3a194486fcdbfc7bd439371cf8f0ae3015` |
| 9 | S09 | 1–67 | entry | 4090 | 4090 | 0 | `391772bfc9345357a654552ae84203809a91e0280f5579693ae73da68119cc53` |
| 10 | S10 | 1–66 | shared | 7931 | 7931 | 0 | `758873659d360c8edfc7bcec7aa6cb58d065d44e221e890166390252810ed481` |
| 11 | S11 | 1–533 | route | 48043 | 48043 | 0 | `6ce8ffe21317027f5d1171579a9059c64c7ae3bd9ba9181ceea91323254b6b25` |
| 12 | S11 | 97–260 | recovery | 15904 | 0 | 15904 | `982762d086b12b0573aa599a3ee79397167fd60b229c963ee768cce498b864ae` |
| 13 | S11 | 257–385 | recovery | 7309 | 0 | 7309 | `3ff3ff4fac7d4d8fb853fcbb3a01cb49dc4a5229390ab33ff948e6a680b7b60c` |
| 14 | S12 | 1–107 | mechanism | 9876 | 9876 | 0 | `172438a4196f13e3b9260845680abc81b98c55d3af044dbda744f5a1ca6932f8` |
| 15 | S13 | 1–221 | shared-details | 17798 | 17798 | 0 | `b930bbd0f0ce2c16d826be6a6d1d0bb4e6fe0155e611232047e0b4205e2dbfc4` |
| 16 | S14 | 1–205 | physics | 11494 | 11494 | 0 | `2c3afa3f25827508ab9bd4826249d6557d08aa44b8f4ba7f0ecd5c988a900bb8` |
| 17 | S15 | 1–219 | physics | 13780 | 13780 | 0 | `a85b0aa76a404143b049a56a25225250b4eddeabdf8ac2fc895c1aee27a4434d` |
| 18 | S16 | 1–114 | solver | 8368 | 8368 | 0 | `62e13854fe0bdddb71a4e5aae874aa13c89431bef3d47526b9af973ec8014b54` |
| 19 | S17 | 1–94 | solver | 6549 | 6549 | 0 | `bb75c9248557ae704992c1f5629c1b0ab563d764cef193f6a8e220907671dc43` |
| 20 | S18 | 1–244 | support | 20914 | 20914 | 0 | `2222800b0e5fa73d281ba68ec21961a6fcc8b38401f9c59d045139e2671dd709` |
| 21 | S19 | 1–255 | qualification | 19959 | 19959 | 0 | `d29eeca69cf60d1a0953f6fb0cc2ac3841c100159961f01394fdbdf17d5403b7` |
| 22 | S20 | 1–145 | water | 8142 | 8142 | 0 | `2ed28feb475c0b764d1a706d777e7c1ba2261c717e4a95a48d4ce28a1108d8de` |
| 23 | S20 | 209–229 | water | 4328 | 4328 | 0 | `02370aaa0e78d8d0d51b9d55e6b651a068c7675fd11161cb69037141d537554e` |
| 24 | S21 | 1–170 | custody | 19110 | 19110 | 0 | `8508b3ceaa49f081ce8c2df1d7cce39249d8f3875fea5a017404c0816cdae2c1` |
| 25 | S20 | 209–227 | recovery | 4328 | 0 | 4328 | `02370aaa0e78d8d0d51b9d55e6b651a068c7675fd11161cb69037141d537554e` |
| 26 | S22 | 193–345 | governance | 8067 | 8067 | 0 | `dc24e5aea984fdccf633f867c424046238cba1b42b4d5d9ee026e355a116e2f7` |
| 27 | S22 | 439–473 | governance | 1262 | 1262 | 0 | `10a0a37a8c056d898225753345522f11e7e483d320190579ff33a3a23c47f0ee` |
| 28 | S23 | 174–194 | measurement | 1333 | 1333 | 0 | `b8f384ce9e739d754139b7f0b56eac2a9f0e15d9d0d6a8d1718b0f98c7f26012` |
| 29 | S24 | 1–12 | governance | 736 | 736 | 0 | `95373d3c7df1f0a7e3ad7640fa22c0e137095e6cf5c4d92e3803846eb3f6ee83` |
| 30 | S25 | 1–400 | external | 33697 | 33697 | 0 | `639aa87e896820edfd92e4eec1a982efd3939a0f534f413fd32fa67aafad5317` |
| 31 | S25 | 401–800 | external | 26155 | 26155 | 0 | `3b537f717a533b1abc3b78d8cb5921c391d2c6c44e34d6b74cf9941b07ba8fc8` |
| 32 | S25 | 801–1200 | external | 46986 | 46986 | 0 | `c0bbac1dd29a63374b46f3656b4a45388e2f2e161d384336f3acff6f373829ee` |
| 33 | S25 | 974–1032 | recovery | 17512 | 0 | 17512 | `49704e9c34ae2e551dd13009c419e53f1d8157e595599e3d918f5e589830f63c` |
| 34 | S25 | 1201–1500 | external | 32839 | 32839 | 0 | `7ebd66be730a0c4374867d007cfe1f45280e1e3fd1ce37c91c1a2c666b5123de` |
| 35 | S25 | 1201–1393 | recovery | 13662 | 0 | 13662 | `a468b17cff7035d6c76eda1514a224073da7b130e5e6a07d14e56061c419dd1e` |
| 36 | S25 | 1501–1900 | external | 30550 | 30550 | 0 | `cc8c68f0428847bff9b01f7d828eb5246dec2755ee44cd249f72a410740828ce` |
| 37 | S25 | 1523–1543 | recovery | 1233 | 0 | 1233 | `2fabf6f2feeceff3d3f48b8437c4a734a4b4177244a900eee1886267716907ae` |
| 38 | S25 | 1901–2300 | external | 25636 | 25636 | 0 | `53035940631a9e02a2d1a368569ed459236a131abeed2d892e5f6339725a75d7` |
| 39 | S25 | 2301–2700 | external | 24891 | 24891 | 0 | `b625ea3e7194ff460e62ffea8cd973a2d058eb7bef091c87d54d7a60bb21519b` |
| 40 | S25 | 2701–3080 | external | 28075 | 28075 | 0 | `408f0689e8de27826a530d05d9784c73e3d1fdb7618d546c95f7b497c5e65dd9` |
| 41 | S26 | 1–400 | external | 26062 | 26062 | 0 | `61ed7a6820824e76657725f9ecf6ef75e9e7c136211628809ef315aaac19a03d` |
| 42 | S26 | 401–800 | external | 66463 | 66463 | 0 | `17b9953fa256c532c0745c792b8de7fe0ff900a2784d7eb065eb5cdab3a490e1` |
| 43 | S26 | 597–640 | recovery | 21361 | 0 | 21361 | `eeadc76306e8dcf8576b64b0e3ef5e79902c2d0837a3b880322026fd6325fa87` |
| 44 | S26 | 801–1100 | external | 13056 | 13056 | 0 | `19facb92737eb4b36e04d328e76df81a94cc5ec6da6fd7a973eab25ece3cd509` |
| 45 | S26 | 1101–1350 | external | 47667 | 47667 | 0 | `5f8db33493d5789989346fb44d467540ad5a135dae3fcafbfc988e6533ebf9aa` |
| 46 | S26 | 1289–1304 | recovery | 2052 | 0 | 2052 | `366588febff10c83f2f34aa3f809e3d7e63f4a8e97373bf1278efea156e93441` |
| 47 | S26 | 1351–1600 | external | 128373 | 128373 | 0 | `b9da13cdbe0cf9a01fcf3cfd536c047ffbfe905f46cec6c24ee34c46cc6fb31d` |
| 48 | S26 | 1411–1430 | recovery | 31882 | 0 | 31882 | `935ce35f6c3939678ee8748c31e0fb362ff3af0ac8fd13c0a1f0aeeae0a54079` |
| 49 | S26 | 1431–1450 | recovery | 40164 | 0 | 40164 | `55fb5b0a7883312900ba2ca08efddbf05ae2536ecabc161b79d9e379844dbdbc` |
| 50 | S26 | 1451–1470 | canonical_external | 3598 | 0 | 3598 | `eb02df9a69fde5bf0abed2c5d69c0069df381da212ee17968f90b9f9f7ac9acd` |
| 51 | S26 | 1471–1600 | canonical_external | 32471 | 0 | 32471 | `fc3c99f3a53a9ef33a33729fed5124f6541889a18efae89e0f3953a3111b039c` |
| 52 | S26 | 1601–1850 | canonical_external | 22222 | 22222 | 0 | `214642b49f57b6d7d631fd0efeff615f3e14710ab6afd1857fb472ff6049c72d` |
| 53 | S26 | 1851–2150 | canonical_external | 61412 | 61412 | 0 | `3c52b916bc989adb0aafaa7d2a0889f3d03148ac23af08907efc86a7858d7d6a` |
| 54 | S26 | 2022–2099 | truncation_recovery | 23998 | 0 | 23998 | `f251008d9234d8cd883a2f84c58cb191ae882d11bf56fdc97aa9a6187f5379f6` |
| 55 | S26 | 2151–2500 | canonical_external | 21876 | 21876 | 0 | `7f1510a26a97435be561c3aa17d8f309d72e51a9950e707c1add4484da17b5b4` |
| 56 | S26 | 2501–2850 | canonical_external | 26992 | 26992 | 0 | `39fb586ba4545623d8161a0d443ad92374392ff97d50801eab755a1cfe0d7455` |
| 57 | S26 | 2851–3200 | canonical_external | 55905 | 55905 | 0 | `96d11e991a94a804fffef442b0a0c84283f7d82b0a988b3e666304678a577ed6` |
| 58 | S26 | 2909–2941 | truncation_recovery | 19902 | 0 | 19902 | `faee2a7ebf882ea96bcb9eea9ec55b1749e791f5709ede10fc73846c41a1dde9` |
| 59 | S26 | 3201–3559 | canonical_external | 27757 | 27757 | 0 | `d1cb15be0bf8c1e91cc72933838c0b563e2b2a2ecd2bbdab610ec959d332f91c` |
| 60 | S27 | 1–400 | canonical_external | 23523 | 23523 | 0 | `6a511af9211e316a35de9637fbbf24450fa835072253ace16acc8a93279d48f8` |
| 61 | S27 | 401–800 | canonical_external | 45278 | 45278 | 0 | `cbe6c729fdee19a080329da4199b6985fb9155405020b1c044d04dee0d9100cd` |
| 62 | S27 | 589–615 | truncation_recovery | 8389 | 0 | 8389 | `79f66976aa4df4b65a7d3b48b650d2967fa2fe67647e0833600f83fdd57584d1` |
| 63 | S27 | 801–1108 | canonical_external | 23751 | 23751 | 0 | `60012ebc389a8a36f5dbf5f7b93fad9cb900441947aa3dca1e5d6448f49d704d` |
| 64 | S28 | 1–180 | experiment | 11541 | 11541 | 0 | `40033ea2e536984ae5a7455e99509e7e760624b78cdb08ee347597fe2f5f115f` |
| 65 | S29 | 1–84 | experiment | 5128 | 5128 | 0 | `1c813fd08e34763808d3ac8a033d18d8c5176857aff76378aa126762166c8883` |
| 66 | S30 | 1–263 | experiment | 25550 | 25550 | 0 | `6f3f0852afbfae138c4e40c62e15bf727c569611d00dec08bc8338e05ebdd9b3` |
| 67 | S31 | 1–59 | experiment | 3718 | 3718 | 0 | `b3ca642a6b4fd94f6e1459f07058cd9fc18fab747368421fc0dc6e591d143183` |
| 68 | S32 | 1–152 | governance | 6742 | 6742 | 0 | `46089ca55bb565b7dc67ea1b2075bbcc16db6902eeba1952572b072abadd0738` |
| 69 | S33 | 1–79 | governance | 3817 | 3817 | 0 | `a597aaf802917e1ed9c21db1f4453c571dde09ad48684c0f7d0ff9062b6645e6` |
| 70 | S34 | 1–221 | governance | 11159 | 11159 | 0 | `64275bad708f353c2c3b66f5b1475c070254591e26278d0a4ec442cad6e6eb0f` |
| 71 | S20 | 146–208 | canonical_closure | 3479 | 3479 | 0 | `fc4b97543523b65c7cdf43d79321b613b6cc1caf76b7d0d28381465d9b9cc253` |
| 72 | S35 | 1–79 | measurement_helper | 3544 | 3544 | 0 | `2aa217325da4b279926aa0780d1aa9427fb21d1e80a533314cbe277474d2bbde` |
| 73 | S04 | 1–22 | synthesis_recheck | 1586 | 0 | 1586 | `00f80e082f41b65a25b6864340cd89b6daffbf0d16ba54d6df5612e31d25d6fc` |
| 74 | S11 | 20–158 | claim_check | 11165 | 0 | 11165 | `7c25d1040394b65ea78422750ee5454b9f11cda92a69500e09bd0f6e3755530e` |
| 75 | S11 | 159–284 | claim_check | 12117 | 0 | 12117 | `e1589823460e06cae9f343a7dbd352ef6a70275e78ebd9ee407a822d0f54fcc0` |
| 76 | S16 | 22–107 | claim_check | 5577 | 0 | 5577 | `3d741fbc8b8ede239521d40b06b0f7e9958757b473dad8c8012090f1928e751f` |
| 77 | S12 | 27–92 | claim_check | 3685 | 0 | 3685 | `3a74984423da6e05f48a6078430c1d5097263163f732e6e7f413f3105e115be9` |
| 78 | S03 | 1–75 | synthesis_recheck | 5304 | 0 | 5304 | `0cb2505989026d5e911d696eb7eda8d96c5ebd5498da421e55a2f7682290929b` |
| 79 | S11 | 387–516 | claim_check | 8598 | 0 | 8598 | `e0c151568f1871d1764fb3b34a2f2e7a4b0e54b452ad2d2167b5b413b4969091` |
| 80 | S19 | 13–63 | claim_check | 5267 | 0 | 5267 | `1a8ed8bd521fa1d460555bd612d63ff3cc3464b42a95ee73d9243444e58463b9` |

### Retained private procedure and completeness check

The private inventory has 55 independently derived review items. The pre-completion self-audit maps all 55 to report content, including the V30 ordering correction and historical command expansion made only in a private draft. These checks are self-review of requirements prose, not independent review of production code. No complete report was written before this final candidate. The exact helper versions below are retained unmodified; none was overwritten with a later helper version. Inventory and draft are private derived notes; generated helper command inputs are not source-authority reads. Helper outputs were metadata/measurement except the line reader's separately logged source displays. The report draft/inventory were authored locally; no additional source-equivalent bytes are charged for their own prose.

| Private file | Bytes | SHA-256 | Role |
| --- | ---: | --- | --- |
| `/tmp/lse-solver09-read.py` | 877 | `e208fa7cc942378f6e33c50b10acd23ef74ee51cc497b231c06125f8f11b1dd1` | measurement/helper |
| `/tmp/lse-solver09-ledger.jsonl` | 26763 | `9b0e3c3a2ba66b9ab4af57dd13bb2071bb97515ad1b5955291ca61cb7a8dffd5` | private derived ledger/notes |
| `/tmp/lse-solver09-search1.txt` | 7443 | `ead84f2d3549a0c40474db91deef6d9470b1190f075976a2734b9b82c315b163` | private derived ledger/notes |
| `/tmp/lse-solver09-search3.txt` | 22406 | `fb3b8d4a92494eb34d1f1d8d460e21595ed3d1801ba0b88fbcf64204c8db5bcd` | private derived ledger/notes |
| `/tmp/lse-solver09-inventory.txt` | 12363 | `1ee14cb151e83225ad4ae0b6e6b18de962783b7976df41a7ae466bed2a2a348c` | private derived ledger/notes |
| `/tmp/lse-solver09-inventory-audit.md` | 2836 | `1cfb150f53de46a3155aae1314153c87448c6d5a5f55a64c911e68dbaae5ddf9` | private derived ledger/notes |
| `/tmp/lse-solver09-measure.py` | 1364 | `1af243d797bbf9a27ab2322edc7b97dc694ebaa1c06a90987f7870c88fae2efb` | measurement/helper |
| `/tmp/lse-solver09-selection.json` | 16719 | `60c019e5442721abd60a65764e62a876617d65ed0140cb260a46bac42b4c941a` | private derived ledger/notes |
| `/tmp/lse-solver09-context-report.json` | 30726 | `6ef1a94827b936140f547161f5f5f18766bc7c58b50b23e59e87c7d28697be84` | private derived ledger/notes |
| `/tmp/lse-solver09-draft-corrections.py` | 2233 | `1d8811b13ec82705e36795ff98f822af0f3caa74cf50f1e8e0232ec8d8eb39a7` | measurement/helper |
| `/tmp/lse-solver09-search-measure.py` | 1242 | `eb3b19b5acd32c86278e5537256c78e40137a8b3d361bf07259a2ca278204261` | measurement/helper |
| `/tmp/lse-solver09-search-accounting.json` | 33409 | `d232135159508c37e620052ce751d9d765ace439ac5daf561a6c3b786515ccf2` | private derived ledger/notes |
| `/tmp/lse-solver09-draft.md` | 47148 | `6f9463f7fb1119a2989f028374ad1a225986a6f7c5c634a07008547868c3168d` | private derived ledger/notes |

Exact private helper versions (no model/runtime telemetry is inferred):

`/tmp/lse-solver09-read.py`

```python
import pathlib,hashlib,json,sys
ledger=pathlib.Path('/tmp/lse-solver09-ledger.jsonl')
p=sys.argv[1]; phase=sys.argv[2]; data=pathlib.Path(p).read_bytes(); lines=data.splitlines(keepends=True); lo=int(sys.argv[3]) if len(sys.argv)>3 else 1; hi=int(sys.argv[4]) if len(sys.argv)>4 else len(lines); b=b''.join(lines[lo-1:hi]); old=[]
if ledger.exists(): old=[json.loads(x) for x in ledger.read_text().splitlines()]
seen=set()
for x in old:
 if x['path']==p: seen.update(range(x['lo'],x['hi']+1))
repeat=sum(len(lines[i-1]) for i in range(lo,hi+1) if i in seen); rec=dict(path=p,phase=phase,lo=lo,hi=hi,sha256=hashlib.sha256(data).hexdigest(),range_sha256=hashlib.sha256(b).hexdigest(),bytes=len(b),new=len(b)-repeat,repeat=repeat)
with ledger.open('a') as f:f.write(json.dumps(rec)+'\n')
print(json.dumps(rec));print(''.join(f'{i}: '+lines[i-1].decode() for i in range(lo,hi+1)))
```

`/tmp/lse-solver09-measure.py`

```python
import pathlib,json,hashlib,collections
root=pathlib.Path('/workdir/openWEPP'); records=[json.loads(x) for x in pathlib.Path('/tmp/lse-solver09-ledger.jsonl').read_text().splitlines()]
readings={}; roles={'solver09':{'bootstrap':[],'expansion':[]}}
for i,r in enumerate(records):
 p=root/r['path']; n=len(p.read_bytes().splitlines(keepends=True));key=f'r{i+1:03}'; readings[key]={'path':r['path'],'lines':[r['lo'],min(r['hi'],n)],'reason':r['phase']}; roles['solver09']['bootstrap' if r['phase'] in ('bootstrap','governance') else 'expansion'].append(key)
pathlib.Path('/tmp/lse-solver09-selection.json').write_text(json.dumps({'readings':readings,'roles':roles},indent=2)+'\n')
groups=collections.defaultdict(lambda:dict(requests=0,requested=0,new=0,repeat=0,files=set()))
for r in records:
 p=r['path'];g='LSE' if '/SC-LANDSURFACEENERGY-001' in p else 'external' if '/contracts/SC-' in p else 'frozen_experiment' if '/20260906-stage3-prospective-' in p else 'measurement_helper' if p.startswith('tools/') else 'governance_package';z=groups[g];z['requests']+=1;z['requested']+=r['bytes'];z['new']+=r['new'];z['repeat']+=r['repeat'];z['files'].add(p)
for z in groups.values():z['files']=len(z['files'])
print(json.dumps(groups,indent=2));print('TOTAL',len(records),sum(r['bytes'] for r in records),sum(r['new'] for r in records),sum(r['repeat'] for r in records))
```

`/tmp/lse-solver09-draft-corrections.py`

```python
from pathlib import Path
p=Path('/tmp/lse-solver09-draft.md');s=p.read_text();s=s.replace('V30 order is part of correctness: original guards → per-map forcing validation → applicable parent-static proof → fresh V8 dynamic validation → same-map forcing-proof consumption at its existing boundary → fallible ingress schedule → native resident revision proof at its original later location → remaining dynamic/solver/output work → finalization/commit.', 'V30 order is part of correctness: run each original guard and join the structural plan exactly when its first replaceable immutable check is reached, never hoisting it ahead of earlier support/duration/transaction/joint/forcing errors. The map then retains its original forcing validation before V8 → fresh V8 structural/dynamic validation and later pointer-identical forcing-proof consumption → fallible ingress schedule → native resident revision proof at its original later location → remaining dynamic/solver/output work → finalization/commit.')
s=s.replace('This is historical retention authority, not the new experiment\'s admission criterion or evidence that those runs occurred here.', '''Its exact historical baseline/candidate command is:

```text
timeout 1800 taskset -c 0 env RUST_MIN_STACK=67108864 CARGO_PROFILE_RELEASE_LTO=false nix develop -c cargo test --release -p openwepp-runner --lib hillslope::tests::stage3_laned_release_one_ofe_positive_baseline_profile -- --ignored --exact --nocapture --test-threads=1
```

The baseline source manifest is `78d756be1fa11ed85ee92b7d19e6c04427b01b122efaf7804d1b55d60536bbbe`, binary SHA-256 `9a91c82f1799382014c3a561e79130b5f5b665bef0667a4bdff613c91d8e573f`. Historical tuples `(run_wall_us,potential_us,rss_kib)` are `(4926758,354838,70696)`, `(4903570,353374,54624)`, `(4896095,353431,59364)`; sort each timing field separately and select its middle integer. Candidate source and binary must each remain identical before/after three repetitions. These are source-reported historical inputs, not inputs recovered or runs performed in this exercise. This is historical retention authority, not the new experiment's admission criterion or evidence that those runs occurred here.''')
p.write_text(s)
```

`/tmp/lse-solver09-search-measure.py`

```python
import pathlib,re,json,hashlib
root=pathlib.Path('/workdir/openWEPP');rows=[json.loads(x) for x in pathlib.Path('/tmp/lse-solver09-ledger.jsonl').read_text().splitlines()];seen={}
for r in rows:seen.setdefault(r['path'],set()).update(range(r['lo'],r['hi']+1))
out=[]
for name,displays in [('search1',2),('search3',1)]:
 p=pathlib.Path('/tmp/lse-solver09-'+name+'.txt');b=p.read_bytes();source=0;new=0;hit=[]
 for l in b.decode().splitlines():
  m=re.match(r'(.+?):(\d+):(.*)',l)
  if not m:continue
  path,n=m.group(1),int(m.group(2));d=(root/path).read_bytes().splitlines(keepends=True)[n-1];source+=len(d);new+=len(d) if n not in seen.setdefault(path,set()) else 0;seen[path].add(n);hit.append([path,n,len(d)])
 out.append(dict(file=str(p),sha256=hashlib.sha256(b).hexdigest(),output_bytes=len(b),display_count=displays,source_equivalent_per_display=source,source_equivalent_exposure=source*displays,additional_unique_source_bytes=new,hits=hit))
p=pathlib.Path('/tmp/lse-solver09-search-accounting.json');p.write_text(json.dumps(out,indent=2)+'\n');print(json.dumps([{k:v for k,v in r.items() if k!='hits'} for r in out],indent=2))
print('reduction_percent',100*(264863-203861)/264863,'requested_change_percent',100*(277811-264863)/264863)
```

Assembly helper retained at `/tmp/lse-solver09-assemble.py`, 9995 bytes, SHA-256 `836d7328c0c4b962442247b4e25dc443823b6780dfc2943c6cc10dd468b0fe07`; it verifies every source hash, joins the immutable ledger and appends these tables. Its exact source is retained privately, not rewritten.

Disposition: **READY — requirements reading complete; implementation/runtime/performance verdict remains unestablished.**
