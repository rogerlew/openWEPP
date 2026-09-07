# Fresh scientific reading: represented-snow Jacobian reuse

Static: independent first-attempt reading of the frozen current canonical LSE directory and its required source. Ran: source reads, byte/hash/range accounting and the read-only context measurement tool only. No production code, tests, numerical workflow, experiment, predecessor evidence, original monolith, Git history, intake, rubric, review, other answer or parent validation output was inspected or executed.

Reader: `/root/solver_iteration3`. Requested role: fresh correctness reader. Effective model/effort and delivered-token telemetry: UNOBSERVED. This is a statement of requirements and reading sufficiency, not an implementation PASS. The assigned report is my only repository write.

## Answer and authority boundary

Represented-snow component-temperature reuse is permitted only as the exact same canonical finite-difference evaluation with a smaller set of recomputed dependency nodes, inside one validated current Jacobian sweep. It is not an approximation, derivative change, cross-evaluation cache, whole-map reuse or alternate solver. The binding anchors are `dependency-replay.md#component-temperature-jacobian-dependency-replay-amendment` (`INV-LANDSURFACEENERGY-164`, `OBL-LANDSURFACEENERGY-C-020`), `nonlinear-solve.md` (`INV-138/139/162/163`), `surface-energy.md`, and the separate historical/prospective qualifications in `qualification.md`. Exact input and error custody also requires `map-custody.md#handoff/#pending`, `water-vapor.md#vapor/#water/#errors/#canonical-invariants/#canonical-obligations`, and their recursively required chapters.

The physical regime is selected from immutable beginning state before evaluation. LSE owns the surface thermal control volume; vegetation owns canopy physiology, persistent canopy liquid and its routing; hydrology owns ground/litter/soil water and authorized withdrawal, infiltration and runoff; soil thermal owns soil temperature/enthalpy; Stage 3 owns represented ground-snow thermodynamics. The snow-covered LSE evaluator consumes the snow lower boundary. Its ground and soil coordinates are identity anchors, not additional snow/soil thermal solves. The external snow--soil Crank--Nicolson receipt and equal/opposite transfer remain physical work of their proper owners. A single lane snow owner is OFE-ground; complete ordered covered/open tile contributions sum once with their original fractions, without covered-fraction renormalization, first-tile soil selection or duplicated lane heat.

`terminal-support.md`, `SC-SNOWENERGY-001#INV-SNOWENERGY-082/083/086/087`, and `SC-COUPLEDTIME-001` prevent confusing inner LSE Jacobian probes with outer covered-map charges. Snow-free support has no covered solver. Represented snow at or below 1 kg m^-2 uses the sole non-CoE terminal one-volume process; above that boundary one canonical covered solver handles its constitutive active sets. The current outer algorithm uses Initial@0 followed by FixedPointAdjudication@1 and, if needed, contiguous MultisecantAdjudication(n)@(n+1), n=1..5. A successfully closed pending map becomes FinalAccepted at the same charge: M=N+2, 2<=M<=7, beneath the eight-map ceiling. It is not physically replayed to construct final ownership. Outer nonclosure consumes history; dependent-only nonclosure consumes adaptive rejection; final-constructor failure cannot fall through to history. No map publishes; only accepted composed-parent commit does. Historical 96-map root-polish, Q-lattice, exact receipt-cycle and fallback methods in the full SnowEnergy file are explicitly superseded numerical dispatch, not reusable current authority.

Native represented snow retains exact optical/lower-boundary receipts in one standard native-identity map. Frozen-litter V3/V4 vapor, phase, storage, current-ingress and WB14 remain inactive with identical bytes. V3 stays snow-free and rejects Stage3SnowCovered. At the accepted terminal event, only a positive exact post-event snow-free child resumes litter physics; zero remaining support performs no physics. Prefix consumption authenticates chronology, adds no under-snow work or WB14 ordinal, and the first snow-free physical ordinal is zero. Exact u128-nanosecond half-open support, common duration conversion, support floor, typed custody, restart and parent rollback survive the replay optimization.

## Canonical numerical evaluation

The coordinate order is four hydraulic potentials, two beta values, sun-leaf, shade-leaf, wet and stem temperatures for each authenticated occupancy, then shared canopy-air temperature, humidity, ground temperature and configured ordered soil temperatures: 10N+3+S coordinates. Preserve the exact canonical order and frozen cap/branch choices.

For coordinate x, delta=sqrt(binary64 epsilon)*max(abs(x), unit_scale). Construct/evaluate minus before plus. Interior columns use `(R_plus-R_minus)/(2*delta)`. Only an admitted exact closed bound with a unique inward admissible probe uses `(R_plus-R_current)/delta` at a lower bound or `(R_current-R_minus)/delta` at an upper bound. Invalid current state or unavailable required stencil gives the existing typed `covered_jacobian_bound` error. This does not permit arbitrary near-bound one-sided differences, shrinking delta, clipping a probe, reversing signs, analytic replacement or altered arithmetic. Scales are 1 K, 0.001 specific humidity, 1 Pa, 1000 mm hydraulic potential and 1 beta; canonical temperature, liquid-temperature, humidity, beta and hydraulic domains remain binding.

The surrounding solver remains the canonical normalized-residual dense partial-pivot LU, deterministic lowest-row tie handling, pivot threshold 64*epsilon*matrix infinity norm, 50 accepted-update bound and halving exponents 0..20 with strict decrease. Energy tolerance is 1e-6 + 1e-10*max(1,sum_abs_terms); water tolerance is 1e-12 + 1e-9*scale. Governed step limits are temperature 1e-8 K, humidity 1e-12, hydraulic potential 1e-7 mm and beta 1e-10. CI is diagnostic, not another LSE coordinate. `INV-139` allows an unchanged-current no-update witness only with the whole current residual passing: after full prospective b=0 fails domain or governed-step acceptance, examine the FIRST domain-valid halved complete evaluation. All its governed steps must pass; do not search later smaller halvings to manufacture a witness. Otherwise retain strict-decrease update or typed rejection. The recorded exponent includes examined work.

