# Schedule Export and Introspection — Spec

Status: Implemented by ARCH23 on 2026-06-04
Evidence mode: Static + Ran
Authored-by: Claude Code (documentation authority per CLAUDE.md)
Scope: developer-facing export and introspection of the hillslope phase
schedule DAG, with the watershed dispatch scheduler as a declared follow-on.

Implementation:

- `crates/openwepp-hillslope-orchestrator/src/schedule_export.rs`
- `crates/openwepp-hillslope-orchestrator/src/bin/openwepp_hillslope_schedule_export.rs`
- `docs/architecture/generated/hillslope-phase-schedule.json`
- `docs/architecture/generated/hillslope-phase-schedule.mmd`
- `docs/architecture/generated/hillslope-phase-schedule.dot`
- `tools/release/check_hillslope_schedule_export.sh`

## 1. Problem Statement

The hillslope phase schedule is a deterministic DAG already materialized as data
in `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
(`HillslopePhaseGraph`, `PhaseDependency { phase, depends_on }`,
`dependency_edges()`, `topological_order()`, `canonical_order()`). Today it is:

- **not exportable** in any human-inspectable form, and
- **mirrored by hand** in `docs/architecture/hillslope-phase-scheduler-graph.md`,
  which has drifted from the code (confirmed in review).

Drift detail (Static, confirmed against the code): `PHASE_COUNT` is **14**, but
the doc enumerates only 9. The current canonical order is:

`normalization`, `storage_bounds`, `decomposition_transition`,
`residue_partition_transition`, `annual_growth_transition`,
`perennial_growth_transition`, `percolation_deep_seepage`, `evapotranspiration`,
`drainage`, `lateral_transfer`, `plant_root_uptake`, `runoff_reconciliation`,
`storage_reconciliation`, `closure_diagnostics`.

The stale doc is also wrong on *relative* order
(`evapotranspiration`/`percolation_deep_seepage` and
`lateral_transfer`/`drainage`). The scheduler **contract**
(`docs/specifications/science-contracts/hillslope-phase-scheduler-contract.md`)
carries the same stale 9-phase set. With no projection from code to a reviewable
artifact, both the diagram and the contract can silently disagree with the
engine — the exact failure mode this spec removes.

## 2. Goals

1. Render the canonical phase DAG **from code** (single source of truth) in a
   human-inspectable form.
2. Support developer workflows beyond visualization (§4) — a first-class
   requirement, not a nicety.
3. Be deterministic: identical code produces byte-identical output, suitable for
   diffing and CI gating.

### Non-goals

- No change to runtime execution; this is a read-only projection of the existing
  graph.
- No new graph definition; it consumes `HillslopePhaseGraph::canonical()`.
- No science-kernel or `SC-*` contract changes.

## 3. Export Formats

Required:

- **Mermaid** flowchart (`flowchart TD`) — inline-renderable in docs and on
  GitHub, diffs as text.
- **JSON** — `{ nodes: [{phase, rank}], edges: [{from, to}], topological_order:
  [...] }` with stable key/element ordering.

Recommended:

- **Graphviz DOT** — for standalone images / large graphs.

Edge-direction convention: a `PhaseDependency { phase, depends_on }` renders as
an arrow `depends_on -> phase` (execution / data-flow direction), so the diagram
reads in run order.

## 4. Developer-Task Support (required)

The capability must serve, at minimum:

1. **Doc-congruence gate.** Regenerate the canonical artifact and compare it to a
   committed copy; nonzero exit on mismatch, wired as a `tools/release/` gate so
   drift like the current ARCH05 mismatch fails CI rather than rotting silently.
2. **Validation diagnostics.** Report cycles, phases unreachable from a root,
   and mismatches between `canonical_order()` and the dependency map. Use
   `HillslopePhaseGraph::topological_order()` for hillslope cycle detection —
   **do not** reuse `openwepp-topology`'s cycle detection, which is
   watershed-specific and private. (If generic DAG validation is wanted later,
   extract a shared deterministic helper first.) Note: `HillslopePhaseGraph`
   internals are private with no public builder for arbitrary dependency maps,
   so injected-drift / malformed-graph tests likely need in-crate test access or
   a minimal added test surface.
3. **Topological-order listing.** Emit the resolved deterministic order from
   `topological_order()`. It returns `Option`; the exporter must wrap `None`
   (cycle) in its own typed error, not a silent empty.
4. **Schedule diff.** Given two JSON exports (e.g. base vs head), print
   added/removed nodes and edges, so a PR that changes the schedule surfaces the
   change explicitly in review.
5. **Per-phase metadata (extensible).** Phase rank, mapped consumer adapter via
   `hillslope_consumer_adapter_for_phase()` in
   `crates/openwepp-hillslope-orchestrator/src/consumer_boundary.rs` (the
   `HillslopeConsumerAdapter` type itself lives in `openwepp-kernel-contract`),
   and precondition notes where cheaply available.

## 5. Interface (recommended)

Public-API constraint: `HillslopePhase::ORDERED` and `PHASE_COUNT` are
`pub(crate)`. External tooling must consume `HillslopePhaseGraph::canonical_order()`
and the public graph methods; if the exporter lives outside the orchestrator
crate it needs a small added public surface, otherwise co-locate it in-crate.

Recommended shape (Codex review): an explicit **generator command** for local
refresh plus a **`#[test]`** that asserts congruence, gated in CI by
`tools/release/check_hillslope_schedule_export.sh` comparing committed
artifacts. Do **not** have a test write repo files by default — keep regeneration
an explicit command. `cargo xtask` is **not** set up in this repo, so the xtask
option is out unless intentionally introduced. Avoid expanding production runner
(`open_wepp_runner`) semantics to own developer introspection unless maintainers
want that.

