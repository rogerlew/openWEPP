# WSHED-W5 Handoff Prompt

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
`docs/work-packages/20260701-wshedw5-old-watershed-runtime-deletion-001/package.md`
sequentially through disposition. Do not stop at analysis unless hard-blocked.

First actionable item: delete the watershed-specific old compatibility runtime
now that WSHED-W4DC01 moved public routing onto typed `WatershedNetworkFrame`
dispatch.

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
- `docs/work-packages/20260701-wshedw4-typed-watershed-network-frame-001/package.md`
- `docs/work-packages/20260701-wshedw4-typed-watershed-network-frame-001/artifacts/disposition.md`
- `docs/work-packages/20260701-wshedw4dc01-typed-routing-kernel-writeback-closure-001/package.md`
- `docs/work-packages/20260701-wshedw4dc01-typed-routing-kernel-writeback-closure-001/artifacts/disposition.md`
- `docs/work-packages/20260701-wshedw4dc01-typed-routing-kernel-writeback-closure-001/artifacts/old-surface-closure-inventory.md`
- `docs/work-packages/20260701-wshedw4dc01-typed-routing-kernel-writeback-closure-001/artifacts/source-guard-evidence.md`
- `docs/work-packages/20260701-wshedw5-old-watershed-runtime-deletion-001/package.md`
- `docs/work-packages/20260701-wshedw5-old-watershed-runtime-deletion-001/artifacts/required-reading-map.md`

Conditional:

- `docs/defect_closure_execplans.md` only if execution discovers a new defect
  rather than behavior-preserving deletion.
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
  and watershed infile contracts.

On-demand:

- `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/dispatch.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/**`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/types.rs`
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs*.rs`
- `crates/openwepp-kernel-contract/src/**`
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
- `tests/integration/**`
- `tests/fixtures/watershed/carnivorous-adobo/README.md`

Required-reading budget: `373948` bytes, `OK`; map:
`docs/work-packages/20260701-wshedw5-old-watershed-runtime-deletion-001/artifacts/required-reading-map.md`.

Files:

- `crates/openwepp-watershed-orchestrator/src/**`
- `crates/openwepp-kernel-contract/src/**` only for watershed-specific
  old-runtime request/symbol deletion proven unused outside W5 scope
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
- `tests/integration/**`
- `docs/specifications/science-contracts/**` only if contract amendments are
  required
- `docs/work-packages/20260701-wshedw5-old-watershed-runtime-deletion-001/**`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`

Task: delete the watershed-specific old compatibility runtime and obsolete
old-surface tests. Keep the public watershed CLI on typed frame-native dispatch
and typed publication. Backfill protected coverage on typed public behavior.

Constraints:

- Do not delete hillslope compatibility/runtime surfaces or generic
  `BoundarySymbol` / `BoundaryValue` infrastructure still owned outside W5.
- Do not preserve watershed old runtime by adding a new adapter, wrapper,
  facade, fallback, public selector, or shadow path.
- Do not change routing, impoundment, sediment, erosion, runoff-partition,
  water-balance, latest-event, or output-schema semantics without canonical
  contract authority.
- Do not loosen fail-closed guards, silently clamp, invent physics, or
  canonicalize invalid domains away.

No surrogate physics: production code must keep actual contract-backed or
baseline-authoritative physics. Surrogate/provisional/proxy/heuristic
stand-ins are forbidden. Missing authority is a hold-for-authority boundary;
known in-scope physics is an implementation obligation.

Real consumer proof: prove the public watershed CLI and orchestrator
production dispatch read the typed route. Wrappers, adapters, skeletons, shadow
paths, and old compatibility paths cannot carry the W5 closure claim.

Conservation/output acceptance: if deletion touches watershed output operands,
record operand lineage; separate plausible aliases in fixtures; reject known
wrong formulas; run independent reconstruction plus real closure/magnitude
audit; align metadata/schema; do not close on one-sided bounds or
self-consistency alone.

Subagent requirement: REQUIRED. Spawn/delegate `comparator_suite_runner` for
heavy full-closure gates, protected-output comparator runs, and release-style
fixture runs when available; do not run full workspace closure or comparator
batches on the parent model unless the subagent is unavailable, and record
command-level evidence if unavailable. This prompt explicitly authorizes
subagent spawning/delegation to `comparator_suite_runner`,
`rust_code_reviewer`, `rust_qa_reviewer`, and `science_contract_reviewer` for
W5 gate execution, protected-output evidence, review, and verification;
outputs: compact metrics/findings plus log or artifact paths; write access:
read-only.

Validation:

- Run focused iteration gates as needed.
- Final complete closure requires:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo nextest run --workspace --profile full`
  - `cargo deny check`
  - focused W5 source-guard, deletion-manifest, protected-output, and
    replacement-coverage gates
- Run scoped docs lint for package artifacts and touched docs.
- Run `git diff --check`.
- If any complete gate cannot run, close `EXECUTED-HOLD` only at a legitimate
  boundary named in `package.md`.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update implementation, tests, deletion inventory, deletion manifest,
consumer-path proof, source-guard evidence, protected-output evidence,
line-count governance, review and verification artifacts, roadmap,
work-package README, worker handoff, and final disposition.
