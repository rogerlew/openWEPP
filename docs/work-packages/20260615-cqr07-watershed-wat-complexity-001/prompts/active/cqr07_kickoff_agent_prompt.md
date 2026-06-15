# CQR07 Kickoff Agent Prompt

Scope: local repository flat-file edits only in `/home/workdir/openWEPP`.

Autonomy: execute the package end-to-end through source refactor, validation,
artifact updates, dual review/verification, and disposition unless a declared
hard blocker is reached.

Objective: decompose
`crates/openwepp-runner/src/watershed_wat.rs` so `read_batch_into` no longer
needs `#[allow(clippy::too_many_lines)]`, preserving WAT reader, aggregation,
optional-column, fail-closed, and public publication behavior.

Required reading budget:

- local bytes total: to be recorded in `artifacts/required-reading-map.md`
- disposition: complete-with-warnings
- map: `artifacts/required-reading-map.md`

## Required Reading

Core:

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260615-cqr07-watershed-wat-complexity-001/package.md`
- `/home/workdir/openWEPP/crates/AGENTS.md`

Conditional:

- `/home/workdir/openWEPP/docs/standards/AGENTS.md`
- `/home/workdir/openWEPP/docs/standards/mechanical-refactor-authoring-guide.md`
- `/home/workdir/openWEPP/docs/standards/code-quality-refactor-authoring-guide.md`
- `/home/workdir/openWEPP/docs/standards/module-test-enhancement-authoring-guide.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/AGENTS.md`
- `/home/workdir/openWEPP/docs/decisions/0021-module-coverage-closure-thresholds.md`

On-demand:

- `/home/workdir/openWEPP/docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/package.md`

## Execution Instructions

1. Record baseline line counts, lint suppression census, coverage, and CRAP
   metrics before production decomposition.
2. Run focused WAT module tests before production edits:
   `cargo test -p openwepp-runner watershed_wat::tests -- --nocapture`.
3. Add targeted reader characterization before production decomposition if
   baseline coverage does not exercise `read_batch_into`.
4. Extract cohesive internal blocks from `read_batch_into` into private helpers
   in the same file. Preserve expression grouping, statement order, branch
   order, alias lookup, optional all-null handling, and fail-closed errors.
5. Remove `#[allow(clippy::too_many_lines)]` only after the function is below
   the lint threshold.
6. Run focused checks, then the required closure gates:
   `cargo fmt --check`;
   `cargo clippy --workspace --all-targets -- -D warnings`;
   `cargo test --workspace`;
   `cargo deny check`.
7. Update all package artifacts with `Static:` and `Ran:` evidence labels.
8. Complete dual reviews, finding disposition, dual verification, final
   disposition, and worker handoff.

Stop conditions:

- Focused WAT characterization fails before production edits.
- A necessary change would alter WAT output formulas, operand lineage,
  thresholds, alias compatibility, optional defaults, public API, or contract
  authority.
- Required tooling is unavailable and no package-conforming fallback can record
  evidence truthfully.
