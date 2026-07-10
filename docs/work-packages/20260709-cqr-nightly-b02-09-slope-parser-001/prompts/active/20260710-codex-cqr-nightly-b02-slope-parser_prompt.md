# CQR Nightly Batch 02 Target 09 Kickoff

Scope: local repository engineering task; flat-file reads/edits only; no
external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in `package.md` sequentially through
disposition.

Required reading:

- Core: `AGENTS.md`, `crates/AGENTS.md`, `tests/AGENTS.md`,
  `docs/work-packages/AGENTS.md`,
  `docs/work-packages/20260709-cqr-nightly-b02-09-slope-parser-001/package.md`,
  `docs/work-packages/cqr-nightly-burndown-execplan.md`,
  `docs/standards/mechanical-refactor-authoring-guide.md`,
  `docs/standards/code-quality-refactor-authoring-guide.md`,
  `docs/decisions/0021-module-coverage-closure-thresholds.md`,
  `docs/standards/prompt-wording-guidance.md`, and
  `artifacts/required-reading-map.md`.
- Conditional: `docs/specifications/science-contracts/AGENTS.md` only if
  contract authority, conservation-sensitive output, or contract-derived tests
  are touched.
- On-demand: `crates/openwepp-input-contract/src/parsers/slope.rs` and
  `tests/integration/infile_slope_parser_contract.rs`.

Required-reading budget: OK; map:
`docs/work-packages/20260709-cqr-nightly-b02-09-slope-parser-001/artifacts/required-reading-map.md`.

Files:

- `crates/openwepp-input-contract/src/parsers/slope.rs`
- `tests/integration/infile_slope_parser_contract.rs`
- `docs/work-packages/20260709-cqr-nightly-b02-09-slope-parser-001/**`
- `docs/work-packages/README.md`

Task: reduce `SlopeParserError::fmt` and `parse_slope_str` to CRAP `<= 30`
behavior-preservingly, or record a legitimate ADR-0021-style hold.

Constraints: preserve public API, file grammar, typed errors, guard IDs,
tolerances, numeric thresholds, fail-closed behavior, and parser output meaning.
No silent defaults, no surrogate behavior, no production unwrap/expect.

Subagent requirement: REQUIRED: spawn `comparator_suite_runner` for all heavy
batch/closure/comparator runs, including full workspace clippy, full nextest,
deny, and any expensive coverage/comparator work. This prompt explicitly
authorizes subagent spawning/delegation to review, verification, and
comparator/closure-runner roles for behavior-preserving CQR review, metric
verification, focused/full gate execution, and output-identity checks; outputs:
compact metrics plus package artifacts/log paths; write access: read-only unless
explicitly assigned a bounded package fix.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts, disposition every review finding, and commit
either completion or hold evidence before Target 10.
