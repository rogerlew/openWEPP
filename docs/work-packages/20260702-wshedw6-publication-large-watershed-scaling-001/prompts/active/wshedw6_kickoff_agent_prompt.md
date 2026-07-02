# WSHED-W6 Handoff Prompt

Scope: local repository Rust implementation, fixture-adoption, and benchmark
task for openWEPP; flat-file reads/edits plus local validation commands only;
no external connectivity.

Standing user authorization for openWEPP subagent delegation is expected in the
session:
`For openWEPP work-package execution, I explicitly authorize Codex to`
`spawn/delegate to subagents whenever the active work package, AGENTS.md, or`
`package governance requires or authorizes review, verification, comparator`
`execution, or parallel agent work.`

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in
`docs/work-packages/20260702-wshedw6-publication-large-watershed-scaling-001/package.md`
sequentially through disposition. Do not stop at analysis unless hard-blocked.

First actionable item: inventory the current typed watershed publication path,
then implement typed publication streaming/direct writing and committed
large-fixture scaling evidence for WSHED-W6.

Required reading:

Core:

- `AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/codex_exec_plans.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/standards/kernel-work-package-preparation.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/architecture/watershed-runtime-architecture-specification.md`
- `docs/decisions/0032-watershed-runtime-ratification.md`
- `docs/work-packages/20260701-wshedw5-old-watershed-runtime-deletion-001/package.md`
- `docs/work-packages/20260701-wshedw5-old-watershed-runtime-deletion-001/artifacts/disposition.md`
- `docs/work-packages/20260701-wshedw5-old-watershed-runtime-deletion-001/artifacts/old-runtime-deletion-manifest.md`
- `docs/work-packages/20260701-wshedw3-bounded-worker-pool-001/artifacts/scaling-evidence.md`
- `tests/fixtures/watershed/carnivorous-adobo/README.md`
- `docs/work-packages/20260702-wshedw6-publication-large-watershed-scaling-001/package.md`
- `docs/work-packages/20260702-wshedw6-publication-large-watershed-scaling-001/artifacts/required-reading-map.md`

Conditional:

- `docs/specifications/science-contract-authoring-procedure.md`,
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`,
  and `docs/specifications/science-contracts/index.md` if canonical contract
  amendments are required.
- Relevant contracts before touching the associated surface:
  `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`,
  `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`,
  `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`,
  `docs/specifications/science-contracts/contracts/SC-SED-001.md`,
  `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`,
  and watershed output/infile contracts.

On-demand:

- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/src/watershed_supervisor.rs`
- `crates/openwepp-runner/src/totalwatsed3.rs`
- `crates/openwepp-runner/src/watershed_wat.rs`
- `crates/openwepp-watershed-orchestrator/src/**`
- `crates/openwepp-watershed-output/src/**`
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
- `crates/openwepp-runner/tests/totalwatsed3_cli_contract.rs`
- `tests/integration/**`
- `tests/fixtures/watershed/**`
- local `/wc1/runs/**/watershed` candidates as source substrates only.

Required-reading budget: `364434` bytes, `OK`; map:
`docs/work-packages/20260702-wshedw6-publication-large-watershed-scaling-001/artifacts/required-reading-map.md`.

Files:

- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/src/watershed_supervisor.rs`
- `crates/openwepp-runner/src/totalwatsed3.rs`
- `crates/openwepp-runner/src/watershed_wat.rs`
- `crates/openwepp-watershed-orchestrator/src/**`
- `crates/openwepp-watershed-output/src/**`
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
- `crates/openwepp-runner/tests/totalwatsed3_cli_contract.rs` if touched
- `tests/integration/**`
- `tests/fixtures/watershed/**`
- `docs/specifications/science-contracts/**` only if contract amendments are
  required
- `docs/work-packages/20260702-wshedw6-publication-large-watershed-scaling-001/**`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`

Task: execute WSHED-W6 end to end. Stream/directly write watershed publication
from typed projection state, adopt committed large or representative watershed
fixtures, run strict committed-fixture scaling, and record fresh same-scope
legacy/openWEPP timing where runnable.

Constraints:

- Do not reintroduce the deleted W5 old watershed request/writeback runtime.
- Do not preserve publication through a wrapper, compatibility row seed, shadow
  path, or route-only evidence if the real public output consumer can read typed
  projection state directly.
- Do not change routing, impoundment, sediment, erosion, runoff-partition,
  water-balance, latest-event, or output-schema semantics without canonical
  contract authority.
- Do not make `/wc1`, scratch, or wepppy paths persistent fixture or benchmark
  evidence.
- Do not default `--jobs` above `1`; explicit `--jobs N` remains the CPU
  scaling mechanism.

No surrogate physics: production code must keep actual contract-backed or
baseline-authoritative physics. Surrogate/provisional/proxy/heuristic
stand-ins are forbidden. Missing authority is a hold-for-authority boundary;
known in-scope physics is an implementation obligation.

Real consumer proof: prove the public watershed CLI and output writers consume
the typed publication path. Wrappers, adapters, skeletons, shadow paths, and
old compatibility paths cannot carry the W6 closure claim.

Conservation/output acceptance: record operand lineage; separate plausible
aliases in fixtures; reject known wrong formulas; run independent
reconstruction plus real closure/magnitude audit; align metadata/schema; do
not close on one-sided bounds or self-consistency alone.

Subagent requirement: REQUIRED. Spawn/delegate `comparator_suite_runner` for
release builds, scaling matrices, same-fixture legacy/openWEPP timing,
protected-output comparator runs, and full closure gates when available; do
not run heavy scaling/comparator batches on the parent model unless the
subagent is unavailable, and record command-level evidence if unavailable.
This prompt explicitly authorizes subagent spawning/delegation to
`comparator_suite_runner`, `fixture_inventory_agent`, `rust_code_reviewer`,
`rust_qa_reviewer`, and `science_contract_reviewer` for W6 fixture discovery,
scaling evidence, review, and verification; outputs: compact metrics/findings
plus log or artifact paths; write access: `fixture_inventory_agent` may make
bounded package-artifact updates if available, all others read-only.

Validation:

- Run focused iteration gates as needed.
- Final complete closure requires:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo nextest run --workspace --profile full`
  - `cargo deny check`
  - focused W6 publication, source-guard, fixture-contract, output-identity,
    scaling, and legacy-comparison gates
  - fixture checksum manifest validation for each adopted fixture
- Run scoped docs lint for package artifacts and touched docs.
- Run `git diff --check`.
- If any complete gate cannot run, close `EXECUTED-HOLD` only at a legitimate
  boundary named in `package.md`.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update implementation, tests, fixture manifests, publication lineage,
consumer-path proof, source-guard evidence, protected-output evidence,
benchmark environment, scaling matrix, legacy comparison, line-count
governance, review and verification artifacts, roadmap, work-package README,
worker handoff, and final disposition.
