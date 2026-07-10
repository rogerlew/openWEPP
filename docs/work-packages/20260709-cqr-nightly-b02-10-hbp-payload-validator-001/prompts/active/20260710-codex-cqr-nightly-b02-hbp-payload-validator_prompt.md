# CQR Nightly Batch 02 Target 10 — HBP Payload Validator Prompt

Scope: local repository code-quality refactor task; flat-file reads/edits only
inside `/home/workdir/openWEPP`; no external connectivity required.

Execution mode: package-end-to-end.

Phase plan: execute all phases in
`docs/work-packages/20260709-cqr-nightly-b02-10-hbp-payload-validator-001/package.md`
sequentially through disposition.

Required reading:

Core:

- `AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/20260709-cqr-nightly-b02-10-hbp-payload-validator-001/package.md`
- `docs/work-packages/cqr-nightly-burndown-execplan.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/work-packages/20260709-cqr-nightly-b02-10-hbp-payload-validator-001/artifacts/required-reading-map.md`

Conditional:

- `docs/specifications/science-contracts/AGENTS.md` and the nearest relevant
  `SC-*` contract only if the target touches contract authority,
  conservation-sensitive outputs, or contract-derived tests.
- `docs/standards/local-ci-gate-selection.md` if focused iteration gates need to
  be narrowed before the final closure loop.

On-demand:

- `crates/openwepp-input-contract/src/parsers/hbp/payload_validator.rs`
- `tests/integration/infile_hbp_parser_contract.rs`
- adjacent HBP parser modules imported by the payload validator

Required-reading budget: `167561` bytes, `WARN`; map:
`artifacts/required-reading-map.md`.

Files:

- `crates/openwepp-input-contract/src/parsers/hbp/payload_validator.rs`
- `tests/integration/infile_hbp_parser_contract.rs`
- `docs/work-packages/20260709-cqr-nightly-b02-10-hbp-payload-validator-001/**`

Task: execute the CQR nightly package for
`crates/openwepp-input-contract/src/parsers/hbp/payload_validator.rs`
end-to-end. Reduce every eligible production function above CRAP `30` to
`<= 30`, or record an ADR-0021-style disposition and hold when
behavior-preserving CQR cannot close the target safely.

Constraints:

- Behavior-preserving CQR only.
- No HBP binary-format, schema, guard ID, threshold, unit, contract-authority,
  serialization, fail-closed, or public-output semantic changes.
- Preserve byte-read order, cursor consumption, numeric scaling, and
  short-circuit behavior.
- No opportunistic cleanup outside the declared target.
- Do not edit unrelated dirty files or active-package work.
- Package-required validation overrides ambient test-skip guidance.

Coverage closure: if characterization tests are added or materially changed,
record ADR-0021 tier assignment, line/region threshold status, per-function
region-floor disposition, and obligation-to-test binding in
`artifacts/coverage-closure.md` before decomposition closes.

Hold handling: local target holds roll back only current-package
production/test implementation edits to the scaffold baseline, preserve and
commit package hold evidence, and may close batch 02 as held. Global/process
holds stop the nightly batch. Do not revert unrelated user changes.

Subagent requirement: REQUIRED: spawn `comparator_suite_runner` for all heavy
batch/closure/comparator runs, including `cargo nextest run --workspace --profile
full`, full-workspace CRAP/coverage runs after implementation, comparator
suites, and population/fixture batches. Do NOT run those heavy gates on the
parent model unless the subagent is unavailable; if unavailable, record
command-level evidence before running locally. This prompt explicitly authorizes
subagent spawning/delegation to comparator/closure-runner, review, and
verification subagents for CQR metric checks, focused/full gate execution,
behavior-identity verification, review, and verification. Outputs:
`artifacts/review_agent_a.md`, `artifacts/review_agent_b.md`,
`artifacts/verification_agent_a.md`, `artifacts/verification_agent_b.md`,
compact metrics, command logs, and artifact paths. Write access: read-only
unless a subagent is explicitly assigned a bounded implementation fix in
`crates/openwepp-input-contract/src/parsers/hbp/payload_validator.rs`,
`tests/integration/infile_hbp_parser_contract.rs`, or package-local artifacts.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update all package artifacts, disposition every review finding, and
leave the package ready for its completion or hold commit.

