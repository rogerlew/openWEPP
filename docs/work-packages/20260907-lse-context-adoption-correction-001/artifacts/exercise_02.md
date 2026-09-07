# Exercise 02 — represented-snow component-temperature Jacobian reuse

Static: requirements selection from the primary current candidate and applicable authority, not an implementation review, production owner audit, terminal verification, or scientific validation result. Ran: source reading and the repository structural context measurement function only. No numerical tests, runner, timing experiment, or memory experiment were executed.

The candidate entry permits this bounded pre-adoption exercise. Its complete declared normative set remains binding; selection is a reading route, not an exemption. I used the current `SC-LANDSURFACEENERGY-001.md` entry, its shared interface, and the represented-snow dependency-replay route, then expanded through its applicable dependencies. There was no byte target. I did not read the Git monolith, a predecessor candidate, intake matrix, reviews/rubrics/predictions, another exercise answer, or parent preservation/test artifacts. The allowed corrective worker handoff incidentally exposed predicted sizes; I disclosed that to the parent and did not use them to narrow the reading. No other-agent output was inspected. A later parent notice says entry/chapters stayed unchanged and only the registry last-reviewed date changed; I had not read that registry and derive no science from it.

## Selected authority and expansion reasons

Paths below are relative to `docs/specifications/science-contracts/contracts/`; chapter names mean `SC-LANDSURFACEENERGY-001/<name>.md`. The request ledger below supplies exact source identities and ranges, including locator searches and repeats.

- Entry and full `interface`: normative-set status, owner/regime/type boundaries and error vocabulary are prerequisite to selecting reuse rules.
- Full `dependency-replay`: INV-LANDSURFACEENERGY-164 and OBL-LANDSURFACEENERGY-C-020 are the central replay, graph, custody, canonical-fallibility and audit obligations. The entire replay section, not merely its headline invariant, applies.
- Full `nonlinear-solve`: canonical finite differences, bounds, branch freezing, source order, normalized residuals, dense solve and trial acceptance cannot be replaced by an optimization. INV-108/112/131/138/139/162/163 and C-017/C-018 also govern anchor and maximum-leaf reuse.
- Full `surface-energy`, full `soil-coupling`, and `water-vapor` through its water/vapor/error requirements: component temperatures affect reciprocal longwave, signed vapor, hydrology, liquid routing, sensible heat, residual tolerances, shared air and downstream outputs. They establish the immutable-water and exact-one lower-boundary constraints.
- `common-details` physical core and purpose/gap/calibration/tests sections: variables, state, algorithm, guards, aliases, constants, units and tolerances, plus claim limits. I did not turn the unselected producer/consumer enforcement tables into an audited implementation claim.
- Full `terminal-support`: represented-snow applicability, exact support, terminal transition, pending maps and external chronology. `map-custody` handoff, pending-map and validation portions establish private immutable authority and validation/error order. Snow-free reseal is a separate path, not a replay license.
- Full `qualification`: historical v31 obligations, EXP-R, PC1 and SG1 restrictions; its referenced frozen prospective kickoff and handoff establish the experiment boundary and pause. Those are not authority to run a production experiment in this reading exercise.
- Entire external single-file `SC-VEGETATION-001.md`, `SC-SNOWENERGY-001.md`, and `SC-COUPLEDTIME-001.md`: current/max leaf and liquid-route semantics, represented-snow lower boundary and carrier, and exact map/support/event chronology. Their whole-file rule was retained, including historical/superseding distinctions. The SnowEnergy ADR-0044 amendment overrides historical numerical dispatch; reading old passages does not revive them.
- Root/science/work-package instructions, applicable role/science guidance, numerical-solver architecture, ADR-0044, and the relevant testing-strategy sections constrain claims and future validation. The bounded selection does not declare exact-head code compliance or close any package gate.

The task does not implement or audit active snow-free litter phase physics, SurfaceLiquid/WB14 infiltration/storage, exact soil owner reconstruction, restart schema changes, or a full production owner chain. Accordingly it does not trigger the separate litter-phase, soil-custody, surface-custody, or external full WATBAL/SURFACELIQUID owner-audit routes. Their ownership is preserved, not reassigned. If the actual change touches those operations, finalization/restart representations, or a previously unproved evaluator read, expand to their complete applicable authority before implementation. Unclassified dependencies never count as independence. No unresolved scientific applicability is used to authorize replay.

## Owner, regime and support requirements

LSE owns the surface thermal transaction and coupled component temperatures; hydrology owns water authorization and transfer, soil thermal owns all soil temperatures/enthalpy, vegetation owns physiological and canopy liquid routing, and coupled time owns accepted support and atomic chronology. Required immutable inputs include full validated model/configuration/topology and ordered occupancies, original owner states and generations, forcing and exposure/support receipts, every soil/root and liquid-store operand, potential/final cap amounts and posture, and the typed snow optical/lower boundary. Tile-ground fluxes enter the OFE-ground owner exactly once with their original fractions; no covered-fraction normalization or repeated weighting is admitted.

Classify from the immutable beginning before iteration. Represented snow executes one native-identity standard Stage-3 covered map carrying that same map's exact optical receipt and `Stage3SnowCovered` lower-boundary operands. Frozen-litter V3/V4 phase, vapor, storage, current ingress and WB14 work remain inactive; their physical/exact owners, predecessor markers and receipt bytes are unchanged. V3 stays snow-free and rejects a Stage-3 boundary. No second legacy envelope, copied optical receipt, fabricated lower-boundary receipt or uncharged physical map is permissible (LSE INV-154/155/161; SnowEnergy INV-083/084/086).

The physical positive-support floor is exactly `60_000_000_000 ns`. Zero support closes only the admitted custody/event operation and runs no Newton solve. Structural one-nanosecond clock identity does not authorize physics. Below-domain support rejects; do not scale a longer result, freeze the receiver, drop a remainder, or split beneath the floor. A terminal accepted event splits support exactly; snow flux stops, only a positive admitted snow-free successor rebuilds receiver physics. The inactive snow-covered prefix becomes authenticated chronology proof only, with zero WB14 work and first snow-free physical ordinal zero (SnowEnergy INV-087; CoupledTime INV-017–024/027/031).

The external outer solver and the inner LSE Jacobian are different scopes. SnowEnergy INV-082 chooses the single canonical outer solver; its prohibition on coordinate finite differences forbids extra outer physical-map sampling, not the explicitly contracted LSE finite-difference Jacobian inside each map. Thin represented snow at/below `1 kg m^-2` uses the existing V22 terminal process; other represented-covered states use the same canonical coupled solver across constitutive active sets. Cold/frozen/mixed/refreeze/layer changes are not historical solver selectors. The v33–v57 recovery, Picard fallback, Q/ULP lattice, receipt-cycle and root-polish chain is quarantined, not available for this optimization.

## Canonical numerical behavior that must remain unchanged

INV-108 fixes ordered occupancy blocks (four hydraulic potentials, sun/shade stress factors and sun/shade/wet/stem temperatures), then the shared-air temperature/humidity, ground and ordered soil coordinates; residual blocks match the specified order. Use the full contracted occupancy/top-rank/ID ordering. For each coordinate,

`delta = sqrt(f64::EPSILON) * max(abs(x), unit_scale)`,

with scales `1 K`, `0.001 kg/kg`, `1 Pa`, `1000 mm`, and `1` for stress, as applicable. Probe minus before plus. Two admissible probes use the centered quotient. Only an admitted exact closed-bound current coordinate with one inadmissible canonical probe may use its unique inward quotient: lower `(R_plus-R_base)/delta`, upper `(R_base-R_minus)/delta`. INV-138 does not authorize a broad near-bound exception. Invalid current, neither admissible probe, or lack of the stated stencil proof fails the canonical `covered_jacobian_bound` constitutive-domain guard. Do not shrink delta, clamp a probe, project an invalid current, substitute analytic/automatic differentiation, or change arithmetic.

Preserve temperature/humidity/stress domains (`200..350 K` overall; liquid vapor at least `273.15 K`; specific humidity `0..0.1`; beta `0..1`) and all inherited potential/ci bounds. At a cap-law equality, `cap <= law` is the cap-active zero-generation-derivative branch; freeze that active set across the probe pair, then reselect at every actual trial/iterate. Domain and branch checks precede reuse.

Dense deterministic partial-pivot LU is retained, including lowest-row tie breaking and the `64*eps*norm_inf` pivot threshold. Fifty completed Newton updates and twenty halvings remain bounded; actual trial acceptance requires canonical strict normalized-residual descent. Preserve energy `1e-6 + 1e-10*max(1,sum_abs)` and water `1e-12 + 1e-9*scale` residual tolerances and step tolerances temperature `1e-8`, humidity `1e-12`, potential `1e-7`, stress `1e-10` in their native units. ci is diagnostic, not an added step-tolerance coordinate.

INV-139 preserves the full b0 evaluation first. When the current residual already passes but b0 is invalid or its step excessive, only the first valid b1..b20 prospective complete evaluation may establish the four governed step norms for acceptance of the unchanged current state. It does not install that trial or skip an earlier valid candidate; its backtrack contribution is counted. Refusal resumes unchanged strict-descent behavior. A fully passing initial FullSupply/final system is iteration zero with zero updates/backtracks and no Jacobian; the full-cap final pass is independently reevaluated from immutable beginnings, not copied from potential output.

INV-131 anchors exact zero-area sun/shade/wet temperatures to `max(T_canopy,273.15)` and stem to `T_canopy`, with exact zero physical operands and a direct deterministic row instead of differencing the max kink. The V10 wet-inactive exception additionally requires the stated nonpositive-assimilation/store-cap posture, preliminary water-rate tolerance and the unanchored physical-energy pass. Never discard a material wet residual. INV-112's diagnostic scaling applies only to the uncapped active V10 nonpositive-potential case; not V1/V8/V9, positive V10 or final-cap evaluation.

