<!--
WORKING DRAFT — openWEPP architecture paper (paper 0001).
Format: Markdown first; convert to Elsevier els-cas LaTeX when stable (see latex-instructions.md).
Target venue: Environmental Modelling & Software (EMS), research article.
Framing (agreed): architecture-primary; AI-native development secondary and disclosed;
audience assumed AI-skeptical; WEPP treated as respected foundation, NOT as "defect-laden";
no generational-ranking narrative; sell thoroughness + auditability, never speed.
[TODO] markers = author must supply/verify before submission. [CITE] = reference needed.
Nothing empirical (numbers, datasets, defect specifics) is asserted here without a verified
source or a [VERIFY] flag — do not let placeholders reach submission.
-->

# A typed, contract-first architecture for reproducible scientific simulation engines, demonstrated by a ground-up reimplementation of WEPP

**Roger Lew**^a^ [TODO: co-authors?]

^a^ University of Idaho, Moscow, ID, USA. [TODO: full postal address; corresponding-author email]

*Corresponding author:* Roger Lew ([TODO: email])

---

## Abstract

*(≤150 words, factual, stand-alone.)*

Long-lived environmental simulation models accumulate implicit couplings, untyped state, and undocumented conventions that make their behaviour hard to reproduce and verify. We present openWEPP, a ground-up reimplementation of the Water Erosion Prediction Project (WEPP) hillslope and watershed engine in which correctness properties that these models have always required — unit consistency, mass and water conservation, deterministic reproducibility, and complete provenance — are enforced by the software architecture rather than left to modeller discipline. The design combines typed state boundaries, top-down machine-checkable science contracts, and a deterministic orchestrator that constructs run order from declared data dependencies. The engine reproduces WEPP's governing science while making its invariants checkable, and its full development history is publicly auditable. We describe the architecture, a governed multi-agent development methodology, and an evaluation covering faithful reproduction, enforced conservation and determinism, and performance. The approach is general to process-based environmental models beyond WEPP.

**Keywords:** erosion; hydrology; scientific-software; reproducibility; verification; contracts; provenance

---

## Highlights

*(3–5 bullets, ≤85 characters each; submitted as a separate file.)*

- Correctness, conservation and determinism enforced by architecture, not convention
- Machine-checkable science contracts sit above the code as correctness authority
- Ground-up Rust reimplementation of WEPP with a fully auditable development history
- Governed multi-agent development with contract-first, evidence-labelled verification
- Approach generalizes to reproducible reconstruction of process-based models

---

## 1. Introduction

Process-based environmental simulation models are long-lived scientific infrastructure. Codes for erosion, hydrology, water quality, and land–atmosphere exchange are developed over decades, outlive their original authors, and are relied upon for research and management decisions. Their scientific formulations are, in general, sound and well documented. The difficulty is rarely the equations; it is the software substrate the equations are expressed in. State shared through global memory, quantities carried as untyped floating-point numbers, run order fixed implicitly by source layout, and numerical faults that propagate silently combine to make the *behaviour* of a mature model difficult to reproduce across platforms and difficult to verify against its own documented intent [CITE: scientific-software reproducibility literature; e.g., legacy-code and computational-reproducibility references].

This paper takes the position that the properties these models most need — dimensional consistency, conservation of mass and water, deterministic and platform-independent results, and a complete, inspectable record of how each result is produced — should be *enforced properties of the engine*, checkable by the machine, rather than conventions entrusted to the modeller's memory and discipline. We present an architecture that makes them so, and we demonstrate it with a working, openly available reimplementation of a widely used model.

The demonstration domain is the Water Erosion Prediction Project (WEPP), a process-based hillslope and watershed erosion model developed by the USDA Agricultural Research Service and applied for three decades across agricultural and forested landscapes [CITE: Flanagan and Nearing (Eds.), 1995, WEPP technical documentation; Flanagan et al., subsequent references]. WEPP is an apt subject precisely because it is mature, trusted, and scientifically substantial: its formulations encode a large body of validated erosion and hydrologic science. Our aim is not to supplant that science but to re-express it in an architecture where the invariants it has always assumed on paper are checked in the code, and where the provenance of every process is open to inspection.

