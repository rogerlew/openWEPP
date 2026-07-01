# WSHED-W3 Handoff Prompt

Scope: local repository Rust implementation task for openWEPP; flat-file
reads/edits plus local validation commands only; no external connectivity.

Standing user authorization for openWEPP subagent delegation is expected in the
session:
`For openWEPP work-package execution, I explicitly authorize Codex to`
`spawn/delegate to subagents whenever the active work package, AGENTS.md, or`
`package governance requires or authorizes review, verification, comparator`
`execution, or parallel agent work.`

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in
`docs/work-packages/20260701-wshedw3-bounded-worker-pool-001/package.md`
sequentially through disposition. Do not stop at analysis unless hard-blocked.

Required reading:

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/decisions/0004-subprocess-hillslope-orchestration.md`
- `docs/decisions/0032-watershed-runtime-ratification.md`
- `docs/architecture/watershed-runtime-architecture-specification.md`
- `docs/work-packages/20260701-wshedw2-serial-watershed-supervisor-skeleton-001/package.md`
- `docs/work-packages/20260701-wshedw2-serial-watershed-supervisor-skeleton-001/artifacts/consumer-path-evidence.md`
- `docs/work-packages/20260701-wshedw2-serial-watershed-supervisor-skeleton-001/artifacts/line-count-governance.md`
- `docs/work-packages/20260701-wshedfixture01-committed-watershed-fixture-adoption-001/package.md`
- `tests/fixtures/watershed/carnivorous-adobo/README.md`
- `docs/work-packages/20260701-wshedw3-bounded-worker-pool-001/package.md`
- `docs/work-packages/20260701-wshedw3-bounded-worker-pool-001/artifacts/required-reading-map.md`

Conditional:

- `docs/specifications/science-contracts/AGENTS.md` and relevant `SC-*`
  contracts before any kernel-affecting or science-contract edit.
- `docs/specifications/science-contracts/AGENTS.md` if implementation changes
  latest-event payload semantics, `NoEvent` handling, routing physics, or
  publication meaning. W3 should not need those changes.

On-demand:

- `crates/openwepp-runner/src/watershed_supervisor.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`
- `crates/openwepp-runner/src/launch.rs`
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
- `tests/integration/infile_watershed_structure_parser_contract.rs`
- adjacent runner/orchestrator modules touched by the implementation.

Required-reading budget: `OK`; map:
`docs/work-packages/20260701-wshedw3-bounded-worker-pool-001/artifacts/required-reading-map.md`.

Files:

- `crates/openwepp-runner/src/watershed_supervisor.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/src/lib.rs` if public exports change
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
- `tests/integration/**` for focused W3 fixture/identity tests if needed
- `tests/fixtures/watershed/carnivorous-adobo/**` only for metadata/runfile
  binding updates required by W3 gates
- `docs/work-packages/20260701-wshedw3-bounded-worker-pool-001/**`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`

Task: implement WSHED-W3 end to end. Add bounded worker-pool execution for
generated watershed hillslope jobs behind `openwepp-cli-watershed --jobs N`,
preserve `--jobs 1` determinism, prove `--jobs 1`/`--jobs N` output identity,
prove fail-closed child/pass behavior before routing, and record canonical
scaling evidence on committed fixture inputs.

Constraints:

- ADR-0032: public entrypoint remains `openwepp-cli-watershed`; omitted
  `--jobs` equals `--jobs 1`; CPU scaling is opt-in through explicit positive
  `--jobs N`.
- ADR-0004: hillslope execution remains subprocess-per-hillslope using
  `std::process::Command` with explicit args; no shell interpolation.
- Do not infer host-wide CPU defaults.
- Do not use child completion order for pass inventory order, routing input
  order, output row order, warnings, or checksums.
- Do not route or publish if any child job or pass inventory validation fails.
- Do not implement `WatershedNetworkFrame` production routing cutover in W3.
- Do not delete the old watershed runtime in W3.
- Do not change watershed routing physics, hillslope physics, erosion,
  sediment equations, latest-event semantics, or output schemas for
  performance.
- Do not compare discovery-on and discovery-off timings as the same benchmark
  surface.
- Do not close on producer-only scaffolding; the real public CLI runner and
  downstream routing/publication consumer must read the new worker-pool path.

Subagent requirement: REQUIRED. Spawn/delegate `comparator_suite_runner` for
heavy final closure gates, comparator/fixture runs, and scaling runs when
available; do not run full workspace closure or scaling batches on the parent
model unless the subagent is unavailable, and record command-level evidence if
unavailable. This prompt explicitly authorizes subagent spawning/delegation to
`comparator_suite_runner`, `rust_code_reviewer`, and `rust_qa_reviewer` for W3
gate execution, scaling evidence, review, and verification; outputs: compact
metrics/findings plus log or artifact paths; write access: read-only.

Validation:

- Run focused iteration gates as needed.
- Final closure requires:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo nextest run --workspace --profile full`
  - `cargo deny check`
  - focused W3 identity/worker-pool/failure/fixture/scaling gates
- Run scoped docs lint for package artifacts and touched docs.
- Run `git diff --check`.
- If any final gate cannot run, close `EXECUTED-HOLD` with the exact blocker.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update implementation, tests, package artifacts, scaling evidence,
consumer-path proof, line-count governance, review/verification disposition,
roadmap, work-package README, and final disposition.