At a frozen availability cap, `cap <= constitutive law` selects the active cap and its zero generalized derivative, including equality. Canonical temperature bounds remain 200..350 K, liquid temperatures 273.15..350 K, shared humidity 0..0.1 and beta 0..1; hydraulic potential domains are unchanged. V10 nonpositive-PAR potential solves alone may use the admitted diagonal unit scaling `J_y=J_x D`, solve in y and recover the step with D. This is not permission to scale positive-PAR, fixed-final or earlier-model solves or change the derivative. Inactive liquid sun/shade/wet anchors are `max(T_canopy,273.15 K)` while stem anchors T_canopy; zero area produces the deterministic anchor row and zero physical contribution. The narrow nonpositive V10 wet-anchor exception additionally needs its uncapped/tiny-store-water predicate and the unanchored physical energy already within tolerance; residual discard is forbidden.

`INV-162` separately permits ground/soil identity-anchor probe assembly after a successful complete represented-snow base. Ground residual is `(T_ground-T_snow)/1e-9`; soil row is `(T_soil-T_begin)/1e-9`, with the existing subtraction/division order. Snow boundary temperature supplies longwave and boundary sensible/vapor supplies shared closure; the anchored coordinate supplies no extra ground vapor, storage or conduction inside this evaluator. Copy all unaffected residual rows and replace the exact anchor row, then use the ordinary finite-difference arithmetic. Base, prospective and backtracking evaluations remain complete. This says nothing about eliminating the outside snow--soil heat receipt.

`INV-163` separately permits one successful current-leaf result to provide its beta-one maximum in the SAME evaluation only when beta bits equal exactly 1, or the canonical Inactive/ExactZeroPar branch proves beta independence. RespirationDominated, PositiveAssimilation or unclassified states require the normal maximum evaluation unless beta is exactly 1. Preserve sun-current, shade-current, sun-maximum, shade-maximum order and source errors. No unsuccessful result or result from another sweep/map may be borrowed. One ULP below beta=1 is not beta=1. External V10 rules matter: exact signed-zero PAR shares the dark branch, current nonpositive carbon exchange has no beta attenuation, full supply only, partial positive supply fails VEG-E119, and the positive low-light bracket extension is the canonical constructive dark bound followed by the same Brent solve, not a fit or new root method.

## Dependency and custody proof

One immutable `ValidatedCoveredComponentReplaySweepBase` binds the successful current evaluation; exact input generation AND all referenced inputs; potential/fixed-final caps and posture; frozen branches; trial bits; authentic map, solve, Newton-iteration and sweep identities; graph version/hash. Drop it before the next iteration, solve, map or retry. A static authenticated graph descriptor may survive as topology metadata; evaluated values may not.

Every actual signed probe gets a new non-Clone `ValidatedCoveredComponentProbeReplay`. The canonical constructor proves its sole-coordinate change, sign, perturbation and probe bits, bound relation and selected stencil, all joined to the same base. Consume once on success OR error. It cannot be transferred, restored, serialized, reused, or used after its base. Exact immutable borrows/generations and compact seals minted at real validation points are allowed; Debug strings, hash length, allocation-independent digest alone, approximate reconstruction, whole-probe cloning and repeated full-vector scans are not equivalent custody. Do not rebuild strings, ordered maps, transitive closure or whole-input hashes in the hot path.

The schema is `covered-component-temperature-dependency-v1`. Stable expanded node/edge records are lexical, duplicate-free, length-prefixed and SHA-256 bound with version,N,S. Inclusive transitive closure comes from the complete direct-edge generator in INV-164, in stable node order. Unknown reads/nodes or missing edges never prove independence: they conservatively make the affected coordinate ineligible before replay. Extra conservative edges must be explicit, versioned and represented in the independent comparison.

Required dependencies include all these families; the canonical direct-edge table, not a reachability sketch, is the complete edge authority:

- Every component-temperature probe changes its layer longwave input; reciprocal `longwave.column` reaches ALL component energy/tolerances and occupancy outputs, lower ground output and final output. No same-occupancy-only radiation shortcut.
- A sun/shade probe reaches matching current and maximum leaf, vapor, hydraulic, sensible, energy/tolerance and output. Current leaf feeds maximum/vapor/hydraulic/energy/tolerance/output; maximum feeds vapor/hydraulic/output; vapor also feeds shared vapor.
- Wet probes reach first route wet/finalize AND second occupancy wet/liquid; they are distinct source calls. First prepare feeds every node reading changed wet area, fraction, store or branch, including longwave and leaf/vapor/hydraulic/sensible/energy/tolerance/liquid/output dependencies as applicable.
- First wet feeds first finalize; route finalization propagates exact incident rain and stemflow prefixes through every lower occupancy, not only the adjacent rank, and into terminal ground-release/stemflow. Both liquid finalizations feed route-match; mismatch fails before later occupancy/shared work.
- Second wet reaches hydraulic, wet energy/tolerance, liquid, output and shared vapor. Sensible reaches matching energy/tolerance/output and shared heat. Hydraulic/energy/tolerance/liquid feed occupancy output. Outputs feed ordered residual rows, reductions and final assembly.
- Lower ground output reaches shared heat/vapor/tolerance and corresponding raw/tolerance rows. Shared rows feed their exact normalization and final output; terminal release and stemflow remain independent result fields. Copied successful values are allowed only outside the inclusive affected closure.