The contribution of this paper is threefold. First, we describe an architecture for scientific simulation engines built on three foundations — typed state boundaries, top-down machine-checkable science contracts, and deterministic dependency-driven orchestration — and we identify the class of latent defect each foundation removes (Section 3). Second, we describe a governed, contract-first development methodology, carried out by a human-supervised multi-agent workflow, that produces the engine without shortcuts and leaves a completely auditable record (Section 4). Third, we evaluate the result: faithful reproduction of WEPP's science where the reference is sound, demonstration of the enforced guarantees (conservation closure, cross-platform determinism, reproducibility), performance, and a small set of illustrative cases in which the contract-and-conservation substrate localizes numerical and conservation edge conditions at the point of computation (Section 5). We argue that the approach generalizes to the reproducible reconstruction and continued maintenance of process-based environmental models more broadly (Section 6).

## 2. Background and related work

### 2.1 The maintenance burden of mature simulation codes

The scientific cores of many environmental models were written in Fortran 77 and its contemporaries, and the engineering practices of that era are load-bearing in ways the source rarely states. State is commonly shared through `COMMON` blocks — global memory any routine may read or write — so a change in one location can silently alter another, and the order in which routines execute becomes significant without being documented. Quantities are represented as untyped real numbers with no dimensional information, six-character names carry meaning only their author fully recalls, and out-of-range reads or numerical faults can flow downstream without surfacing. These are not defects of the science; they are properties of the substrate, and they are the reason mature models are hard to trust in detail even when their aggregate behaviour is well validated [CITE: computational reproducibility / legacy scientific software].

Successive modernizations of environmental codes have improved this substrate incrementally — restructuring monolithic programs into modules, moving to Fortran 90 and later standards, and, more recently, adopting automated tests and continuous integration [CITE: examples of modularized/tested environmental models; e.g., SWAT+, model-restructuring papers]. Each such step was a rational response to the tools available at its time, and each improved maintainability. What these steps generally do not change is what the *compiler guarantees*: shared mutable global state, silent unit and type errors, and untracked ownership of who may write which value survive a language-standard upgrade. The properties that make a model hard to verify are, to a large degree, exactly the ones a syntactic modernization leaves in place.

### 2.2 Reproducibility, verification, and provenance as first-class concerns

A parallel line of work treats reproducibility, verification, and provenance not as post-hoc audits but as first-class properties to be designed in [CITE: reproducible-research / provenance literature]. Our architecture is squarely in this tradition and extends it in three respects: it moves dimensional and conservation checks into the type system and into machine-checkable contracts so that violations are caught at the point of production; it constructs execution order from declared dependencies so that ordering is explicit and validated rather than implicit; and it treats the complete development record — decisions, dead ends, and corrections — as a published artifact rather than a private working history.

### 2.3 Reconstructing a model whose reference carries known limitations

A methodological problem specific to modernizing mature codes is that the legacy program cannot serve as the correctness oracle. Long-lived codes carry documented limitations, regime-specific workarounds, and routines disabled to route around known issues; matching the legacy binary bit-for-bit would reproduce those limitations along with the science [CITE: Feathers, 2004, Working Effectively with Legacy Code; model-limitation documentation]. Our methodology therefore pins legacy behaviour as a *baseline to compare against* rather than as the definition of correct, and elevates an explicit science contract — grounded in the model's technical documentation, the primary literature, and physical conservation laws — to the role of correctness authority. Where the reconstruction and the reference diverge, the contract adjudicates. This posture is what allows the modernization to preserve the validated science while surfacing, rather than inheriting, the substrate-level issues.

## 3. Architecture

openWEPP is a ground-up reimplementation of the WEPP hillslope and watershed engine in Rust. The process formulations are preserved as their governing equations; everything around them — state ownership, orchestration order, input/output, and provenance — is rebuilt so that a set of correctness properties hold by construction. This section describes the design goals and the mechanisms that enforce them.

### 3.1 Design goals: properties enforced by construction

The architecture is organized around four properties, each paired with a class of latent defect it is intended to remove:

1. **Dimensional consistency.** Every quantity that crosses a process boundary carries its unit and physical meaning in its type; an equation that mixes incompatible units fails to compile. This removes the class of error in which a depth is added to a flux, or millimetres are supplied where metres are expected.
2. **Conservation and physical validity.** Mass- and water-balance closure and bound constraints (finiteness, non-negativity, admissible ranges) are checked at defined commit points; a violation is reported with the location and time step that produced it rather than propagating downstream.
3. **Deterministic reproducibility.** Identical inputs produce identical outputs, in a fixed and reproducible order, on any machine — a prerequisite for verification and for scientific reuse.
4. **Provenance and auditability.** The correctness authority, the design decisions, and the complete development history are recorded and inspectable, so that any result can be traced to the contract and code that produced it.

### 3.2 Typed state and unit-safe boundaries

Quantities crossing process boundaries are represented by types that carry their units and identity, and a central registry records the unit for every such quantity so that meaning is never inferred from a truncated name. Dimensional checking that is otherwise performed by hand becomes a compile-time property. The canonical WEPP symbols from the technical documentation are retained as the authoritative names; where an internal name differs it is recorded as an explicit alias rather than replacing the documented symbol, preserving continuity with the model literature. [TODO: name the crates/registries: `openwepp-unit-boundary`, `openwepp-sim-contract` (units, symbols); the symbol-alias and unit-governance registries.]

Ownership of state is tracked by the language: the architecture forbids two routines from holding simultaneous write access to the same value, which converts the aliasing class of defect — change one quantity, silently alter another — into a compile-time error rather than a debugging session.

### 3.3 Contract-first science contracts

Above the code sits a layer of **science contracts**: written, versioned specifications of the governing equations, the state surfaces each process consumes and produces, and the invariants each must satisfy. Contracts are authored top-down from the model's technical references, the primary literature, and physical invariants, and they carry explicit authority anchors distinguishing primary from secondary sources. Each contract states machine-checkable invariants — conservation identities, bound constraints, admissible regimes — that are enforced at runtime and exercised by tests. [TODO: describe the SC-* registry and the INV-* invariant convention; give the count/structure of contracts.]

The correctness authority is the contract, not any binary. A legacy comparison is an investigation signal, not a gold standard, and the same contract gate is applied to every implementation regardless of origin. This inverts the usual trust relationship in model modernization: the specification exists before, and outranks, the code that implements it.

### 3.4 Deterministic orchestration and topology validation

In the legacy design, the order in which routines run is fixed by the order of call statements; nothing records *why* one process must precede another. openWEPP has each process declare the state it requires and the state it produces, and an orchestrator constructs the run order from those declarations, identically on every execution. Before a simulation begins, the full set of declarations is checked for consistency: a missing input or a cyclic dependency halts the run with a diagnostic rather than producing a plausible but incorrect result. The ordering that previously lived in the source layout and in the modeller's memory is thereby made explicit and validated. [TODO: name the orchestrator crates and the topology-validation gate.]

Within a run, no process writes into shared state at will. Each phase computes over typed inputs, its results pass through typed guards (finite, non-negative, in-range, conservation-closed), and only values that survive those guards cross into carry state at a single guarded commit point. A physically impossible value is rejected at the moment it is produced, with the lane and day that produced it, so that the "what overwrote my variable?" class of defect cannot occur. [TODO: reference the day-frame / commit-point design.]

### 3.5 Process model, interchange, and output

The engine follows the established WEPP process pattern of independent hillslopes routed through a watershed. A watershed run orchestrates hillslope executions as subprocesses; inter-binary state crosses the filesystem as typed interchange shards, and outputs are written in a columnar format (parquet) consumable by existing tooling. [TODO: name the three/four executables — single-hillslope CLI, watershed CLI, replay/debug CLI, and the output-aggregation tier; the HBP interchange contract; the parquet schema authority.] A separate replay facility can reopen a completed run's recorded state and re-examine it without re-executing the model — comparing two runs day-by-day to locate a first divergence, isolating a single process, or replaying a window — so that questions previously answered by adding print statements and rerunning are answered by reading a record the run already kept.

### 3.6 Provenance and auditability

Provenance is treated as a first-class output. The correctness authority (the contracts), the architecture decisions (recorded as decision records), the development increments (recorded as work packages), and static audits of the code against the contracts are all versioned in the open repository alongside the code, together with the complete commit history. The result is that the derivation of every process — including where it was genuinely difficult — is open to inspection. We argue in Section 6 that this level of provenance is itself a methodological contribution: it is the artifact a sceptical reader needs in order to check the work, published rather than withheld.

