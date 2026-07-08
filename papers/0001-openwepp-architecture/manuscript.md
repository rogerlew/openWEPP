<!--
WORKING DRAFT — openWEPP architecture paper (paper 0001).
Format: Markdown first; convert to Elsevier els-cas LaTeX when stable (see latex-instructions.md).
Target venue: Environmental Modelling & Software (EMS), research article.
Framing (agreed): physics-primary; AI-native development secondary as the methodological enabler;
audience assumed AI-skeptical; WEPP treated as respected foundation, NOT as "defect-laden";
sell thoroughness + auditability, never speed.
Tone rules (2026-07-07 rewrite): no superlatives; claims scoped to numerical/engineering
fidelity — predictive-skill validation is explicitly deferred and stated up front; "guarantee"
only for properties the type system or a fail-closed check actually enforces; internal
codenames defined at first use or replaced with standard vocabulary; negative results
(abandoned hybrid stepper, mesh-coarsening inversion) are reported as first-class outcomes,
not omitted.
Post-2026-07-07 engine state this draft reflects: hybrid IMEX stepper ABANDONED (ADR-0037);
Tier-2 mesh coarsening DEAD as a speedup — the convergence ladder ratified FINER meshes
(fixed10 fails WA annual sediment; dx5 is the clean candidate at ~4.85× cost); erosion is
hydrograph-resolved per ADR-0036 (legacy is steady-at-peak).
[TODO] markers = author must supply/verify before submission. [CITE] = reference needed.
Nothing empirical (numbers, datasets, defect specifics) is asserted here without a verified
source or a [VERIFY] flag — do not let placeholders reach submission. This includes Sections
3.4, 3.5, and 4.
-->

# openWEPP: A Hydrograph-Resolved, Convergence-Verified Reconstruction of the WEPP Erosion Model, Built Under Machine-Checkable Science Contracts

