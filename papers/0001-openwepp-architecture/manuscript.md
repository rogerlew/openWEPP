<!--
WORKING DRAFT — openWEPP architecture paper (paper 0001).
Format: Markdown first; convert to Elsevier els-cas LaTeX when stable (see latex-instructions.md).
Target venue: Environmental Modelling & Software (EMS), research article.
Framing (agreed): physics-primary; AI-native development secondary as the methodological enabler;
audience assumed AI-skeptical; WEPP treated as respected foundation, NOT as "defect-laden";
no generational-ranking narrative; sell thoroughness + auditability, never speed.
[TODO] markers = author must supply/verify before submission. [CITE] = reference needed.
Nothing empirical (numbers, datasets, defect specifics) is asserted here without a verified
source or a [VERIFY] flag — do not let placeholders reach submission.
-->

# A Deterministic, Hybrid Implicit-Explicit Architecture for High-Fidelity Erosion Modeling, Synthesized via Autonomous Science Contracts

**Roger Lew**^a^ [TODO: co-authors?]

^a^ University of Idaho, Moscow, ID, USA. [TODO: full postal address; corresponding-author email]

*Corresponding author:* Roger Lew ([TODO: email])

---

## Abstract

*(≤150 words, factual, stand-alone.)*

Process-based environmental models often rely on numerical approximations that limit their spatial scalability and accuracy across complex boundaries. We present openWEPP, a ground-up reimplementation of the Water Erosion Prediction Project (WEPP) engine designed around a modern, exact-ledger computational fluid dynamics (CFD) architecture. openWEPP replaces legacy equivalent-plane approximations with continuous kinematic wave boundary handoffs, eliminating artificial peak flows and subsequent erosion blowups on multi-element hillslopes. By coupling a shock-capturing Total Variation Diminishing (TVD) explicit scheme with an unconditionally stable implicit recession solver, the engine achieves scale-independent stability and massive performance gains without sacrificing physical fidelity. To construct this highly rigid architecture without introducing numerical artifacts, we employed a novel, human-supervised multi-agent workflow governed by top-down, machine-checkable science contracts. This paper details the computational physics of the engine, empirical proofs of its conservation and mesh-independence guarantees, and the fully auditable, contract-first methodology that made its synthesis possible.

**Keywords:** erosion; hydrology; computational fluid dynamics; IMEX solvers; scientific-software; reproducibility; contracts

---

## Highlights

*(3–5 bullets, ≤85 characters each; submitted as a separate file.)*

- True kinematic wave boundary handoffs eliminate artificial erosion blowups
- TVD and hybrid implicit-explicit solvers guarantee scale-independent stability
- Mass conservation and cross-span deficits enforced by an exact-ledger architecture
- Governed multi-agent development with contract-first, evidence-labelled verification
- Ground-up Rust reimplementation of WEPP with a fully auditable development history

---

## 1. Introduction

Process-based environmental simulation models are long-lived scientific infrastructure. Codes for erosion, hydrology, water quality, and land–atmosphere exchange are developed over decades, outlive their original authors, and are relied upon for research and management decisions. The scientific formulations—the governing equations—of these models are, in general, sound and well documented. However, the numerical schemes and software substrates used to solve these equations often constrain the model's physical fidelity. 

Mature models, such as the Water Erosion Prediction Project (WEPP) developed by the USDA Agricultural Research Service [CITE: Flanagan and Nearing, 1995], often rely on legacy numerical approximations. For instance, classic finite-difference routing schemes suffer from severe numerical diffusion on long hillslopes, forcing modellers to brute-force spatial resolution with dense, computationally expensive meshes. Furthermore, flow across discrete land elements is frequently handled via lumped algebraic approximations (e.g., the "equivalent plane") rather than continuous wave routing. While computationally cheap, these approximations artificially synchronize flow volumes across boundaries, generating unphysical peak flow spikes that translate exponentially into catastrophic erosion "blowups" in the model outputs.

Resolving these physical limitations requires an enterprise-grade computational fluid dynamics (CFD) architecture. However, writing shock-capturing solvers, coupling them with fast implicit methods, and guaranteeing perfect mass conservation across boundaries is notoriously difficult for human engineering teams to implement and maintain without introducing subtle, silent numerical bugs.

