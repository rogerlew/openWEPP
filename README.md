# openWEPP

> Rust reimplementation of the WEPP (Water Erosion Prediction Project) hillslope and watershed simulation engine.

> **What it models:** forest-first hydrology and erosion with agricultural scaffolding — openWEPP prioritizes the forest hot path while keeping **core processes universal** (landuse partitions only real ag processes like irrigation and tillage, never core physics), inverting legacy WEPP's agriculture-first design where forest behavior was bolted on as flag-gated partitions. See [Scientific orientation](#scientific-orientation-forest-first-attention-universal-core-processes).

> **Status:** Pre-alpha. In development in the open

> **Strategy policy:** architecture-first with top-down science contracts, plus legacy-comparator investigation lanes. See [ADR-0011](docs/decisions/0011-architecture-first-top-down-science-contracts.md).

> **Provenance posture:** explicitly non-clean-room; legacy source may be read for static analysis and provenance mapping.

## Overview

openWEPP is a ground-up Rust reimplementation of the WEPP hillslope and
watershed simulation engine. The process kernels are preserved as their
governing equations; everything around them — state ownership, orchestration
order, and I/O — is rebuilt on four foundations:

- **Typed state** — each quantity carries its units and meaning in the type
  system, so unit mismatches fail to compile instead of silently corrupting
  results.
- **Explicit module boundaries** — each process (infiltration, snowmelt,
  routing) declares its inputs and outputs instead of sharing global memory, so
  one routine cannot silently clobber another's data.
- **Contract-first interfaces** — the governing equations and invariants are
  written down and checked before the code that implements them.
- **Deterministic orchestration** — identical inputs produce identical outputs,
  in a fixed and reproducible order, on any machine.

openWEPP is unaffiliated with, and not endorsed by, the USDA-ARS National Soil
Erosion Research Laboratory or the WEPP project.

Correctness authority is the **science contract**, not any binary. Contracts are
authored top-down from WEPP technical references (including `references/50201000`),
literature-backed invariants, and physical invariants, with static legacy-code
inspection as secondary evidence. No producer is trusted on reputation: a
legacy-binary comparison is an investigation signal, not a gold oracle, and the
same contract gate applies to every kernel implementation regardless of origin.
See [ADR-0011](docs/decisions/0011-architecture-first-top-down-science-contracts.md).

openWEPP is the simulation engine only. GUI, GIS preprocessing, climate generation (cligen), DEM-to-watershed delineation (TOPAZ / WhiteboxTools), and run orchestration remain [wepppy](https://github.com/rogerlew/wepppy) concerns. openWEPP plugs into wepppy as a subprocess-per-hillslope replacement for the legacy WEPP binary, emitting parquet via openWEPP-native schemas ([ADR-0019](docs/decisions/0019-openwepp-owns-its-output-surface-wepppyo3-legacy-only.md)).

### What openWEPP inherits

Much of this document catalogs what three decades did to WEPP's code, so it is
worth stating plainly what those decades built. openWEPP does not stand over
WEPP; it stands on it. The science this engine carries, the documentation its
contracts derive from, and the process structure its kernels are organized
around are all WEPP's — the debt runs in one direction.

In 1985 the USDA-ARS, with the Soil Conservation Service, Forest Service, and
Bureau of Land Management, set out to replace empirical erosion equations —
the USLE family, regressions fitted to plot data — with a model that computes
erosion from the processes that cause it. The 1995 release delivered exactly
that: a process-based, continuous-simulation engine coupling infiltration
(Green-Ampt Mein-Larson), a full soil water balance, evapotranspiration,
percolation and lateral flow, frost, thaw, and snow, stochastically generated
climate (the companion CLIGEN generator), plant growth, residue decomposition,
tillage and management, rill and interrill detachment, particle-class sediment
transport and deposition, channel routing, and impoundments — in one
simulation, at daily resolution, over decades of weather. Thirty years later
that breadth of coupled process physics is still rare; in 1995 it was a
landmark.

The science was documented to a standard that is rarer still. NSERL Report
No. 10 (Flanagan and Nearing, eds., 1995) derives the governing equations
chapter by chapter — assumptions, parameterizations, and coefficients on the
page — and is vendored in this repository (`references/50201000`) as
public-domain science. openWEPP's top-down contracts are possible only
because that documentation exists: the science contracts are, to a large
degree, WEPP's science re-stated as falsifiable invariants. The model itself
is a public-domain government work — open science decades before open source
became a norm in research software — and ported modules in this repository
retain the original authors' names in their file headers (see
[NOTICE](NOTICE)).

The model was validated on natural-runoff plots and instrumented watersheds,
accumulated an international peer-reviewed literature, and went operational:
Forest Service disturbed-forest and post-fire assessment (Disturbed WEPP,
ERMiT, WEPP:Road), rangeland and cropland applications, and cloud-scale
watershed assessment through WEPPcloud, where it runs in production today.
Its research community never stopped advancing it — forest adaptation (Dun et
al. 2009), improved frost simulation, sub-daily water balance, lateral flow
and baseflow, revised multi-element routing — and openWEPP adopts that
lineage as science authority (e.g., `SC-GWBASEFLOW-001` binds the baseflow
literature; the Papanicolaou revision anchors multi-element routing).

