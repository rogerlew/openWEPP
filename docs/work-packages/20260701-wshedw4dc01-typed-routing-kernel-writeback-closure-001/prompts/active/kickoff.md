# WSHED-W4DC01 Handoff Prompt

Scope: local repository Rust implementation task for openWEPP; flat-file
reads/edits plus local validation commands only; no external connectivity.

Standing user authorization for openWEPP subagent delegation is expected in the
session:
`For openWEPP work-package execution, I explicitly authorize Codex to`
`spawn/delegate to subagents whenever the active work package, AGENTS.md, or`
`package governance requires or authorizes review, verification, comparator`
`execution, or parallel agent work.`

Execution mode: package-end-to-end defect closure (default).

Phase plan: execute all phases in
`docs/work-packages/20260701-wshedw4dc01-typed-routing-kernel-writeback-closure-001/package.md`
sequentially through disposition. Do not stop at analysis unless hard-blocked.

First actionable item: close defect `WSHED-W4-HOLD-001` end to end by replacing
`compatibility_writeback_surface` + `execute_watershed_dispatch_with_kernel`
production routing with `WatershedNetworkFrame`-native dispatch.

Required reading:

Core:

- `AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/codex_exec_plans.md`
- `docs/defect_closure_execplans.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/standards/kernel-work-package-preparation.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/architecture/watershed-runtime-architecture-specification.md`
- `docs/decisions/0032-watershed-runtime-ratification.md`
- `docs/work-packages/20260701-wshedw4-typed-watershed-network-frame-001/package.md`
- `docs/work-packages/20260701-wshedw4-typed-watershed-network-frame-001/artifacts/disposition.md`
- `docs/work-packages/20260701-wshedw4-typed-watershed-network-frame-001/artifacts/old-surface-inventory.md`
- `docs/work-packages/20260701-wshedw4-typed-watershed-network-frame-001/artifacts/consumer-path-evidence.md`
- `docs/work-packages/20260701-wshedw4-typed-watershed-network-frame-001/artifacts/source-guard-evidence.md`
- `docs/work-packages/20260701-wshedw4-typed-watershed-network-frame-001/artifacts/operand-lineage.md`
- `docs/work-packages/20260701-wshedw4-typed-watershed-network-frame-001/artifacts/protected-output-evidence.md`
- `docs/work-packages/20260701-wshedw4dc01-typed-routing-kernel-writeback-closure-001/package.md`
- `docs/work-packages/20260701-wshedw4dc01-typed-routing-kernel-writeback-closure-001/artifacts/required-reading-map.md`
- `docs/work-packages/20260701-wshedw4dc01-typed-routing-kernel-writeback-closure-001/artifacts/correction-authority-envelope.md`

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
  `docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md`,
  `docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md`,
  and
  `docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md`.

On-demand:

- `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/dispatch.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/**`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/types.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
- `tests/integration/**`
- `tests/fixtures/watershed/carnivorous-adobo/README.md`
- adjacent modules touched by the implementation.

Required-reading budget: `OK`; map:
`docs/work-packages/20260701-wshedw4dc01-typed-routing-kernel-writeback-closure-001/artifacts/required-reading-map.md`.

Files:

- `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/dispatch.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/**`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/types.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/mod.rs`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
- `tests/integration/**` for typed routing/source-guard/protected-output tests
- `docs/specifications/science-contracts/**` only if contract amendments are
  required
- `docs/work-packages/20260701-wshedw4dc01-typed-routing-kernel-writeback-closure-001/**`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`

Task: close `WSHED-W4-HOLD-001` end to end. Implement frame-native watershed
dispatch so the public CLI routes through `WatershedNetworkFrame` without
materializing `WatershedWritebackSurface`; replace compatibility-harvested
publication with typed routed-state publication; prove protected output
identity or contract-governed deltas; record conservation reconstruction and
closure/magnitude audit.

Correction Authority Envelope: use the envelope in `package.md` and
`artifacts/correction-authority-envelope.md`. If the package establishes a
reproducible root cause inside the envelope and authority supports the expected
behavior, proceed through contract amendment when needed, contract-derived
tests, pre-implementation gate, production correction, validation, and review.
Do not close as `HOLD` merely because further investigation is possible.

Constraints:

- Do not preserve production routing by wrapping `WatershedWritebackSurface` in
  a typed facade.
- Do not use producer-only, adapter-only, shadow-only, source-counter-only, or
  test-only evidence for closure.
- Do not change routing, impoundment, sediment, runoff-partition, water-balance,
  latest-event, or output-schema semantics without canonical contract authority.
- Do not loosen fail-closed guards, silently clamp, invent physics, or
  canonicalize invalid domains away.
- Keep remaining old-surface code path-scoped as replay/comparator/diagnostic or
  obsolete-test code only.

Conservation/output acceptance: record operand lineage updates if needed;
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
W4DC01 gate execution, protected-output evidence, review, and verification;
outputs: compact metrics/findings plus log or artifact paths; write access:
read-only.

Validation:

- Run focused iteration gates as needed.
- Final complete closure requires:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo nextest run --workspace --profile full`
  - `cargo deny check`
  - focused W4DC typed routing/source-guard/protected-output/conservation gates
- Run scoped docs lint for package artifacts and touched docs.
- Run `git diff --check`.
- If any complete gate cannot run, close `EXECUTED-HOLD` only at a legitimate
  boundary named in `package.md`.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update implementation, tests, correction-authority evidence,
seven-gate bar, contract-gate evidence, consumer-path proof, source-guard
evidence, protected-output evidence, conservation reconstruction, line-count
governance, review/verification disposition, roadmap, work-package README, and
final disposition.