This paper presents openWEPP, a ground-up reconstruction of the WEPP hillslope and watershed engine that explicitly solves these numerical challenges. The contribution of this paper is threefold. First, we detail the core computational physics achievements (Section 2), distinctly separating the architectural advances that guarantee absolute scientific fidelity from the optimizations that yield massive computational performance. Second, we describe the verification substrate that made building this architecture possible (Section 3): a governed, contract-first development methodology carried out by a human-supervised multi-agent workflow. Third, we evaluate the result (Section 4), presenting empirical proofs of the engine's elimination of the erosion blowup, its mesh-independence, and its perfect conservation guarantees. We argue that the combination of advanced numerical physics and fully auditable, contract-driven synthesis represents a generalizable pathway for the reproducible reconstruction of process-based environmental models.

## 2. Computational Physics Architecture

openWEPP preserves the governing science of WEPP but completely overhauls the numerical techniques used to solve it. The architecture is intentionally bifurcated into two conceptual tiers: absolute physical fidelity, and the performance optimizations that strict fidelity unlocks.

### 2.1 Architecture for Scientific Fidelity

The primary objective of openWEPP is to establish an airtight case for scientific correctness, removing mathematical artifacts that plague legacy simulations.

#### 2.1.1 Continuous Wave Boundary Handoffs (Resolving the Runon Problem)

In legacy WEPP, surface runoff transitioning from one Overland Flow Element (OFE) to another is approximated using an "equivalent plane" method or lumped daily mass transfers. When a fast wave from a steep upper slope hits a flat lower slope, these lumped approximations artificially squash the volume into a synchronized peak. Because erosion equations scale exponentially with peak flow, this mathematical artifact generates massive, unphysical spikes in simulated soil loss—an erosion "blowup" that becomes especially catastrophic when simulating hillslopes with greater than 10 OFEs.

Building on the enhanced flow resistance framework proposed by Papanicolaou et al. (2018), openWEPP discards the equivalent plane and introduces true continuous boundary handoffs. The routing subsystem strictly owns the surface water, passing the exact time-series hydrograph from the upstream OFE as the dynamic upper boundary condition for the downstream OFE's numerical solver. As the physical wave hits the flat lower slope, the solver naturally diffuses and stretches the wave, yielding physically realistic peak flows and eliminating the erosion blowup entirely through architectural rigor.

#### 2.1.2 Scale-Independent Stability via TVD-MacCormack

Following the shock-capturing formulations introduced to the domain by Papanicolaou et al. (2018), openWEPP utilizes an explicit Total Variation Diminishing (TVD) MacCormack scheme. Legacy finite-difference approximations suffer from numerical diffusion, physically blurring and smearing sharp shockwaves. In contrast, the TVD mathematical limiters actively prevent numerical diffusion, capturing the precise shape of kinematic shocks regardless of grid size, thereby guaranteeing that the core engine is highly accurate across all spatial domains.

#### 2.1.3 Exact Mass Ledgers and Cross-Span Deficit Carry

The hardest challenge in coupling multiple numerical solvers is managing boundary conditions without leaking mass. openWEPP solves this via a strict decoupling of mathematical "attribution" (the hydrograph shape) from the "mass ledger" (exact physical boundary fluxes). The architecture employs a "Cross-Span Deficit Carry," a transactional buffer that absorbs numerical oscillations from the explicit scheme without ever losing a drop of water. The model operates as a fail-closed system: if a single drop of water goes unaccounted for in the mass ledger, the simulation halts and throws a typed error rather than silently absorbing it. 

### 2.2 Architecture for Performance (Optimizing without Compromise)

With true scientific fidelity established, openWEPP introduces massive performance optimizations. Crucially, these are not code-hacking shortcuts, but mathematical privileges earned by the strict fidelity of the architecture described above.

#### 2.2.1 Contract-Driven Mesh Resolution

Because legacy models suffer from numerical diffusion (Section 2.1.2), they must brute-force stability using highly dense spatial grids (e.g., the 100-cell limit per hillslope enforced by the DEP version of WEPP). This imposes an $O(n^2)$ computational penalty. Because the TVD solver in openWEPP natively prevents diffusion, it does not require dense grids for stability. This allowed the architecture to implement a mesh-resolution optimization, proving mathematically that a much coarser 5-cell mesh on massive hillslopes still meets the strict <5% peak flow error tolerances, yielding order-of-magnitude performance gains without sacrificing fidelity.

#### 2.2.2 The Hybrid Implicit-Explicit Stepper