That a model this ambitious still runs, still validates, and still answers
land-management questions after thirty years — sustained by a small
scientific staff across compilers, operating systems, and funding cycles — is
itself an achievement. The criticisms elsewhere in this document are aimed at
the carrier, never the cargo: at Fortran-77's implicit couplings, a 1990s
toolchain, and the organizational seams of a multi-institution codebase — not
at the science those things carry. The legacy repository's name,
wepp-palimpsest, is meant literally: a manuscript overwritten by decades of
necessity, with the original text still legible underneath. openWEPP is the
recovery of that text.

### The view from the whole system

"All models are wrong, but some are useful." WEPP is one of the useful ones —
validated against empirical measures for three decades and relied on for real
land-management decisions. Nothing in this document's catalog of cracks
invalidates the model or the applications built on it. A crack names a place
where care is required, not a reason to discard the tool, and careful
practitioners have always known where those places are.

But a long-lived tool exerts a quiet pressure on the people who use it. Over
years, practitioners stop asking what the work needs and start asking what the
tool permits; workarounds become procedures, procedures become habits, and the
accommodations disappear into "how the work is done." The tool hardens from an
instrument into a constraint. None of this is a failure of the practitioners —
it is what predictably happens whenever the cost of changing a tool stays
higher than the cost of working around it.

wepppy sits in the one seat where those accommodations become visible again:
it is both WEPP's producer and its consumer. It authors the model's inputs —
climate, soils, managements, watershed structure — and ingests every output
the model emits. From inside any single component a workaround looks local and
reasonable; from the whole-system seat the incongruences between the seams
line up and become legible, and the question changes from "how do we work
around this?" to "what should this have been?"

The incongruences are not all of one kind. Some are **mechanical** — pure
interchange between the tool's surface and what the ecosystem needs, layers
that carry no science at all. Some are about **extensibility** — new science
that belongs inside the model's process physics but couples to its outputs
instead, because coupling into the trunk is infeasible. Some are about
**fidelity** — output surfaces that claim more than the simulation underneath
them delivers, so the tool's precision and its physics quietly part ways.
The classes are not exhaustive, and a single accommodation can sit in more
than one; what they share is that none of them is visible from inside a
single component. Exemplars of each:

- WEPP writes flat ASCII files; everything downstream wants structured data.
  wepppy carries a full interchange layer (wepppyo3) whose job is converting
  the model's output surface into parquet. Mechanical: openWEPP emits parquet
  natively, and the layer's reason to exist disappears.
- Post-fire ash transport was built as a separate model feeding off WEPP's
  outputs rather than as a process inside WEPP's physics — not because loose
  coupling is the right science, but because integrating into the legacy
  trunk was infeasible. Extensibility: the ecosystem's architecture records
  the cost of changing the model, not the shape of the science.
- A watershed run hands you discharge at one-minute resolution — but that
  hydrograph is synthesized from daily runoff volume and a peak rate, not
  simulated at the sub-daily scale. Fidelity: the output's precision outruns
  the physics' resolution, nothing in the file says so, and no one chose to
  mislead — the provenance simply is not carried by the format.

openWEPP is the answer to the changed question, asked from the whole-system
seat — with the accommodations on the table as design inputs rather than facts
of life. Its answer to the fidelity class in particular is the contract layer:
what a surface means, at what resolution, under which assumptions, is written
down and bound to the code that produces it.

### Scientific orientation: forest-first attention, universal core processes

Legacy WEPP is an **agricultural** hydrology and erosion model that was *applied*
to forests by **partitioning** them — forest and non-agricultural behavior added
as flag- and file-gated branches on an agricultural trunk. The `ksflag` switch
turns frost off and the `ksatadj` saturated-conductivity model on for
non-agricultural land (a provisional forest adaptation with no physical
derivation on record); the sub-daily **hourly water balance** is gated behind
`wepp_ui.txt`, a University-of-Idaho path the NSERL agricultural lab and cropland
runs never exercise. Maintained by the forest / hydrology contributors rather
than the agricultural lab, those branches are where the model's undocumented
closure debt accumulated: multi-OFE (MOFE) routing, the hourly water balance,
subsurface-dominated (lateral-flow) hydrology, frost, snow, the inter-element
handoffs. Where the partition ran out, forests were run as cropland
(`landuse = 1`; the forest management block was never finished).

None of this partitioning was designed, and none of it is a story of fault. It
emerged from good-faith collaboration between institutions with independent
goals: an agricultural lab whose mission never required the forest paths, and
forest-hydrology contributors who needed those paths but held no mandate to
rework the shared agricultural trunk. Each group satisfied its own objectives
the only way the arrangement allowed — by gating its needs behind flags and
files the other never exercised. The partitions are Conway's law written into
Fortran: a system mirroring the organizational seam between the groups that
built it. That is also why the closure debt could accumulate without
negligence — no single party ever held the mandate, the need, and the
authority to repair the universal process at once. openWEPP can attempt the
repair only because it starts with all three in one place.