Complete evaluator and replay call ONE shared canonical implementation for common nodes and tail. Do not mirror, translate or reorder physical, branch, tolerance, residual or output math. Exact chronology is trial admission, top-rain guard, top-to-bottom prepare/wet/finalize/incident-stemflow; reciprocal longwave; per occupancy sun current, shade current, sun maximum, shade maximum, vapor, second wet, hydraulic/root, sensible, energy/tolerance, second liquid, route-match, output; lower ground output; shared heat then vapor and tolerances; ground/soil residual rows; normalization; complete result assembly. Recompute reachable nodes at these original positions and retain first-error order.

`INV-159` supports validated custody, not guard erasure: parent-static structural validation occurs once, its plan is created lazily at the first charged map; exact per-map forcing normalization/validation and dynamic checks remain fresh. Structural/forcing/native resident proofs are joined only where their original validation occurred. Native resident proof comes from independently validated exact resident revision and pointer/allocation identity, not a V8 endorsement. Successor residents and restart/external/durable/untrusted boundaries require full validation. No same-value foreign input is automatically the same authority.

## Errors and immutable resources

Before creating replay capability select complete evaluation for non-Stage3, non-component, inadmissible/multicoordinate or conservatively unknown dependency cases. A topology mismatch found before replay capability can select complete evaluation with identical full operands. In contrast, stale/foreign base, wrong probe binding or double consumption directly returns `ConstitutiveDomain("covered_component_dependency_replay_integrity")`. Once replay starts, every ordinary constituent error returns immediately under its existing type and position; never retry complete evaluation to repair it. All failures preserve rollback and publish nothing.

The C-020 matrix is substantive. Current leaf is fallible and crossable: retain genuine successful bases and canonical signed probes that trigger the same first source-real errors in complete/replay, including both leaf positions. Maximum leaf is fallible but no crossing is established; supply per-guard implication proofs and authentic boundary-success evidence, and add paired error evidence if a real counterexample exists. Prepare/first wet/first finalize/second wet/second liquid, longwave, hydraulic/root, route-match and lower ground are fallible: historical noncrossability claims need exact guard-specific proof from admitted operands, plus authentic boundary successes, not merely a once-successful base or synthetic flags. Arithmetic assembly/reduction/residual/normalization/result nodes classified infallible need exact field parity, not invented errors. Keep pre-admission and private-capability integrity tests distinct from physical crossing. Exercise integrity through authentic constructors/API misuse, not mutation of private fields. A failed broad search is not a noncrossability theorem.

The full solver error order remains serialization, identity, topology/owner, nonfinite, unsupported domains, constitutive evaluation, water-request authorization, pivot, backtrack, iteration limit, accepted step/residual, component closure, control-volume closure, cross-owner closure. Preserve exact errors and accumulated validated diagnostics. Competing-poison tests must prove the first real error, not just any error.

Potential and fixed-final evaluations use the same immutable pre-ingress beginning water and exact accepted resource/cap authorization. The potential demand D, available A, and final F transaction joins do not license replacement resource objects or repeated withdrawal. Current rain can affect canopy routing but cannot retroactively finance the same solve's ground-water demand. Fixed-final full-supply seeding copies only accepted potential coordinates, reevaluates the COMPLETE fixed-cap system from the immutable beginning, and permits iteration-zero acceptance only when all normal residual/step rules pass. It cannot copy potential fluxes/candidates/receipts. Universal P-001..004 remain required: provide complete unit/lineage state and components; independently validate and atomically publish; expose branch/input chronology without silent defaults; reject censored schema-v8 terminal liquid/energy/time absent the separately reviewed atomic recipient cutover. Signed vapor carries its exact mass/enthalpy identity, and independent energy/water/equal-opposite closure cannot be replaced by a producer residual.

## Required evidence and experiments

For an implementation review I would require exact current source/build/toolchain/fixture and all validated input identities; authentic base/probe/caps/branch/graph records; complete node inputs/results/errors and order; independent direct-graph and stencil enumeration; full solve/boundary/owner/result bytes; source-real first-error and rollback evidence; and real-runner output operands. None of those live implementation/test inputs was supplied or read in this exercise, so no implementation-conformance or performance verdict is available.

C-020 requires independent exact direct-node/edge/hash oracles at N=1,S=1 and real N=2,S=6, with every required edge deletion/change detected. Require node-level forced-complete comparison, complete residual/Jacobian/full-solve and accepted-boundary bitwise parity, reciprocal radiation, upper-wet/all-lower routing, both source-distinct wet/liquid calls and route-match, full caps and active-boundary cases, current/maximum leaf branches, genuine errors, stale/foreign/reuse and topology/input/branch poisons, exact rollback and no publication. Preserve C-018 leaf-reuse oracles, C-019 anchor/stencil/dependency oracles, and combined-workload solver/error order. Contract-derived focused vectors do not replace authentic release coverage or independent operand reconstruction.

For a fully centered interior sweep, logical probes=2*(10N+3+S), identity anchors=2*(1+S), component replay=8N, complete=12N+4. N=2,S=6 gives 58=14+16+28. Actual bound stencils contribute one inward probe, not two. Four hydraulic, two beta and shared-air columns retain full evaluation as specified. Count actual starts/completions/errors, not just eligibility. Reconcile Completed/Failed/real ShortCircuited lifecycles, and distinguish per-column RejectedBeforeProbe from sweep lifecycle. Stop records at the first actual error; unattempted probes are not calls. All logical bucket sums, potential/fixed-final sweeps, all maps/retries and reset/drop boundaries require source-authentic identities and independent reconciliation. Do not duplicate ordinals, use addresses/hashes as authentic event identities, drop failed records or copy producer labels into expected results.

