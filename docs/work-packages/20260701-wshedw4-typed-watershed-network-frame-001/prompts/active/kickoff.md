# WSHED-W4 Handoff Prompt

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
`docs/work-packages/20260701-wshedw4-typed-watershed-network-frame-001/package.md`
sequentially through disposition. Do not stop at analysis unless hard-blocked.

Required reading:

Core:

- `AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/standards/kernel-work-package-preparation.md`
- `docs/architecture/watershed-runtime-architecture-specification.md`
- `docs/decisions/0032-watershed-runtime-ratification.md`
- `docs/work-packages/20260701-wshedw3-bounded-worker-pool-001/package.md`
- `docs/work-packages/20260701-wshedw3-bounded-worker-pool-001/artifacts/consumer-path-evidence.md`
- `docs/work-packages/20260701-wshedw3-bounded-worker-pool-001/artifacts/line-count-governance.md`
- `docs/work-packages/20260701-wshedw3-bounded-worker-pool-001/artifacts/scaling-evidence.md`
- `docs/work-packages/20260701-wshedw4-typed-watershed-network-frame-001/package.md`
- `docs/work-packages/20260701-wshedw4-typed-watershed-network-frame-001/artifacts/required-reading-map.md`

Conditional:

- `docs/specifications/science-contracts/AGENTS.md` before any
  kernel-affecting, routing-semantic, publication-semantic, unit, guard, or
  canonical contract edit.
- `docs/specifications/science-contract-authoring-procedure.md`,
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`,
  and `docs/specifications/science-contracts/index.md` if canonical contract
  amendments are required.
- Relevant `SC-*` contracts before touching the associated surface:
  `SC-ROUTE-001`, `SC-IMPOUND-001`, `SC-RUNOFFPART-001`, `SC-SED-001`,
  `SC-WATBAL-001`, `SC-INFILE-WATERSHED-STRUCTURE-001`,
  `SC-INFILE-WATERSHED-CHANNEL-001`, and
  `SC-INFILE-WATERSHED-IMPOUNDMENT-001`.

On-demand:

- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/src/watershed_supervisor.rs`
- `crates/openwepp-watershed-orchestrator/src/**`
- `crates/openwepp-watershed-output/src/**`
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
- `tests/integration/**`
- `tests/fixtures/watershed/carnivorous-adobo/README.md`
- adjacent modules touched by the implementation.

Required-reading budget: `OK`; map:
`docs/work-packages/20260701-wshedw4-typed-watershed-network-frame-001/artifacts/required-reading-map.md`.

Files:

- `crates/openwepp-watershed-orchestrator/src/**`
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/src/watershed_supervisor.rs` only for handoff changes
- `crates/openwepp-runner/src/lib.rs` if public exports change
- `crates/openwepp-watershed-output/src/**` if publication helpers move
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
- `tests/integration/**` for W4 typed-frame/source-guard/protected-output tests
- `tests/fixtures/watershed/carnivorous-adobo/**` only for metadata or manifest
  updates required by W4 gates
- `docs/work-packages/20260701-wshedw4-typed-watershed-network-frame-001/**`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`

Task: implement WSHED-W4 end to end. Replace production routing/publication
through `WatershedWritebackSurface` with typed `WatershedNetworkFrame` and
`WatershedPublicationFrame` fields in the real public
`openwepp-cli-watershed` path. Preserve W2/W3 run-plan, worker-pool,
pass-inventory, fail-closed behavior, and protected output identity unless a
delta is explicitly contract-governed.

Constraints:

- Do not wrap the old `WatershedWritebackSurface` runtime in a typed facade and
  call that W4 complete.
- Do not close on producer-only, shadow-only, adapter-only, counter-only, or
  test-only evidence.
- Production routing loops must not perform `BoundarySymbol` or `BoundaryValue`
  lookup for the W4 claim.
- Production publication for protected outputs must consume typed publication
  frame operands.
- Keep any compatibility projection explicit and outside the production
  routing/publication claim.
- Delete or migrate obsolete map-key tests with protected-coverage backfill;
  do not preserve old internal surfaces solely to keep obsolete tests passing.
- Do not change routing physics, erosion, sediment equations, impoundment
  equations, latest-event semantics, or output schemas for performance.
- Do not canonicalize-and-proceed on domain violations unless a canonical
  `SC-*` contract explicitly authorizes bounded normalization.

Conservation/output acceptance: record operand lineage before production edits;
separate plausible aliases in fixtures; reject known wrong formulas and
adjacent aliases; run independent reconstruction plus real closure/magnitude
audit; align metadata/schema with the accepted lineage; do not close on
one-sided bounds or exact self-consistency alone.

Subagent requirement: REQUIRED. Spawn/delegate `comparator_suite_runner` for
heavy final closure gates, protected-output comparator runs, and release-style
fixture runs when available; do not run full workspace closure or comparator
batches on the parent model unless the subagent is unavailable, and record
command-level evidence if unavailable. This prompt explicitly authorizes
subagent spawning/delegation to `comparator_suite_runner`,
`rust_code_reviewer`, `rust_qa_reviewer`, and `science_contract_reviewer` for
W4 gate execution, protected-output evidence, review, and verification;
outputs: compact metrics/findings plus log or artifact paths; write access:
read-only.

Validation:

- Run focused iteration gates as needed.
- Final closure requires:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo nextest run --workspace --profile full`
  - `cargo deny check`
  - focused W4 typed-frame/source-guard/protected-output/conservation gates
- Run scoped docs lint for package artifacts and touched docs.
- Run `git diff --check`.
- If any final gate cannot run, close `EXECUTED-HOLD` with the exact blocker.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update implementation, tests, package artifacts, operand lineage,
consumer-path proof, source-guard evidence, line-count governance,
review/verification disposition, roadmap, work-package README, and final
disposition.
