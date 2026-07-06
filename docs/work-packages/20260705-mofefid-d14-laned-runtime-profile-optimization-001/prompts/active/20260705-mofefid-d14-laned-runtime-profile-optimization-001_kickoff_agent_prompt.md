# D14 Kickoff Prompt

Scope: local repository science-contract/kernel performance task; flat-file
reads/edits only; no external connectivity required.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in
`docs/work-packages/20260705-mofefid-d14-laned-runtime-profile-optimization-001/package.md`
sequentially through disposition.

Required reading (read before edits):

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/work-packages/20260705-mofefid-d14-laned-runtime-profile-optimization-001/package.md`
- `docs/work-packages/20260705-mofefid-d14-laned-runtime-profile-optimization-001/artifacts/required-reading-map.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/planning/mofe-fidelity-campaign-strategy.md` §6.1
- `docs/work-packages/20260705-mofefid-d13-routed-hydrograph-erosion-shape-001/artifacts/final-disposition.md`
- `docs/work-packages/20260705-mofefid-d13-routed-hydrograph-erosion-shape-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260705-mofefid-d12-melt-limb-hourly-shape-001/artifacts/final-disposition.md`
- `docs/work-packages/20260706-mofefid-d11-gap007-dynamic-friction-closure-001/artifacts/final-disposition.md`
- `docs/work-packages/20260705-mofefid-d10-shock-numerics-gap005-001/artifacts/disposition.md`
- `docs/work-packages/20260705-mofefid-laned-activation-increment-001/package.md`
- `crates/openwepp-runner/src/hillslope/laned_shadow.rs`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/seam.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/friction.rs`
- `tests/integration/laned_shadow_h2637.rs`

Conditional and on-demand reading:

- Additional files listed in `artifacts/required-reading-map.md` when touched.

Required-reading budget: 356,093 bytes, OK (<400,000-byte WARN threshold);
map: `artifacts/required-reading-map.md`.

Files: declared write set in `package.md`.

Task: execute D14 end to end. Profile the Lane D runtime physics path, break
the H2637 shadow overhead into named slots, land only behavior-preserving
optimizations justified by the profile, and hand D15 a fresh runtime budget.

Constraints: no production/default activation; no D10 shock-numerics
correction or tolerance loosening; no D11/D12/D13 semantic changes; no
surrogate/provisional/proxy/heuristic physics; no output-affecting
optimization without separate contract authority.

Evidence expectations: exact timing commands, wall/user/sys values, slot-level
timing, before/after H2637 endpoint timing, protected-output identity,
routed-path closure/diagnostic parity, and local-CI-agent-discoverable
diagnostics.

Subagent requirement: REQUIRED. Spawn `comparator_suite_runner` for heavy
H2637 timing and full-suite gates when available; do NOT run them on the
parent model unless the subagent is unavailable, in which case record
command-level evidence. This prompt explicitly authorizes subagent
spawning/delegation to `rust_code_reviewer`, `rust_qa_reviewer`, `explorer`,
and `comparator_suite_runner` for read-only source audit, profiling review,
optimization review, verification, H2637/Lane D evidence, and heavy gate
execution. Outputs: compact findings, timing metrics, gate metrics, log
paths, and package-local artifact text. Write access: read-only unless the
operator assigns a bounded write set.

Autonomy: execute package phases end to end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