Historical revision-31 retention and prospective EXP-R are different predicates. Historical retention required three same-binary CPU-0 release runs, authentic aggregate centered coverage, exact outputs/counts, median run <=4,803,570 us and potential solve <=253,431 us (each >=100,000 us improvement over 4,903,570/353,431), and every RSS <=65,536 KiB. Failure requires complete reversion, not partial retention. Historical source/outlet/storage amounts are 0.8488061229561478, 0.8471105124736579 and 0.0016956104824910018, clamp zero; 48/56/20/32/4 workload counts remain named historical evidence. A fixture-only centered sweep is not the authentic runner requirement, and old FAIL/HOLD is not repaired here.

`EXP-STAGE3-20260906-R` is separately authorized A versus R=A+one replay delta, independent of F=A+feed-forward. Never R on F or add their gains. Freeze new coherent source/input/toolchain/binary identities; historical identity is not admission. Only this prospective comparison replaces the centered existential and fixed historical timing/64 MiB predicates with independent stencil-aware real coverage and reviewed paired timing/memory. Require nonzero completed replay on the real workload, ordered independently enumerated actual stencils/classification, full start/error/drop reconciliation and forced-complete/science/output admission before timing. Do not manipulate state for a centered pattern or hardcode 54/14/16/24. No new equation, constitutive domain, derivative, numerical tolerance, stopping criterion, adaptive/event cadence, custody, restart or publication rule is authorized.

PC1 is a narrow prospective leaf-temperature eligibility qualification: affected sun/shade beta must be exactly one before capability creation. Recompute its current leaf at the signed probe temperature, then use the successful current result for maximum only via INV-163. Wet/stem cases require unchanged exact leaf operands including temperature, humidity, gas/biochemical state, conductance and beta; routing changes cannot be ignored. Other leaf-temperature cases select complete evaluation before replay. Exact-one/adjacent-beta vectors and an independent classifier plus dual review are required; a zero-hit search, including the retained 198 search observation, is not a proof.

SG1 narrowly qualifies the five recomputed liquid families: route.prepare, first route.wet, first route.finalize, second occ.wet and second occ.liquid, including lower descendants. A successful base plus a temperature bound does not establish the CLM liquid saturation denominator margin `pressure-0.378*e_sat`. Substitute the exact canonical signed-probe operands/branches and execute the SAME guard/body at the SAME original source position, retaining the same result or first error. This result-or-error proof substitutes only for the historical noncrossability proof of those recomputed nodes. Copied/skipped nodes still require exact unchanged operands, successful base and complete graph, including earlier guards; other fallibility requirements remain. Do not claim the earlier noncrossability assertion was proved, waive C-020 wholesale, fabricate a crossing or catch a new error with complete-evaluator fallback.

The frozen protocol specifies two fresh-process warmups then 12 balanced A/B pairs, with a single possible extension to 24 total when uncertainty changes the decision; median wall improvement >=5%, lower paired-bootstrap interval >0, median process-CPU regression <=2%, fixed seed 20260906 and 10,000 resamples. Builds and detailed oracle work are outside the complete real-runner interval; actual publication remains inside. Separate six-pair memory series, 100-ms status sampling and explicit pre-fixture/pre-run/end-run/post-validation/post-drop metrics distinguish endpoint VmRSS, sampled active maximum, lifetime HWM and mapping evidence from live heap bytes. Explicit drops, three processes with ten teardown repetitions, and conditional three-pair 10/19-OFE checks do not prove long-run stability. Preserve all valid slow/high-RSS samples and failures, actual CPU affinity/environment, binary identity, independent WAT5/HBP/storage/clamp reconstruction and raw output comparison. Report existing scaled budgets separately. F's single day-frame counter qualification is F-specific and gives R no general counter normalization license.

The authority-reproduction document requires full exact-base checkout plus arm runtime kit and canonical adjunct, not a narrow archive alone. Its current recipe separates A04 SG1, F05 common and R05 SG1 and distinguishes current labels from historical A cuts. This is input selection/reconstruction evidence, not an executed scientific workflow. The current experiment handoff says PAUSED BY OWNER REQUEST, not complete or qualified. Timing, memory, teardown and scale comparisons are NOT RUN; optimized full runs, broad Clippy and release golden retain FAIL. Recorded earlier focused admissions do not establish current exact-head complete correctness. This exercise neither resumes those experiments nor authorizes promotion or solver redesign.

## Reading decisions and limits

The entry and interface were bootstrap authority; dependency-replay was the selected task route. It explicitly expanded to all nonlinear-solve, surface-energy and qualification, map handoff/pending sections and complete water/vapor/error/invariant/obligation sections. The interface required common-details; nonlinear required solve-boundary, terminal-support and vegetation; surface-energy required soil-coupling and vegetation; represented-snow boundary/support required FULL SnowEnergy and CoupledTime. External single-file requirements were honored completely, including historical sections and later superseding amendments. Qualification required the four frozen prospective documents. I also read whole map-custody/water-vapor instead of only their required sections, and whole audit-details to resolve historical applicability/promotability. Those conservative reads are real cost, not eliminated by a later smaller selection.

No active snow-free litter implementation or phase closure is reviewed, so litter-phase is not triggered. No exact soil/surface-storage implementation, arithmetic reconstruction or restart-wire redesign is reviewed, so soil-custody/surface-custody are not triggered. This is not a V28–30 profile/enforcement-classification audit, so binding-index's corresponding profile tables are not required. Referenced legacy literature/code establishes provenance in the current contract; I did not independently audit that source. The old experiment kickoff's predecessor-evidence and test/source instructions govern executing that package, not this frozen reading exercise; the current handoff explicitly prohibits those materials, and no old execution instruction was treated as permission to read them. Missing actual run inputs remain missing.