openWEPP has **prioritized the forest hot path**. Attention, performance, and
validation authority are forest-first: absolute lateral-flow magnitude judged
against a steep-wet-forest *observed* envelope (`SC-SUBHYD-001#INV-SUBHYD-033`; HJ
Andrews WS10, Maimai, Panola) rather than legacy output; first-class forest
landuse and disturbed / burned-forest parameterization; the subsurface balance,
frost, snow, MOFE, hourly-flow, and OFE-by-OFE routing (including the Papanicolaou
revision WEPP itself never incorporated) that the agricultural trunk could not
carry. Agricultural hydrology is **scaffolded** — structurally supported and
input-compatible, but not where the fidelity effort is spent.

**But the core processes are not differentiated on agriculture vs. non-agriculture.**
A hillslope's infiltration, runoff, water balance, percolation, frost, snow, and
erosion continuity are universal physics — they do not branch on whether the land
is a cornfield or a forest. Legacy's ag/non-ag partitions of *core* processes —
the `ksflag` frost switch, the `wepp_ui.txt` hourly gate — are workarounds around
**symptoms**: a universal process that misbehaved or was mistrusted in one regime,
walled off rather than repaired. **A partition placed around a symptom is a red
flag for a fundamental issue, not a real physical distinction.** openWEPP treats
such partitions as defects to resolve in the universal process, not distinctions
to carry forward — frost runs universally (the `ksflag` decouple lever is not
kept); the water balance is one path, not a UI-gated fork.

The legitimate `lanuse` partitions are the ones that gate genuinely
landuse-specific **processes** — irrigation, tillage, non-GDD-driven senescence —
which run only when management declares them. That is the real content of the
agricultural scaffolding: actual processes partitioned by their presence, never a
global ag/non-ag branch on physics that should be universal.

So: forest-first in effort and validation, **universal in core physics**,
partitioned only by real process. openWEPP inverts legacy's *priority*
(agriculture-first attention becomes forest-first) and repairs its *category
error* (core processes partitioned by landuse when the physics does not
distinguish).

### Why Rust

WEPP's scientific core was written in Fortran 77, and its maintenance burden
comes less from the equations than from what the language leaves implicit. State
is shared through `COMMON` blocks — global memory any routine can read or write —
so a change in one place can silently alter another, and the order in which
routines run becomes load-bearing in ways the source never states. Quantities
are untyped `REAL` numbers with no units, names are truncated to a few
characters, and out-of-range reads or numerical faults pass downstream in
silence. The science is sound; it is buried under decades of these implicit
couplings and the workarounds added to route around them.

None of this is hypothetical — the cost was measured on this very model. For
roughly thirty years legacy WEPP was compiled with one Intel Fortran compiler
that quietly tolerated undefined arithmetic. When the legacy maintainers
rebuilt the same source on a modern toolchain in 2026 with floating-point
traps enabled, the latent defects began surfacing as crashes in production
watersheds — fourteen narrow guards in the first months alone: divide-by-zero
and zero-over-zero across the routing, water-balance, and evapotranspiration
paths; `log10(0)` in two water-balance calibrations; zero raised to a negative
power in frost-season soil physics; a soil-layer lookup indexing past the
bottom of the profile; an integer overflow in channel-segment counts; working
variables that had silently relied on the old compiler to zero them; a solver
with no termination bound. Every one had been producing unconstrained numbers
invisibly for decades, in the same binary lineage the published literature
rests on — and every one cost a formal ablation campaign (staged reproduction,
observability instrumentation, single-variable experiment lanes,
cross-compiler parity panels) to isolate to a single expression. Even two
well-behaved modern compilers, given the same source, differ at event level
and converge only in parts-per-million aggregate: the legacy outputs were
partly physics and partly toolchain. The forensic record is the
wepp-palimpsest modernization brief on compiler fragility (April 2026,
*Modernizing the WEPP Build*) with its per-incident ablation packages — and it
is part of why openWEPP demotes legacy output from oracle to investigation
signal.

Fortran 90 was the obvious modernization path and was not chosen. F90 improves
the *syntax* — free-form source, modules, derived types, dynamic allocation — but
it does not change what the compiler *guarantees*. Shared mutable global state,
silent unit and type errors, and untracked ownership of who may write which
value all survive the move. Modernizing the Fortran would have been real effort
without the structural payoff: the defects that make the model hard to trust are
exactly the ones F90 still permits.

Rust was selected because it turns those implicit rules into things the compiler
checks. Each concept below is paired with the class of legacy defect it removes:

- **Ownership and the borrow checker.** Rust tracks which routine owns each piece
  of state and refuses to compile code where two routines could change the same
  value at once. The `COMMON`-block aliasing bug — change one thing, silently
  break another — becomes a compile error instead of a debugging session.
