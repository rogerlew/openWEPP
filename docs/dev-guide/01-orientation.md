# 1. Orientation

What openWEPP is, what the executable pieces are, where code and authority
live in the repository, and which neighboring repositories matter.

## 1.1 What openWEPP is

openWEPP is a ground-up Rust reimplementation of the WEPP (Water Erosion
Prediction Project) hillslope and watershed simulation engine. It is the
**simulation engine only**: it reads WEPP-format inputs (soil, management,
slope, climate, plus legacy sidecar files), runs the daily water-balance /
plant-growth / erosion model, and writes parquet, HBP, and loss outputs.
Everything around the engine — GUI, GIS preprocessing, climate generation,
DEM delineation, run orchestration — belongs to
[wepppy](https://github.com/rogerlew/wepppy), which invokes openWEPP as a
subprocess exactly as it invoked the legacy Fortran binary.

Two properties define the project more than any feature:

1. **The correctness authority is the science contract, not the legacy
   binary.** Legacy WEPP carries known defects and disabled routines; its
   output is treated as forensic evidence to investigate, never as the answer
   key ([ADR-0011](../decisions/0011-architecture-first-top-down-science-contracts.md),
   [ADR-0017](../decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md)).
2. **The architecture came first, the physics second.** Typed state, explicit
   module boundaries, deterministic orchestration, and conservation gates were
   built before and around the process physics, so that a physics defect
   surfaces as a *named, located* violation instead of a mysterious number.

Chapter 2 unpacks why; this chapter is the map.

## 1.2 The executables

All binaries are built from the `openwepp-runner` crate
(`crates/openwepp-runner/src/bin/`):

| Binary | Role |
|---|---|
| `openwepp-cli-hill` | Runs one hillslope (1–N OFEs) forward in time from WEPP inputs + a TOML runfile. Produces the HBP pass shard, loss report, and parquet outputs. This is where the simulation physics actually executes. |
| `openwepp-cli-watershed` | Orchestrates a watershed: fans out one `openwepp-cli-hill` subprocess per hillslope, validates the produced pass artifacts, then routes channels/impoundments over them and publishes watershed outputs. |
| `openwepp-cli-totalwatsed3` | Output-aggregation tier ([ADR-0020](../decisions/0020-totalwatsed3-dedicated-output-aggregation-cli.md)): reads completed per-hillslope interchange parquet (read-only) and emits the openWEPP-native totalwatsed3 water-balance aggregation. Never simulates. |
| `open_wepp_runner` | Launcher / release boundary ([runner contract](../contracts/openwepp-runner-contract.md)); resolves which engine binary a release invocation dispatches to. |
| `openwepp-snowbench` | Physics benchmark harness for candidate snow models against observed datasets; a validation tool, not part of a simulation run. |

The hillslope and watershed processes are separate on purpose: the watershed
CLI talks to hillslope runs **only through the filesystem** (HBP pass shards),
matching the legacy WEPP process model that wepppy already drives
([ADR-0004](../decisions/0004-subprocess-hillslope-orchestration.md)). There is
no shared-memory or in-process coupling to preserve.

A debug/comparator third binary, `openwepp-replay`, is part of the ratified
three-binary architecture ([ADR-0006](../decisions/0006-three-binaries-incl-replay.md))
but is still pre-implementation.

## 1.3 The crate workspace

`crates/` is a Cargo workspace. The layering, from the outside in:

```text
openwepp-runner                  CLI binaries, runfile intake, run-dir staging,
                                 direct-runtime production wiring, publication
  ├── openwepp-hillslope-orchestrator   the hillslope engine: direct runtime
  │                                     (frames, phases), hydrology kernels,
  │                                     winter (snow/frost) column solver
  ├── openwepp-watershed-orchestrator   deterministic channel/impoundment
  │                                     dispatch over completed hillslope passes
  ├── openwepp-input-contract           WEPP input-file parsers (soil, management,
  │                                     slope, climate) and their typed surfaces
  ├── openwepp-legacy-bridge            legacy sidecar (.txt) discovery + HBP
  │                                     compatibility adapters at the edges
  ├── openwepp-hillslope-output /       parquet + HBP + loss writers for each
  │   openwepp-watershed-output         tier's output surface
  └── openwepp-summary-accumulator      deterministic daily→monthly→yearly rollups
```

Substrate crates used across tiers:

| Crate | Purpose |
|---|---|
| `openwepp-sim-contract` | Simulation contract substrate: canonical symbols, units registry, guard/status taxonomies. |
| `openwepp-unit-boundary` | Unit-safe wrapper types for state and flux quantities crossing boundaries. |
| `openwepp-kernel-contract` | Kernel invocation/writeback contract types. Since the 2026-06-30 kernel-boundary deletion these are **edge and watershed-transition surfaces only** — the hillslope hot path is fully typed and does not construct them (see chapter 6). |
| `openwepp-topology` | Topology graph model + the pre-execution validation gate. |
| `openwepp-climate-runtime-adapter` | Shared climate parser-to-runtime adapter for both orchestrators. |
| `openwepp-meteorology` | Meteorological primitives for candidate snow/frost physics. |
| `openwepp-comparator-metadata` | Typed comparator confidence-tier routing metadata (see §1.5). |

A first-session rule of thumb: **runtime behavior questions almost always land
in `openwepp-hillslope-orchestrator` (the engine) or `openwepp-runner` (the
wiring)**; input questions in `openwepp-input-contract`; output-format
questions in the `*-output` crates.

## 1.4 Where authority lives in `docs/`

openWEPP is unusually explicit about *which document gets to decide what*.
When two documents disagree, precedence is resolved by kind, not recency of
reading:

| Kind | Location | Decides |
|---|---|---|
| Science contracts (`SC-<DOMAIN>-<NNN>`) | `docs/specifications/science-contracts/` | What the physics must do: governing equations, invariants (`INV-*`), tolerances, units. The correctness authority for kernels. |
| ADRs | `docs/decisions/` | Architecture and strategy decisions, numbered and immutable once ratified; superseded by later ADRs, never edited into disagreement. |
| Interface contracts | `docs/contracts/` | File formats and process boundaries: the TOML runfile, HBP shards, parquet schemas, CLI/ABI, release naming. |
| Architecture specifications | `docs/architecture/` | Runtime design authority (e.g. the [array-native runtime spec](../architecture/array-native-runtime-specification.md), the [watershed runtime spec](../architecture/watershed-runtime-architecture-specification.md)). |
| Roadmap | [docs/ROADMAP.md](../ROADMAP.md) | The canonical forward-only planning queue. Completed work is *removed* and lives in the work-package log. |
| Work packages | `docs/work-packages/` | The dated execution log: scope, evidence, reviews, and closure for every initiative. **Evidence, not authority** — a work-package artifact never overrides a contract. |
| Standards | `docs/standards/` | Coding, refactor, testing, and prompt-authoring norms. |
| Backlog | `docs/backlog/` | Concept-stage ideas that have not been promoted to a work package. |

Two habits make this system navigable:

- **Read the ADR trail before questioning a design.** If something looks
  over-engineered, there is usually a numbered decision recording the failure
  that motivated it, often with measured evidence.
- **Search the work-package log before re-deriving.** `docs/work-packages/` is
  an honest record including dead ends; a surprising amount of "why is it done
  this way?" is answered by a package that tried the alternative and measured
  it failing.

## 1.5 Neighboring repositories

| Repo | Relationship |
|---|---|
| `wepp-palimpsest` (was `wepp-forest`) | The legacy WEPP implementation surface, kept for **static analysis and comparator signals**. The pinned normative baseline is `/workdir/wepp-forest_260430_baseline` (legacy binary `wepp_260430`, [ADR-0012](../decisions/0012-legacy-wepp-260430-baseline-anchor.md)). openWEPP reads this source for provenance mapping — it is explicitly *not* a clean-room project ([ADR-0010](../decisions/0010-non-clean-room-direct-port-policy.md)) — but the legacy binary is a comparator flag, not an oracle. |
| [wepppy](https://github.com/rogerlew/wepppy) | The consumer: GIS, climate, orchestration, UI. openWEPP is its subprocess engine. |
| `wepppyo3` (in wepppy) | Owns the *legacy* parquet interchange schemas. openWEPP owns its own output surface going forward ([ADR-0019](../decisions/0019-openwepp-owns-its-output-surface-wepppyo3-legacy-only.md)). |

## 1.6 Building and running

```bash
cargo build --release            # rust-toolchain.toml pins the channel
cargo nextest run --workspace --profile quick  # fast local confidence loop
```

Use `cargo nextest run --workspace --profile full` for branch-head closure when
the package or change class requires the full suite. Local gate tiering and
timing diagnostics are described in
[local-ci-gate-selection.md](../standards/local-ci-gate-selection.md).

A hillslope run needs a run directory of WEPP inputs and a TOML runfile
(schema `openwepp-hillslope-runfile-v1`, see the
[runfile contract](../contracts/)) naming inputs and desired outputs:

```bash
openwepp-cli-hill \
  --run-dir <dir with .sol/.man/.slp/.cli and sidecars> \
  --run-file <runfile>.run \
  --output-dir <out> \
  --policy compat --legacy-sidecar-discovery
```

End-user CLI documentation lives in [/usersum](../../usersum/); the validation
gates a change must pass before landing are in chapter 7.