All actual requested LSE bytes are charged below. The actual 229,664-byte LSE unique union is only a 13.29% reduction from the supplied 264,863-byte comparison number, so it does NOT itself demonstrate the package's >=20% actual-reading target. That comparison uses the task's stated baseline size, not a read of the forbidden monolith. A narrower routed section selection may differ; this report does not relabel the actual reads after the fact. Authority sufficiency and efficiency disposition are separate; there is no threshold-driven authority omission.

The read-only structural measurements below are not session token/cost/quota telemetry. Requested exposure includes text that a tool truncated and the subsequent recovery requests; it is not a claim that truncated bytes were delivered. Every known omitted source segment was recovered before this answer. Initial source requests before the local ledger helper are reconstructed from my own read commands, not another agent or predecessor report. Known automatically supplied instructions and task text exist in this session, but exact transport bytes/effective automatic context, delivered tokens, workflow-wide parent/child exposure and runtime memory are UNOBSERVED.


## Measured source inventory

Units are exact UTF-8 source bytes including line endings. All ranges are inclusive. Source SHA-256 is always the FULL source, even for a range. Working-tree identities were checked unchanged when producing this appendix. Requests before the temporary ledger helper are listed first; two `rg` source-line searches are grouped at the end rather than falsely assigned a wall-clock order.

| Class | Bootstrap unique | Bootstrap exposure | Expansion unique | Expansion exposure | Combined unique | Total requested exposure |
|---|---:|---:|---:|---:|---:|---:|
| all | 109401 | 109401 | 1148097 | 1627287 | 1200004 | 1736688 |
| governance_task | 49178 | 49178 | 41892 | 42317 | 81693 | 91495 |
| LSE | 60223 | 60223 | 217558 | 231353 | 229664 | 291576 |
| external_contract | 0 | 0 | 839166 | 1304136 | 839166 | 1304136 |
| external_frozen_evidence | 0 | 0 | 45937 | 45937 | 45937 | 45937 |
| measurement_tool | 0 | 0 | 3544 | 3544 | 3544 | 3544 |

Bootstrap/expansion unique columns overlap when a later request repeats bootstrap bytes; combined unique is their finite union, not their sum. Request exposure sums all requests including retries and source-line searches. The 32–64 KiB role bootstrap and preferably <=16 KiB common governance figures are targets, never admission limits. The substantial necessary expansion is the three full external contracts plus the connected replay/solver/physics/support authority. No source bytes are credited as eliminated merely because they were reached later.