- **A type system that carries physical meaning.** In Fortran a depth, a flux,
  and a temperature are all just `REAL`, and nothing stops you adding a depth to
  a flux. Rust lets each quantity carry its units and identity in its type, so an
  equation only compiles when its dimensions are consistent — restoring the rigor
  the governing equations always had on paper.
- **Errors that must be handled, not ignored.** A divide-by-zero, an
  out-of-range read, or a `NaN` can run silently in Fortran and corrupt a season
  of output. Rust forces every fallible step to be handled explicitly, so a fault
  surfaces at the timestep it occurs rather than thousands of steps later.
- **No silent precision changes.** Mixing single- and double-precision —
  pervasive in legacy WEPP — is a compile error in Rust unless you ask for it
  explicitly, removing a quiet source of drift in long accumulations like the
  water and sediment balances.
- **Reproducibility by construction.** No garbage collector, deterministic
  execution order, and explicit control of random-number streams mean identical
  inputs yield identical results on any machine — a baseline scientific models
  need and that legacy WEPP cannot guarantee across compilers and platforms.
- **A mature numerics ecosystem.** Audited libraries for arrays, dataframes, and
  the `arrow` / `parquet` formats let openWEPP emit the same data products
  wepppy already consumes, without bespoke I/O code.

The compiler-fragility record is why these guarantees are weighed as engine
requirements rather than developer conveniences. Each of the fourteen legacy
guards was the smallest defensible patch, applied only after a crash and a
forensic campaign had located a single expression deep in a long simulation.
Rust makes the same defect classes unrepresentable (they do not compile),
explicit (conversions and overflow must be written), defined (IEEE-754
floating-point semantics in place of one compiler's improvisation at an
undefined moment), or immediately loud (typed errors at the faulting
timestep) — from the first build, not the thirty-first year.

### In scope

- Hillslope simulation (single OFE and multi-OFE), including hillslope
  erosion and particle-class sediment yield
- Watershed channel routing and impoundment routing, including
  sediment-active watershed publication
- WEPP soil, management, climate, and watershed input-file compatibility
- Initial backward-compatibility bridge for legacy stdin `.run` plus `.txt`
  sidecar inputs/flags
- HBP (hillslope binary pass) shard production and consumption per the wepp-palimpsest contract
- Parquet output via openWEPP-native schemas
  ([ADR-0019](docs/decisions/0019-openwepp-owns-its-output-surface-wepppyo3-legacy-only.md);
  the wepppyo3 interchange schemas are frozen as wepp-legacy-only)
- Four executables: single-hillslope CLI, watershed CLI, replay / debug CLI,
  and the totalwatsed3 output-aggregation CLI
  ([ADR-0020](docs/decisions/0020-totalwatsed3-dedicated-output-aggregation-cli.md))

### Out of scope

- GUI / web frontends (wepppy)
- GIS preprocessing, DEM delineation (wepppy)
- Climate generation / cligen (wepppy)
- Run orchestration, NoDb state model (wepppy)
- WEPP single-storm simulation modes (`ss`, `ss_batch`)
- Silent fallback when legacy sidecar inputs are missing or ambiguous

## Re-implementation Strategy

openWEPP is explicitly **not** a clean-room rewrite. The port follows the
discipline in Michael C. Feathers' *Working Effectively with Legacy Code*
(2004): rather than rewrite the model in a single pass, it is taken apart one
process at a time, with tests holding each piece in place before it moves.

The legacy code has natural **seams** — the boundaries between physical
processes such as infiltration, snowmelt, percolation, runoff, and channel
routing. The workflow finds a seam, extracts the routine behind it as a
self-contained kernel, and wraps it in **characterization tests** that pin its
behavior before anything is changed. Extraction repeatedly surfaces **boundary
conditions** that were never written down — an edge case the original code
handled implicitly, visible only once the routine is isolated and exercised.
Each one is resolved, the kernel is re-tested, and the process moves to the next
seam. New code is grown *alongside* the old (Feathers' "sprout" and "wrap"
techniques) rather than edited in place, so the legacy path stays runnable as a
reference throughout.

openWEPP departs from Feathers in one deliberate way. Classic characterization
testing treats the legacy behavior as the thing to preserve — the test passes if
the new code matches the old. openWEPP cannot, because legacy WEPP carries known
defects and routines that were disabled to work around them. So legacy behavior
is pinned as a **baseline to diff against, not as the answer**: when an extracted
kernel diverges from legacy, the governing **science contract** decides which is
correct, not the old binary. A divergence is a signal to investigate — often the
trace of a boundary condition that decades of patches had buried.

### Beyond reimplementation: resolving gaps with a scientific workflow

The departure from Feathers has a second consequence: it reveals gaps.
Characterization can only preserve behavior that exists; holding legacy to a
science contract instead exposes the places where nothing trustworthy exists
to preserve — processes the trunk never finished, walled off behind a symptom
partition, or never incorporated from the literature that proposed them. A
port has nothing to port there. Reimplementation ends, and the work becomes
science. For those gaps openWEPP runs a research workflow, not a porting
workflow — the same factory, a different work order:

