Scope: local repository tooling/docs task; flat-file reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- docs/work-packages/20260604-arch23-schedule-export-and-introspection-001/package.md
- docs/architecture/schedule-export-and-introspection.md
- docs/architecture/hillslope-phase-scheduler-graph.md
- docs/specifications/science-contracts/hillslope-phase-scheduler-contract.md
- docs/specifications/science-contracts/index.md
- docs/work-packages/20260521-arch05-hillslope-phase-scheduler-graph-001/package.md
- docs/work-packages/20260521-arch05-hillslope-phase-scheduler-graph-001/artifacts/worker-handoff.md
- crates/openwepp-hillslope-orchestrator/src/scheduler.rs
- crates/openwepp-hillslope-orchestrator/src/phase.rs
- crates/openwepp-hillslope-orchestrator/src/constants.rs
- crates/openwepp-hillslope-orchestrator/src/consumer_boundary.rs
- crates/openwepp-hillslope-orchestrator/src/lib.rs
- crates/openwepp-kernel-contract/src/lib.rs
- crates/openwepp-topology/src/lib.rs
- tools/release/README.md
Files:
- crates/openwepp-hillslope-orchestrator/src/lib.rs
- crates/openwepp-hillslope-orchestrator/src/scheduler.rs
- crates/openwepp-hillslope-orchestrator/src/schedule_export.rs
- crates/openwepp-hillslope-orchestrator/src/bin/openwepp_hillslope_schedule_export.rs
- crates/openwepp-hillslope-orchestrator/src/tests.rs
- docs/architecture/generated/hillslope-phase-schedule.mmd
- docs/architecture/generated/hillslope-phase-schedule.json
- docs/architecture/generated/hillslope-phase-schedule.dot
- docs/architecture/hillslope-phase-scheduler-graph.md
- docs/architecture/schedule-export-and-introspection.md
- docs/specifications/science-contracts/hillslope-phase-scheduler-contract.md
- tools/release/check_hillslope_schedule_export.sh
- tools/release/README.md
- docs/work-packages/README.md
- docs/work-packages/20260604-arch23-schedule-export-and-introspection-001/**
Task: execute ARCH23 end to end for the declared scope by implementing deterministic hillslope schedule export/introspection from `HillslopePhaseGraph::canonical()`, adding a local generator and release congruence gate, committing generated artifacts, reconciling stale schedule docs, and completing review/verification/disposition artifacts.
Constraints: consume the live canonical graph as the single source of truth; do not define a second graph; do not change runtime scheduler execution behavior; do not edit `SC-*` contracts; use deterministic ordering; use typed exporter errors for cycles, missing phases, malformed diff input, and topological-order `None`; do not use `openwepp-topology` cycle detection for hillslope schedule validation; tests must not write repository files by default; no silent defaults or panics in production paths.
Kernel-scope HOLD trigger: if implementation requires runtime phase-order, scheduler branch, kernel writeback, or canonical `SC-*` physics changes, stop production edits, record the blocker in `artifacts/kernel-scope-screen.md`, set disposition to HOLD, and queue a kernel-affecting follow-on package under root `AGENTS.md`.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Outputs: update package artifacts/disposition for all completed phases, including scope screen, spec disposition, export/gate evidence, implementation/test evidence, dual review, finding disposition, dual verification, and final disposition.