<!-- [TODO: title decision — alternative with methodology de-emphasized:
"Hydrograph-resolved erosion modeling with convergence-adjudicated numerics:
the openWEPP reconstruction of WEPP". Pick one before LaTeX conversion.] -->

**Roger Lew**^a^ [TODO: co-authors?]

^a^ University of Idaho, Moscow, ID, USA. [TODO: full postal address; corresponding-author email]

*Corresponding author:* Roger Lew ([TODO: email])

---

## Abstract

*(≤150 words, factual, stand-alone.)*

Process-based environmental models are long-lived scientific infrastructure whose governing equations often outlast the numerical methods used to solve them. We present openWEPP, a ground-up Rust reimplementation of the Water Erosion Prediction Project (WEPP) hillslope and watershed engine that preserves WEPP's process science while replacing its numerical substrate. Where WEPP-lineage codes collapse each event to a steady peak discharge, openWEPP routes transient hydrographs with a shock-capturing scheme, hands them continuously across element boundaries, and resolves erosion against the modeled flow — under fail-closed mass accounting and a convergence ladder that adjudicates mesh resolution against fidelity contracts. That machinery audits the engine itself: it exposed a latent router defect and a mesh-induced sediment error in openWEPP's own production configuration, an error class legacy codes cannot measure internally. The engine was built by human-supervised AI agents under machine-checkable science contracts, leaving a complete, re-runnable evidence record.

**Keywords:** erosion; hydrology; kinematic wave; convergence verification; scientific software; reproducibility; software verification

---

## Highlights

*(3–5 bullets, ≤85 characters each; submitted as a separate file.)*

- Transient hydrograph routing replaces legacy steady-at-peak erosion collapse
- Continuous kinematic-wave handoffs replace equivalent-plane runon approximations
- Fail-closed mass accounting halts the run on any water-balance closure failure
- A convergence ladder exposed a sediment error legacy codes cannot measure
- Human-supervised AI agents built the engine under machine-checkable contracts

---

## 1. Introduction

Process-based environmental simulation models are long-lived scientific infrastructure. Codes for erosion, hydrology, water quality, and land–atmosphere exchange are developed over decades, outlive their original authors, and are relied upon for research and management decisions. The scientific formulations — the governing equations — of these models are, in general, sound and well documented. The numerical schemes and software substrates used to solve those equations, however, reflect the computational constraints of the era in which each model was engineered, and those constraints can limit the physical fidelity achievable today.

The Water Erosion Prediction Project (WEPP), developed by the USDA Agricultural Research Service [CITE: Flanagan and Nearing, 1995], is a case in point and the respected foundation on which this work builds. WEPP's process science — infiltration, water balance, plant growth, hillslope hydraulics, and detachment/transport mechanics — remains the standard for physically based erosion prediction three decades after its release. Like other models of its generation, however, WEPP never solves the transient routing problem. Each runoff event is reduced by a semi-analytic kinematic approximation [CITE: Stone et al., peak runoff / irs.for lineage; VERIFY] to three scalars — runoff volume, peak rate, and effective duration — and the erosion solver integrates steady-state sediment continuity at that single peak discharge over a normalized profile. This was a sound engineering trade for the hardware of the early 1990s — the justification was computational, and at the time it was compelling. Three decades of hardware later, the justification has expired while the simplification remains. It is important to state what the simplification implies precisely: there is no transient wave, no falling limb, and therefore no representation of recession-limb deposition — that physics is structurally absent rather than under-resolved. Equally important, the resulting error cannot be diagnosed from within the model, because a scalar collapse has no refinement dial: there is no finer solution to converge toward. Flow across discrete land elements is likewise handled by a lumped algebraic approximation (the "equivalent plane") rather than by routing, which synchronizes flow delivery across element boundaries and — through the power-law dependence of detachment and transport on flow rate and shear stress — inflates simulated soil loss on multi-element hillslopes [VERIFY: documented instances; show in Section 4.1].

Replacing this substrate with transient conservation-law numerics — limiter-stabilized shock-capturing schemes, continuous boundary handoffs, mass accounting that stays closed across element and solver seams — is demanding for any engineering team, because the failure modes are silent: a small conservation leak, an unflagged clamp, or an under-resolved mesh degrades results without crashing anything.

This paper presents openWEPP, a ground-up reconstruction of the WEPP hillslope and watershed engine that addresses these numerical limitations while preserving WEPP's governing science. The contribution is threefold. First, we detail the numerical architecture (Section 2): hydrograph-resolved routing and erosion in place of the steady-at-peak collapse, continuous kinematic-wave boundary handoffs in place of the equivalent plane, fail-closed mass accounting, and a convergence ladder that adjudicates mesh resolution against explicit fidelity contracts. Second, we describe the development methodology that produced it (Section 3): a human-supervised multi-agent workflow governed by machine-checkable science contracts, which leaves a complete and re-runnable evidence trail — and which adjudicates its own proposals, including rejecting them: we report an entire solver subsystem built to completion and then abandoned under that governance, and a mesh-coarsening optimization inverted by its own verification ladder. Third, we evaluate the result (Section 4) on *numerical and engineering fidelity*: removal of the boundary-handoff peak artifact, water-balance closure at stated tolerances, convergence-adjudicated mesh resolution, and end-to-end computational cost, reported plainly. We state the scope at the outset: this paper's fidelity claims concern conservation, convergence, and artifact removal. Validation of predictive skill against observed runoff and erosion data is ongoing and is deferred to future work (Section 5.3).

## 2. Numerical Architecture

openWEPP preserves the governing science of WEPP and overhauls the numerical techniques used to solve it. Section 2.1 describes the numerics required for physical fidelity. Section 2.2 describes how computational cost is governed — including two optimizations that the project's own verification machinery adjudicated *against*, which we report as results rather than omissions.

### 2.1 Architecture for Numerical Fidelity

#### 2.1.1 Hydrograph-Resolved Routing and Erosion

Legacy WEPP's steady-at-peak formulation discards the hydrograph three times: the erosion solve consumes only peak and duration; the hillslope-to-watershed interchange serializes only scalar summaries; and channel routing then re-synthesizes triangular hydrographs from those scalars [VERIFY: align with ADR-0036 record]. openWEPP instead carries the modeled transient flow through the stack as a first-class surface. Overland flow is routed as a kinematic wave with full time resolution; the erosion solver consumes the resolved flow (per hydraulically-active interval) rather than a single representative discharge; and the hillslope-to-watershed interchange serializes the shape, not a scalar summary [VERIFY: current production form — per-hour quasi-steady sediment continuity per ADR-0036 D1; state precisely what is resolved]. The consequence with the most physical significance is on the falling limb: because the transient is represented, recession-limb deposition is representable — a surface that the steady-at-peak formulation cannot express at all.

#### 2.1.2 Continuous Wave Boundary Handoffs (the Runon Problem)

In legacy WEPP, surface runoff transitioning from one Overland Flow Element (OFE) to another is approximated by the equivalent-plane method or by lumped daily mass transfers. When a fast wave from a steep upper element reaches a flatter lower element, the lumped approximation delivers the volume as a synchronized pulse rather than a routed wave; through the power-law dependence of detachment and transport on flow rate, the synchronized pulse inflates simulated soil loss, and the effect compounds with element count [VERIFY: characterize scaling with OFE count from the Section 4.1 evaluation set]. openWEPP replaces the equivalent plane with continuous boundary handoffs: the routing subsystem owns the surface-water trajectory end to end, and the full time-resolved hydrograph leaving the upstream OFE becomes the dynamic upper boundary condition of the downstream OFE's solver. When a steep-slope wave enters a flat element, the solver attenuates and stretches it as the kinematic-wave equations dictate. The peak artifact is removed by routing the wave, not by post-hoc correction. This design extends the flow-resistance and routing framework of Papanicolaou et al. (2018) [VERIFY: confirm the precise relationship — what is adopted from that framework versus new here]. A practical consequence: legacy WEPP is limited to roughly ten OFEs per hillslope [VERIFY: confirm the ceiling and its origin], whereas openWEPP's routed handoffs carry no such structural limit.

#### 2.1.3 Shock Capturing and the Convergence Ladder

openWEPP routes overland flow with an explicit MacCormack predictor–corrector scheme stabilized by total variation diminishing (TVD) flux limiting [CITE: MacCormack scheme; CITE: Harten 1983 / TVD theory; CITE: TVD-MacCormack in shallow-water/overland-flow applications], following the application of shock-capturing schemes to hillslope runoff by Papanicolaou et al. (2018) [VERIFY: confirm attribution — introduced there, or applied from an existing line of work?]. TVD limiting suppresses spurious oscillation while preserving front sharpness.

The scheme's accuracy at any given mesh resolution is treated as an empirical question with a standing verification instrument: a *convergence ladder* — a fixed suite of runs at successively refined meshes, anchored at the bottom by analytical kinematic-wave solutions [CITE: Iwagaki 1955; VERIFY: exact solutions used] and extended upward by fine-mesh self-convergence on realistic fixtures. A mesh resolution is admissible for production only if the ladder shows it meeting the named error tolerances of the governing science contract. Section 2.2.2 reports what the ladder actually found — which was not what the project expected — and Section 4.3 presents the data. We emphasize the structural point here: the ladder is a refinement dial the legacy formulation does not possess. A steady-at-peak scalar collapse has no finer solution to converge toward, so its discretization error is silent by construction; a transient solver's error is measurable, and openWEPP measures it as a matter of contract.

#### 2.1.4 Mass Ledgers and Cross-Span Deficit Carry

Coupling numerical solvers across element boundaries risks conservation leaks at the seams. openWEPP addresses this by decoupling *attribution* (the hydrograph shape produced by the solver) from the *mass ledger* (the accounting of physical boundary fluxes). A mechanism we call the cross-span deficit carry — a transactional buffer at each element boundary — absorbs the oscillatory flux errors inherent to explicit schemes and repays them within the span, so accounting closure is maintained even when instantaneous fluxes oscillate. The system is fail-closed: water-balance closure is checked at a named tolerance every step, and a violation halts the simulation with a typed error rather than being silently absorbed. We do not claim exact arithmetic — floating-point computation is not exact — but closure at the stated tolerance is enforced, not merely monitored, and Section 4.2 reports the achieved residuals.

### 2.2 Computational Cost: Adjudicated, Not Assumed

openWEPP's numerics cost more than legacy WEPP's scalar approximations, and this paper does not claim otherwise. We regard the measured costs below as deferred, not incurred: they are the price of physics that the approximations of the early 1990s were adopted to avoid, payable now because the computational constraint that justified the avoidance no longer holds. The project's posture is that cost decisions must be adjudicated by the same contract machinery as physics decisions — with the consequence that two optimizations the project pursued were decided *against* by their own evidence. We report all three outcomes.

#### 2.2.1 Array-Native Runtime

The initial openWEPP runtime prioritized auditability: a dynamic, symbol-keyed state map preserved one-to-one semantic correspondence with legacy WEPP variables, at the cost of dictionary lookups dominating the hot loop. That build ran approximately 73× slower than the legacy Fortran reference on the benchmark fixture [VERIFY: fixture, hardware, measurement provenance], with profiling attributing the gap to lookup overhead rather than arithmetic. The engine was restructured to an array-native runtime: the hot loop executes over contiguous arrays of static structs, while symbolic and diagnostic structures moved to the system's edges — input parsing, shadow validation against legacy records, and output projection retain the full symbolic mapping. On the same fixture, the restructuring reduced execution time by roughly a factor of 20 (≈670 s to ≈32.7 s) and memory by roughly 66% (≈229 MB to ≈77 MB) [VERIFY: current numbers, fixture, hardware]. The engine remains slower than the legacy reference — a transient solve costs more than a scalar approximation — and Section 4.4 reports the current end-to-end standing on stated hardware [TODO: current net vs-legacy figure].

#### 2.2.2 Mesh Resolution: an Optimization Inverted by Its Own Ladder

The project initially expected the TVD scheme's resistance to numerical diffusion to permit substantial mesh *coarsening* at contract accuracy, with order-of-magnitude cost savings. The convergence ladder adjudicated against this — in the opposite direction. On the real evaluation cohort, the working production resolution (10 cells per OFE) and two coarser candidates all *failed* the annual-sediment fidelity tolerance against the fine-mesh reference (the production configuration missing by 5.8% on the annual deposition surface of a forested watershed-class member [VERIFY: fixture IDs, tolerance, contract revision]); the only candidate meeting tolerance was a *finer* mesh (Δx = 5 m class), at approximately 4.85× the routing cost, concentrated on exactly the long-hillslope members where resolution matters [VERIFY: per-member timings]. The provisional reading is that the production configuration had been carrying a mesh-induced annual-sediment error of roughly 6% on watershed-class members — an error discovered *by the engine's own verification instrument*, and recorded in the governing contract by holding the production resolution as "operational but not fidelity-ratified" until adjudication completes [VERIFY: contract revision recording this posture; the finer-reference adjudication was in progress at the time of writing].

We regard this inversion as a result, not a setback, for three reasons. First, it is precisely the behavior a verification-first architecture is for: the expected optimization was tested against its contract and lost. Second, the ladder's first execution also exposed a latent defect — a positivity-preservation flaw in the watershed router whose amplification was masked at the production resolution and visible only at fine resolution — which was then fixed on the production path for every resolution [VERIFY: commit/record reference]. Third, the finding reframes the cost of fidelity: the finer mesh's ~5× price buys a convergence-backed sediment answer on the member class where the steady-at-peak formulation is structurally weakest and cannot measure its own error (Section 5.1). Local-numerics optimizations (analytic celerity evaluation, transcendental-call reduction) remain available to discount the fine-mesh cost and are future work [TODO: status at submission].

#### 2.2.3 A Negative Result: the Abandoned Hybrid Implicit–Explicit Stepper

The project designed, built, and fully verified a hybrid implicit–explicit (IMEX) stepper: backward-Euler implicit bins for source-quiet recession spans, coupled to the explicit TVD scheme for storm dynamics, with deterministic switching and cross-span deficit carry across the solver handoff. The engineering succeeded — the subsystem reached its full test surface with all workspace gates green [VERIFY: final state per the abandonment record]. It was then abandoned, because its *evidence base* failed: the only demonstrated performance win was on a synthetic stress topology, while on every real cohort member the stepper either fell back to the plain path (hash-identical outputs, zero value) or regressed when forced [VERIFY: cohort results per the abandonment ADR]. Under the project's governance the option's expected value was adjudicated as approximately zero against a permanent carrying cost — a second stepper path, its contract, its test surface, and invariant-preservation obligations on every future routing change — and the subsystem was archived and stripped from the production path, with byte-identity of plain-path outputs as the removal acceptance gate [VERIFY: ADR reference]. Section 3.5 discusses why we publish this outcome rather than omitting it.

## 3. The Verification Substrate: How the Architecture Was Built

The architecture of Section 2 — fail-closed accounting, contract-bounded numerics, typed boundaries — demands a level of implementation discipline that is difficult for any team to sustain across years of maintenance, because its failure modes are silent. openWEPP's approach was to make the discipline structural: the reconstruction was carried out by a human-supervised multi-agent AI workflow in which every mechanical rule is enforced by machine-checkable gates rather than by convention. We describe the substrate here because it is inseparable from the result: the architecture is the kind of software this methodology makes tractable, and — as Sections 2.2.2, 2.2.3, and 3.4 show — the same machinery adjudicates the project's own proposals, in both directions.

[TODO: this section currently under-specifies the workflow for a methodology-interested reviewer. Add: (a) which agent systems/models were used and in what roles (implementation, review, adjudication), with versions; (b) the human supervision protocol — what humans authored (contracts? adjudications?), what they reviewed, at what cadence, and what they never delegated; (c) gate statistics if recoverable — how often gates rejected proposed changes; (d) approximate cost (compute/tokens/wall-clock) of the reconstruction; (e) known failure modes of the workflow and how they were caught.]

### 3.1 Typed Boundaries and Deterministic Orchestration

The orchestrator constructs execution order dynamically from declared data dependencies, so the continuous boundary handoffs of Section 2.1.2 are always fed by completed upstream state. The architecture forbids shared global memory; processes communicate through typed boundaries, and ownership of each trajectory (producer versus consumer) is enforced by the Rust borrow checker. Quantities crossing boundaries carry physical units in their types, making dimensional consistency a compile-time property rather than a review-time check. Determinism is a stated policy, not an aspiration: runtime policy selectors must be pure functions of run inputs — wall-clock time, host load, and measured runtime counters are prohibited inputs [VERIFY: determinism policy reference].

### 3.2 Top-Down Science Contracts, Fixtures, and weppcloud

Above the code sits a layer of machine-checkable science contracts: named specifications with explicit invariants and tolerance bounds that define correctness for each process domain. Contracts were formulated from primary literature and conservation laws *before* implementation, and correctness is defined against the contract — not against the output of the legacy binary, which would merely reproduce the legacy numerics' limitations. Legacy comparison is retained as a flagging mechanism for investigation, never as an acceptance oracle; Section 5.1 discusses a case where the two would actively disagree and the contract must win.

Test fixtures derive from two sources. Analytical fixtures come from the primary literature — the convergence ladder of Section 2.1.3 is anchored on the Iwagaki analytical solutions for kinematic-wave runoff [CITE: Iwagaki 1955; VERIFY]. Real-world fixtures come from **weppcloud** [CITE: weppcloud], a platform that assembles high-resolution WEPP inputs (topography, soils, climate forcings) for arbitrary locations within minutes, providing realistic parameter combinations at scale. The engine additionally maintains strict legacy input-file compatibility, so the decades-old library of WEPP management and climate datasets remains natively usable.

### 3.3 Governed Autonomous Agents

Implementation against the contracts was executed by AI agents operating inside a governance envelope of mechanical gates: formatting, strict static analysis, conservation checks, and test suites, all of which must pass before a change can be proposed for human review. Two rules are central. First, numerical faults must surface as typed errors: agents are forbidden from substituting defaults, applying numerical clamps (e.g., `max(0, depth)`), or scaffolding surrogate physics to route around a failing check — the practices by which silent errors historically enter simulation codes. Second, all substantive work occurs inside authorized work-packages (Section 5.2) with explicit execution plans and closure conditions. The agents' role is to satisfy the bureaucracy of the contracts exhaustively; the humans' role is to author the contracts, adjudicate ambiguities, and review outcomes [VERIFY: confirm this division of labor is stated accurately; align with the TODO at the top of Section 3].

### 3.4 Case Study: The Snow and Frost Fidelity Campaign

To make the methodology concrete, we describe one sustained campaign: resolving long-standing gaps in WEPP's winter hydrology. [VERIFY: every quantity in this subsection must be sourced from campaign records before submission; none currently carry citations to run artifacts.]

The campaign began with data, not code. The team assembled observational baselines from SNOTEL, SCAN, and USFS canopy-stratified sites, explicitly distinguishing physical measurements (e.g., frost tubes) from proxy measurements (e.g., 0 °C soil isotherms). To isolate single-hillslope physics from watershed-scale confounding, weppcloud was used to scaffold fixtures — topography, soils, and high-resolution climate forcings (DAYMET, GRIDMET, PRISM) — for the instrumented sites.

Rather than tuning to single-residual targets, which invites compensating errors, the campaign fixed an evaluation rubric in advance. Signatures were classified as *forcing-robust* (e.g., melt-out timing, densification trajectories), which carry verdict weight, or *forcing-limited* (e.g., absolute SWE magnitudes, which are sensitive to gauge undercatch), which are reported but not used as acceptance thresholds. Anti-tuning guards forbade modifying shared radiation inputs to fix melt behavior, protecting the evapotranspiration balance from compensating adjustment.

Under this rubric, agents ran controlled experiments in an opt-in physics sandbox. Candidate modernizations — a revised shortwave energy balance, psychrometric rain/snow partitioning — were implemented, evaluated, and *rejected* when the rubric showed they worsened maritime-site residuals [VERIFY]; the rubric adjudicated, not the preference for newer physics.

Systematic testing then isolated a melt-realization gap: the model computed sufficient melt energy, but a legacy density-gate proxy prevented the mass loss from being realized, producing an aggregate depth-loss deficit of 24.1 m across the evaluation set and failing snow-control tolerances on 1,147 of 1,415 observed thaw-ablation windows [VERIFY: define "snow-control window" and the tolerance; cite campaign records]. Replacing the proxy with a physical retained-liquid-capacity formulation reduced the aggregate deficit to 15.5 m; paired with an updated bulk-density compaction model, window failures fell from 1,147 to 498 [VERIFY]. We note the honest remainder: roughly a third of windows still fail, and a substantial deficit remains [TODO: characterize where the remaining failures concentrate (site class, forcing regime) and what physics is implicated; without this context the residual is uninterpretable].

A parallel effort addressed frost depth. Early builds inherited a freeze-index proxy that estimated frost depth as a fraction of accumulated air temperature, capping depth at 0.20 m, and a ratchet mechanism over-persisted frozen-soil duration by a median of +258 days [VERIFY]. Governed work-packages replaced the proxy with a layered energy-balance heat-flow model: fine-layer discretization, in-hour resistance feedbacks, and surface-temperature synthesis incorporating radiation, wind, canopy, and residue conductance. The 0.20 m cap disappeared as a consequence of the physics rather than by adjustment; median depth correlation against reference physics improved from 0.13 to 0.763, and the frozen-duration over-persistence fell from +258 to +61 days [VERIFY: define "reference physics" and the correlation measure; +61 days is still large — state what drives it and whether it is under active work], while water-balance closure held at numerical noise (maximum absolute residual $5.1\times10^{-7}$ mm [VERIFY]).

### 3.5 Negative Results as Governed Outcomes

A verification methodology that only ever confirms its proposals is not exerting selection pressure. We therefore report, as first-class outcomes, the two cases where openWEPP's machinery adjudicated against the project's own engineering.

The hybrid IMEX stepper (Section 2.2.3) was carried through design, contract authorship, implementation, and a full verification surface — and was abandoned when its performance evidence failed to generalize beyond a synthetic topology. Two properties of that abandonment are worth stating for methodology-minded readers. First, the viability review had *predeclared* kill criteria before the evidence came in; the final record states plainly that the criteria as written did not technically fire, and that the actual ground was an evidence-base discount outside the predeclared list — the record documents the real reason rather than retrofitting a falsifier [VERIFY: abandonment ADR]. Second, the removal itself was gated: the stepper's contract had guaranteed byte-identity of plain-path outputs with the hybrid disabled throughout its life, so the strip was accepted only on hash-identical outputs before and after [VERIFY].

The mesh-coarsening inversion (Section 2.2.2) exhibits the complementary discipline. When one cohort member missed a predeclared adequacy rule marginally (roughly 20% over the threshold [VERIFY]), the adjudication ran a finer reference rather than amending the rule — because amending a predeclared tolerance at the margin is precisely the tolerance-fitting the rule exists to prevent [VERIFY: adjudication record; resolution was pending at the time of writing].

We publish these outcomes because they are the evidence that the contracts govern: the same gates that accepted the retained-liquid-capacity snow physics (Section 3.4) rejected a completed solver subsystem and inverted an expected optimization. Selection pressure that runs in both directions is what distinguishes verification from ratification.

## 4. Evaluation

*(Scope: this section evaluates numerical and engineering fidelity — conservation, convergence, artifact removal, and computational cost. It does not evaluate predictive skill against observed erosion or runoff data; see Section 5.3. Every quantity below is [VERIFY] until sourced from a current, reproducible run.)*

### 4.1 Removal of the Boundary-Handoff Peak Artifact

Comparative hydrographs and soil-loss metrics across multi-OFE slopes. [VERIFY: show legacy equivalent-plane peak synchronization versus openWEPP continuous-handoff dispersion on the same inputs; quantify the reduction in peak-flow artifact and the resulting soil-loss difference as a function of OFE count. This is the paper's central fidelity exhibit — it must carry the Section 2.1.2 claims on its own.]

### 4.2 Conservation at Stated Tolerances

Water- and mass-balance closure is enforced fail-closed at named tolerances and reported per run. [VERIFY: report the closure residual distribution across the evaluation set against the named tolerance; state the tolerance and its contract ID. Do not describe closure as "exact" — report the achieved residuals.]

### 4.3 Convergence-Adjudicated Mesh Resolution

The convergence-ladder results, presented as an error-versus-resolution study rather than a single pass/fail point. [VERIFY: (a) analytical-anchor rungs (Iwagaki) — routing error versus resolution; (b) the real-cohort rungs — annual sediment surfaces at fixed10/dx20/dx10/dx5 against the fine-mesh reference, showing the production resolution's 5.8% deposition miss and dx5 meeting tolerance; (c) the watershed-router positivity defect exposed at fine resolution and its fix, as a worked example of the ladder as an audit instrument; (d) the final ratified mesh policy and its contract revision. Frame honestly: the ladder adjudicated *against* coarsening — this section reports the discovery of a production-configuration error, not a mesh-independence victory.]

### 4.4 End-to-End Computational Cost

Timing from launch through completed outputs, versus the legacy reference, on stated hardware. [VERIFY: (a) the array-native runtime trajectory (≈670 s → ≈32.7 s; ≈229 MB → ≈77 MB); (b) the current net end-to-end comparison against legacy WEPP on the same inputs and hardware; (c) the cost of the fidelity-ratified mesh (≈4.85× on long-hillslope members: n_idaho ≈0.96 s → ≈21.2 s, WA ≈15.9 s → ≈62.8 s) if dx5 ratifies. Do not frame cost parity or speedup as a goal met; frame the measured cost as the price of a convergence-backed answer (Section 5.1) and identify the local-numerics offsets under consideration. This section's numbers determine the wording of Sections 2.2, 5.3, and 6.]

## 5. Discussion

### 5.1 What a Refinement Dial Buys: Measurable Error versus Silent Error

The mesh-resolution finding of Sections 2.2.2 and 4.3 deserves interpretation beyond its cost implication, because it illustrates the central structural difference between openWEPP and its predecessor.

openWEPP's ladder found a ~6% mesh-induced sediment error in its own production configuration because a transient solver has a refinement dial: there is always a finer solution to converge toward, so discretization error is measurable — and under openWEPP's contracts, measured by obligation. The steady-at-peak formulation has no counterpart to this. Its collapse of each event to a single steady discharge is not a resolution choice that can be refined; it is a model-form projection whose error is invisible from inside the model. Legacy WEPP cannot be "run finer" to check itself.

We can reason — though we emphasize this is inference, not measurement — about where that silent error is likely largest. The steady-at-peak picture assumes the whole plane reaches equilibrium simultaneously, which requires storm duration to dominate time-to-equilibrium; time-to-equilibrium grows with plane length, so long single OFEs lean hardest on the partial-equilibrium corrections. WEPP's erodibility and transport parameters were calibrated on plot-scale data of order 10–30 m [CITE], so a 300 m OFE extrapolates roughly tenfold past the calibration envelope. And recession-limb deposition — precisely the surface where openWEPP's ladder found mesh sensitivity — is structurally absent from the steady formulation, not under-resolved by it. If halving a transient solution's resolution moves annual deposition by ~6% on a watershed-class member, collapsing the entire transient to one steady scalar plausibly moves shape-sensitive sediment surfaces by substantially more on long hillslopes. Converting that inference to evidence would require running the fine-reference transient against legacy outputs on identical long-OFE inputs — a comparator study we identify as future work, not a claim of this paper [TODO: scope as future work; do not let this paragraph harden into an unverified quantitative claim].

Two consequences follow. For openWEPP's own governance, legacy-comparator flags on long-OFE sediment surfaces carry essentially no adjudication weight — divergence there is expected, and the contract, not the legacy binary, decides. For the field, the ~5× cost of the fidelity-ratified mesh is properly understood not as overhead to match legacy but as the price of a convergence-backed sediment answer on the member class where the legacy formulation is structurally weakest and cannot measure its own error. Set against the orders-of-magnitude growth in commodity compute since WEPP's formulation was fixed, single-digit multipliers are precisely the kind of price that headroom exists to pay: the simplifications were justified by a constraint that no longer binds. The generalizable pattern — replace scalar collapses with resolved solutions *so that* discretization error becomes measurable and contractible — applies to any process-based model whose legacy substrate hides its own error. Precedents for large-scale model restructuring exist — MODFLOW 6's reframing of MODFLOW [CITE: Langevin et al., 2017; VERIFY] and SWAT+'s restructuring of SWAT [CITE: Bieger et al., 2017; VERIFY] — but those efforts retained substantial code and formulation lineage; openWEPP differs in reconstructing from the governing science downward, with correctness defined by contract rather than by comparison to the predecessor.

### 5.2 Structural Provenance as a Byproduct of Stateless Development

A reader may reasonably be cautious of software produced with substantial automated assistance. Our answer is not an appeal to trust but the provision of means to verify — and we argue the verification record here is unusually complete for a structural reason worth stating.

openWEPP was constructed by stateless AI agents: each session begins without memory of previous sessions. To function at all under this constraint, all substantive work must be governed by *work-packages* — scoped task directories that define the execution plan and closure conditions, and that accumulate the artifacts (physical justification, deductive reasoning, executable tests) an agent produces as it works. This documentation is not an administrative mandate that can decay under schedule pressure; it is load-bearing. A subsequent agent can only continue the work if the record left behind is sufficient to reconstruct context, so an inadequate record halts development rather than merely degrading it. To date the reconstruction comprises more than 780 work-packages [VERIFY: exact count at submission; confirm all counted entries are completed work-packages], each with its own layer of evidence — including the work-packages of the abandoned hybrid stepper, which remain on the mainline record while the code they produced does not (Section 3.5).

For modelers, the consequence is a traceability level uncommon in scientific codes: implemented equations trace to artifacts stating their physical justification and to the tests that check them, because that traceability was a functional requirement of the development process rather than a documentation policy. We present this provenance record as a methodological contribution in its own right — the artifact an AI-skeptical reader needs in order to check the work rather than trust it. [TODO: state concretely how a reader accesses and audits the record — repository paths, an example trace from one equation to its work-package and test.]

Two honest caveats. First, provenance is not determinism: the agents are stochastic, and re-running the workflow would not reproduce the same code. What is reproducible is the *verification* — every contract check and test in the record can be re-executed against the current code. Second, completeness of the record is itself a claim to audit, not to assert; we invite exactly that audit.

### 5.3 Limitations

**Predictive skill is not yet evaluated.** The evaluation in Section 4 establishes numerical and engineering fidelity: conservation, convergence, artifact removal, and cost. It does not establish that openWEPP predicts observed runoff and erosion as well as or better than calibrated legacy WEPP. That validation — against instrumented catchment and plot datasets — is ongoing and will be reported separately. [TODO: name the planned validation datasets and scope.]

**openWEPP costs more than legacy WEPP.** A transient, convergence-verified solve is more expensive than a scalar approximation, and the fidelity-ratified mesh adds a further ≈5× on long-hillslope members [VERIFY: final ratified figures per Section 4.4]. We argue this is the price of a measurable answer (Section 5.1), but users with throughput-dominated workloads should weigh it. Local-numerics optimizations to discount the fine-mesh cost are identified but not yet landed [TODO: status at submission].

**The long-OFE legacy-error argument is inference.** Section 5.1's reasoning about where legacy's silent error is largest is mechanistic inference, not measurement; the comparator study that would test it has not been run.

**Winter-hydrology residuals remain.** After the campaign of Section 3.4, roughly a third of snow-control windows still fail tolerance and a frozen-duration over-persistence of +61 days median remains [VERIFY]. These are open physics questions, not closed items.

**Mesh adjudication was in progress at the time of writing.** [TODO: update before submission — state the ratified mesh policy and close out the "operational but not fidelity-ratified" posture, or report it honestly if still open.]

**Domain coverage.** [TODO: enumerate WEPP capabilities not yet implemented (e.g., irrigation, specific management operations, channel-process status) and the maturity stage of the software at submission.]

**Methodology generality.** The development methodology has been exercised on one model by one team. Its costs [TODO: report], its dependence on the quality of human-authored contracts, and its transferability to teams without contract-writing expertise are open questions.

## 6. Conclusion

We have presented openWEPP, a ground-up reimplementation of the WEPP engine that replaces the steady-at-peak scalar collapse of WEPP-lineage codes with hydrograph-resolved routing and erosion, replaces the equivalent plane with continuous kinematic-wave boundary handoffs, and enforces water-balance closure fail-closed at contract tolerances — while preserving the WEPP process science it is built on. The engine's convergence ladder gives it a property its predecessor structurally lacks: its discretization error is measurable, and the first use of that instrument exposed both a latent router defect and a mesh-induced sediment error in the engine's own production configuration. The same governance that built the engine also adjudicated against it where the evidence demanded — abandoning a completed solver subsystem and inverting an expected optimization — and we report those outcomes as evidence that the contracts, not the preferences of the builders, decide. The engine was constructed by human-supervised AI agents under machine-checkable science contracts, a methodology whose structural byproduct is a complete, auditable, re-runnable evidence record. We offer the architecture and the methodology together as a pathway for the reproducible reconstruction and continued stewardship of process-based environmental models beyond WEPP.

---

## Software and data availability

- **Name:** openWEPP
- **Developer / contact:** Roger Lew, University of Idaho [TODO: email]
- **Year first available:** 2026
- **Repository:** [TODO: public GitHub URL] (to be made public at preprint posting)
- **License:** Apache-2.0 [VERIFY: confirm NOTICE for USDA WEPP provenance is complete]
- **Programming language:** Rust
- **Hardware required:** commodity x86-64/ARM64; [TODO: memory/runtime envelope]
- **Software required:** Rust toolchain (pinned channel); [TODO]
- **Program size:** [TODO: SLOC / repository size]
- **Availability and cost:** open source, no cost
- **Data:** evaluation fixtures and their provenance are in the repository; observational datasets used as fixtures are cited to their sources [TODO: list; confirm no copyrighted third-party material is redistributed].

## Data statement

[TODO: EMS Option C — state that evaluation data/fixtures are deposited and linked in the repository, and cite the underlying observational datasets to their original sources; note any data that cannot be redistributed and why.]

## Declaration of competing interest

[TODO: state competing interests, or "The authors have no competing interests to declare."]

## Funding

[TODO: funding sources per EMS format, or the "no specific grant" statement if applicable.]

## CRediT author statement

[TODO: assign CRediT roles — e.g., Roger Lew: Conceptualization, Methodology, Software, Supervision, Validation, Writing – original draft, Writing – review & editing. Add co-authors as applicable.]

## Declaration of generative AI and AI-assisted technologies in the manuscript preparation process

[TODO: per EMS policy. Note this declaration concerns *manuscript preparation*; the use of AI agents in *building the software* is a subject of the paper and is described in Section 3. Draft: "During the preparation of this work the author(s) used [tool/service] in order to [drafting assistance / language]. After using this tool/service, the author(s) reviewed and edited the content as needed and take full responsibility for the content of the published article." AI tools are not listed as authors.]

## Acknowledgements

[TODO: acknowledge the WEPP developers and the body of erosion/hydrologic science the reimplementation builds on — framed as continuity and respect; any individuals who assisted; observational-data providers.]

## References

*(EMS Harvard author–year. All entries below are [VERIFY] — confirm each is real and correctly cited before submission; do not submit with unverified references.)*

- [CITE] Flanagan, D.C., Nearing, M.A. (Eds.), 1995. USDA–Water Erosion Prediction Project: Hillslope Profile and Watershed Model Documentation. NSERL Report No. 10, USDA-ARS National Soil Erosion Research Laboratory, West Lafayette, IN. [VERIFY]
- [CITE] Stone, J.J., Lane, L.J., Shirley, E.D., 1992. Infiltration and runoff simulation on a plane. Transactions of the ASAE, 35(1), pp.161–170. [VERIFY: confirm this is the correct citation for the WEPP semi-analytic peak-runoff formulation (irs.for lineage)]
- Papanicolaou, A.N., Abban, B.K.B., Dermisis, D.C., Giannopoulos, C.P., Flanagan, D.C., Frankenberger, J.R. and Wacha, K.M., 2018. Flow resistance interactions on hillslopes with heterogeneous attributes: Effects on runoff hydrograph characteristics. Water Resources Research, 54(1), pp.359-380. [VERIFY: confirm this supports both the flow-resistance framework (2.1.2) and the TVD-MacCormack attribution (2.1.3); if not, split citations]
- [CITE] Iwagaki, Y., 1955. Fundamental studies on the runoff analysis by characteristics. Bulletin No. 10, Disaster Prevention Research Institute, Kyoto University. [VERIFY: exact analytical solutions used by the convergence ladder]
- [CITE] Harten, A., 1983. High resolution schemes for hyperbolic conservation laws. Journal of Computational Physics, 49(3), pp.357–393. [VERIFY]
- [CITE: MacCormack scheme original reference — MacCormack, R.W., 1969] [VERIFY]
- [CITE: TVD-MacCormack applied to shallow-water / overland flow — e.g., Garcia-Navarro et al.] [VERIFY]
- [CITE] Langevin, C.D., Hughes, J.D., Banta, E.R., Niswonger, R.G., Panday, S., Provost, A.M., 2017. Documentation for the MODFLOW 6 Groundwater Flow Model. USGS Techniques and Methods 6-A55. [VERIFY]
- [CITE] Bieger, K., Arnold, J.G., Rathjens, H., White, M.J., Bosch, D.D., Allen, P.M., Volk, M., Srinivasan, R., 2017. Introduction to SWAT+, a completely restructured version of the Soil and Water Assessment Tool. JAWRA, 53(1), pp.115–130. [VERIFY]
- [CITE: WEPP plot-scale calibration provenance for erodibility/transport parameters — for the Section 5.1 calibration-envelope point] [VERIFY]
- [CITE: weppcloud platform reference — Lew et al.] [VERIFY]
- [CITE: reproducible-research / computational-provenance reference(s)]
- [CITE: additional WEPP science references as cited in-text]
- [TODO: complete reference list]