Error order is observable: serialization → model/config/state/transaction identity → topology/ownership → nonfinite → unsupported regime → constitutive domain → request/cap → pivot → backtracking → iteration limit → acceptance → component closure → control-volume closure → cross-owner closure. Return the first existing typed failure, retaining identities, norms/counts, bounds/caps and bracket/pivot/matrix/rollback evidence. A failed path yields no usable candidate. The more specific graph-integrity and source-position rules below refine this order, not replace it.

## Exact permitted reuse and dependency graph

INV-162/C-017 permits only same-solve validated covered ground/soil Jacobian anchor reuse. The covered ground row is `(T_ground-T_snow)/1e-9`, each soil row `(T_i-T_i,begin)/1e-9`. Construct the same canonical probes/domains/stencils, copy the current normalized residual vector and replace only the exact affected anchor subtraction/division, then perform the existing finite difference and dense solve. Ground snow temperature still governs boundary longwave/sensible/vapor; covered LSE adds no ground vapor/storage or soil conduction. Base, update and backtracking evaluations remain complete. Require bitwise forced-complete parity for every anchor, stencil, iterate, poison, counter, runner closure/map count and publication result.

INV-163/C-018 permits a successful current `LeafTrialState` to supply its same-occupancy maximum evaluation only when beta has exact `1.0` bits or the admitted `Inactive`/`ExactZeroPar` branch proves identical operands. `RespirationDominated`, positive and unclassified branches need a full maximum call unless exact beta-one independently applies. Current sun, then current shade, precede maximum sun and maximum shade. Never reuse a failed current result. It is a private stack-local Copy within one evaluation, never a cross-call/iteration cache. Test every leaf field, current/max branch and failure position, beta-one and its immediate lower neighbor, gas/domain bounds, source order, call counts and complete solve parity.

INV-164/C-020 permits component-temperature replay only for one validated represented-snow current Jacobian sweep. It changes neither equations nor derivatives, stencil arithmetic, tolerance, iteration/acceptance policy or physical-map scheduling.

The immutable `ValidatedCoveredComponentReplaySweepBase` binds the complete validated input generation and every referenced value, potential versus fixed-final amounts/posture, frozen cap branches, every current trial bit, independently authentic map/solve/iteration/sweep identities, and graph version/hash. It expires before the next iterate, solve, map, retry or trust boundary. A fresh private non-Clone `ValidatedCoveredComponentProbeReplay` is minted by the canonical one-coordinate probe constructor for each sign and binds exact base, coordinate, sign, delta bits, probe bits and stencil. The constructor proves that only that coordinate changed; no later whole-vector comparison manufactures this fact. Consume once on success or failure. No serialization, publication, transfer, restoration or second use.

Use validated immutable references/generations/seals to establish exact source value custody, not Debug bytes, hash length, approximate reconstructions, a probe clone, allocation-independent hashes or repeated whole-input scans. Construct/authenticate the topology graph once per applicable immutable generation; do not build strings/maps/transitive closures or whole-input hashes per probe. It is a dependency descriptor, not cached evaluator results.

The complete direct-edge table and canonical source order in `dependency-replay.md` lines 24–149 are mandatory, together with its entire fallibility matrix. Schema is `covered-component-temperature-dependency-v1`, parameterized by actual occupancy count N and soil count S; stable inclusive transitive closure selects affected nodes. Canonical lexical sorting, duplicate rejection, length framing, schema/N/S identity and SHA-256 are exact. Explicit conservative extra edges require graph versioning and oracle coverage. Unknown reads, nodes or missing edges disable the affected replay route before a capability exists.

Concretely the affected computations and dependencies include all of the following; the canonical table remains the exact spelling/order of every direct-edge record:

- The selected component-temperature probe reaches its reciprocal-longwave layer and its matching component evaluators. Longwave layer → complete column → every occupancy energy, tolerance and output, lower-ground and result assembly; a local temperature is not radiatively local.
- Sun/shade temperature reaches current/max leaf, matching vapor, hydraulic balance, sensible heat, energy/tolerance and output. Current leaf feeds maximum, vapor, hydraulic and its downstream output; maximum feeds vapor/hydraulic/output. Vapor feeds hydraulic, energy/tolerance/output and shared vapor.
- Wet temperature reaches first routing wet/finalization, occupancy wet vapor, sensible/energy/tolerance/liquid/output, and all genuine downstream routing. Stem reaches sensible/energy/tolerance/output, in addition to reciprocal radiation.
- Incident rain → preparation; preparation → wet/finalization, longwave layer and every node reading its areas, wet fractions, stores or branches. First wet → first finalization → next incident rain, stemflow prefix, route-match, terminal ground release and terminal stemflow. The incident chain reaches every lower occupancy, not merely the immediate neighbor.
- Occupancy wet vapor reaches hydraulic, wet energy/tolerance, liquid, output and shared vapor. Sensible heat reaches energy/tolerance/output and shared heat. Hydraulic, energy, tolerance and liquid feed output. Both canonical finalizations feed route-match before occupancy output/later occupancies.
- Outputs feed raw/tolerance/shared/result tails; lower ground feeds shared heat/vapor, tolerance, raw rows and result. Shared heat and vapor feed shared tolerances/rows/result; raw and tolerance rows feed normalized residuals; all residuals and terminal ground/stemflow outputs feed final result assembly.

One shared canonical node/tail implementation serves complete and replay evaluation. Do not mirror the evaluator, translate its formulas or reorder arithmetic. Preserve: trial admission; top rain; top-to-bottom prepare → first wet → first finalization → incident/stem routing; reciprocal longwave; for each occupancy current sun → current shade → maximum sun → maximum shade → vapor → second wet → hydraulic/root → sensible → energy/tolerance → second finalization → route-match → output; lower ground; shared heat then vapor/tolerances; ground/soil rows; normalization; assembly. First and second wet/finalization calls are distinct source positions.

Independent graph tests must reconstruct the exact direct-edge record set and fixed golden hashes for at least N=1/S=1 and N=2/S=6; deleting/changing any edge must fail. Reachability-only tests cannot validate an incorrect direct-edge graph. Follow this with forced-complete bitwise equality of every affected and copied node field, normalized residual and Jacobian column across all component positions, potential/final caps, routed multi-occupancy cases and canonical stencil sides.

## Eligibility, integrity and fallibility

Ineligible coordinates, multi-coordinate changes, malformed probes, non-Stage3 state, unknown graph reach or unproved independence take complete evaluation before capability creation. All psi/beta/shared-air coordinates remain complete; ground/soil has only INV-162 authority. A graph mismatch discovered before capability creation with identical operands can use complete evaluation and must retain the same first error. A stale, foreign, transferred, wrong-base/coordinate/sign/delta/probe/stencil or second-use capability fails directly as `ConstitutiveDomain("covered_component_dependency_replay_integrity")`: no complete fallback, no mutation. Once replay starts, a reachable canonical evaluator error returns directly with no fallback or later evaluator work.

Crossability means an authentic successful base and its unchanged-operand, canonical infinitesimal admitted probe can reach a fallible guard. Injected private state, changed tolerance, noncanonical steps or forged proofs are not physical counterexamples.

- Current leaf is crossable. Pair replay and forced complete at every authentic error and both leaf/occupancy positions; require identical first error/source position, no later work/fallback and exact rollback.
- Maximum leaf is historically not established crossable. Its proof must bind successful current evaluation, unchanged beta-one/branch operands and canonical probe, including exact branch/beta boundaries. Produce an authentic counterexample before reclassification; unsuccessful bounded searching is not proof.
- Routing preparation, first wet, first finalization, occupancy wet and occupancy liquid historically require successful-base and unchanged immutable rain/store/area/cap/frozen-branch/temperature-domain proofs for every guard, plus wet/dry/zero/exact-cap and routing boundary vectors. The SG1 experiment below narrowly supersedes this proof only for recomputed route families.
- Longwave noncrossability needs every finite arithmetic guard under admitted areas/temperature bounds, with reciprocal multi-occupancy, zero-area and exact-domain vectors. Hydraulics needs successful leaf/wet operands, immutable root/soil/cap/frozen inputs and every guard, with active/inactive/root-limit/tolerance boundaries and all six residual/tolerance fields.
- Route-match uses the same canonical finalization over exactly the same immutable operands; test all downstream occupancies, zero/nonzero drainage and every terminal field. A forged mismatch is integrity poison, not source-real physical crossability.
- Lower-ground proof binds unchanged validated lower-boundary/cap/frozen operands and resistance domains; compare every field. Pre-admission shape/top-rain/bounds/graph and capability integrity are separately tested with the real constructor and authentic lifetime transitions, not private mutation.
- Infallible residual/tolerance/shared/output arithmetic receives exact field parity, not invented errors. Every claimed noncrossable fallible node needs a specific guard implication plus authentic success-boundary vectors. A natural-error corpus remains a catchall for identical first-error/no-fallback/rollback behavior.

## Physical fields and conservation the oracle must protect

Reciprocal longwave uses each current component's own `T^4`, exact emissive-area weights, and `tau=exp(-0.8*Omega*(LAI+SAI))`; zero emissive area has tau one and net zero. Do not substitute a shared-air, bulk, stale or post-event temperature. Preserve complementary canopy/snow exchange and its equal-opposite ledger. Sensible and vapor are signed current surface/shared-node exchanges; retain negative condensation, one reference exchange and all occupancy plus ground terms. Neutral transfer uses the admitted geometry, positive wind and exact constants; no hidden wind/resistance floor, attenuation factor or alternate canopy-air node. Stage3 exposed-snow geometry and the covered ISBA-MEB resistances remain their separate owner-specific authorities.

Liquid vapor uses the contracted saturation polynomial and positive pressure denominator, not merely a temperature-bound check. Wet liquid carries `h_l=4218*(T-273.15)` and `Lv=2.501e6-2369*(T-273.15)`; vapor energy is signed vapor times `(h_l+Lv)`. Condensation is distinct liquid mass/enthalpy credit; latent-only closure is insufficient. Rain retains the admitted Harder–Pomeroy hourly provider temperature (with the named Kelvin conversion); runon retains its source temperature. Neither gets a substitute receiver temperature.