### 3.7 Extensibility

Because processes communicate through typed, contract-governed boundaries rather than shared global state, a new or alternative process formulation is introduced by satisfying the relevant contract at its boundary, not by threading a value through global memory. [TODO: give a concrete example of an alternative-formulation module introduced behind a contract without disturbing the default path — e.g., an opt-in alternative routing formulation — as evidence of extensibility.]

## 4. Development methodology: governed, contract-first, multi-agent

The reconstruction was carried out by a human-supervised multi-agent workflow. We describe it here because it is the reason the provenance is complete and because it bears directly on the credibility of the result; it is a means to the architecture of Section 3, not the contribution itself. The design principle throughout is *institutionalized distrust*: no producer — legacy binary, human, or software agent — is trusted on reputation, and every change is admitted only through fixed gates.

### 4.1 Contract-first sequencing

For process work the order is enforced: amend the science contract, derive tests from the contract, record a pre-implementation gate, and only then write the implementation. The correctness authority exists before the code, so an implementer — human or agent — cannot define its own success criterion. [TODO: state the exact gate sequence and where it is documented.]

### 4.2 Governed autonomy and the verification substrate

Agents operate with autonomy inside a governance envelope, not as an absence of control. Every change must clear mechanical gates — formatting, static analysis with warnings denied, the test suite, dependency-license and advisory policy — together with contract-invariant and conservation/closure checks on the state surfaces it touches. Numerical faults must surface as typed errors for the orchestrator to handle; an agent may not substitute a default value for a violation. Claims in review and audit artifacts are labelled by evidence class — read-and-reasoned versus command-actually-executed — so that the record never overstates what was checked. Findings pass independent adversarial review and are dispositioned with rationale before a change is accepted. A guard on the checks themselves refuses any change that quietly weakens or removes a required comparison. In this structure the software agents are the most-scrutinized contributors in the pipeline, held to the same or stronger distrust than the legacy code, not a lower bar. [TODO: cross-reference the AGENTS/CLAUDE operating guides, the gate scripts, and the anti-evasion guard.]

### 4.3 Empirical and rational grounding

Correctness decisions are anchored to something outside the agents. Test fixtures are grounded in field observations rather than synthetic or model-generated data [TODO: name the observational datasets used as fixtures and their provenance, e.g., long-term experimental watershed records; state licensing/citation]. Formulation constants and equations are traced to primary literature, with secondary-via-carrier citations marked explicitly and un-audited provenance stated as such rather than papered over. Process routines are extracted, implemented, and tested one at a time, with conservation enforced as a per-increment hard stop before integration. Divergences from the legacy reference are adjudicated against physical law and field-observed envelopes — testing a law rather than a single number — never against the legacy binary's output and never against agent confidence. [TODO: give the external-authority mechanism and one worked adjudication.]

### 4.4 Human authority and reliability discipline