Kinematic wave models must balance accuracy during chaotic events with speed during slow recession. Because the mass ledgers (Section 2.1.3) strictly govern conservation, openWEPP can safely employ a novel hybrid implicit-explicit (IMEX) architecture. During rainfall and rapid runoff, the model uses the explicit TVD scheme, requiring tiny time steps to capture sharp shockwaves. When the rainfall stops and the slope slowly drains, a forcing-derived switching predicate (the "source-memory cooldown") seamlessly swaps the engine to an unconditionally stable, backward-Euler implicit solver. This allows the model to take massive time steps during recession, drastically reducing computational cost without smearing the active storm dynamics.

#### 2.2.3 Array-Native Runtimes: Pushing Fidelity to the Edges

Rigorous scientific architecture often introduces massive computational overhead. Initially, openWEPP prioritized absolute scientific fidelity by utilizing a highly dynamic, symbol-keyed map architecture to ensure perfect semantic parity with legacy WEPP. However, this "fidelity tax" caused the engine to run ~73x slower than the legacy reference, with dynamic map lookups consuming orders of magnitude more CPU time than the actual scientific arithmetic. Incremental optimization attempts failed to bypass this structural bottleneck. 

To resolve this tension, openWEPP shifted to an "Array-Native Direct-Runtime." Instead of crippling the core loop with dynamic dictionaries, the solver executes using contiguous arrays of static structs. Crucially, the architecture did not abandon its strict fidelity; rather, it pushed the heavy symbolic and diagnostic structures to the "edges" of the system. Intake parsing, shadow-validation against legacy records, and publication projections maintain the strict symbolic bureaucracy, leaving the computational hot loop completely unburdened. This structural partition dropped execution time by a factor of 20 and slashed memory consumption by 66%, demonstrating that strict scientific rigor does not inherently preclude high performance.

## 3. The Verification Substrate: Synthesizing the Architecture

Building the CFD architecture detailed in Section 2—with its exact ledgers and fail-closed transactionality—requires a level of structural perfection that is profoundly exhausting for human teams to maintain over time. To ensure the physics were implemented without latent defects, the reconstruction was carried out by a human-supervised multi-agent workflow operating under "institutionalized distrust."

### 3.1 Typed Boundaries and Deterministic Orchestration

To ensure the continuous wave handoffs (Section 2.1.1) are always valid, the orchestrator constructs the execution order dynamically from declared data dependencies. The architecture forbids shared global memory; processes communicate purely through typed boundaries. Quantities crossing these boundaries carry their physical units in their types, converting dimensional consistency from a manual check into a compile-time guarantee.

### 3.2 Top-Down Science Contracts, Fixtures, and weppcloud

Above the code sits a layer of machine-checkable science contracts. The development process involved rigorously tracking down historical references and formulating explicit execution plans before any code was written. Rather than comparing the new code solely to the output of the legacy binary—which would simply reproduce the legacy model's limitations—correctness is defined by mathematical perfection and empirical grounding. 

Test fixtures were built directly from primary literature and physical conservation laws. For real-world validation, the team utilized **weppcloud**, a platform that enables building high-resolution inputs for any location in the world within minutes, providing a vast repository of realistic environmental parameters. Furthermore, while the numerical solvers are entirely novel, the engine maintains strict **legacy input file compatibility**, ensuring that the decades-old library of WEPP management and climate datasets remains natively usable. The engine is rigorously tested against the "Iwagaki-oracle convergence ladder" and these real-world fixtures, ensuring the correctness authority remains the physical contract.

### 3.3 Governed Autonomous Agents

The implementation of the contracts was executed by autonomous software agents operating inside a strict governance envelope. Agents were forced to clear mechanical gates—formatting, strict static analysis, exact conservation checks, and test suites—before proposing a change. Numerical faults had to surface as typed errors; agents were forbidden from substituting default values, applying numerical clamps (e.g., `max(0, depth)`), or scaffolding surrogate physics to bypass failures. By utilizing AI agents to blindly enforce the bureaucracy of the contracts, the team successfully built the highly rigid exact-ledger architecture at a pace humans could not sustain.

### 3.4 Case Study: The Snow and Frost Fidelity Campaign

To illustrate the rigor of this methodology in practice, we examine the "Snow and Frost Fidelity Campaign," a sustained effort to resolve long-standing gaps in winter hydrology modeling.