Water availability is the immutable beginning, before current rain/runon/release. Preserve one uncapped potential request, one hydrology authorization and one capped final evaluation, with `F=f_tile*q*dt` and `0 <= F <= A <= D`. Current ingress cannot feed same-interval evaporation/transpiration, authorize a second request or donate unused cap. V8/vegetation keeps separate terminal throughfall, initial drainage, second drainage and stemflow, with their own wet-surface temperature/enthalpy; downstream interception uses throughfall/drainage in order while stemflow bypasses lower foliage. Exact zero PAR, nonpositive assimilation, beta-one and gas-solver branches retain vegetation V10 authority; gas-root or historical stress tolerances cannot loosen same-runtime replay bit parity.

The represented snow-to-soil interface is bottom snow volume to the first OFE soil node, not per-tile: `g=2/(dz_snow/lambda_snow + dz_soil/lambda_soil)`, `G=g*(T_snow-T_soil)`, with the contracted beginning/end Crank–Nicolson mean evaluated inside the fixed point. Apply snow/soil equal-opposite heat once; covered ground/soil anchor rows do not duplicate storage or conduction. The receipt retains heat actually consumed, endpoint identities and exact operand lineage. Endpoint reconstruction tolerances `1e-9 J/m2` and `1e-8 K` are independent of physical energy closure `1e-6 J/m2`; no stale postsolve receipt, sign/node substitution, reseal repair, lost high-plus-carry or double area factor is excused.

SnowEnergy V22 actual vapor truncates sublimation mass and linked latent energy together, retains deposition, and projects total water/enthalpy once with separate solid/liquid/energy closure. Positive solid is never tolerance-deleted; fusion heat is not liquid sensible heat. Phase equality and accepted terminal liquid custody remain unchanged. Parcel mass and advected energy consume the identical ordered route-distinct sealed set, including a present empty set; aggregate equality does not excuse a missing/repeated parcel.

The outer accepted candidate is tested against its own fresh map output, not a previous output. SnowEnergy TOL-007's continuous table remains: canopy/snow temperature `1e-5 K + 1e-9 relative`; top-soil temperature `1e-8 K`; heat flux `1e-5 W/m2 + 1e-8 relative`; vapor `1e-10 kg/m2/s + 1e-6 relative`; snow water `1e-6 kg/m2 + 1e-9 relative`; snow/soil/CN energy `1e-6 J/m2 + 1e-10 relative`; density `1e-6 kg/m3`; thickness `1e-9 m + 1e-9 relative`; humidity `1e-12 kg/kg + 1e-8 relative`. Discrete branch/topology/settling/custody stays exact. These outer tolerances do not alter the inner Jacobian stencil or exact differential oracle.

## Work accounting and map custody

A fully interior N-occupancy/S-soil Jacobian has `2*(10*N+3+S)` signed probes: `2*(1+S)` anchor reuses, `8*N` component replay probes and `12*N+4` complete probes. For N=2/S=6 the named sweep is `58=14+16+28`; the remaining complete columns include eight psi, four beta and two shared-air coordinates. This is a named sweep, never a release-total expectation or a universal inward-stencil count.

Reset local audit before the first column. Record actual ordered starts/completions/failures, potential/fixed-final posture, topology, coordinate/stencil and independent authentic map/solve/iteration/sweep identity. `Completed` covers all completed work; `Failed` includes attempted work. `ShortCircuited` is allowed only on a real reachable nonerror canonical path; otherwise absent. `RejectedBeforeProbe` is per-column and does not manufacture a failed sweep. Logical starts partition exactly into anchor/replay/complete actual starts; completions reconcile separately. IDs cannot be copied ordinal labels or pointer hashes.

Reset the release audit separately and retain every sealed sweep record without filtering: all lifecycle states, caps, inward and failed sweeps, histogram and sums joined to authentic map/solve records. Historical C-020 requires a real all-centered 58 record within the release, not a fabricated counter. The seven expected-red declarations establish structure only; actual dispatcher connectivity, runtime counters and full oracles still need proof.

Canonical outer map chronology is `Initial@0`, `FixedPointAdjudication@1`, then zero through five contiguous `MultisecantAdjudication(n)@(n+1)` maps. Successful `M=N_multisecant+2` lies in 2..7 under the unchanged eight-map ceiling. Every map has complete physical/custody validation. Initial is private physical-only. Each later map is one non-Clone/non-wire pending value consumed exactly once as history on outer nonclosure, typed adaptive rejection on dependent-only nonclosure, or its own final envelope after both close. FinalAccepted is a disposition, not a second charge or replay. No final-only constructors precede adjudication; constructor failure cannot become history. Success has M validated endpoints, M−1 nonfinal outcomes, one final disposition/attempt/envelope. Physical failure has charge but no valid endpoint; constructor failure has accepted physical endpoint/attempt but no envelope. Every map/direct/rejected/unselected candidate publishes zero; only accepted composed parent commits all owners and publication once (LSE INV-161/C-016; SnowEnergy INV-086/C-054; CoupledTime INV-030/OBL-013).

Validation-once optimizations do not relax semantic guards. LSE INV-159/C-014 requires one complete successful validation and immutable exact generation/value custody, not wire/digest-only authority. V30/C-019 preserves lazy source order: forcing validation before V8 proof; fresh dynamic V8 state; fallible ingress after V8 and before native-resident proof. A private exact resident revision can remove only the stated duplicate beginning validation/canonical-byte work. Earlier errors win competing poisons. Authentic workload counts, fresh restart validation and source-real poison tests are required. CoupledTime private slab/publication capabilities are different proofs with their own live incarnation/revision/owner-tail bindings; they cannot supply Jacobian replay authority or survive restart.

## Experimental limits and required future evidence

Historical LSE v31 remains reverted FAIL/HOLD. C-020's historical release protocol requires three CPU-0 runs on one unchanged source/binary, stack `67108864`, release LTO disabled and the named ignored runner profile with timeout 1800. Median target/total ceilings are `253431 us`/`4803570 us`, each a 100000-us improvement from `353431`/`4903570`; every run peak RSS is at most `65536 KiB`. Exact source/outlet/storage are `0.8488061229561478`, `0.8471105124736579`, `0.0016956104824910018`, clamp zero, and workload `48/56/20/32/4`. The full-run sweep aggregate and genuine named 58 sweep, complete bitwise solves and protected closure all matter. Any failing criterion requires full reversion, not a partial salvage or changed science tolerance. Historical source/binary identities are bindings recorded by the chapter, not present-day success evidence.

EXP-STAGE3-20260906-R is separately frozen baseline A plus replay; never F plus replay, never old retained artifacts as new identity authority. Only this prospective experiment replaces the historical all-centered existential and fixed timing/RSS retention decisions with its actual-stencil and relative protocols. Production remains HOLD and the experiment is paused. No timing result, coherent kit/hash, absence of a found counterexample, or structural expected red proves admission.

PC1 narrows experimental sun/shade replay to exact beta-one base bits. The current probe leaf call executes; the canonical maximum call may reuse only under INV-163 equality. Adjacent-below-one, other beta or unproved classification takes complete evaluation before capability creation. Wet/stem replay must prove all leaf-call operands unchanged, including routed descendants. Every other fallibility-matrix obligation survives. The reported bounded 198-case search is not a noncrossability proof. Independent dual review plus runtime exact-beta/branch/position/helper/pre-cap/error/rollback vectors and nonzero completed replay are required.

SG1 applies only to the five recomputed wet-route families and affected descendants. Successful base plus admitted temperature bounds does not guarantee the `pressure - 0.378*e_sat` guard remains valid. For recomputed nodes, invoke the same canonical body with the same probe operands, branches and source position, obtaining either bit-identical success or the existing first error. This narrowly replaces a noncrossability proof for recomputed nodes; it does not authorize copying a fallible result. Every skipped/copied guard still needs successful-base plus exact-unchanged-operands and graph proof. Keep first/second wet/finalization distinct. PC1/current-leaf, longwave, hydraulic, route-match and lower-ground proofs stay binding. Require independent dual review of real canonical calls and copy/recompute distinctions, successful boundary vectors and natural-error catchall; neither injected guard errors nor unsuccessful witness search replaces the proof.

The prospective actual-stencil oracle independently derives coordinates, bounds, topology and graph: centered contributes two, inward one, rejected contributes no unattempted call. Reconcile ordered starts, completions and failures, disjoint work classes and every dropped/started sweep. Require nonzero completed replay, not eligibility labels; no hardcoded 54/58 or state manipulation to obtain a desired count. Keep independent correctness instrumentation separate from matched timing instrumentation.

Before any future timing: complete forced-full node/evaluation/Jacobian/solve oracles; independent direct-edge hashes; authentic error/custody matrix; potential/final caps; canonical boundary stencils; no-fallback source/call proofs; real runner output, independent operand closure, adaptive/map/publication counts and rollback/restart parity. Exact cross-arm equality is an experiment assertion, not a new stopping criterion. Required A0/A1/A3 protections remain nondeferrable; source-coupled or observational checks cannot replace them. Future implementation validation must classify the real diff and run its applicable direct checks/full regression under the governing package; this report runs none.

The frozen prospective protocol admits independently reviewed F first and R separately: coherent source/build/input/toolchain/executable identity, builds outside measurement, two warmups and twelve fresh alternating AB/BA pairs, matched affinity/environment/audits and direct immutable discovered binaries. Require at least 5% median whole-run gain with uncertainty and no CPU regression; at most one prescribed extension to 24 combined pairs. Memory uses six separate pairs, distinguishes endpoint VmRSS, lifetime peak and named phase boundaries, includes explicit teardown for both arms, three processes of ten one-day repetitions and 10/19-OFE checks. No global allocator, stack/ASLR manipulation, censored samples or arithmetic reinterpretation of RSS as live heap. Keep engineering budget failures separate. The handoff retains a full-regression FAIL and unrun timing/memory; this selection neither resolves that pause nor promotes a candidate.

## Reading and measurement disclosure