| ID | Class | Source | Full bytes | Full lines | Full-source SHA-256 |
|---|---|---|---:|---:|---|
| S01 | governance_task | `AGENTS.md` | 9508 | 121 | `9bd7887bcae731debc64689a4774ac222d529bcd527ebea2293e3ffd600289f6` |
| S02 | governance_task | `docs/specifications/science-contracts/AGENTS.md` | 6532 | 84 | `bace4ab3e33119e0239f700f92c53707d884f297e92d2454a86c477758c930c7` |
| S03 | governance_task | `docs/work-packages/AGENTS.md` | 6642 | 105 | `29e2f48a0b69fd9f43eb8d3a6dc2b72d30792337f90a2141b89df105726282a5` |
| S04 | governance_task | `docs/work-packages/20260907-lse-context-adoption-correction-001/package.md` | 5304 | 75 | `0cb2505989026d5e911d696eb7eda8d96c5ebd5498da421e55a2f7682290929b` |
| S05 | governance_task | `docs/work-packages/20260907-lse-context-adoption-correction-001/artifacts/worker-handoff.md` | 1586 | 22 | `00f80e082f41b65a25b6864340cd89b6daffbf0d16ba54d6df5612e31d25d6fc` |
| S06 | governance_task | `docs/work-packages/role-review.md` | 1400 | 7 | `2193e9538ad22dad5678fc5bb59080f686a543f697fec4983208214a73cc5eb1` |
| S07 | governance_task | `docs/work-packages/science-obligations.md` | 6149 | 98 | `1a0377a3362a6c34d783796b6756854f8630e1f05e994dc3d963eaafd16c542e` |
| S08 | governance_task | `docs/standards/prompt-wording-guidance.md` | 12057 | 202 | `294385612b565fe8be3e9e530caa0792c901406ecf53abc9402649e5568ffe3f` |
| S09 | LSE | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md` | 4130 | 65 | `930c682e5c923083683e8a030457c9e5e90e6ca9cd2f70a425ee0aa79c326c72` |
| S10 | LSE | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/interface.md` | 7976 | 66 | `0335d3b03acab1bdd711b3be589b429dcd63b7023e516561e269700006fbb2ff` |
| S11 | LSE | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/dependency-replay.md` | 48117 | 534 | `64d228281992124ea9b038813fd01cd142603cc11d2939eb0797edc29af7ad83` |
| S12 | LSE | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/common-details.md` | 22401 | 298 | `20d4bfdd94cce60ed9ee13e1ded2c1d8d6c1f99cbf69b74483e10d4efd041e2a` |
| S13 | LSE | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/nonlinear-solve.md` | 18979 | 247 | `4869499290647ead40d332927b87e19b0e7f20e2ab11b09245831fdc12240f4d` |
| S14 | LSE | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/surface-energy.md` | 11425 | 205 | `7b77973b1d8be2a700b56063d097ac8f0ab8ae46b1ea265bc28c99f9ddca7495` |
| S15 | LSE | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/solve-boundary.md` | 8225 | 113 | `fe37a1a64327aa46ffb1afbc63e00aedab67c83083a3eb3ccbd2edbc17d3174e` |
| S16 | LSE | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/soil-coupling.md` | 13249 | 208 | `582c8591001247342a1a0fa919dcd6e3cd267f03ceec04101c87890a6b0ec094` |
| S17 | LSE | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/terminal-support.md` | 20940 | 245 | `9eb3f77d55de9ed404f89bff9c313194780dd816bb43fb087b7e4927ca0e0007` |
| S18 | LSE | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/qualification.md` | 19322 | 246 | `be7b5b41d11e39e5adb3fb7af18233d9e567938936345194c7b92eca76472ebe` |
| S19 | LSE | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/map-custody.md` | 19223 | 171 | `4778df5b572c4a09b3dfedc1f5bba4c84d376bcfcd0021ae7e9fbb6200158b8c` |
| S20 | LSE | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/water-vapor.md` | 15924 | 227 | `7d7e0816257f51d2878450c7cbf5cb5d69e9e5fb061c0a49f52bae5a0beef3ce` |
| S21 | external_contract | `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md` | 248829 | 3080 | `3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa` |
| S22 | external_contract | `docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md` | 92552 | 1108 | `1d10611b5143be201381f283ec5594d9f65c99f8252e944dfae57076a8a699f7` |
| S23 | external_contract | `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md` | 497785 | 3559 | `94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d` |
| S24 | external_frozen_evidence | `docs/work-packages/20260906-stage3-prospective-mechanism-experiments-001/artifacts/experiment-protocol.md` | 11541 | 180 | `40033ea2e536984ae5a7455e99509e7e760624b78cdb08ee347597fe2f5f115f` |
| S25 | external_frozen_evidence | `docs/work-packages/20260906-stage3-prospective-mechanism-experiments-001/artifacts/authority-input-reproduction.md` | 5128 | 84 | `1c813fd08e34763808d3ac8a033d18d8c5176857aff76378aa126762166c8883` |
| S26 | external_frozen_evidence | `docs/work-packages/20260906-stage3-prospective-mechanism-experiments-001/prompts/active/kickoff.md` | 25550 | 263 | `6f3f0852afbfae138c4e40c62e15bf727c569611d00dec08bc8338e05ebdd9b3` |
| S27 | external_frozen_evidence | `docs/work-packages/20260906-stage3-prospective-mechanism-experiments-001/artifacts/worker-handoff.md` | 3718 | 59 | `b3ca642a6b4fd94f6e1459f07058cd9fc18fab747368421fc0dc6e591d143183` |
| S28 | governance_task | `docs/standards/testing-and-gate-strategy.md` | 25142 | 494 | `56277d9848d80b0672ebeaaa8277ac2ad9888c90e54b4f31aa7c5b8ca9b04230` |
| S29 | governance_task | `docs/prompt_templates/assurance-findings-template.md` | 736 | 12 | `95373d3c7df1f0a7e3ad7640fa22c0e137095e6cf5c4d92e3803846eb3f6ee83` |
| S30 | measurement_tool | `tools/agents/context_report.py` | 3544 | 79 | `2aa217325da4b279926aa0780d1aa9427fb21d1e80a533314cbe277474d2bbde` |
| S31 | governance_task | `docs/specifications/correctness-authority-model.md` | 11159 | 221 | `64275bad708f353c2c3b66f5b1475c070254591e26278d0a4ec442cad6e6eb0f` |
| S32 | governance_task | `docs/standards/AGENTS.md` | 4052 | 58 | `b62a33d9a70a3d3f45e7eccdcf05ca3a194486fcdbfc7bd439371cf8f0ae3015` |
| S33 | governance_task | `docs/standards/numerical-solver-architecture.md` | 6742 | 152 | `46089ca55bb565b7dc67ea1b2075bbcc16db6902eeba1952572b072abadd0738` |
| S34 | LSE | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/audit-details.md` | 19753 | 155 | `e3311b94e58cc04dff37cedaa54a742d56b9c4fb9bf4d89c62a904b7d94e624f` |

## Every requested source range

The source ID resolves path and complete hash above. Repeated/overlapping rows are intentionally not deduplicated. A one-line search row includes that source line and its newline; tool-added pathname/line-number prefixes are not source bytes.

| Request | Source | Inclusive lines | Requested bytes | Phase |
|---:|---|---|---:|---|
| 1 | S01 | 1–121 | 9508 | bootstrap |
| 2 | S02 | 1–84 | 6532 | bootstrap |
| 3 | S03 | 1–105 | 6642 | bootstrap |
| 4 | S04 | 1–75 | 5304 | bootstrap |
| 5 | S05 | 1–22 | 1586 | bootstrap |
| 6 | S06 | 1–7 | 1400 | bootstrap |
| 7 | S07 | 1–98 | 6149 | bootstrap |
| 8 | S08 | 1–202 | 12057 | bootstrap |
| 9 | S09 | 1–65 | 4130 | bootstrap |
| 10 | S10 | 1–66 | 7976 | bootstrap |
| 11 | S11 | 1–534 | 48117 | bootstrap |
| 12 | S11 | 1–270 | 24575 | expansion |
| 13 | S11 | 271–534 | 23542 | expansion |
| 14 | S12 | 1–160 | 12604 | expansion |
| 15 | S12 | 161–298 | 9797 | expansion |
| 16 | S13 | 1–247 | 18979 | expansion |
| 17 | S14 | 1–205 | 11425 | expansion |
| 18 | S15 | 1–113 | 8225 | expansion |
| 19 | S16 | 1–208 | 13249 | expansion |
| 20 | S17 | 1–245 | 20940 | expansion |
| 21 | S18 | 1–246 | 19322 | expansion |
| 22 | S19 | 1–171 | 19223 | expansion |
| 23 | S20 | 1–227 | 15924 | expansion |
| 24 | S21 | 1–750 | 56713 | expansion |
| 25 | S21 | 1–375 | 31962 | expansion |
| 26 | S21 | 376–750 | 24751 | expansion |
| 27 | S21 | 751–1150 | 45678 | expansion |
| 28 | S21 | 751–1150 | 45678 | expansion |
| 29 | S21 | 1151–1650 | 51438 | expansion |
| 30 | S21 | 1350–1470 | 12394 | expansion |
| 31 | S21 | 1651–2200 | 31876 | expansion |
| 32 | S21 | 2201–2700 | 35049 | expansion |
| 33 | S21 | 2701–3080 | 28075 | expansion |
| 34 | S22 | 1–400 | 23523 | expansion |
| 35 | S21 | 3040–3080 | 5444 | expansion |
| 36 | S22 | 401–750 | 39325 | expansion |
| 37 | S22 | 751–1108 | 29704 | expansion |
| 38 | S23 | 1–400 | 26062 | expansion |
| 39 | S23 | 401–750 | 63231 | expansion |
| 40 | S23 | 401–600 | 25654 | expansion |
| 41 | S23 | 601–750 | 37577 | expansion |
| 42 | S23 | 751–1150 | 18136 | expansion |
| 43 | S23 | 1151–1450 | 138123 | expansion |
| 44 | S23 | 1151–1300 | 21418 | expansion |
| 45 | S23 | 1301–1400 | 40212 | expansion |
| 46 | S23 | 1335–1340 | 3997 | expansion |
| 47 | S23 | 1401–1550 | 92072 | expansion |
| 48 | S23 | 1425–1440 | 35341 | expansion |
| 49 | S23 | 1423–1424 | 6153 | expansion |
| 50 | S23 | 1441–1455 | 21660 | expansion |
| 51 | S23 | 1551–1600 | 20490 | expansion |
| 52 | S23 | 1601–1850 | 22222 | expansion |
| 53 | S23 | 1851–2300 | 70912 | expansion |
| 54 | S23 | 2020–2100 | 25346 | expansion |
| 55 | S23 | 2101–2140 | 17940 | expansion |
| 56 | S23 | 2301–2650 | 25851 | expansion |
| 57 | S23 | 2651–3000 | 56173 | expansion |
| 58 | S23 | 2900–2940 | 26742 | expansion |
| 59 | S23 | 2893–2899 | 6208 | expansion |
| 60 | S23 | 3001–3300 | 20124 | expansion |
| 61 | S23 | 3301–3559 | 20882 | expansion |
| 62 | S24 | 1–180 | 11541 | expansion |
| 63 | S25 | 1–84 | 5128 | expansion |
| 64 | S26 | 1–263 | 25550 | expansion |
| 65 | S27 | 1–59 | 3718 | expansion |
| 66 | S28 | 193–345 | 8067 | expansion |
| 67 | S28 | 439–473 | 1262 | expansion |
| 68 | S29 | 1–12 | 736 | expansion |
| 69 | S30 | 1–79 | 3544 | expansion |
| 70 | S31 | 1–221 | 11159 | expansion |
| 71 | S32 | 1–58 | 4052 | expansion |
| 72 | S33 | 1–152 | 6742 | expansion |
| 73 | S08 | 174–202 | 1896 | expansion |
| 74 | S34 | 1–155 | 19753 | expansion |
| 75 | S04 | 1–75 | 5304 | expansion |
| 76 | S05 | 1–22 | 1586 | expansion |
| 77 | S11 | 1–140 | 11710 | expansion |
| 78 | S28 | 8–8 | 14 | expansion |
| 79 | S28 | 17–17 | 17 | expansion |
| 80 | S28 | 33–33 | 35 | expansion |
| 81 | S28 | 52–52 | 17 | expansion |
| 82 | S28 | 77–77 | 30 | expansion |
| 83 | S28 | 129–129 | 25 | expansion |
| 84 | S28 | 131–131 | 18 | expansion |
| 85 | S28 | 137–137 | 26 | expansion |
| 86 | S28 | 164–164 | 28 | expansion |
| 87 | S28 | 172–172 | 25 | expansion |
| 88 | S28 | 182–182 | 30 | expansion |
| 89 | S28 | 193–193 | 28 | expansion |
| 90 | S28 | 219–219 | 26 | expansion |
| 91 | S28 | 224–224 | 14 | expansion |
| 92 | S28 | 230–230 | 22 | expansion |
| 93 | S28 | 235–235 | 22 | expansion |
| 94 | S28 | 240–240 | 13 | expansion |
| 95 | S28 | 274–274 | 44 | expansion |
| 96 | S28 | 292–292 | 26 | expansion |
| 97 | S28 | 314–314 | 42 | expansion |
| 98 | S28 | 346–346 | 25 | expansion |
| 99 | S28 | 368–368 | 39 | expansion |
| 100 | S28 | 386–386 | 17 | expansion |
| 101 | S28 | 403–403 | 34 | expansion |
| 102 | S28 | 417–417 | 35 | expansion |
| 103 | S28 | 427–427 | 46 | expansion |
| 104 | S28 | 439–439 | 33 | expansion |
| 105 | S28 | 457–457 | 26 | expansion |
| 106 | S28 | 474–474 | 18 | expansion |
| 107 | S28 | 487–487 | 18 | expansion |
| 108 | S06 | 5–5 | 579 | expansion |
| 109 | S08 | 7–7 | 12 | expansion |
| 110 | S08 | 174–174 | 24 | expansion |
| 111 | S08 | 187–187 | 83 | expansion |
| 112 | S08 | 195–195 | 22 | expansion |
| 113 | S19 | 3–3 | 16 | expansion |
| 114 | S19 | 14–14 | 25 | expansion |
| 115 | S19 | 15–15 | 14 | expansion |
| 116 | S19 | 18–18 | 26 | expansion |
| 117 | S19 | 19–19 | 63 | expansion |
| 118 | S19 | 20–20 | 53 | expansion |
| 119 | S19 | 24–24 | 33 | expansion |
| 120 | S19 | 25–25 | 24 | expansion |
| 121 | S19 | 36–36 | 21 | expansion |
| 122 | S19 | 37–37 | 63 | expansion |
| 123 | S19 | 38–38 | 53 | expansion |
| 124 | S19 | 88–88 | 21 | expansion |
| 125 | S19 | 89–89 | 58 | expansion |
| 126 | S19 | 90–90 | 48 | expansion |
| 127 | S19 | 157–157 | 34 | expansion |
| 128 | S19 | 158–158 | 24 | expansion |
| 129 | S19 | 165–165 | 35 | expansion |
| 130 | S19 | 166–166 | 25 | expansion |
| 131 | S20 | 3–3 | 16 | expansion |
| 132 | S20 | 16–16 | 25 | expansion |
| 133 | S20 | 17–17 | 14 | expansion |
| 134 | S20 | 20–20 | 19 | expansion |
| 135 | S20 | 21–21 | 46 | expansion |
| 136 | S20 | 22–22 | 37 | expansion |
| 137 | S20 | 65–65 | 19 | expansion |
| 138 | S20 | 66–66 | 71 | expansion |
| 139 | S20 | 67–67 | 62 | expansion |
| 140 | S20 | 118–118 | 20 | expansion |
| 141 | S20 | 119–119 | 44 | expansion |
| 142 | S20 | 120–120 | 35 | expansion |
| 143 | S20 | 146–146 | 52 | expansion |
| 144 | S20 | 147–147 | 43 | expansion |
| 145 | S20 | 171–171 | 77 | expansion |
| 146 | S20 | 172–172 | 67 | expansion |
| 147 | S20 | 209–209 | 34 | expansion |
| 148 | S20 | 210–210 | 24 | expansion |
| 149 | S20 | 218–218 | 35 | expansion |
| 150 | S20 | 219–219 | 25 | expansion |
| 151 | S34 | 3–3 | 16 | expansion |
| 152 | S34 | 8–8 | 27 | expansion |
| 153 | S34 | 9–9 | 42 | expansion |
| 154 | S34 | 13–13 | 21 | expansion |
| 155 | S34 | 14–14 | 11 | expansion |
| 156 | S34 | 30–30 | 67 | expansion |
| 157 | S34 | 31–31 | 57 | expansion |
| 158 | S34 | 50–50 | 55 | expansion |
| 159 | S34 | 51–51 | 45 | expansion |
| 160 | S34 | 72–72 | 48 | expansion |
| 161 | S34 | 73–73 | 38 | expansion |
| 162 | S34 | 83–83 | 33 | expansion |
| 163 | S34 | 84–84 | 24 | expansion |
| 164 | S34 | 125–125 | 59 | expansion |
| 165 | S34 | 126–126 | 49 | expansion |
| 166 | S34 | 129–129 | 51 | expansion |
| 167 | S34 | 130–130 | 41 | expansion |

## Truncation, repeated exposure and tooling record

Known truncations and their recoveries (all source-range requests are already charged above):

- Initial combined interface/full dependency-replay display truncated replay; recovered replay 1–270 and 271–534. Interface had been visible completely.
- Vegetation 1–750 recovered 1–375/376–750; 751–1150 repeated with larger output allowance; 1151–1650 recovered the omitted test-table area with 1350–1470; 2701–3080 recovered the ending with 3040–3080.
- SnowEnergy 401–750 recovered 401–600/601–750. The 1151–1450 request exceeded available model context and was not treated as delivered: recovered 1151–1300, 1301–1400 and 1401–1550, with further recovery below.
- SnowEnergy 1301–1400 had a short INV-086 omission, recovered 1335–1340. The very long 1401–1550 table omitted its central rows; recovered 1423–1424, 1425–1440, 1441–1455.
- SnowEnergy 1851–2300 recovered the omitted 2020–2100 and 2101–2140 table area. SnowEnergy 2651–3000 recovered the omitted change-log area with 2893–2899 and 2900–2940.
- Other repetitions were deliberate final anchor/accounting checks: dependency-replay 1–140, wording guidance 174–202, current package/handoff, and the listed source-line searches. Temporary ledger reprinting is derivative measurement metadata, not another canonical source request.

Ran: `tools/agents/find-agents --for <assigned report>` resolved root and work-package guidance; `cat` initial bootstrap reads; `.venv/bin/python /tmp/lse_r3_read.py <path> [first last]` measured and printed later selections; the two recorded `rg -n` source searches; `.venv/bin/python tools/agents/context_report.py --help`; and `.venv/bin/python tools/agents/context_report.py /tmp/lse_r3_selection.json --root /workdir/openWEPP`. Source read/measurement commands completed; individual display truncations above are not full-delivery evidence. The final measurement script verified all recorded full-source hashes and generated this appendix. No validator, build, kernel test, numerical run, timing or memory workflow was performed.

Temporary measurement selection and computed JSON are `/tmp/lse_r3_selection.json` and `/tmp/lse_r3_context.json`; the durable report contains their entire source identity/range content and resulting counts. Their temporary location is not claimed as a scientific reproducibility kit. Effective automatic context, delivered tokens and whole-workflow telemetry remain UNOBSERVED.

Verdict: scientific rules and required source were read and the question answered within the stated boundary. Implementation correctness and experimental outcomes are NOT ASSESSED. Actual-reading efficiency does not establish the >=20% target. Independent parent assessment remains required.