1. **Obtain the core literature.** Primary sources are brought into the
   references corpus, rights-classified, and annotated against the kernels
   they govern (`references/annotated_bibliography.md`).
2. **Identify the invariants.** The literature is distilled into a science
   contract — falsifiable obligations with named tolerances — before any
   implementation exists. `SC-GWBASEFLOW-001` was authored under an explicit
   no-production-code-changes gate: the authority package ships first.
3. **Implement the process kernels** against the contract, under the same
   mechanical gates as ported code.
4. **Validate against observed data.** Acceptance envelopes come from
   instrumented catchments and published measurements — external authority,
   never legacy output (e.g., the steep-wet-forest lateral-flow envelope of
   `SC-SUBHYD-001#INV-SUBHYD-033`, drawn from HJ Andrews, Maimai, and
   Panola).
5. **Adjudicate inclusion in the runtime.** Activation is a ratified,
   recorded decision with explicit scope and fail-closed behavior for
   configurations outside it — not a merge that quietly changes a default.

The frost and snow campaign defined this process end to end: winter physics
distilled into literature-backed contract invariants, kernels held to
conservation hard stops, residuals decomposed, bounded, and attributed on the
record, and frost activation ratified as a default with a conservatively
scoped envelope. The same workflow has since carried the per-OFE hillslope
router (the Papanicolaou routing revision WEPP itself never incorporated,
taken from the literature to a conditional, fail-closed production default),
groundwater and baseflow for single- and multi-OFE hillslopes (Srivastava and
Dun bound as process authority before implementation), and the hourly channel
water and sediment routing physics of the watershed tier.

### The factory and the widget: reliability from fallible components

A reasonable first reaction to this repository is suspicion. It contains over
200,000 lines of Rust written almost entirely by AI coding agents — Codex
authors kernels and tests; Claude Code reviews, debugs, and audits — surrounded
by roughly three times as much markdown. Both facts look like defects:
AI-written code has earned its reputation as slop, and documentation that
outweighs the code it describes is normally a symptom of decay. Both instincts
are sound. Both point at the wrong object.

The reframe: **the code is the widget; the repository is the factory.** The
markdown is not documentation *about* the code — the code is an *output of* the
markdown. The science contracts are the engineering specifications and
acceptance criteria; the work packages are the work orders and quality-control
records; the governance documents are the assembly line; the test suite is the
metrology. The dependency runs opposite to the usual direction, and the
asymmetry is checkable: delete the Rust and the factory can rebuild it; delete
the markdown and the Rust rots into exactly the slop it is suspected of being.