All requests are recorded below in order. `ranges` are inclusive one-based source lines; full-file requests resolve to `[1,last]`. `bytes` counts original UTF-8 source bytes in those ranges, including original line endings and excluding display line numbers/tool envelopes. `sha256` is the SHA-256 of the whole current source file, not a selected excerpt. Search requests list only emitted source lines; search scans are not treated as full-file comprehension. Metadata requests have empty ranges and zero text exposure, though the program accessed the source to measure it. Truncated requests remain in requested exposure; their complete requested payload is not claimed delivered. Repairs establish the final finite observed union. The exact number of source bytes/tokens actually delivered by truncated tools is UNOBSERVED.

Non-source locator/tool requests, separately from source exposure: `pwd`; `tools/agents/find-agents --for <LSE entry> <this report>`; `context_report.py --help`; and `rg --files` over the LSE chapter directory/decisions filtered for chapter paths and ADR-0044. They returned working-directory, instruction-chain, help and filename metadata only. The later source-heading searches are individual ledger rows, including repeats. No Git source was requested. Final report construction reaccessed the listed files for byte/hash/range accounting and executed the already-read `context_report.py` via `runpy`; those are mechanical measurement accesses, not additional prose readings or token savings. The report file itself was not fed back for answer repair.

The metrics below were computed by `tools/agents/context_report.py::report` from these explicit ranges in the working tree. They measure structural declared exposure, not session telemetry. Bootstrap and expansion unique values can overlap; the combined finite union deduplicates `(path,source SHA256,byte position)`. Exposure sums repeats and requested truncated ranges, so it is not delivered context. Automatic runtime instructions, output overhead, compaction, reasoning, parent workflow and delivered tokens are not measured. No whole-workflow or token-efficiency claim follows.

```json
{
  "bootstrap": {
    "unique_bytes": 42228,
    "exposure_bytes": 42228
  },
  "expansion": {
    "unique_bytes": 1109339,
    "exposure_bytes": 1323252
  },
  "combined_unique_bytes": 1151567,
  "requested_exposure_bytes": 1365480,
  "delivered_tokens": "UNOBSERVED",
  "workflow_total": "UNOBSERVED",
  "runtime_automatic_context": "UNOBSERVED beyond declared instructions",
  "measure": "UTF-8 requested source ranges; finite union deduplicates original path plus source SHA256; repeated and repaired truncated requests count as exposure; original handoff uses verified exact snapshot for measurement"
}
```

Complete request list:

```json
[
  {
    "request": 1,
    "path": "AGENTS.md",
    "ranges": [
      [
        1,
        121
      ]
    ],
    "bytes": 9508,
    "sha256": "9bd7887bcae731debc64689a4774ac222d529bcd527ebea2293e3ffd600289f6",
    "phase": "bootstrap",
    "kind": "text",
    "status": "observed",
    "reason": "root instructions, cat full",
    "source_bytes": 9508
  },
  {
    "request": 2,
    "path": "AGENTS.md",
    "ranges": [],
    "bytes": 0,
    "sha256": "9bd7887bcae731debc64689a4774ac222d529bcd527ebea2293e3ffd600289f6",
    "phase": "bootstrap",
    "kind": "metadata",
    "status": "observed",
    "reason": "repeated full-source hash inventory; content not re-emitted",
    "source_bytes": 9508
  },
  {
    "request": 3,
    "path": "docs/specifications/science-contracts/AGENTS.md",
    "ranges": [
      [
        1,
        84
      ]
    ],
    "bytes": 6532,
    "sha256": "bace4ab3e33119e0239f700f92c53707d884f297e92d2454a86c477758c930c7",
    "phase": "bootstrap",
    "kind": "text",
    "status": "observed",
    "reason": "instruction chain and primary current entry",
    "source_bytes": 6532
  },
  {
    "request": 4,
    "path": "docs/work-packages/AGENTS.md",
    "ranges": [
      [
        1,
        105
      ]
    ],
    "bytes": 6642,
    "sha256": "29e2f48a0b69fd9f43eb8d3a6dc2b72d30792337f90a2141b89df105726282a5",
    "phase": "bootstrap",
    "kind": "text",
    "status": "observed",
    "reason": "instruction chain and primary current entry",
    "source_bytes": 6642
  },
  {
    "request": 5,
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md",
    "ranges": [
      [
        1,
        63
      ]
    ],
    "bytes": 3958,
    "sha256": "c94f30d0e4462284769d498ca31fe80e7a5b0d5026c233c85c1ddd310d2b6058",
    "phase": "bootstrap",
    "kind": "text",
    "status": "observed",
    "reason": "instruction chain and primary current entry",
    "source_bytes": 3958
  },
  {
    "request": 6,
    "path": "docs/work-packages/20260907-lse-context-adoption-correction-001/package.md",
    "ranges": [
      [
        1,
        75
      ]
    ],
    "bytes": 5304,
    "sha256": "0cb2505989026d5e911d696eb7eda8d96c5ebd5498da421e55a2f7682290929b",
    "phase": "bootstrap",
    "kind": "text",
    "status": "observed",
    "reason": "scope-only package/handoff, then shared interface; batch remainder truncated but this file observed",
    "source_bytes": 5304
  },
  {
    "request": 7,
    "path": "docs/work-packages/20260907-lse-context-adoption-correction-001/artifacts/worker-handoff.md",
    "ranges": [
      [
        1,
        26
      ]
    ],
    "bytes": 2174,
    "sha256": "e35455905556092e67eaf8e4c79cc2b8747b4bcd7a1d55089e162d185498797c",
    "phase": "bootstrap",
    "kind": "text",
    "status": "observed",
    "reason": "scope-only package/handoff, then shared interface; batch remainder truncated but this file observed",
    "source_bytes": 2174,
    "source_snapshot": "docs/work-packages/20260907-lse-context-adoption-correction-001/artifacts/reading-snapshots/worker-handoff-before-fresh.md"
  },
  {
    "request": 8,
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/interface.md",
    "ranges": [
      [
        1,
        66
      ]
    ],
    "bytes": 8110,
    "sha256": "22ed48fa58a32920ddfde47c339650896c16978b06bf4a722dc1b094106e08f6",
    "phase": "bootstrap",
    "kind": "text",
    "status": "observed",
    "reason": "scope-only package/handoff, then shared interface; batch remainder truncated but this file observed",
    "source_bytes": 8110
  },
  {
    "request": 9,
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/dependency-replay.md",
    "ranges": [
      [
        1,
        299
      ]
    ],
    "bytes": 28905,
    "sha256": "3526a4ab967543dc23b8c0fd174241607ea1d1dd060c059682497829df83e9b1",
    "phase": "expansion",
    "kind": "text",
    "status": "truncated_request_repaired",
    "reason": "entry snow replay route; batch output truncated",
    "source_bytes": 28905
  },
  {
    "request": 10,
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/dependency-replay.md",
    "ranges": [
      [
        1,
        125
      ]
    ],
    "bytes": 10487,
    "sha256": "3526a4ab967543dc23b8c0fd174241607ea1d1dd060c059682497829df83e9b1",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "repair truncated replay reading",
    "source_bytes": 28905
  },
  {
    "request": 11,
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/dependency-replay.md",
    "ranges": [
      [
        126,
        299
      ]
    ],
    "bytes": 18418,
    "sha256": "3526a4ab967543dc23b8c0fd174241607ea1d1dd060c059682497829df83e9b1",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "repair truncated replay reading",
    "source_bytes": 28905
  },
  {
    "request": 12,
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/common-details.md",
    "ranges": [
      [
        1,
        1
      ],
      [
        2,
        2
      ],
      [
        3,
        3
      ],
      [
        4,
        4
      ],
      [
        5,
        5
      ],
      [
        6,
        6
      ],
      [
        7,
        7
      ],
      [
        8,
        8
      ],
      [
        9,
        9
      ],
      [
        10,
        10
      ],
      [
        11,
        11
      ],
      [
        12,
        12
      ],
      [
        13,
        13
      ],
      [
        14,
        14
      ],
      [
        15,
        15
      ],
      [
        16,
        16
      ],
      [
        17,
        17
      ],
      [
        18,
        18
      ],
      [
        19,
        19
      ],
      [
        20,
        20
      ],
      [
        21,
        21
      ],
      [
        22,
        22
      ],
      [
        23,
        23
      ],
      [
        24,
        24
      ],
      [
        25,
        25
      ],
      [
        26,
        26
      ],
      [
        27,
        27
      ],
      [
        30,
        30
      ],
      [
        31,
        31
      ],
      [
        50,
        50
      ],
      [
        51,
        51
      ],
      [
        72,
        72
      ],
      [
        73,
        73
      ],
      [
        74,
        74
      ],
      [
        107,
        107
      ],
      [
        108,
        108
      ],
      [
        109,
        109
      ],
      [
        134,
        134
      ],
      [
        135,
        135
      ],
      [
        136,
        136
      ],
      [
        176,
        176
      ],
      [
        177,
        177
      ],
      [
        195,
        195
      ],
      [
        196,
        196
      ],
      [
        197,
        197
      ],
      [
        220,
        220
      ],
      [
        221,
        221
      ],
      [
        231,
        231
      ],
      [
        232,
        232
      ],
      [
        273,
        273
      ],
      [
        274,
        274
      ],
      [
        277,
        277
      ],
      [
        278,
        278
      ],
      [
        279,
        279
      ],
      [
        296,
        296
      ],
      [
        297,
        297
      ],
      [
        298,
        298
      ],
      [
        313,
        313
      ],
      [
        314,
        314
      ],
      [
        315,
        315
      ],
      [
        330,
        330
      ],
      [
        331,
        331
      ],
      [
        332,
        332
      ],
      [
        361,
        361
      ],
      [
        362,
        362
      ],
      [
        363,
        363
      ],
      [
        378,
        378
      ],
      [
        379,
        379
      ],
      [
        380,
        380
      ],
      [
        408,
        408
      ],
      [
        409,
        409
      ],
      [
        418,
        418
      ],
      [
        419,
        419
      ]
    ],
    "bytes": 3685,
    "sha256": "a4e06643713c94fb771e87719cbafc9994cf2bd88050ff5c9e0ae9ef81cf454a",
    "phase": "expansion",
    "kind": "search",
    "status": "observed",
    "reason": "heading/anchor and leading dependency scan; emitted lines only",
    "source_bytes": 41620
  },
  {
    "request": 13,
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/nonlinear-solve.md",
    "ranges": [
      [
        1,
        1
      ],
      [
        2,
        2
      ],
      [
        3,
        3
      ],
      [
        4,
        4
      ],
      [
        5,
        5
      ],
      [
        6,
        6
      ],
      [
        7,
        7
      ],
      [
        8,
        8
      ],
      [
        9,
        9
      ],
      [
        10,
        10
      ],
      [
        11,
        11
      ],
      [
        12,
        12
      ],
      [
        13,
        13
      ],
      [
        14,
        14
      ],
      [
        15,
        15
      ],
      [
        16,
        16
      ],
      [
        17,
        17
      ],
      [
        18,
        18
      ],
      [
        19,
        19
      ],
      [
        20,
        20
      ],
      [
        21,
        21
      ],
      [
        22,
        22
      ],
      [
        23,
        23
      ],
      [
        24,
        24
      ],
      [
        25,
        25
      ],
      [
        26,
        26
      ],
      [
        27,
        27
      ],
      [
        110,
        110
      ],
      [
        111,
        111
      ],
      [
        142,
        142
      ],
      [
        143,
        143
      ],
      [
        192,
        192
      ],
      [
        193,
        193
      ],
      [
        247,
        247
      ],
      [
        248,
        248
      ],
      [
        318,
        318
      ],
      [
        319,
        319
      ],
      [
        320,
        320
      ],
      [
        384,
        384
      ],
      [
        385,
        385
      ],
      [
        386,
        386
      ],
      [
        434,
        434
      ],
      [
        435,
        435
      ],
      [
        450,
        450
      ],
      [
        451,
        451
      ]
    ],
    "bytes": 3830,
    "sha256": "a5396285816d2f8c6c1b58b314b9d583440f552437e4e09a95f4c8ef1e8f2513",
    "phase": "expansion",
    "kind": "search",
    "status": "observed",
    "reason": "heading/anchor and leading dependency scan; emitted lines only",
    "source_bytes": 37448
  },
  {
    "request": 14,
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/surface-energy.md",
    "ranges": [
      [
        1,
        1
      ],
      [
        2,
        2
      ],
      [
        3,
        3
      ],
      [
        4,
        4
      ],
      [
        5,
        5
      ],
      [
        6,
        6
      ],
      [
        7,
        7
      ],
      [
        8,
        8
      ],
      [
        9,
        9
      ],
      [
        10,
        10
      ],
      [
        11,
        11
      ],
      [
        12,
        12
      ],
      [
        13,
        13
      ],
      [
        14,
        14
      ],
      [
        15,
        15
      ],
      [
        16,
        16
      ],
      [
        17,
        17
      ],
      [
        18,
        18
      ],
      [
        19,
        19
      ],
      [
        20,
        20
      ],
      [
        21,
        21
      ],
      [
        22,
        22
      ],
      [
        23,
        23
      ],
      [
        24,
        24
      ],
      [
        25,
        25
      ],
      [
        26,
        26
      ],
      [
        27,
        27
      ],
      [
        29,
        29
      ],
      [
        30,
        30
      ],
      [
        47,
        47
      ],
      [
        48,
        48
      ],
      [
        83,
        83
      ],
      [
        84,
        84
      ],
      [
        124,
        124
      ],
      [
        125,
        125
      ],
      [
        191,
        191
      ],
      [
        192,
        192
      ],
      [
        200,
        200
      ],
      [
        201,
        201
      ]
    ],
    "bytes": 3157,
    "sha256": "f741c16f6dcbe37cf3a3ab5c2e7dfcb418fed6b03900b41fda483d5ec53a03e2",
    "phase": "expansion",
    "kind": "search",
    "status": "observed",
    "reason": "heading/anchor and leading dependency scan; emitted lines only",
    "source_bytes": 11388
  },
  {
    "request": 15,
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/qualification.md",
    "ranges": [
      [
        1,
        1
      ],
      [
        2,
        2
      ],
      [
        3,
        3
      ],
      [
        4,
        4
      ],
      [
        5,
        5
      ],
      [
        6,
        6
      ],
      [
        7,
        7
      ],
      [
        8,
        8
      ],
      [
        9,
        9
      ],
      [
        10,
        10
      ],
      [
        11,
        11
      ],
      [
        12,
        12
      ],
      [
        13,
        13
      ],
      [
        14,
        14
      ],
      [
        15,
        15
      ],
      [
        16,
        16
      ],
      [
        17,
        17
      ],
      [
        18,
        18
      ],
      [
        19,
        19
      ],
      [
        20,
        20
      ],
      [
        21,
        21
      ],
      [
        22,
        22
      ],
      [
        23,
        23
      ],
      [
        24,
        24
      ],
      [
        25,
        25
      ],
      [
        26,
        26
      ],
      [
        27,
        27
      ],
      [
        64,
        64
      ],
      [
        65,
        65
      ],
      [
        124,
        124
      ],
      [
        125,
        125
      ],
      [
        184,
        184
      ],
      [
        185,
        185
      ]
    ],
    "bytes": 2871,
    "sha256": "8f3644331eb66e2dab120dad203b1cc55167d97961f68d1d737a99b012926a62",
    "phase": "expansion",
    "kind": "search",
    "status": "observed",
    "reason": "heading/anchor and leading dependency scan; emitted lines only",
    "source_bytes": 18915
  },
  {
    "request": 16,
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/map-custody.md",
    "ranges": [
      [
        1,
        1
      ],
      [
        2,
        2
      ],
      [
        3,
        3
      ],
      [
        4,
        4
      ],
      [
        5,
        5
      ],
      [
        6,
        6
      ],
      [
        7,
        7
      ],
      [
        8,
        8
      ],
      [
        9,
        9
      ],
      [
        10,
        10
      ],
      [
        11,
        11
      ],
      [
        12,
        12
      ],
      [
        13,
        13
      ],
      [
        14,
        14
      ],
      [
        15,
        15
      ],
      [
        16,
        16
      ],
      [
        17,
        17
      ],
      [
        18,
        18
      ],
      [
        19,
        19
      ],
      [
        20,
        20
      ],
      [
        21,
        21
      ],
      [
        22,
        22
      ],
      [
        23,
        23
      ],
      [
        24,
        24
      ],
      [
        25,
        25
      ],
      [
        26,
        26
      ],
      [
        27,
        27
      ],
      [
        35,
        35
      ],
      [
        36,
        36
      ],
      [
        37,
        37
      ],
      [
        87,
        87
      ],
      [
        88,
        88
      ],
      [
        89,
        89
      ],
      [
        126,
        126
      ],
      [
        127,
        127
      ],
      [
        128,
        128
      ],
      [
        195,
        195
      ],
      [
        196,
        196
      ],
      [
        197,
        197
      ],
      [
        338,
        338
      ],
      [
        339,
        339
      ],
      [
        347,
        347
      ],
      [
        348,
        348
      ]
    ],
    "bytes": 2508,
    "sha256": "dd67c49876597d991f9aef28f83d14658d0b5ad0bc1a7b47342ead747bdf42f3",
    "phase": "expansion",
    "kind": "search",
    "status": "observed",
    "reason": "heading/anchor and leading dependency scan; emitted lines only",
    "source_bytes": 36738
  },
  {
    "request": 17,
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/water-vapor.md",
    "ranges": [
      [
        1,
        1
      ],
      [
        2,
        2
      ],
      [
        3,
        3
      ],
      [
        4,
        4
      ],
      [
        5,
        5
      ],
      [
        6,
        6
      ],
      [
        7,
        7
      ],
      [
        8,
        8
      ],
      [
        9,
        9
      ],
      [
        10,
        10
      ],
      [
        11,
        11
      ],
      [
        12,
        12
      ],
      [
        13,
        13
      ],
      [
        14,
        14
      ],
      [
        15,
        15
      ],
      [
        16,
        16
      ],
      [
        17,
        17
      ],
      [
        18,
        18
      ],
      [
        19,
        19
      ],
      [
        20,
        20
      ],
      [
        21,
        21
      ],
      [
        22,
        22
      ],
      [
        23,
        23
      ],
      [
        24,
        24
      ],
      [
        25,
        25
      ],
      [
        26,
        26
      ],
      [
        27,
        27
      ],
      [
        65,
        65
      ],
      [
        66,
        66
      ],
      [
        67,
        67
      ],
      [
        118,
        118
      ],
      [
        119,
        119
      ],
      [
        120,
        120
      ],
      [
        146,
        146
      ],
      [
        147,
        147
      ],
      [
        171,
        171
      ],
      [
        172,
        172
      ],
      [
        209,
        209
      ],
      [
        210,
        210
      ],
      [
        218,
        218
      ],
      [
        219,
        219
      ]
    ],
    "bytes": 3012,
    "sha256": "6fd1c830c7d1fc9fbe465c28a50eade50d1585cdd30b8c599a5ab8e520fe64ff",
    "phase": "expansion",
    "kind": "search",
    "status": "observed",
    "reason": "heading/anchor and leading dependency scan; emitted lines only",
    "source_bytes": 16549
  },
  {
    "request": 18,
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/nonlinear-solve.md",
    "ranges": [
      [
        1,
        191
      ]
    ],
    "bytes": 12845,
    "sha256": "a5396285816d2f8c6c1b58b314b9d583440f552437e4e09a95f4c8ef1e8f2513",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "canonical dependency expansion",
    "source_bytes": 37448
  },
  {
    "request": 19,
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/nonlinear-solve.md",
    "ranges": [
      [
        192,
        455
      ]
    ],
    "bytes": 24603,
    "sha256": "a5396285816d2f8c6c1b58b314b9d583440f552437e4e09a95f4c8ef1e8f2513",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "canonical dependency expansion",
    "source_bytes": 37448
  },
  {
    "request": 20,
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/surface-energy.md",
    "ranges": [
      [
        1,
        205
      ]
    ],
    "bytes": 11388,
    "sha256": "f741c16f6dcbe37cf3a3ab5c2e7dfcb418fed6b03900b41fda483d5ec53a03e2",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "canonical dependency expansion",
    "source_bytes": 11388
  },
  {
    "request": 21,
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/qualification.md",
    "ranges": [
      [
        1,
        245
      ]
    ],
    "bytes": 18915,
    "sha256": "8f3644331eb66e2dab120dad203b1cc55167d97961f68d1d737a99b012926a62",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "canonical dependency expansion",
    "source_bytes": 18915
  },
  {
    "request": 22,
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/common-details.md",
    "ranges": [
      [
        1,
        219
      ]
    ],
    "bytes": 17980,
    "sha256": "a4e06643713c94fb771e87719cbafc9994cf2bd88050ff5c9e0ae9ef81cf454a",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "canonical dependency expansion",
    "source_bytes": 41620
  },
  {
    "request": 23,
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/common-details.md",
    "ranges": [
      [
        277,
        444
      ]
    ],
    "bytes": 12707,
    "sha256": "a4e06643713c94fb771e87719cbafc9994cf2bd88050ff5c9e0ae9ef81cf454a",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "canonical dependency expansion",
    "source_bytes": 41620
  },
  {
    "request": 24,
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/water-vapor.md",
    "ranges": [
      [
        1,
        145
      ]
    ],
    "bytes": 8262,
    "sha256": "6fd1c830c7d1fc9fbe465c28a50eade50d1585cdd30b8c599a5ab8e520fe64ff",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "canonical dependency expansion",
    "source_bytes": 16549
  },
  {
    "request": 25,
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/soil-coupling.md",
    "ranges": [
      [
        1,
        208
      ]
    ],
    "bytes": 13488,
    "sha256": "9a9fc30b02736875d961ece3df220f243abc1b9ea446cb582bbbcf8d2f8b07a9",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "canonical dependency expansion",
    "source_bytes": 13488
  },
  {
    "request": 26,
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/terminal-support.md",
    "ranges": [],
    "bytes": 0,
    "sha256": "c74b6cfdcf3bc10bc59c4ff9ec23b022e28bb6a55fc395d847dc03790afe3f16",
    "phase": "expansion",
    "kind": "metadata",
    "status": "observed",
    "reason": "source size and line-count inventory only; no content exposure",
    "source_bytes": 21230
  },
  {
    "request": 27,
    "path": "docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md",
    "ranges": [],
    "bytes": 0,
    "sha256": "3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa",
    "phase": "expansion",
    "kind": "metadata",
    "status": "observed",
    "reason": "source size and line-count inventory only; no content exposure",
    "source_bytes": 248829
  },
  {
    "request": 28,
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md",
    "ranges": [],
    "bytes": 0,
    "sha256": "94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d",
    "phase": "expansion",
    "kind": "metadata",
    "status": "observed",
    "reason": "source size and line-count inventory only; no content exposure",
    "source_bytes": 497785
  },
  {
    "request": 29,
    "path": "docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md",
    "ranges": [],
    "bytes": 0,
    "sha256": "1d10611b5143be201381f283ec5594d9f65c99f8252e944dfae57076a8a699f7",
    "phase": "expansion",
    "kind": "metadata",
    "status": "observed",
    "reason": "source size and line-count inventory only; no content exposure",
    "source_bytes": 92552
  },
  {
    "request": 30,
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/terminal-support.md",
    "ranges": [
      [
        1,
        200
      ]
    ],
    "bytes": 13875,
    "sha256": "c74b6cfdcf3bc10bc59c4ff9ec23b022e28bb6a55fc395d847dc03790afe3f16",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "canonical dependency expansion",
    "source_bytes": 21230
  },
  {
    "request": 31,
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/terminal-support.md",
    "ranges": [
      [
        201,
        245
      ]
    ],
    "bytes": 7355,
    "sha256": "c74b6cfdcf3bc10bc59c4ff9ec23b022e28bb6a55fc395d847dc03790afe3f16",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "canonical dependency expansion",
    "source_bytes": 21230
  },
  {
    "request": 32,
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/map-custody.md",
    "ranges": [
      [
        35,
        86
      ]
    ],
    "bytes": 3900,
    "sha256": "dd67c49876597d991f9aef28f83d14658d0b5ad0bc1a7b47342ead747bdf42f3",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "canonical dependency expansion",
    "source_bytes": 36738
  },
  {
    "request": 33,
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/map-custody.md",
    "ranges": [
      [
        126,
        240
      ]
    ],
    "bytes": 8748,
    "sha256": "dd67c49876597d991f9aef28f83d14658d0b5ad0bc1a7b47342ead747bdf42f3",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "canonical dependency expansion",
    "source_bytes": 36738
  },
  {
    "request": 34,
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/map-custody.md",
    "ranges": [
      [
        241,
        355
      ]
    ],
    "bytes": 18405,
    "sha256": "dd67c49876597d991f9aef28f83d14658d0b5ad0bc1a7b47342ead747bdf42f3",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "canonical dependency expansion",
    "source_bytes": 36738
  },
  {
    "request": 35,
    "path": "docs/work-packages/role-review.md",
    "ranges": [
      [
        1,
        7
      ]
    ],
    "bytes": 1400,
    "sha256": "2193e9538ad22dad5678fc5bb59080f686a543f697fec4983208214a73cc5eb1",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "applicable governance or qualification frozen authority",
    "source_bytes": 1400
  },
  {
    "request": 36,
    "path": "docs/standards/AGENTS.md",
    "ranges": [
      [
        1,
        58
      ]
    ],
    "bytes": 4052,
    "sha256": "b62a33d9a70a3d3f45e7eccdcf05ca3a194486fcdbfc7bd439371cf8f0ae3015",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "applicable governance or qualification frozen authority",
    "source_bytes": 4052
  },
  {
    "request": 37,
    "path": "docs/standards/numerical-solver-architecture.md",
    "ranges": [
      [
        1,
        152
      ]
    ],
    "bytes": 6742,
    "sha256": "46089ca55bb565b7dc67ea1b2075bbcc16db6902eeba1952572b072abadd0738",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "applicable governance or qualification frozen authority",
    "source_bytes": 6742
  },
  {
    "request": 38,
    "path": "docs/work-packages/20260906-stage3-prospective-mechanism-experiments-001/prompts/active/kickoff.md",
    "ranges": [
      [
        1,
        263
      ]
    ],
    "bytes": 25550,
    "sha256": "6f3f0852afbfae138c4e40c62e15bf727c569611d00dec08bc8338e05ebdd9b3",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "applicable governance or qualification frozen authority",
    "source_bytes": 25550
  },
  {
    "request": 39,
    "path": "docs/standards/prompt-wording-guidance.md",
    "ranges": [
      [
        1,
        1
      ],
      [
        7,
        7
      ],
      [
        174,
        174
      ],
      [
        195,
        195
      ]
    ],
    "bytes": 84,
    "sha256": "294385612b565fe8be3e9e530caa0792c901406ecf53abc9402649e5568ffe3f",
    "phase": "expansion",
    "kind": "search",
    "status": "observed",
    "reason": "locate role assurance instructions",
    "source_bytes": 12057
  },
  {
    "request": 40,
    "path": "docs/work-packages/science-obligations.md",
    "ranges": [
      [
        1,
        98
      ]
    ],
    "bytes": 6149,
    "sha256": "1a0377a3362a6c34d783796b6756854f8630e1f05e994dc3d963eaafd16c542e",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "science governance",
    "source_bytes": 6149
  },
  {
    "request": 41,
    "path": "docs/standards/prompt-wording-guidance.md",
    "ranges": [
      [
        174,
        194
      ]
    ],
    "bytes": 1333,
    "sha256": "294385612b565fe8be3e9e530caa0792c901406ecf53abc9402649e5568ffe3f",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "role wording and assurance scope",
    "source_bytes": 12057
  },
  {
    "request": 42,
    "path": "docs/work-packages/20260906-stage3-prospective-mechanism-experiments-001/artifacts/worker-handoff.md",
    "ranges": [
      [
        1,
        59
      ]
    ],
    "bytes": 3718,
    "sha256": "b3ca642a6b4fd94f6e1459f07058cd9fc18fab747368421fc0dc6e591d143183",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "frozen experiment authority status; no execution or prior artifact lookup",
    "source_bytes": 3718
  },
  {
    "request": 43,
    "path": "docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md",
    "ranges": [
      [
        1,
        230
      ]
    ],
    "bytes": 20059,
    "sha256": "3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external single-file contract retained whole-file reading",
    "source_bytes": 248829
  },
  {
    "request": 44,
    "path": "docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md",
    "ranges": [
      [
        231,
        450
      ]
    ],
    "bytes": 16760,
    "sha256": "3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external single-file contract retained whole-file reading",
    "source_bytes": 248829
  },
  {
    "request": 45,
    "path": "docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md",
    "ranges": [
      [
        451,
        700
      ]
    ],
    "bytes": 16514,
    "sha256": "3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external single-file contract retained whole-file reading",
    "source_bytes": 248829
  },
  {
    "request": 46,
    "path": "docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md",
    "ranges": [
      [
        701,
        980
      ]
    ],
    "bytes": 20878,
    "sha256": "3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external single-file contract retained whole-file reading",
    "source_bytes": 248829
  },
  {
    "request": 47,
    "path": "docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md",
    "ranges": [
      [
        981,
        1130
      ]
    ],
    "bytes": 26451,
    "sha256": "3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external single-file contract retained whole-file reading",
    "source_bytes": 248829
  },
  {
    "request": 48,
    "path": "docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md",
    "ranges": [
      [
        1131,
        1400
      ]
    ],
    "bytes": 20775,
    "sha256": "3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external single-file contract retained whole-file reading",
    "source_bytes": 248829
  },
  {
    "request": 49,
    "path": "docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md",
    "ranges": [
      [
        1401,
        1700
      ]
    ],
    "bytes": 36214,
    "sha256": "3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external single-file contract retained whole-file reading",
    "source_bytes": 248829
  },
  {
    "request": 50,
    "path": "docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md",
    "ranges": [
      [
        1701,
        2010
      ]
    ],
    "bytes": 19061,
    "sha256": "3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external single-file contract retained whole-file reading",
    "source_bytes": 248829
  },
  {
    "request": 51,
    "path": "docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md",
    "ranges": [
      [
        2011,
        2300
      ]
    ],
    "bytes": 19151,
    "sha256": "3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external single-file contract retained whole-file reading",
    "source_bytes": 248829
  },
  {
    "request": 52,
    "path": "docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md",
    "ranges": [
      [
        2301,
        2610
      ]
    ],
    "bytes": 18860,
    "sha256": "3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external single-file contract retained whole-file reading",
    "source_bytes": 248829
  },
  {
    "request": 53,
    "path": "docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md",
    "ranges": [
      [
        2611,
        2870
      ]
    ],
    "bytes": 16521,
    "sha256": "3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external single-file contract retained whole-file reading",
    "source_bytes": 248829
  },
  {
    "request": 54,
    "path": "docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md",
    "ranges": [
      [
        2871,
        3080
      ]
    ],
    "bytes": 17585,
    "sha256": "3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external single-file contract retained whole-file reading",
    "source_bytes": 248829
  },
  {
    "request": 55,
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md",
    "ranges": [
      [
        1,
        160
      ]
    ],
    "bytes": 9378,
    "sha256": "94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external whole-file expansion; bounded chunks and explicit truncation repair",
    "source_bytes": 497785
  },
  {
    "request": 56,
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md",
    "ranges": [
      [
        161,
        340
      ]
    ],
    "bytes": 12649,
    "sha256": "94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external whole-file expansion; bounded chunks and explicit truncation repair",
    "source_bytes": 497785
  },
  {
    "request": 57,
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md",
    "ranges": [
      [
        341,
        530
      ]
    ],
    "bytes": 12662,
    "sha256": "94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external whole-file expansion; bounded chunks and explicit truncation repair",
    "source_bytes": 497785
  },
  {
    "request": 58,
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md",
    "ranges": [
      [
        531,
        710
      ]
    ],
    "bytes": 48679,
    "sha256": "94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d",
    "phase": "expansion",
    "kind": "text",
    "status": "truncated_request_repaired",
    "reason": "external whole-file expansion; bounded chunks and explicit truncation repair",
    "source_bytes": 497785
  },
  {
    "request": 59,
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md",
    "ranges": [
      [
        531,
        620
      ]
    ],
    "bytes": 32692,
    "sha256": "94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external whole-file expansion; bounded chunks and explicit truncation repair",
    "source_bytes": 497785
  },
  {
    "request": 60,
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md",
    "ranges": [
      [
        621,
        710
      ]
    ],
    "bytes": 15987,
    "sha256": "94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external whole-file expansion; bounded chunks and explicit truncation repair",
    "source_bytes": 497785
  },
  {
    "request": 61,
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md",
    "ranges": [
      [
        711,
        900
      ]
    ],
    "bytes": 13918,
    "sha256": "94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external whole-file expansion; bounded chunks and explicit truncation repair",
    "source_bytes": 497785
  },
  {
    "request": 62,
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md",
    "ranges": [
      [
        901,
        1130
      ]
    ],
    "bytes": 9275,
    "sha256": "94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external whole-file expansion; bounded chunks and explicit truncation repair",
    "source_bytes": 497785
  },
  {
    "request": 63,
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md",
    "ranges": [
      [
        1131,
        1270
      ]
    ],
    "bytes": 12275,
    "sha256": "94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external whole-file expansion; bounded chunks and explicit truncation repair",
    "source_bytes": 497785
  },
  {
    "request": 64,
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md",
    "ranges": [
      [
        1271,
        1350
      ]
    ],
    "bytes": 34412,
    "sha256": "94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external whole-file expansion; bounded chunks and explicit truncation repair",
    "source_bytes": 497785
  },
  {
    "request": 65,
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md",
    "ranges": [
      [
        1351,
        1500
      ]
    ],
    "bytes": 97469,
    "sha256": "94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d",
    "phase": "expansion",
    "kind": "text",
    "status": "truncated_request_repaired",
    "reason": "external whole-file expansion; bounded chunks and explicit truncation repair",
    "source_bytes": 497785
  },
  {
    "request": 66,
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md",
    "ranges": [
      [
        1351,
        1412
      ]
    ],
    "bytes": 21720,
    "sha256": "94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external whole-file expansion; bounded chunks and explicit truncation repair",
    "source_bytes": 497785
  },
  {
    "request": 67,
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md",
    "ranges": [
      [
        1413,
        1427
      ]
    ],
    "bytes": 22265,
    "sha256": "94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external whole-file expansion; bounded chunks and explicit truncation repair",
    "source_bytes": 497785
  },
  {
    "request": 68,
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md",
    "ranges": [
      [
        1428,
        1437
      ]
    ],
    "bytes": 22971,
    "sha256": "94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external whole-file expansion; bounded chunks and explicit truncation repair",
    "source_bytes": 497785
  },
  {
    "request": 69,
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md",
    "ranges": [
      [
        1438,
        1446
      ]
    ],
    "bytes": 21165,
    "sha256": "94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d",
    "phase": "expansion",
    "kind": "text",
    "status": "truncated_request_repaired",
    "reason": "external whole-file expansion; bounded chunks and explicit truncation repair",
    "source_bytes": 497785
  },
  {
    "request": 70,
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md",
    "ranges": [
      [
        1438,
        1445
      ]
    ],
    "bytes": 17871,
    "sha256": "94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external whole-file expansion; bounded chunks and explicit truncation repair",
    "source_bytes": 497785
  },
  {
    "request": 71,
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md",
    "ranges": [
      [
        1446,
        1547
      ]
    ],
    "bytes": 21810,
    "sha256": "94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external whole-file expansion; bounded chunks and explicit truncation repair",
    "source_bytes": 497785
  },
  {
    "request": 72,
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md",
    "ranges": [
      [
        1548,
        1603
      ]
    ],
    "bytes": 21921,
    "sha256": "94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external whole-file expansion; bounded chunks and explicit truncation repair",
    "source_bytes": 497785
  },
  {
    "request": 73,
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md",
    "ranges": [
      [
        1604,
        1848
      ]
    ],
    "bytes": 21949,
    "sha256": "94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external whole-file expansion; bounded chunks and explicit truncation repair",
    "source_bytes": 497785
  },
  {
    "request": 74,
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md",
    "ranges": [
      [
        1849,
        2031
      ]
    ],
    "bytes": 21746,
    "sha256": "94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external whole-file expansion; bounded chunks and explicit truncation repair",
    "source_bytes": 497785
  },
  {
    "request": 75,
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md",
    "ranges": [
      [
        2032,
        2101
      ]
    ],
    "bytes": 21647,
    "sha256": "94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external whole-file expansion; bounded chunks and explicit truncation repair",
    "source_bytes": 497785
  },
  {
    "request": 76,
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md",
    "ranges": [
      [
        2102,
        2216
      ]
    ],
    "bytes": 21962,
    "sha256": "94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external whole-file expansion; bounded chunks and explicit truncation repair",
    "source_bytes": 497785
  },
  {
    "request": 77,
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md",
    "ranges": [
      [
        2217,
        2544
      ]
    ],
    "bytes": 21898,
    "sha256": "94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external whole-file expansion; bounded chunks and explicit truncation repair",
    "source_bytes": 497785
  },
  {
    "request": 78,
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md",
    "ranges": [
      [
        2545,
        2829
      ]
    ],
    "bytes": 21867,
    "sha256": "94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external whole-file expansion; bounded chunks and explicit truncation repair",
    "source_bytes": 497785
  },
  {
    "request": 79,
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md",
    "ranges": [
      [
        2830,
        2911
      ]
    ],
    "bytes": 21971,
    "sha256": "94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external whole-file expansion; bounded chunks and explicit truncation repair",
    "source_bytes": 497785
  },
  {
    "request": 80,
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md",
    "ranges": [
      [
        2912,
        3000
      ]
    ],
    "bytes": 21933,
    "sha256": "94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external whole-file expansion; bounded chunks and explicit truncation repair",
    "source_bytes": 497785
  },
  {
    "request": 81,
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md",
    "ranges": [
      [
        3001,
        3322
      ]
    ],
    "bytes": 21998,
    "sha256": "94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external whole-file expansion; bounded chunks and explicit truncation repair",
    "source_bytes": 497785
  },
  {
    "request": 82,
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md",
    "ranges": [
      [
        3323,
        3559
      ]
    ],
    "bytes": 19008,
    "sha256": "94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external whole-file expansion; bounded chunks and explicit truncation repair",
    "source_bytes": 497785
  },
  {
    "request": 83,
    "path": "docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md",
    "ranges": [
      [
        1,
        377
      ]
    ],
    "bytes": 21981,
    "sha256": "1d10611b5143be201381f283ec5594d9f65c99f8252e944dfae57076a8a699f7",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external single-file chronology contract retained whole-file reading",
    "source_bytes": 92552
  },
  {
    "request": 84,
    "path": "docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md",
    "ranges": [
      [
        378,
        591
      ]
    ],
    "bytes": 21768,
    "sha256": "1d10611b5143be201381f283ec5594d9f65c99f8252e944dfae57076a8a699f7",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external single-file chronology contract retained whole-file reading",
    "source_bytes": 92552
  },
  {
    "request": 85,
    "path": "docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md",
    "ranges": [
      [
        592,
        765
      ]
    ],
    "bytes": 21956,
    "sha256": "1d10611b5143be201381f283ec5594d9f65c99f8252e944dfae57076a8a699f7",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external single-file chronology contract retained whole-file reading",
    "source_bytes": 92552
  },
  {
    "request": 86,
    "path": "docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md",
    "ranges": [
      [
        766,
        1046
      ]
    ],
    "bytes": 21994,
    "sha256": "1d10611b5143be201381f283ec5594d9f65c99f8252e944dfae57076a8a699f7",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external single-file chronology contract retained whole-file reading",
    "source_bytes": 92552
  },
  {
    "request": 87,
    "path": "docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md",
    "ranges": [
      [
        1047,
        1108
      ]
    ],
    "bytes": 4853,
    "sha256": "1d10611b5143be201381f283ec5594d9f65c99f8252e944dfae57076a8a699f7",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "external single-file chronology contract retained whole-file reading",
    "source_bytes": 92552
  },
  {
    "request": 88,
    "path": "tools/agents/context_report.py",
    "ranges": [
      [
        10,
        10
      ],
      [
        68,
        68
      ],
      [
        73,
        73
      ],
      [
        77,
        77
      ],
      [
        78,
        78
      ],
      [
        79,
        79
      ]
    ],
    "bytes": 296,
    "sha256": "2aa217325da4b279926aa0780d1aa9427fb21d1e80a533314cbe277474d2bbde",
    "phase": "expansion",
    "kind": "search",
    "status": "observed",
    "reason": "measurement-tool input schema discovery",
    "source_bytes": 3544
  },
  {
    "request": 89,
    "path": "docs/standards/testing-and-gate-strategy.md",
    "ranges": [
      [
        1,
        1
      ],
      [
        8,
        8
      ],
      [
        17,
        17
      ],
      [
        33,
        33
      ],
      [
        52,
        52
      ],
      [
        77,
        77
      ],
      [
        129,
        129
      ],
      [
        131,
        131
      ],
      [
        137,
        137
      ],
      [
        164,
        164
      ],
      [
        172,
        172
      ],
      [
        182,
        182
      ],
      [
        193,
        193
      ],
      [
        219,
        219
      ],
      [
        224,
        224
      ],
      [
        230,
        230
      ],
      [
        235,
        235
      ],
      [
        240,
        240
      ],
      [
        274,
        274
      ],
      [
        292,
        292
      ],
      [
        314,
        314
      ],
      [
        346,
        346
      ],
      [
        368,
        368
      ],
      [
        386,
        386
      ],
      [
        403,
        403
      ],
      [
        417,
        417
      ],
      [
        427,
        427
      ],
      [
        439,
        439
      ],
      [
        457,
        457
      ],
      [
        474,
        474
      ],
      [
        487,
        487
      ]
    ],
    "bytes": 827,
    "sha256": "56277d9848d80b0672ebeaaa8277ac2ad9888c90e54b4f31aa7c5b8ca9b04230",
    "phase": "expansion",
    "kind": "search",
    "status": "observed",
    "reason": "locate role-review linked sections",
    "source_bytes": 25142
  },
  {
    "request": 90,
    "path": "tools/agents/context_report.py",
    "ranges": [
      [
        1,
        79
      ]
    ],
    "bytes": 3544,
    "sha256": "2aa217325da4b279926aa0780d1aa9427fb21d1e80a533314cbe277474d2bbde",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "measurement-tool schema and counting semantics",
    "source_bytes": 3544
  },
  {
    "request": 91,
    "path": "docs/decisions/0044-prohibit-accretive-production-solver-dispatch.md",
    "ranges": [
      [
        1,
        79
      ]
    ],
    "bytes": 3817,
    "sha256": "a597aaf802917e1ed9c21db1f4453c571dde09ad48684c0f7d0ff9062b6645e6",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "root and solver architecture authority",
    "source_bytes": 3817
  },
  {
    "request": 92,
    "path": "docs/standards/testing-and-gate-strategy.md",
    "ranges": [
      [
        193,
        345
      ],
      [
        439,
        473
      ]
    ],
    "bytes": 9329,
    "sha256": "56277d9848d80b0672ebeaaa8277ac2ad9888c90e54b4f31aa7c5b8ca9b04230",
    "phase": "expansion",
    "kind": "text",
    "status": "observed",
    "reason": "role-review correctness/evidence/anti-evasion/closure requirements",
    "source_bytes": 25142
  },
  {
    "request": 93,
    "path": "docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md",
    "ranges": [],
    "bytes": 0,
    "sha256": "3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa",
    "phase": "expansion",
    "kind": "metadata",
    "status": "observed",
    "reason": "final external whole-source identity inventory; no text re-emitted",
    "source_bytes": 248829
  },
  {
    "request": 94,
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md",
    "ranges": [],
    "bytes": 0,
    "sha256": "94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d",
    "phase": "expansion",
    "kind": "metadata",
    "status": "observed",
    "reason": "final external whole-source identity inventory; no text re-emitted",
    "source_bytes": 497785
  },
  {
    "request": 95,
    "path": "docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md",
    "ranges": [],
    "bytes": 0,
    "sha256": "1d10611b5143be201381f283ec5594d9f65c99f8252e944dfae57076a8a699f7",
    "phase": "expansion",
    "kind": "metadata",
    "status": "observed",
    "reason": "final external whole-source identity inventory; no text re-emitted",
    "source_bytes": 92552
  },
  {
    "request": 96,
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md",
    "ranges": [],
    "bytes": 0,
    "sha256": "94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d",
    "phase": "expansion",
    "kind": "metadata",
    "status": "observed",
    "reason": "reconstruct exact prior 23000-byte-bounded truncated request end 1446; no text re-emitted",
    "source_bytes": 497785
  },
  {
    "request": 97,
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/dependency-replay.md",
    "ranges": [
      [
        3,
        3
      ],
      [
        18,
        18
      ],
      [
        19,
        19
      ],
      [
        22,
        22
      ],
      [
        23,
        23
      ],
      [
        24,
        24
      ],
      [
        150,
        150
      ],
      [
        151,
        151
      ],
      [
        162,
        162
      ],
      [
        163,
        163
      ],
      [
        289,
        289
      ],
      [
        290,
        290
      ],
      [
        295,
        295
      ],
      [
        296,
        296
      ]
    ],
    "bytes": 588,
    "sha256": "3526a4ab967543dc23b8c0fd174241607ea1d1dd060c059682497829df83e9b1",
    "phase": "expansion",
    "kind": "search",
    "status": "observed",
    "reason": "exact report anchor locator; repeated headings only",
    "source_bytes": 28905
  },
  {
    "request": 98,
    "path": "AGENTS.md",
    "ranges": [],
    "bytes": 0,
    "source_bytes": 9508,
    "sha256": "9bd7887bcae731debc64689a4774ac222d529bcd527ebea2293e3ffd600289f6",
    "phase": "expansion",
    "kind": "metadata",
    "status": "metadata_only",
    "reason": "reconcile original handoff reading identity against preserved snapshot; no updated instruction or science prose read"
  },
  {
    "request": 99,
    "path": "docs/specifications/science-contracts/AGENTS.md",
    "ranges": [],
    "bytes": 0,
    "source_bytes": 6532,
    "sha256": "bace4ab3e33119e0239f700f92c53707d884f297e92d2454a86c477758c930c7",
    "phase": "expansion",
    "kind": "metadata",
    "status": "metadata_only",
    "reason": "reconcile original handoff reading identity against preserved snapshot; no updated instruction or science prose read"
  },
  {
    "request": 100,
    "path": "docs/work-packages/AGENTS.md",
    "ranges": [],
    "bytes": 0,
    "source_bytes": 6642,
    "sha256": "29e2f48a0b69fd9f43eb8d3a6dc2b72d30792337f90a2141b89df105726282a5",
    "phase": "expansion",
    "kind": "metadata",
    "status": "metadata_only",
    "reason": "reconcile original handoff reading identity against preserved snapshot; no updated instruction or science prose read"
  },
  {
    "request": 101,
    "path": "docs/work-packages/20260907-lse-context-adoption-correction-001/package.md",
    "ranges": [],
    "bytes": 0,
    "source_bytes": 5304,
    "sha256": "0cb2505989026d5e911d696eb7eda8d96c5ebd5498da421e55a2f7682290929b",
    "phase": "expansion",
    "kind": "metadata",
    "status": "metadata_only",
    "reason": "reconcile original handoff reading identity against preserved snapshot; no updated instruction or science prose read"
  },
  {
    "request": 102,
    "path": "docs/work-packages/20260907-lse-context-adoption-correction-001/artifacts/worker-handoff.md",
    "ranges": [],
    "bytes": 0,
    "source_bytes": 4408,
    "sha256": "dc48700f924b96364201b464e090c97938017c12261aee6e5d15cdadf2c9bcb4",
    "phase": "expansion",
    "kind": "metadata",
    "status": "metadata_only",
    "reason": "reconcile original handoff reading identity against preserved snapshot; no updated instruction or science prose read"
  },
  {
    "request": 103,
    "path": "docs/work-packages/20260907-lse-context-adoption-correction-001/artifacts/reading-snapshots/worker-handoff-before-fresh.md",
    "ranges": [],
    "bytes": 0,
    "source_bytes": 2174,
    "sha256": "e35455905556092e67eaf8e4c79cc2b8747b4bcd7a1d55089e162d185498797c",
    "phase": "expansion",
    "kind": "metadata",
    "status": "metadata_only",
    "reason": "reconcile original handoff reading identity against preserved snapshot; no updated instruction or science prose read"
  }
]
```

Metadata reconciliation: the allowed handoff changed after my initial read. That read remains exactly 2174 bytes / SHA256 `e35455905556092e67eaf8e4c79cc2b8747b4bcd7a1d55089e162d185498797c`, preserved at `docs/work-packages/20260907-lse-context-adoption-correction-001/artifacts/reading-snapshots/worker-handoff-before-fresh.md`. I did not read the newer 4408-byte handoff prose. The final measurement uses the verified original snapshot, mapped to its original path/source identity; it does not attribute later instructions to the initial request. `source_bytes` identifies complete source size while `bytes` identifies requested textual ranges. The first measurement was recalculated solely to correct this source-identity drift; no scientific answer, rubric, review, or other exercise output was consulted. Mechanical byte/hash computation and the measurement tool revisit listed sources without emitting their prose and are excluded from textual exposure; the two report-construction/measurement passes are not delivered-token telemetry.
