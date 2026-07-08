<!--
WORKING DRAFT — openWEPP architecture paper (paper 0001).
Format: Markdown first; convert to Elsevier els-cas LaTeX when stable (see latex-instructions.md).
Target venue: Environmental Modelling & Software (EMS), research article.
Framing (agreed): physics-primary; AI-native development secondary as the methodological enabler;
audience assumed AI-skeptical; WEPP treated as respected foundation, NOT as "defect-laden";
sell thoroughness + auditability, never speed.
Tone rules (2026-07-07 rewrite): no superlatives ("massive", "perfect", "airtight", "flawless",
"pristine", "absolute"); claims scoped to numerical/engineering fidelity — predictive-skill
validation is explicitly deferred and stated up front; "guarantee" only for properties the
type system or a fail-closed check actually enforces; internal codenames defined at first use
or replaced with standard vocabulary.
[TODO] markers = author must supply/verify before submission. [CITE] = reference needed.
Nothing empirical (numbers, datasets, defect specifics) is asserted here without a verified
source or a [VERIFY] flag — do not let placeholders reach submission. This includes Section 3.4.
-->

# openWEPP: A Deterministic, Hybrid Implicit–Explicit Reconstruction of the WEPP Erosion Model, Built Under Machine-Checkable Science Contracts