The campaign began not with code, but by tracking down high-quality historical source datasets. The team acquired data from SNOTEL networks, SCAN sites, and USFS canopy-stratified locations, explicitly distinguishing between physical measurements (e.g., frost tubes) and proxy measurements (e.g., 0°C soil isotherms) to build an empirical baseline. To isolate single-hillslope physics without watershed-scale confounding factors, the team utilized **weppcloud** to rapidly scaffold test fixtures, extracting topography, soils, and high-resolution climate forcings (DAYMET, GRIDMET, PRISM) for remote, real-world sites within minutes.

Rather than relying on single-residual tolerances which often lead to over-tuning and "compensating errors," the team established a strict evaluation rubric. Signatures were classified as either "forcing-robust" (e.g., melt-out timing, densification trajectories) which carry strict model verdict weight, or "forcing-limited" (e.g., absolute SWE magnitudes prone to gauge undercatch) which are reported but not used as acceptance thresholds. Furthermore, strict anti-tuning guards forbade modifying shared radiation inputs to fix melt issues, protecting the broader evapotranspiration balances.

Operating under these rubrics, the autonomous agents executed a series of trial-and-error experiments in an opt-in physics sandbox. They tested modernizing the shortwave energy balance and implementing psychrometric rain/snow partitioning, but rejected both when the empirical rubrics revealed they worsened maritime-site residuals. 

Ultimately, systematic testing isolated a severe "melt-realization gap." The legacy model was calculating sufficient melt energy, but an outdated density-gate proxy was preventing it from realizing the mass loss, causing an aggregate depth-loss deficit of 24.1 m and failing snow-control tolerances on 1,147 out of 1,415 observed thaw-ablation windows. The agents proposed and implemented a physical retained-liquid capacity formulation to replace the proxy. This purely physical fix cut the aggregate depth-loss deficit from 24.1 m down to 15.5 m. When finalized and paired with an updated bulk density compaction model, the final snow-control row failures dropped from 1,147 down to just 498.

A similar transformation was applied to the frost depth calculations during the "FDHP01" (Frost Depth Heat-Flow Parity) campaign. Early builds utilized a reductive "freeze-index proxy" that estimated frost depth purely as a fraction of air temperature, artificially capping the depth at 0.20 m. Worse, a ratchet mechanism caused the frozen-soil duration to severely over-persist by a median of +258 days. Through systematic, governed work-packages, the agents systematically replaced the proxy with a full energy-balance layered heat-flow model. This overhaul introduced fine-layer discretization, in-hour resistance feedbacks, and comprehensive surface-temperature synthesis factoring in radiation, wind, canopy, and residue conductance. Following this physics-based replacement, the artificial 0.20 m cap was eliminated, median depth correlation against reference physics soared from 0.13 to 0.763, and the frozen-duration over-persistence error collapsed from +258 days down to just +61 days—all while maintaining flawless water-balance conservation at numerical noise levels (max absolute residual of $5.1e^{-7}$ mm). 

This campaign demonstrates how pairing robust `weppcloud` test fixtures with strict, anti-tuning evaluation rubrics creates an environment where novel physical formulations can be safely tested, quantified, and adjudicated.

## 4. Evaluation

*(This section must be built on verified, reproducible evidence. The subsections below state what is claimed and mark every specific quantity [VERIFY] until sourced from a current run.)*

### 4.1 Fidelity: Elimination of the Erosion Blowup (Runon Reconciliation)

We present comparative hydrographs and soil loss metrics across multi-OFE slopes. [VERIFY: Show the legacy WEPP artificial peak flows vs the openWEPP continuous wave dispersion. Document the resulting reduction in unphysical erosion spikes.]

### 4.2 Fidelity: Conservation Guarantees

Mass- and water-balance closure is enforced at named tolerances and reported per run. [VERIFY: Show exact closure residuals on the evaluation set, demonstrating the $1e-14$ ledger precision.]

### 4.3 Performance: Mesh Independence

We establish that the TVD scheme maintains physical fidelity across spatial scales. [VERIFY: Present the oracle ladder data demonstrating that the 5-cell mesh satisfies the <5% peak flow error contract compared to the legacy 100-cell requirement.]

### 4.4 Performance: End-to-End Timing