The spec requires only that (1) the generator is invocable both in CI and
locally, and (2) the canonical artifact lives in-repo and is gate-checked for
congruence.

## 6. Determinism and Error Handling

- Deterministic output ordering (sort by `rank()`); use `BTreeMap`/`BTreeSet` or
  rank-sorted vectors — no `HashMap` iteration leaking into JSON/Mermaid/DOT/diff
  output.
- Typed errors; no `.unwrap()` / `.expect()` in production paths. Cycles and
  missing phases are typed errors, not panics or silent empties (per AGENTS.md).
  Place an exporter-specific error type with the exporter module rather than
  overloading `HillslopeSchedulerError`.
- No network egress, no telemetry.

## 7. Acceptance Criteria

- Generated Mermaid + JSON reflect the live `HillslopePhaseGraph::canonical()`
  for all `PHASE_COUNT` (14) phases and their edges.
- The congruence gate fails on an intentionally drifted committed artifact and
  passes after regeneration.
- The diff path correctly reports a synthetic added and removed edge.
- `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`, `cargo deny`
  clean; new tests cover format output, cycle detection, and diff.
- The drifted `hillslope-phase-scheduler-graph.md` **and** the stale
  `hillslope-phase-scheduler-contract.md` phase lists are reconciled — replaced
  by, or linked to, the generated artifact (keep prose for preconditions and
  halt semantics).

## 8. Review Dispositions (Codex, 2026-06-04)

Recommendations from review; final selection is the maintainer's.

- **Interface placement** — explicit generator command + `#[test]` + a
  `tools/release/` gate. No xtask; do not auto-write repo files from tests.
- **Spec/doc home** — keep this draft in `docs/architecture/` for review; promote
  the stable interface spec to `docs/specifications/subsystems/` after
  disposition. `docs/backlog/` would undersell it (implementation-ready, not
  concept-stage).
- **Watershed scope** — follow-on. Same shape, but topology-derived nodes and
  different diagnostics; hillslope first is cleaner.
- **ARCH05 reconciliation** — replace the hand-maintained phase lists/edges with
  generated output; keep prose for preconditions/halt semantics; also fix the
  stale scheduler contract.
- **Process weight** — a lightweight **formal work-package** (changes tooling /
  docs / tests). It does **not** require the SC kernel contract-first procedure
  unless runtime scheduler behavior or kernel-controlling projection changes.

## 9. References

- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `crates/openwepp-hillslope-orchestrator/src/phase.rs`,
  `crates/openwepp-hillslope-orchestrator/src/constants.rs`
- `crates/openwepp-hillslope-orchestrator/src/consumer_boundary.rs`
  (`hillslope_consumer_adapter_for_phase()`)
- `crates/openwepp-kernel-contract/src/lib.rs` (`HillslopeConsumerAdapter`)
- `crates/openwepp-topology/src/lib.rs` (`validate_pre_execution_topology`,
  `TopologyValidationReport` — watershed-specific, not a generic DAG validator)
- `docs/architecture/hillslope-phase-scheduler-graph.md` (drifted)
- `docs/specifications/science-contracts/hillslope-phase-scheduler-contract.md`
  (drifted)
- `tools/release/README.md` (gate-script convention)
- `AGENTS.md` (error handling, validation gates, work-package procedure)