<!-- [TODO: title decision — alternative with methodology de-emphasized:
"A deterministic, hybrid implicit–explicit numerical architecture for process-based
erosion modeling: the openWEPP reconstruction of WEPP". Pick one before LaTeX conversion.] -->

**Roger Lew**^a^ [TODO: co-authors?]

^a^ University of Idaho, Moscow, ID, USA. [TODO: full postal address; corresponding-author email]

*Corresponding author:* Roger Lew ([TODO: email])

---

## Abstract

*(≤150 words, factual, stand-alone.)*

Process-based environmental models are long-lived scientific infrastructure whose governing equations often outlast the numerical schemes used to solve them. We present openWEPP, a ground-up Rust reimplementation of the Water Erosion Prediction Project (WEPP) hillslope and watershed engine that preserves WEPP's process science while replacing its numerical substrate. Continuous kinematic-wave boundary handoffs replace lumped equivalent-plane approximations, removing artificial peak-flow synchronization on multi-element hillslopes. A shock-capturing total variation diminishing (TVD) scheme coupled to an implicit recession solver provides stable routing at stated accuracy tolerances on coarse meshes, and fail-closed mass accounting enforces water-balance closure on every run. The engine was constructed by human-supervised AI agents working under machine-checkable science contracts, a process that leaves a complete, re-runnable evidence trail. We describe the architecture, the contract-governed methodology, and the verification evidence, and argue the combination offers a reproducible pathway for modernizing legacy environmental models.

**Keywords:** erosion; hydrology; kinematic wave; IMEX solvers; scientific software; reproducibility; software verification

---

## Highlights

*(3–5 bullets, ≤85 characters each; submitted as a separate file.)*

- Continuous kinematic-wave handoffs replace equivalent-plane runon approximations
- Shock-capturing TVD routing holds stated accuracy tolerances on coarse meshes
- Fail-closed mass accounting halts the run on any water-balance closure failure
- Human-supervised AI agents built the engine under machine-checkable contracts
- Each change traces to a work-package artifact and an executable test

---

## 1. Introduction

Process-based environmental simulation models are long-lived scientific infrastructure. Codes for erosion, hydrology, water quality, and land–atmosphere exchange are developed over decades, outlive their original authors, and are relied upon for research and management decisions. The scientific formulations — the governing equations — of these models are, in general, sound and well documented. The numerical schemes and software substrates used to solve those equations, however, reflect the computational constraints of the era in which each model was engineered, and those constraints can limit the physical fidelity achievable today.

The Water Erosion Prediction Project (WEPP), developed by the USDA Agricultural Research Service [CITE: Flanagan and Nearing, 1995], is a case in point and the respected foundation on which this work builds. WEPP's process science — infiltration, water balance, plant growth, hillslope hydraulics, and detachment/transport mechanics — remains the standard for physically based erosion prediction three decades after its release. Like other models of its generation, however, WEPP solves its routing equations with numerical techniques that were appropriate to the hardware of the early 1990s. Classic finite-difference routing schemes exhibit numerical diffusion on long hillslopes, which is conventionally countered by increasing spatial resolution at computational cost. Flow across discrete land elements is handled by lumped algebraic approximations (the "equivalent plane") rather than continuous wave routing. These approximations are computationally cheap and were a reasonable engineering trade at the time, but they synchronize flow volumes across element boundaries, producing peak-flow artifacts that propagate — through the power-law dependence of detachment on flow rate and shear stress — into large, unphysical spikes in simulated soil loss on multi-element hillslopes [VERIFY: documented instances of the multi-OFE erosion blowup; cite or show in Section 4.1].

Removing these artifacts requires shock-capturing conservation-law numerics: limiter-stabilized explicit schemes, implicit solvers for stiff recession, and mass accounting that remains closed across solver and element boundaries. Implementing such a stack correctly — and keeping it correct over years of maintenance — is demanding for any engineering team, because the failure modes are silent: a small conservation leak or an unflagged clamp degrades results without crashing anything.

This paper presents openWEPP, a ground-up reconstruction of the WEPP hillslope and watershed engine that addresses these numerical limitations while preserving WEPP's governing science. The contribution is threefold. First, we detail the numerical architecture (Section 2): continuous kinematic-wave boundary handoffs, a TVD-MacCormack routing scheme, fail-closed mass accounting, and the mesh-coarsening and hybrid implicit–explicit stepping that the stricter numerics make safe. Second, we describe the development methodology that produced it (Section 3): a human-supervised multi-agent workflow governed by machine-checkable science contracts, which leaves a complete and re-runnable evidence trail. Third, we evaluate the result (Section 4) on *numerical and engineering fidelity*: elimination of the boundary-handoff peak artifact, water-balance closure at stated tolerances, mesh-independence of routing accuracy, and end-to-end computational cost. We state the scope plainly at the outset: this paper's fidelity claims concern conservation, convergence, and artifact removal. Validation of predictive skill against observed runoff and erosion data is ongoing and is deferred to future work (Section 5.3).

## 2. Numerical Architecture

openWEPP preserves the governing science of WEPP and overhauls the numerical techniques used to solve it. The architecture separates two concerns: the numerics required for physical fidelity (Section 2.1), and the performance measures that the stricter numerics make safe to apply (Section 2.2).

### 2.1 Architecture for Numerical Fidelity

#### 2.1.1 Continuous Wave Boundary Handoffs (the Runon Problem)

In legacy WEPP, surface runoff transitioning from one Overland Flow Element (OFE) to another is approximated by the equivalent-plane method or by lumped daily mass transfers. When a fast wave from a steep upper element reaches a flatter lower element, the lumped approximation delivers the volume as a synchronized pulse rather than a routed wave. Because detachment and transport capacity depend on flow rate and shear stress through power laws, the synchronized pulse inflates simulated soil loss; the effect compounds with element count and becomes pronounced on hillslopes with many OFEs [VERIFY: characterize the scaling with OFE count from the Section 4.1 evaluation set].

openWEPP replaces the equivalent plane with continuous boundary handoffs. The routing subsystem owns the surface-water trajectory end to end: the full time-resolved hydrograph leaving the upstream OFE becomes the dynamic upper boundary condition of the downstream OFE's solver. When a steep-slope wave enters a flat element, the solver attenuates and stretches it as the kinematic-wave equations dictate, producing physically consistent peak flows. The peak artifact is removed by routing the wave rather than by post-hoc correction. This design extends the flow-resistance and routing framework of Papanicolaou et al. (2018) [VERIFY: confirm the precise relationship — what is adopted from that framework versus new here].

#### 2.1.2 Shock Capturing via TVD-MacCormack

openWEPP routes overland flow with an explicit MacCormack predictor–corrector scheme stabilized by total variation diminishing (TVD) flux limiting [CITE: MacCormack scheme; CITE: Harten 1983 / TVD theory; CITE: TVD-MacCormack in shallow-water/overland-flow applications], following the application of shock-capturing schemes to hillslope runoff by Papanicolaou et al. (2018) [VERIFY: confirm attribution — did that work introduce TVD-MacCormack to hillslope routing, or apply an existing line of work?]. Classic first-order finite-difference schemes smear sharp wave fronts through numerical diffusion, and the conventional remedy is a finer mesh. TVD limiting suppresses spurious oscillation while preserving front sharpness, so routing accuracy degrades far more slowly as the mesh coarsens. We treat the resulting mesh-independence as an empirical property to be demonstrated, not a theorem: Section 4.3 presents convergence evidence against analytical solutions across mesh resolutions.

#### 2.1.3 Mass Ledgers and Cross-Span Deficit Carry

Coupling multiple numerical solvers across element boundaries risks conservation leaks at the seams. openWEPP addresses this by decoupling *attribution* (the hydrograph shape produced by the solver) from the *mass ledger* (the accounting of physical boundary fluxes). A mechanism we call the cross-span deficit carry — a transactional buffer at each element boundary — absorbs the oscillatory flux errors inherent to explicit schemes and repays them within the span, so that accounting closure is maintained even when instantaneous fluxes oscillate. The system is fail-closed: water-balance closure is checked at a named tolerance every step, and a violation halts the simulation with a typed error rather than being silently absorbed. We do not claim exact arithmetic — floating-point computation is not exact — but closure at the stated tolerance is enforced, not merely monitored, and Section 4.2 reports the achieved residuals.

### 2.2 Performance Measures Enabled by the Fidelity Architecture

The measures in this section are not shortcuts; each is an optimization whose safety is established by the numerics of Section 2.1 and checked by the contracts of Section 3.

#### 2.2.1 Contract-Bounded Mesh Coarsening

Because first-order schemes must counter numerical diffusion with dense grids (e.g., the 100-cell-per-hillslope resolution used by the WEPPcloud/DEP configuration of legacy WEPP [VERIFY: confirm which configuration enforces this and cite]), and because explicit schemes are CFL-limited — halving the cell size roughly halves the admissible time step — routing cost grows approximately quadratically with cell count. The TVD scheme's resistance to diffusion allows much coarser meshes at equal accuracy. openWEPP exploits this under an explicit contract: a mesh resolution is admissible only if it meets a stated peak-flow error tolerance (<5% [VERIFY: confirm the contract value and its ID]) against the convergence ladder of Section 4.3. On the evaluation fixtures, a 5-cell mesh satisfies the tolerance where the legacy configuration uses 100 cells [VERIFY: Section 4.3 data], reducing routing cost by roughly an order of magnitude. The claim is empirical and contract-bounded, not an asymptotic guarantee.

#### 2.2.2 The Hybrid Implicit–Explicit Stepper

Kinematic-wave routing has two regimes with conflicting numerical demands: storm dynamics, where sharp fronts require small explicit steps, and recession, where the solution is smooth and slow but stiff enough to make small steps wasteful. openWEPP couples the explicit TVD scheme to a backward-Euler implicit solver in a hybrid implicit–explicit (IMEX) arrangement. A forcing-derived switching predicate — a cooldown keyed to the memory of recent source terms (rainfall, run-on) — hands the solution to the implicit solver once forcing has quiesced, and returns it to the explicit scheme when forcing resumes. Backward Euler is A-stable in the linear theory; for this nonlinear system we verify stability and accuracy of the large recession steps empirically against the fully explicit solution and the convergence ladder [VERIFY: Section 4.3/4.4 evidence; state the step-size bounds actually used]. The mass ledgers of Section 2.1.3 span the solver handoff, so closure is enforced across regime switches, not just within each solver.

#### 2.2.3 Array-Native Runtime: Moving Symbolic Structure to the Edges

The initial openWEPP runtime prioritized auditability: a dynamic, symbol-keyed state map preserved one-to-one semantic correspondence with legacy WEPP variables, at the cost of dictionary lookups dominating the hot loop. That build ran approximately 73× slower than the legacy Fortran reference on the benchmark fixture [VERIFY: fixture, hardware, and measurement provenance], and profiling attributed the gap to lookup overhead rather than arithmetic. Incremental optimization did not remove the structural bottleneck.

The engine was therefore restructured to an array-native runtime: the hot loop executes over contiguous arrays of static structs, while the symbolic and diagnostic structures were moved to the system's edges — input parsing, shadow validation against legacy records, and output projection retain the full symbolic mapping. On the same fixture, the restructuring reduced execution time by roughly a factor of 20 (≈670 s to ≈32.7 s) and memory by roughly 66% (≈229 MB to ≈77 MB) [VERIFY: current benchmark numbers, fixture, hardware].

We report the arithmetic plainly: a 73× deficit reduced by a factor of 20 leaves the engine of that build approximately 3–4× slower than the legacy reference on that fixture. The mesh coarsening (Section 2.2.1) and IMEX stepping (Section 2.2.2) apply on top of the runtime restructuring; Section 4.4 reports current end-to-end timing against legacy on stated hardware [TODO: obtain and report the current net vs-legacy figure; frame Section 2.2 conclusions to match it — "computational cost competitive with legacy despite substantially stricter accounting and routing" if the engine is near parity, or the measured speedup if it is ahead. Do not let the abstract or conclusion claim a net speedup unless this number supports it].

## 3. The Verification Substrate: How the Architecture Was Built

The architecture of Section 2 — fail-closed accounting, contract-bounded optimizations, typed boundaries — demands a level of implementation discipline that is difficult for any team to sustain across years of maintenance, because its failure modes are silent. openWEPP's approach was to make the discipline structural: the reconstruction was carried out by a human-supervised multi-agent AI workflow in which every mechanical rule is enforced by machine-checkable gates rather than by convention. We describe the substrate here because it is inseparable from the result: the architecture is the kind of software this methodology makes tractable.

[TODO: this section currently under-specifies the workflow for a methodology-interested reviewer. Add: (a) which agent systems/models were used and in what roles (implementation, review, adjudication), with versions; (b) the human supervision protocol — what humans authored (contracts? adjudications?), what they reviewed, at what cadence, and what they never delegated; (c) gate statistics if recoverable — how often gates rejected proposed changes; (d) approximate cost (compute/tokens/wall-clock) of the reconstruction; (e) known failure modes of the workflow and how they were caught.]

### 3.1 Typed Boundaries and Deterministic Orchestration

The orchestrator constructs execution order dynamically from declared data dependencies, so the continuous boundary handoffs of Section 2.1.1 are always fed by completed upstream state. The architecture forbids shared global memory; processes communicate through typed boundaries, and ownership of each trajectory (producer versus consumer) is enforced by the Rust borrow checker. Quantities crossing boundaries carry physical units in their types, making dimensional consistency a compile-time property rather than a review-time check.

### 3.2 Top-Down Science Contracts, Fixtures, and weppcloud

Above the code sits a layer of machine-checkable science contracts: named specifications with explicit invariants and tolerance bounds that define correctness for each process domain. Contracts were formulated from primary literature and conservation laws *before* implementation, and correctness is defined against the contract — not against the output of the legacy binary, which would merely reproduce the legacy numerics' limitations. Legacy comparison is retained as a flagging mechanism for investigation, never as an acceptance oracle.

Test fixtures derive from two sources. Analytical fixtures come from the primary literature — notably a convergence ladder built on the Iwagaki analytical solutions for kinematic-wave runoff [CITE: Iwagaki 1955; VERIFY: exact solutions used], against which routing accuracy is measured at successive mesh resolutions. Real-world fixtures come from **weppcloud** [CITE: weppcloud], a platform that assembles high-resolution WEPP inputs (topography, soils, climate forcings) for arbitrary locations within minutes, providing realistic parameter combinations at scale. The engine additionally maintains strict legacy input-file compatibility, so the decades-old library of WEPP management and climate datasets remains natively usable.

### 3.3 Governed Autonomous Agents

Implementation against the contracts was executed by AI agents operating inside a governance envelope of mechanical gates: formatting, strict static analysis, conservation checks, and test suites, all of which must pass before a change can be proposed for human review. Two rules are central. First, numerical faults must surface as typed errors: agents are forbidden from substituting defaults, applying numerical clamps (e.g., `max(0, depth)`), or scaffolding surrogate physics to route around a failing check — the practices by which silent errors historically enter simulation codes. Second, all substantive work occurs inside authorized work-packages (Section 5.2) with explicit execution plans and closure conditions. The agents' role is to satisfy the bureaucracy of the contracts exhaustively; the humans' role is to author the contracts, adjudicate ambiguities, and review outcomes [VERIFY: confirm this division of labor is stated accurately; align with the TODO at the top of Section 3].

### 3.4 Case Study: The Snow and Frost Fidelity Campaign

To make the methodology concrete, we describe one sustained campaign: resolving long-standing gaps in WEPP's winter hydrology. [VERIFY: every quantity in this subsection must be sourced from campaign records before submission; none currently carry citations to run artifacts.]

The campaign began with data, not code. The team assembled observational baselines from SNOTEL, SCAN, and USFS canopy-stratified sites, explicitly distinguishing physical measurements (e.g., frost tubes) from proxy measurements (e.g., 0 °C soil isotherms). To isolate single-hillslope physics from watershed-scale confounding, weppcloud was used to scaffold fixtures — topography, soils, and high-resolution climate forcings (DAYMET, GRIDMET, PRISM) — for the instrumented sites.

Rather than tuning to single-residual targets, which invites compensating errors, the campaign fixed an evaluation rubric in advance. Signatures were classified as *forcing-robust* (e.g., melt-out timing, densification trajectories), which carry verdict weight, or *forcing-limited* (e.g., absolute SWE magnitudes, which are sensitive to gauge undercatch), which are reported but not used as acceptance thresholds. Anti-tuning guards forbade modifying shared radiation inputs to fix melt behavior, protecting the evapotranspiration balance from compensating adjustment.

Under this rubric, agents ran controlled experiments in an opt-in physics sandbox. Candidate modernizations — a revised shortwave energy balance, psychrometric rain/snow partitioning — were implemented, evaluated, and *rejected* when the rubric showed they worsened maritime-site residuals [VERIFY]; the rubric adjudicated, not the preference for newer physics.

Systematic testing then isolated a melt-realization gap: the model computed sufficient melt energy, but a legacy density-gate proxy prevented the mass loss from being realized, producing an aggregate depth-loss deficit of 24.1 m across the evaluation set and failing snow-control tolerances on 1,147 of 1,415 observed thaw-ablation windows [VERIFY: define "snow-control window" and the tolerance; cite campaign records]. Replacing the proxy with a physical retained-liquid-capacity formulation reduced the aggregate deficit to 15.5 m; paired with an updated bulk-density compaction model, window failures fell from 1,147 to 498 [VERIFY]. We note the honest remainder: roughly a third of windows still fail, and a substantial deficit remains [TODO: characterize where the remaining failures concentrate (site class, forcing regime) and what physics is implicated; without this context the residual is uninterpretable].

A parallel effort addressed frost depth. Early builds inherited a freeze-index proxy that estimated frost depth as a fraction of accumulated air temperature, capping depth at 0.20 m, and a ratchet mechanism over-persisted frozen-soil duration by a median of +258 days [VERIFY]. Governed work-packages replaced the proxy with a layered energy-balance heat-flow model: fine-layer discretization, in-hour resistance feedbacks, and surface-temperature synthesis incorporating radiation, wind, canopy, and residue conductance. The 0.20 m cap disappeared as a consequence of the physics rather than by adjustment; median depth correlation against reference physics improved from 0.13 to 0.763, and the frozen-duration over-persistence fell from +258 to +61 days [VERIFY: define "reference physics" and the correlation measure; +61 days is still large — state what drives it and whether it is under active work], while water-balance closure held at numerical noise (maximum absolute residual $5.1\times10^{-7}$ mm [VERIFY]).

The campaign illustrates the pattern this paper argues for: fixtures from primary data, rubrics fixed before experimentation, anti-tuning guards, and adjudication by contract — under which candidate physics can be tested and rejected as readily as accepted.

## 4. Evaluation

*(Scope: this section evaluates numerical and engineering fidelity — conservation, convergence, artifact removal, and computational cost. It does not evaluate predictive skill against observed erosion or runoff data; see Section 5.3. Every quantity below is [VERIFY] until sourced from a current, reproducible run.)*

### 4.1 Removal of the Boundary-Handoff Peak Artifact

Comparative hydrographs and soil-loss metrics across multi-OFE slopes. [VERIFY: show legacy equivalent-plane peak synchronization versus openWEPP continuous-handoff dispersion on the same inputs; quantify the reduction in peak-flow artifact and the resulting soil-loss difference as a function of OFE count. This is the paper's central fidelity exhibit — it must carry the Section 2.1.1 claims on its own.]

### 4.2 Conservation at Stated Tolerances

Water- and mass-balance closure is enforced fail-closed at named tolerances and reported per run. [VERIFY: report the closure residual distribution across the evaluation set against the named tolerance; state the tolerance and its contract ID. Do not describe closure as "exact" — report the achieved residuals.]

### 4.3 Mesh Independence of Routing Accuracy

Convergence of the TVD scheme against analytical solutions across mesh resolutions. [VERIFY: present the Iwagaki convergence-ladder results; demonstrate the 5-cell mesh meeting the <5% peak-flow error contract where the legacy configuration uses 100 cells; show the error-versus-resolution curve rather than a single pass/fail point.]

### 4.4 End-to-End Computational Cost

Timing from launch through completed outputs, versus the legacy reference, on stated hardware. [VERIFY: (a) the array-native runtime trajectory (≈670 s → ≈32.7 s; ≈229 MB → ≈77 MB); (b) the incremental contributions of mesh coarsening and IMEX stepping; (c) the current net end-to-end comparison against legacy WEPP on the same inputs and hardware. Item (c) determines how Sections 2.2 and 6 may characterize performance — see the TODO in Section 2.2.3.]

## 5. Discussion

### 5.1 Generality beyond WEPP

The pattern presented here — replacing lumped legacy approximations with contract-bounded conservation-law numerics, under fail-closed accounting — applies to other process-based models whose fidelity is constrained by their numerical substrate rather than their science. Precedents for large-scale model restructuring exist — MODFLOW 6's object-oriented reframing of MODFLOW [CITE: Langevin et al., 2017; VERIFY] and SWAT+'s restructuring of SWAT [CITE: Bieger et al., 2017; VERIFY] — but those efforts retained substantial code lineage. openWEPP differs in reconstructing from the governing science downward, with correctness defined by contract rather than by comparison to the predecessor. [TODO: sharpen this comparison after the CITE items are verified; identify 1–2 candidate model classes (hydrologic, water-quality, land-surface) where the pattern applies most directly.]

### 5.2 Structural Provenance as a Byproduct of Stateless Development

A reader may reasonably be cautious of software produced with substantial automated assistance. Our answer is not an appeal to trust but the provision of means to verify — and we argue the verification record here is unusually complete for a structural reason worth stating.

openWEPP was constructed by stateless AI agents: each session begins without memory of previous sessions. To function at all under this constraint, all substantive work must be governed by *work-packages* — scoped task directories that define the execution plan and closure conditions, and that accumulate the artifacts (physical justification, deductive reasoning, executable tests) an agent produces as it works. This documentation is not an administrative mandate that can decay under schedule pressure; it is load-bearing. A subsequent agent can only continue the work if the record left behind is sufficient to reconstruct context, so an inadequate record halts development rather than merely degrading it. To date the reconstruction comprises more than 780 work-packages [VERIFY: exact count at submission; confirm all counted entries are completed work-packages], each with its own layer of evidence.

For modelers, the consequence is a traceability level uncommon in scientific codes: implemented equations trace to artifacts stating their physical justification and to the tests that check them, because that traceability was a functional requirement of the development process rather than a documentation policy. We present this provenance record as a methodological contribution in its own right — the artifact an AI-skeptical reader needs in order to check the work rather than trust it. [TODO: state concretely how a reader accesses and audits the record — repository paths, an example trace from one equation to its work-package and test.]

Two honest caveats. First, provenance is not determinism: the agents are stochastic, and re-running the workflow would not reproduce the same code. What is reproducible is the *verification* — every contract check and test in the record can be re-executed against the current code. Second, completeness of the record is itself a claim to audit, not to assert; we invite exactly that audit.

### 5.3 Limitations

**Predictive skill is not yet evaluated.** The evaluation in Section 4 establishes numerical and engineering fidelity: conservation, convergence, artifact removal, and cost. It does not establish that openWEPP predicts observed runoff and erosion as well as or better than calibrated legacy WEPP. That validation — against instrumented catchment and plot datasets — is ongoing and will be reported separately. [TODO: name the planned validation datasets and scope.]

**Winter-hydrology residuals remain.** After the campaign of Section 3.4, roughly a third of snow-control windows still fail tolerance and a frozen-duration over-persistence of +61 days median remains [VERIFY]. These are open physics questions, not closed items.

**Performance standing.** [TODO: state plainly, from the Section 4.4 result, whether openWEPP is currently faster than, at parity with, or slower than legacy WEPP end-to-end, and by how much.]

**Domain coverage.** [TODO: enumerate WEPP capabilities not yet implemented (e.g., irrigation, specific management operations, channel processes status) and the maturity stage of the software at submission.]

**Methodology generality.** The development methodology has been exercised on one model by one team. Its costs [TODO: report], its dependence on the quality of human-authored contracts, and its transferability to teams without contract-writing expertise are open questions.

## 6. Conclusion

We have presented openWEPP, a ground-up reimplementation of the WEPP engine that removes legacy numerical artifacts through continuous kinematic-wave boundary handoffs, shock-capturing TVD routing, fail-closed mass accounting, and a hybrid implicit–explicit stepper — while preserving the WEPP process science it is built on. The stricter numerics make principled performance measures safe, including contract-bounded mesh coarsening and large implicit recession steps [TODO: characterize net performance per Section 4.4 before finalizing this sentence]. The engine was constructed by human-supervised AI agents governed by machine-checkable science contracts, a methodology whose structural byproduct is a complete, auditable, re-runnable evidence record. We offer the architecture and the methodology together as a pathway for the reproducible reconstruction and continued stewardship of process-based environmental models beyond WEPP.

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
- Papanicolaou, A.N., Abban, B.K.B., Dermisis, D.C., Giannopoulos, C.P., Flanagan, D.C., Frankenberger, J.R. and Wacha, K.M., 2018. Flow resistance interactions on hillslopes with heterogeneous attributes: Effects on runoff hydrograph characteristics. Water Resources Research, 54(1), pp.359-380. [VERIFY: confirm this supports both the flow-resistance framework (2.1.1) and the TVD-MacCormack attribution (2.1.2); if not, split citations]
- [CITE] Iwagaki, Y., 1955. Fundamental studies on the runoff analysis by characteristics. Bulletin No. 10, Disaster Prevention Research Institute, Kyoto University. [VERIFY: exact analytical solutions used by the convergence ladder]
- [CITE] Harten, A., 1983. High resolution schemes for hyperbolic conservation laws. Journal of Computational Physics, 49(3), pp.357–393. [VERIFY]
- [CITE: MacCormack scheme original reference — MacCormack, R.W., 1969] [VERIFY]
- [CITE: TVD-MacCormack applied to shallow-water / overland flow — e.g., Garcia-Navarro et al.] [VERIFY]
- [CITE] Langevin, C.D., Hughes, J.D., Banta, E.R., Niswonger, R.G., Panday, S., Provost, A.M., 2017. Documentation for the MODFLOW 6 Groundwater Flow Model. USGS Techniques and Methods 6-A55. [VERIFY]
- [CITE] Bieger, K., Arnold, J.G., Rathjens, H., White, M.J., Bosch, D.D., Allen, P.M., Volk, M., Srinivasan, R., 2017. Introduction to SWAT+, a completely restructured version of the Soil and Water Assessment Tool. JAWRA, 53(1), pp.115–130. [VERIFY]
- [CITE: weppcloud platform reference — Lew et al.] [VERIFY]
- [CITE: reproducible-research / computational-provenance reference(s)]
- [CITE: additional WEPP science references as cited in-text]
- [TODO: complete reference list]