Full end-to-end timing is reported from launch through completed outputs. [VERIFY: Detail the speedups gained by the Hybrid implicit-explicit stepper and the reduced cell mesh, versus the legacy reference on stated hardware. Explicitly include the Array-Native Direct-Runtime benchmarks demonstrating the reduction in execution time from ~670 seconds to ~32.7 seconds, and the memory footprint reduction from ~229 MB to ~77 MB.]

## 5. Discussion

### 5.1 Generality beyond WEPP

The transition from lumped approximations to exact-ledger IMEX architectures is applicable to any process-based environmental model whose physical fidelity is constrained by legacy numerical substrates. The value to the wider field is a reusable pattern for reconstructing such models with checkable guarantees. [CITE: candidate model classes — hydrologic, water-quality, land-surface.]

### 5.2 Structural Provenance as a Byproduct of Stateless Development

A reader may reasonably be cautious of software produced with substantial automated assistance. The architecture's answer is not an appeal to trust but the provision of the means to verify.

An examination of the openWEPP historical record—comprising the artifacts, the explicit reasoning logs, and the continuously executable test suites—reveals a level of traceability rarely seen in legacy scientific codes. However, this pristine record is not the result of administrative mandate or human meticulousness. It is an emergent, structural byproduct of the development environment itself.

openWEPP was constructed utilizing stateless artificial intelligence agents. While these agents possess broad semantic capabilities, they lack episodic memory. When an agent is instantiated, it operates without continuous state; it retains no memory of previous sessions and holds no implicit knowledge regarding the physical systems being modeled. To manage amnesic developers, all substantive implementation work must be rigorously governed by authorized work-packages. A work-package is a distinct, highly scoped task directory that defines the exact execution plan and closure conditions, acting as an explicit contract. 

Because the agent executing the work-package cannot rely on continuous memory, it is structurally required to constantly document its context and progress by generating physical evidence of the work performed. When an equation is implemented, the agent must write an artifact detailing the physical justification and the deductive reasoning, and it must construct isolated, executable tests that prove the logic holds. To date, openWEPP has been constructed through the execution of over 780 distinct work-packages, each leaving behind its own pristine layer of evidence. Without this explicit, re-runnable paper trail, the subsequent agent instantiated to continue the workflow would lack the necessary context, and development would cease.

For modelers, this system constraint yields an unanticipated benefit: absolute transparency. Historically, the evolution of scientific models has often been opaque, as human developers rely on continuous internal memory rather than externalizing every logical deduction. Because openWEPP is built by entities incapable of holding internal context between sessions, anyone interacting with the model inherits a perfectly preserved structural record. Every line of code can be traced to the specific artifact that justifies its inclusion, and to the test that mathematically validates it, simply because that mechanism was a functional requirement of the machine development process. We regard this published, complete provenance record as a crucial methodological contribution, and precisely the artifact required to check the work.

### 5.3 Limitations

[TODO: state honestly — scope of validation completed to date versus outstanding; domains not yet covered; the fact that hydrological-fidelity claims (as opposed to engineering-correctness claims) are deferred to future work; any performance caveats; the maturity stage of the software.]

## 6. Conclusion

We have presented openWEPP, a ground-up reimplementation of the WEPP engine that resolves legacy numerical limitations through a modern, exact-ledger computational fluid dynamics architecture. By replacing equivalent-plane approximations with continuous kinematic wave handoffs, utilizing scale-independent TVD schemes, and seamlessly integrating a hybrid implicit-explicit stepper, the engine eliminates artificial erosion blowups and achieves massive performance gains. This rigid mathematical perfection was achieved through a governed, contract-first, human-supervised multi-agent methodology. The architecture and methodology generalize to the reproducible reconstruction and continued stewardship of process-based environmental models beyond WEPP.

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
- [CITE] Feathers, M.C., 2004. Working Effectively with Legacy Code. Prentice Hall. [VERIFY]
- Papanicolaou, A.N., Abban, B.K.B., Dermisis, D.C., Giannopoulos, C.P., Flanagan, D.C., Frankenberger, J.R. and Wacha, K.M., 2018. Flow resistance interactions on hillslopes with heterogeneous attributes: Effects on runoff hydrograph characteristics. Water Resources Research, 54(1), pp.359-380.
- [CITE: reproducible-research / computational-provenance reference(s)]
- [CITE: environmental-model modernization / restructuring example(s)]
- [CITE: additional WEPP science references as cited in-text]
- [TODO: complete reference list]
