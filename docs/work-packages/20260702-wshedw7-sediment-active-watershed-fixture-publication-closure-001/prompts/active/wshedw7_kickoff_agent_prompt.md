# WSHED-W7 Handoff Prompt

Scope: local repository Rust implementation, fixture-adoption, and
conservation-publication task for openWEPP; flat-file reads/edits plus local
validation commands only; no external connectivity.

Standing user authorization for openWEPP subagent delegation is expected in the
session:
`For openWEPP work-package execution, I explicitly authorize Codex to`
`spawn/delegate to subagents whenever the active work package, AGENTS.md, or`
`package governance requires or authorizes review, verification, comparator`
`execution, or parallel agent work.`

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in
`docs/work-packages/20260702-wshedw7-sediment-active-watershed-fixture-publication-closure-001/package.md`
sequentially through disposition. Do not stop at analysis unless hard-blocked.

First actionable item: inventory committed and local candidate full watershed
fixtures for actual nonzero sediment response, then select the W7 acceptance
fixture path or record the exact hold boundary if no valid full fixture can be
adopted in scope.

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
- `docs/work-packages/20260702-wshedw6-publication-large-watershed-scaling-001/package.md`
- `docs/work-packages/20260702-wshedw6-publication-large-watershed-scaling-001/artifacts/review-disposition.md`
- `docs/work-packages/20260702-wshedw6-publication-large-watershed-scaling-001/artifacts/scaling-matrix-evidence.md`
- `docs/work-packages/20260702-wshedw6-publication-large-watershed-scaling-001/artifacts/publication-operand-lineage.md`
- `tests/fixtures/watershed/carnivorous-adobo/README.md`
- `tests/fixtures/watershed/onshore-xenophobia/README.md`
- `docs/work-packages/20260702-wshedw7-sediment-active-watershed-fixture-publication-closure-001/package.md`
- `docs/work-packages/20260702-wshedw7-sediment-active-watershed-fixture-publication-closure-001/artifacts/required-reading-map.md`

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
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md` and pinned
  baseline source files only if W7 becomes a legacy migration/parity package.

On-demand:

- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/src/watershed_supervisor.rs`
- `crates/openwepp-watershed-orchestrator/src/**`
- `crates/openwepp-watershed-output/src/**`
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
- `tests/integration/**`
- `tests/fixtures/watershed/**`
- local `/wc1/runs/**/watershed` candidates as source substrates only.

Required-reading budget: `380992` bytes, `OK`; map:
`docs/work-packages/20260702-wshedw7-sediment-active-watershed-fixture-publication-closure-001/artifacts/required-reading-map.md`.

Files:

- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/src/watershed_supervisor.rs`
- `crates/openwepp-watershed-orchestrator/src/**`
- `crates/openwepp-watershed-output/src/**`
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
- `tests/integration/**`
- `tests/fixtures/watershed/**`
- `docs/specifications/science-contracts/**` only if contract amendments are
  required
- `docs/work-packages/20260702-wshedw7-sediment-active-watershed-fixture-publication-closure-001/**`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`

Task: execute WSHED-W7 end to end. Adopt or prove a committed full watershed
fixture with actual nonzero sediment response, run public watershed serial and
parallel execution through typed dispatch/publication, compare all required
parquet outputs, and independently reconstruct sediment-sensitive publication
operands.

Constraints:

- Do not subset the watershed fixture for acceptance.
- Do not make `/wc1`, scratch, or wepppy paths persistent fixture or benchmark
  evidence.
- Do not implement W8 channel-balance authority, W9 `NoEvent` authority, W10
  `chan.inp` default authority, or watershed CQR maintenance unless the package
  is amended before implementation.
- Do not change routing, impoundment, sediment, erosion, runoff-partition,
  water-balance, latest-event, or output-schema semantics without canonical
  contract authority.
- Preserve `--jobs 1` as the deterministic default; explicit `--jobs N`
  remains the CPU-scaling mechanism.

No surrogate physics: production code must keep actual contract-backed or
baseline-authoritative physics. Surrogate/provisional/proxy/heuristic
stand-ins are forbidden. Missing authority is a hold-for-authority boundary;
known in-scope physics is an implementation obligation.

Real consumer proof: prove the public watershed CLI and output writers consume
the typed watershed frame/publication path for the sediment-active fixture.
Wrappers, adapters, skeletons, shadow paths, and old compatibility paths cannot
carry the W7 closure claim.

Conservation/output acceptance: record operand lineage; separate plausible
aliases in fixtures; reject known wrong formulas; run independent
reconstruction plus real sediment magnitude/closure audit; align
metadata/schema; do not close on one-sided bounds or self-consistency alone.

Subagent requirement: REQUIRED. Spawn/delegate `comparator_suite_runner` for
release builds, full watershed serial/parallel runs, output-identity
comparison, protected-output comparator runs, and full closure gates when
available; do not run heavy batches on the parent model unless the subagent is
unavailable, and record command-level evidence if unavailable. This prompt
explicitly authorizes subagent spawning/delegation to
`comparator_suite_runner`, `fixture_inventory_agent`, `rust_code_reviewer`,
`rust_qa_reviewer`, and `science_contract_reviewer` for W7 fixture discovery,
output identity, review, and verification; outputs: compact metrics/findings
plus log or artifact paths; write access: `fixture_inventory_agent` may make
bounded package-artifact updates if available, all others read-only.

Validation:

- Run focused iteration gates as needed.
- Final complete closure requires:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo nextest run --workspace --profile full`
  - `cargo deny check`
  - focused W7 fixture, source-guard, output-identity, publication, and
    conservation-reconstruction gates
  - fixture checksum manifest validation for each accepted or newly adopted
    fixture
- Run scoped docs lint for package artifacts and touched docs.
- Run `git diff --check`.
- If any complete gate cannot run, close `EXECUTED-HOLD` only at a legitimate
  boundary named in `package.md`.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update implementation, tests, fixture manifests, sediment fixture
inventory, publication lineage, consumer-path proof, source-guard evidence,
output identity evidence, conservation reconstruction, line-count governance,
review and verification artifacts, roadmap, work-package README, worker
handoff, and final disposition.