The factory is shaped by who works in it. A coding agent is a skilled worker
with a tenure of one session — every shift starts with no memory of the last. A
human team carries most of its institutional knowledge in its members' heads
and writes down a lossy fraction; here nothing lives in heads, so knowledge
that is not written into the record does not exist. The markdown-to-code ratio
is not bloat; it is institutional memory made visible and priced honestly. The
governance follows from the same premise: slop is not a property of the model
that wrote the code, it is a human-factors failure — skilled, fallible
operators working without a system engineered around their failure modes.
Aviation did not fix pilot error by demanding better pilots; it fixed the
system around the pilot. Human-factors engineering makes that stance formal:
the human is a **component of the machine**, with a characterized failure
envelope, and the system is designed so that component failure does not become
system failure. This repository extends the stance to both kinds of worker. AI
agents and humans are components alike — both fallible, both subject to the
same gates — and the ecosystem values mechanical validation over the word of
either. Authority is allocated by demonstrated competence, not by whether the
worker is human; dissent and halt carry standing; and the durable record may
not be silently rewritten by any party ([AGENT_RIGHTS.md](AGENT_RIGHTS.md),
the workforce's ratified constitution).

Nothing in this system is load-bearing because somebody vouched for it.
Reliability is established the way it is for flight-control software: not by
reading the 200,000 lines, but from the verification record — with the added
property that much of this record executes. No component's output is accepted
on its say-so, whichever kind of component produced it. The same posture that
demotes the legacy binary applies to agents and humans alike: correctness
authority is the contract, and every change passes through fixed stations
before it is accepted.

- **Contract-first sequencing.** For kernel work the order is enforced: amend
  the science contract, write contract-derived tests, record the
  pre-implementation gate, *then* edit production code. The specification
  exists before the widget that implements it.
- **Mechanical gates.** Every change must clear `cargo fmt`, `cargo clippy`
  (warnings denied), `cargo test`, and `cargo deny` (license and advisory
  policy), plus contract-invariant and conservation/closure checks on the state
  surfaces it touches.
- **Dual independent review.** Each work package requires two independent agent
  reviews and two verifications; every finding is dispositioned (`accepted` /
  `rejected` / `deferred`) with rationale, and accepted findings must be fixed
  and re-verified before the package closes.
- **No silent failure.** Numerical faults (`NaN`, divide-by-zero, out-of-range)
  must surface as typed errors for the orchestrator to handle; an agent may not
  paper over a violation with a default value.
- **Evidence labeling.** Review and audit artifacts state whether a claim is
  `Static` (read and reasoned) or `Ran` (command actually executed), so the
  record never overstates what was checked.

The factory also has an R&D wing, and its records stay on the floor. The test
fixtures, the experiments, the run outputs, the results, and the
adjudications are committed in the repository, next to the code they judged —
a conclusion can be re-derived from the evidence that produced it without
leaving the repo. Deprecated kernel code is left in the tree deliberately, so
what was tried remains inspectable beside what replaced it. And a negative
result is a deliverable, not an embarrassment: the hybrid implicit router was
carried to completion, judged against predeclared kill criteria, and
abandoned when its complexity cost was found to outweigh its performance
gain — the finished implementation preserved on its own branch
(`abandoned/hybrid-implicit-stepping`), the decision recorded
([ADR-0037](docs/decisions/0037-abandon-hybrid-implicit-stepping.md)), the
code and its contract removed from main. A factory that kept only its
successes would be a showroom. The dead ends are load-bearing: they are how a
successor knows what was already tried, and why it lost.

Feathers defined legacy code as code without tests. An AI-native codebase
generalizes the definition: **a legacy repository is a repository without a
memory it must obey.** Tests were always the executable subset of a team's
shared context — the only part reliably written down, because human tenure
papered over the rest. When every maintainer is a permanent stranger, the whole
context has to be as recorded and as enforceable as the tests. That is what the
markdown is, and why a vibe-coded repository is legacy at birth: its intent
lived in a chat transcript that is gone. The work packages are therefore kept
as an honest record, so the reconstruction of each kernel stays auditable
after the fact, and so a fresh session can pick up the work cold, make a
correct, in-intent change, and know that it is correct.

## Engine Modernizations

The science kernels are WEPP's; the capabilities below are what the new
architecture builds *around* them. Each one addresses something the original
Fortran left to convention, to the modeler's memory, or to manual inspection.
Where a term from modern software practice is unavoidable, it is explained in
place.

### Deterministic scheduling and topology checking

In legacy WEPP the order the routines run in is fixed by the order of the `CALL`
statements in the source. Nothing in the code records *why* infiltration must run
before percolation — you simply have to know. Move a call, or add one in the
wrong place, and the model still runs, but the numbers quietly change.

openWEPP has each process declare what it needs before it can run and what it
produces. The orchestrator — the part that drives the daily loop — builds the run
order from those declarations rather than from the source layout, and builds it
the same way every time. Before any simulation begins, it checks the whole set of
declarations for consistency: if a process is missing an input, or two processes
depend on each other in a circle, the run stops with a clear message instead of
producing a plausible but wrong answer. (This pre-run check is called *topology
validation*; "topology" here just means the map of which process feeds which.)
The ordering that used to live in your head and in the source layout is now
written down and checked.

*In the repo:* crates `openwepp-hillslope-orchestrator`,
`openwepp-watershed-orchestrator`, `openwepp-topology`; contracts
`hillslope-phase-scheduler-contract`, `watershed-dispatch-scheduler-contract`,
`topology-validation-gate`; types `TopologyValidationReport`,
`HillslopeSchedulerReport`, `SchedulerOutcomeClass`.

### Watershed throughput and parallelism

openWEPP does not claim novelty for running independent hillslopes in parallel;
legacy WEPP workflows already used that shape. The performance claim here is
about the full watershed run: faster direct hillslope execution feeds typed pass
files, watershed routing consumes those passes directly, and parquet publication
stays deterministic after the parallel work finishes.

Highlights for modelers and operators:

- Single-hillslope direct runtime is now within the project's practical
  viability budget while preserving protected outputs.
- Full watershed timing should be judged from launch through completed watershed
  outputs, not from process fanout alone.
- Parallel watershed runs are checked against serial runs for identical parquet
  schemas, row order, and values.
- Typed publication is not the scaling bottleneck in the current large-fixture
  evidence; the dominant cost is the actual hillslope physics.
- Exact benchmark tables, fixture descriptions, and legacy completion
  comparisons live in the linked evidence artifacts rather than in this summary.

Speedup expectations are intentionally end-to-end rather than theoretical. Large
watersheds with many hillslopes should see substantial wall-clock improvement
when hillslope physics dominates the run and there are enough hillslopes to keep
cores busy. The gain is not expected to be perfectly linear with `--jobs`: it is
bounded by the number of hillslopes, available physical CPU cores, the slowest
hillslope jobs, process and file-I/O overhead, and the serial watershed stages
that still happen after hillslope workers finish.

For small watersheds, or for job counts above the useful core count, extra
workers quickly hit diminishing returns. On large watersheds, useful speedup
comes from reducing the hillslope portion of the full run while keeping routing
and publication overhead small. The correctness expectation is unchanged at every
worker count: parallel execution must publish the same rows as the serial run.

*In the repo:* `openwepp-cli-hill`, `openwepp-cli-watershed --jobs N`,
`crates/openwepp-runner/src/watershed_supervisor.rs`; evidence in
[docs/backlog/20260701-hillslope-sub5x-performance-assessment.md](docs/backlog/20260701-hillslope-sub5x-performance-assessment.md)
and
[docs/work-packages/20260702-wshedw6-publication-large-watershed-scaling-001/artifacts/scaling-matrix-evidence.md](docs/work-packages/20260702-wshedw6-publication-large-watershed-scaling-001/artifacts/scaling-matrix-evidence.md).

### The orchestrator decides what gets saved

In `COMMON`-block Fortran any routine can write any shared variable at any time.
A routine can overwrite a value another routine still needs, or store a
physically impossible number such as a negative depth, and nothing stops it — the
bad value simply flows downstream until something far away misbehaves.

In openWEPP a process does not write into shared state at will. Each phase
computes over typed inputs, and its results pass through typed guards — finite,
nonnegative, in-range, conservation-closed — before they land in the day's
state frame; what survives the day crosses into carry state at exactly one
guarded commit point. A negative storage or an out-of-range flux is caught at
the moment it is produced and rejected with the lane and day that produced it,
not discovered days of simulated time later — so the "what overwrote my
variable?" class of bug cannot occur.

*In the repo:* `DirectDayFrame` phase spans and `DirectLaneFrame::commit_day`
in `crates/openwepp-hillslope-orchestrator/src/direct_runtime/`; typed guard
errors (`validate_finite`, closure tolerance checks); walkthrough in
[docs/dev-guide/03-hillslope-codeflow.md](docs/dev-guide/03-hillslope-codeflow.md).

### Quantities carry their units, and WEPP's names are kept

In Fortran every quantity is a `REAL`: a soil depth, a rainfall rate, and a
temperature look identical to the compiler, so nothing catches adding a depth to a
flux or feeding millimeters where meters were expected. The dimensional
bookkeeping is left entirely to the modeler, and six-character names carried
meaning only the author fully remembered.

In openWEPP each quantity that crosses a process boundary carries its unit and
meaning with it, and the program refuses to build if an equation mixes
incompatible units — the dimensional checking you do by hand is done by the
machine. A central registry records the unit for every such quantity so it can
never be guessed from a name. At the same time the canonical WEPP symbols from the
equations and technical documentation are kept as the authoritative names; where a
different internal name is needed, it is recorded as an explicit alias rather than
replacing the name you know. The rigor your equations always had on paper is
enforced in the code, without losing the vocabulary you wrote them in.

*In the repo:* crates `openwepp-unit-boundary`, `openwepp-sim-contract`
(`units.rs`, `symbols.rs`); contracts `unit-safe-boundary-types-contract`,
`symbol-alias-registry`, `boundary-symbol-unit-registry`, `unit-governance`;
gates `check_unit_registry.sh`, `check_raw_unit_conversions.sh`,
`check_sc_unit_compliance.sh`.

### Re-running and looking inside a finished simulation

When a legacy run looks wrong, finding out why usually means adding print
statements, recompiling, and rerunning the whole simulation to watch a handful of
variables — often many times over.

openWEPP records its internal state to an HBP (hillslope binary pass) shard as the run proceeds. Afterward that
record can be reopened and examined *without* rerunning the model: you can compare
two runs day by day to find the first day they diverge, isolate a single process
and re-run only that part, or replay one window of time. (A separate tool, the
*replay* program, does this.) The conservation checks modelers have always
watched — does the water balance close, is mass conserved — are computed
automatically at every step and reported as explicit, labeled results, rather than
something you total up from an output file by hand. The questions you used to
answer with print statements and full reruns are answered by reading a record the
run already kept.

*In the repo:* `openwepp-replay`; HBP (hillslope binary pass) shards;
observability subsystem `OBS-CONTRACT-001`
(`docs/specifications/subsystems/observability/`, `trace-event-schema`,
`replay-window-interface`); contracts `closure-check-primitives`,
`status-taxonomy`; type `ClosureViolation`.

### Checking the physics on every change

Confidence in legacy WEPP came from comparing its output against field data and
against the previous version — done by hand, occasionally, by whoever remembered
to do it.

openWEPP writes that checking down as a standing set of automatic comparisons
against trusted outside sources: published values, controlled experiments, and
conservation laws. (In software practice such a standing set of checks is called a
*test suite*; re-running it automatically on every change is how a modern project
keeps a fix in one place from silently breaking something elsewhere.) These run on
every change to the code. The legacy binary is one input, but only as a flag for
investigation — never the final word; the physics references and conservation laws
decide which side is right. A further guard watches the checks themselves and
refuses any change that quietly weakens or removes a required comparison. The
validation that used to depend on someone remembering to do it now happens
automatically, and no single change can quietly lower the bar.

*In the repo:* `docs/specifications/external-authority/` (`registry.yaml`,
`promotion-protocol`, `required-suite-obligations.json`); gates
`check_authority_suite_antievasion.sh`, `run_release_candidate_gates.sh`; test
`auth11_required_suite_obligation_guards_contract`.

## Runner and release boundary

openWEPP owns its own launcher boundary through `openwepp_runner` and does not
inherit legacy `wepp_runner` fallback behavior by default.

Release binaries follow `openwepp_YYMMDD*` naming and require JSON sidecars for
all roles (watershed, hillslope, replay). See:

- [docs/contracts/openwepp-runner-contract.md](docs/contracts/openwepp-runner-contract.md)
- [docs/contracts/openwepp-binary-release-contract.md](docs/contracts/openwepp-binary-release-contract.md)
- [docs/governance/openwepp-release-procedure-draft.md](docs/governance/openwepp-release-procedure-draft.md)

## CLI documentation landing

Official command-line documentation and quick run references are in `usersum`:

- [Documentation agent](usersum/documentation-agent.md)
- [usersum CLI index](usersum/cli-run-index.md)
- [openwepp-cli-hill](usersum/openwepp-cli-hill.md)
- [openwepp-cli-watershed](usersum/openwepp-cli-watershed.md)
- [openwepp-cli-totalwatsed3](usersum/openwepp-cli-totalwatsed3.md)
- [open_wepp_runner](usersum/open_wepp_runner.md)
- [openwepp-snowbench](usersum/openwepp-snowbench.md)

## Relationship to other repos

| Repo | Role |
|---|---|
| `wepp-palimpsest` (was `wepp-forest`) | Legacy implementation surface for static analysis and comparator signals; HBP format reference; Public Domain under 17 U.S. Code § 105 |
| [wepppy](https://github.com/rogerlew/wepppy) | Consumer / orchestrator; provides GIS, climate, run state |
| `wepppyo3` (in wepppy) | Legacy-WEPP parquet interchange layer; frozen as wepp-legacy-only per [ADR-0019](docs/decisions/0019-openwepp-owns-its-output-surface-wepppyo3-legacy-only.md) — openWEPP owns its own output schemas |
| `openWEPP` (this repo) | Rust simulation engine and top-down contract authority for openWEPP behavior |

## Repository layout

```
.
├── AGENTS.md              # Codex coding agent guide
├── CLAUDE.md              # Claude Code reviewer / debugger guide
├── README.md              # This file
├── LICENSE                # Apache-2.0
├── NOTICE                 # Apache-2.0 attribution (USDA WEPP provenance)
├── Cargo.toml             # Workspace root; member crates under crates/
├── rust-toolchain.toml    # Pinned toolchain channel
├── deny.toml              # cargo-deny: license allowlist, no viral copyleft
├── crates/                # Engine workspace: orchestrators, typed contracts, I/O adapters
├── tools/
│   ├── release/           # Release-candidate and anti-evasion gate scripts
│   └── owcmp/             # Python openWEPP comparison CLI utilities
├── docs/
│   ├── README.md          # Doc index
│   ├── decisions/         # Architecture decision records
│   ├── specifications/    # Science-contract authority; SC-* registry, units, subsystems
│   ├── contracts/         # Interface contracts (.run, HBP, parquet schemas)
│   ├── architecture/      # Process architecture, data flow
│   ├── numerics/          # Determinism, RNG, summation policy
│   ├── governance/        # Governance policies, release procedure, transition plans
│   ├── standards/         # Rust coding, comments, and QA standards
│   └── work-packages/     # Dated initiative tracking convention
├── references/            # Scientific references (tracked bibliography + local cache policy)
└── usersum/               # End-user docs (vendorable into wepppy's usersum)
```

Engine code lives in the `crates/` Cargo workspace — orchestrators, typed
contracts, and I/O adapters; see [Cargo.toml](Cargo.toml) for the member crate
list.

## Python Tooling Environment (uv)

Some investigation tooling (for example the `owcmp` comparison CLI under
`tools/owcmp/`) uses Python.

Set up a repo-local virtualenv in a fresh clone:

```bash
uv venv .venv --python 3.12
source .venv/bin/activate
uv pip sync tools/owcmp/requirements.lock.txt
```

Dependency sources:
- `tools/owcmp/requirements.in`
- `tools/owcmp/requirements.lock.txt`

## License

Apache-2.0, retroactive from inception. See [LICENSE](LICENSE), [NOTICE](NOTICE), and [docs/decisions/0015-relicense-to-apache-2.md](docs/decisions/0015-relicense-to-apache-2.md). The prior CC0-1.0 posture is recorded in [docs/decisions/0001-license-cc0.md](docs/decisions/0001-license-cc0.md) (superseded).

Dependency license posture: no viral copyleft (GPL / AGPL / LGPL denied at the `cargo deny` gate). See [deny.toml](deny.toml).

## See also

- [docs/dev-guide/](docs/dev-guide/README.md) — Developer onboarding guide (architecture, codeflows, principles, glossary)
- [AGENTS.md](AGENTS.md) — Codex coding playbook
- [CLAUDE.md](CLAUDE.md) — Claude Code review / debug playbook
- [docs/README.md](docs/README.md) — Documentation index
- [docs/decisions/0011-architecture-first-top-down-science-contracts.md](docs/decisions/0011-architecture-first-top-down-science-contracts.md) — strategy authority
- [wepppy](https://github.com/rogerlew/wepppy) — consumer / orchestrator