The workflow is designed and supervised as a human-reliability system. Stop criteria, escalation points, and the definition of "done" are specified in advance; an agent failure is treated as a system deficiency to be designed against rather than a one-off to be corrected; and the documentation functions as the operator interface to the process. [TODO: connect briefly to the author's human-factors/human-reliability background as the design lineage; keep to one or two sentences.] Ultimate accountability for every result rests with the human authors, consistent with the responsibility that only humans can hold.

## 5. Evaluation

*(This section must be built on verified, reproducible evidence. The subsections below state what is claimed and mark every specific quantity [VERIFY] until sourced from a current run.)*

### 5.1 Faithful reproduction of the reference science

We first establish that openWEPP reproduces WEPP's science where the reference is sound. [TODO: describe the reproduction test set — representative hillslope and watershed fixtures — and the agreement achieved (state metric and tolerance). Lead with agreement, not divergence. VERIFY all figures.]

### 5.2 Enforced guarantees

- **Conservation.** Mass- and water-balance closure is enforced at named tolerances and reported per run. [VERIFY: closure residuals on the evaluation set.]
- **Determinism and reproducibility.** Identical inputs produce byte-identical outputs across repeated runs and across platforms; parallel execution produces the same rows as serial execution. [VERIFY: the determinism/parallel-equivalence evidence.]

### 5.3 Performance

Full end-to-end timing is reported from launch through completed outputs, not from process fan-out alone. [VERIFY: current single-hillslope and watershed timing versus the reference on stated hardware and fixtures. Report thoroughness and reproducibility of the measurement; do not headline speed.]

### 5.4 Illustrative rigor cases

We present a small number of cases in which the contract-and-conservation substrate localizes a numerical or conservation edge condition at the point of computation. [TODO: select 2–3 of the cleanest, most legible cases. Frame each neutrally and technically — what the architecture surfaces and where — attributing to neutral causes (e.g., compiler-evolution-exposed floating-point behaviour, undocumented boundary conditions) rather than to any individual's error. These are illustrations of what the architecture enables, not a defect tally, and must not be framed as an indictment of the reference model. VERIFY each case is reproducible from the public repository.]

## 6. Discussion

### 6.1 Generality beyond WEPP

Nothing in the architecture is specific to erosion. Typed state boundaries, contracts as correctness authority above the code, dependency-driven deterministic orchestration, and published provenance are applicable to any process-based environmental model whose maintenance burden stems from implicit substrate rather than from the science. The value to the wider field is a reusable pattern for reconstructing and then maintaining such models with checkable guarantees. [CITE: candidate model classes — hydrologic, water-quality, land-surface.]

### 6.2 Provenance and auditability as the answer to a sceptical reader

A reader may reasonably be cautious of software produced with substantial automated assistance. The architecture's answer is not an appeal to trust but the provision of the means to verify: the correctness authority, the gates, and the complete decision history are public, and every correctness decision is anchored to a physical invariant, a field observation, or a primary reference rather than to the automated process itself. We regard the published, complete provenance record as the paper's most consequential methodological claim — more transparency than is conventional for operational environmental models, and precisely the artifact required to check the work.

### 6.3 Limitations

[TODO: state honestly — scope of validation completed to date versus outstanding; domains not yet covered; the fact that hydrological-fidelity claims (as opposed to engineering-correctness claims) are deferred to future work; any performance caveats; the maturity stage of the software.]

## 7. Conclusion

We have presented an architecture in which the correctness properties that process-based environmental models require — dimensional consistency, conservation, deterministic reproducibility, and complete provenance — are enforced by construction rather than left to convention, and we have demonstrated it with a ground-up, openly auditable reimplementation of WEPP that reproduces the reference science while making its invariants checkable. The reconstruction was produced by a governed, contract-first, human-supervised multi-agent methodology whose defining principle is that no producer is trusted on reputation. The architecture and methodology generalize to the reproducible reconstruction and continued stewardship of process-based environmental models beyond WEPP.

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

[TODO: per EMS policy. Note this declaration concerns *manuscript preparation*; the use of AI agents in *building the software* is a subject of the paper and is described in Section 4. Draft: "During the preparation of this work the author(s) used [tool/service] in order to [drafting assistance / language]. After using this tool/service, the author(s) reviewed and edited the content as needed and take full responsibility for the content of the published article." AI tools are not listed as authors.]

## Acknowledgements

[TODO: acknowledge the WEPP developers and the body of erosion/hydrologic science the reimplementation builds on — framed as continuity and respect; any individuals who assisted; observational-data providers.]

## References

*(EMS Harvard author–year. All entries below are [VERIFY] — confirm each is real and correctly cited before submission; do not submit with unverified references.)*

- [CITE] Flanagan, D.C., Nearing, M.A. (Eds.), 1995. USDA–Water Erosion Prediction Project: Hillslope Profile and Watershed Model Documentation. NSERL Report No. 10, USDA-ARS National Soil Erosion Research Laboratory, West Lafayette, IN. [VERIFY]
- [CITE] Feathers, M.C., 2004. Working Effectively with Legacy Code. Prentice Hall. [VERIFY]
- [CITE: reproducible-research / computational-provenance reference(s)]
- [CITE: environmental-model modernization / restructuring example(s)]
- [CITE: additional WEPP science references as cited in-text]
- [TODO: complete reference list]
