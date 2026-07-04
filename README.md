# openWEPP

> Rust reimplementation of the WEPP (Water Erosion Prediction Project) hillslope and watershed simulation engine.

> **What it models:** a **forest** hydrology and erosion model with scaffolding for agricultural hydrology — inverting legacy WEPP, an agricultural model with forests bolted on as flag-gated partitions. See [Scientific orientation](#scientific-orientation-a-forest-model-with-agricultural-scaffolding).

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

openWEPP is the simulation engine only. GUI, GIS preprocessing, climate generation (cligen), DEM-to-watershed delineation (TOPAZ / WhiteboxTools), and run orchestration remain [wepppy](https://github.com/rogerlew/wepppy) concerns. openWEPP plugs into wepppy as a subprocess-per-hillslope replacement for the legacy WEPP binary, emitting parquet via the existing `wepppyo3` interchange schemas.

### Scientific orientation: a forest model with agricultural scaffolding

Legacy WEPP is an **agricultural** hydrology and erosion model that has been
*applied* to forests. It did not so much ignore forests as **partition** them:
forest and non-agricultural behavior lives in flag- and file-gated branches
bolted onto an agricultural trunk. Frost is switched off for non-agricultural
land and the `ksatadj` saturated-conductivity model is switched on through
`ksflag` (a provisional forest adaptation with no physical derivation on record);
the sub-daily **hourly water balance** is gated behind `wepp_ui.txt` — a
University-of-Idaho path that the NSERL agricultural lab and cropland runs never
exercise. Those forest partitions were maintained, when they were maintained at
all, by the forest / hydrology contributors rather than the agricultural lab —
which is precisely why they carry the model's undocumented closure debt: the
multi-OFE (MOFE) routing, the hourly water balance, the subsurface-dominated
(lateral-flow) hydrology, frost, snow, and the inter-element handoffs. Where the
partition ran out, forests were simply run as cropland (`landuse = 1`; the forest
management block was never finished).

openWEPP inverts the partition. It is a **forest** hydrology and erosion model
with **scaffolding for agricultural hydrology**. Forest is the trunk: fidelity
investment and validation authority are forest-first — absolute lateral-flow
magnitude judged against a steep-wet-forest *observed* envelope
(`SC-SUBHYD-001#INV-SUBHYD-033`; HJ Andrews WS10, Maimai, Panola) rather than
legacy output; first-class forest landuse and disturbed / burned-forest
parameterization instead of the cropland masquerade; the subsurface water
balance, frost, and snow that legacy left to under-maintained partitions; and the
MOFE, hourly-flow, and OFE-by-OFE routing (including the Papanicolaou revision
WEPP itself never incorporated) that the agricultural trunk could not carry.
Agricultural hydrology is **scaffolded** — the cropland, tillage, and management
paths are structurally supported and exercised, and legacy agricultural inputs
run through a compatibility bridge — but it is the secondary surface, not where
fidelity is spent.

This is where the effort goes, not a rebranding. Every hard problem in this
project — the MOFE water blow-up, subsurface lateral magnitude, disturbed-forest
runoff, frost-depth fidelity, snow — is a forest problem the agricultural model
was never built to get right, and several were partitioned off precisely so the
agricultural path would not have to carry them.

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

### In scope

- Hillslope simulation (single OFE and multi-OFE)
- Watershed channel routing and impoundment routing
- WEPP soil, management, climate, and watershed input-file compatibility
- Initial backward-compatibility bridge for legacy stdin `.run` plus `.txt`
  sidecar inputs/flags
- HBP (hillslope binary pass) shard production and consumption per the wepp-palimpsest contract
- Parquet output via the wepppy / wepppyo3 interchange schemas
- Three executables: single-hillslope CLI, watershed CLI, replay / debug CLI

### Out of scope

- GUI / web frontends (wepppy)
- GIS preprocessing, DEM delineation (wepppy)
- Climate generation / cligen (wepppy)
- Run orchestration, NoDb state model (wepppy)
- WEPP single-storm simulation modes (`ss`, `ss_batch`)
- Silent fallback when legacy sidecar inputs are missing or ambiguous
- Sediment routing physics (deferred to the wepp-palimpsest sediment kernelization program)

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

### Trust but verify (Codex and Claude)

The implementation is written by coding agents — Codex authors kernels and
tests; Claude Code reviews, debugs, and audits — and no agent's output is trusted
on its say-so. The same posture that demotes the legacy binary applies to the
agents: correctness authority is the contract, and every change passes through
fixed gates before it is accepted.

- **Contract-first sequencing.** For kernel work the order is enforced: amend the
  science contract, write contract-derived tests, record the pre-implementation
  gate, *then* edit production code. The authority exists before the code that
  implements it.
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

The work packages that drive this process are kept as an honest record —
progress and dead ends alike — so the reconstruction of each kernel, including
where it was genuinely hard, stays auditable after the fact.

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
| `wepppyo3` (in wepppy) | Defines parquet interchange schemas openWEPP emits |
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
