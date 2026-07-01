# WSHED-W2 Handoff Prompt

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
`docs/work-packages/20260701-wshedw2-serial-watershed-supervisor-skeleton-001/package.md`
sequentially through disposition. Do not stop at analysis unless hard-blocked.

Required reading:

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/decisions/0004-subprocess-hillslope-orchestration.md`
- `docs/decisions/0032-watershed-runtime-ratification.md`
- `docs/architecture/watershed-runtime-architecture-specification.md`
- `docs/work-packages/20260701-wshedfixture01-committed-watershed-fixture-adoption-001/package.md`
- `tests/fixtures/watershed/carnivorous-adobo/README.md`
- `docs/work-packages/20260701-wshedw2-serial-watershed-supervisor-skeleton-001/package.md`
- `docs/work-packages/20260701-wshedw2-serial-watershed-supervisor-skeleton-001/artifacts/required-reading-map.md`

Conditional:

- `docs/specifications/science-contracts/AGENTS.md` and relevant `SC-*`
  contracts if implementation admits a valid `NoEvent` state instead of
  hard-erroring missing latest-event payloads.
- `docs/specifications/science-contracts/AGENTS.md` before any kernel-affecting
  or science-contract edit.

On-demand:

- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`
- `crates/openwepp-runner/src/bin/open_wepp_runner.rs`
- `crates/openwepp-runner/src/launch.rs`
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
- `tests/integration/infile_watershed_structure_parser_contract.rs`
- adjacent runner/orchestrator modules touched by the implementation.

Required-reading budget: `OK`; map:
`docs/work-packages/20260701-wshedw2-serial-watershed-supervisor-skeleton-001/artifacts/required-reading-map.md`.

Files:

- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/src/**` for watershed supervisor modules only
- `crates/openwepp-runner/tests/**`
- `tests/integration/**` for focused W2 tests
- `tests/fixtures/watershed/carnivorous-adobo/**` only for metadata/manifest
  updates required by W2 gates
- `docs/work-packages/20260701-wshedw2-serial-watershed-supervisor-skeleton-001/**`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`

Task: implement WSHED-W2 end to end. Add the serial watershed supervisor
skeleton for `--jobs 1`: `WatershedRunPlan`, `HillslopeJob`, and
`PassInventory`; deterministic serial hillslope subprocess execution with
isolated outputs/logs; pass inventory validation before routing; explicit
routed-stage reuse mode preserved; consumer-path proof that the public runner
uses the new plan/inventory path.

Constraints:

- ADR-0032: public entrypoint remains `openwepp-cli-watershed`; omitted
  `--jobs` equals `--jobs 1`; scaling beyond 1 is W3, not W2.
- ADR-0004: hillslope execution remains subprocess-per-hillslope using
  `std::process::Command` with explicit args; no shell interpolation.
- Do not implement worker-pool concurrency in W2.
- Do not implement `WatershedNetworkFrame` production routing cutover in W2.
- Do not delete the old watershed runtime in W2.
- Do not change watershed routing physics, hillslope physics, erosion,
  sediment equations, or output schemas for performance.
- Do not silently synthesize zero runoff, zero duration, zero detachment, zero
  deposition, or zero sediment concentrations from absent latest-event payloads.
- If existing authority does not clearly admit a valid `NoEvent` state, missing
  latest-event payloads must be a typed fail-closed error in W2.
- Do not close on producer-only scaffolding; the real public CLI runner and
  downstream routing/publication consumer must read the new path.

Subagent requirement: REQUIRED. Spawn/delegate `comparator_suite_runner` for
heavy final closure gates and comparator/fixture runs when available; do not run
full workspace closure on the parent model unless the subagent is unavailable,
and record command-level evidence if unavailable. This prompt explicitly
authorizes subagent spawning/delegation to `comparator_suite_runner`,
`rust_code_reviewer`, and `rust_qa_reviewer` for W2 gate execution, review, and
verification; outputs: compact metrics/findings plus log or artifact paths;
write access: read-only.

Validation:

- Run focused iteration gates as needed.
- Final closure requires:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo nextest run --workspace --profile full`
  - `cargo deny check`
  - focused W2 identity/consumer-path/pass-inventory/fixture gates
- Run scoped docs lint for package artifacts and touched docs.
- Run `git diff --check`.
- If any final gate cannot run, close `EXECUTED-HOLD` with the exact blocker.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update implementation, tests, package artifacts, consumer-path proof,
line-count governance, review/verification disposition, roadmap,
work-package README, and final disposition.
