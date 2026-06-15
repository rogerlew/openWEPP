# CQR08 Kickoff Agent Prompt

Scope: local repository flat-file edits only in `/home/workdir/openWEPP`.

Autonomy: execute the package end-to-end through source refactor, validation,
artifact updates, dual review/verification, and disposition unless a declared
hard blocker is reached.

Objective: decompose
`crates/openwepp-hillslope-orchestrator/src/runtime_inputs/00_core_types.rs` so
`HillslopeRuntimeInputError::fmt` no longer needs
`#[allow(clippy::too_many_lines)]`, preserving stable error codes, display text,
typed variant semantics, runtime projection guard behavior, and public API
behavior.

Required reading budget:

- local bytes total: to be recorded in `artifacts/required-reading-map.md`
- disposition: complete
- map: `artifacts/required-reading-map.md`

## Required Reading

Core:

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260615-cqr08-runtime-core-types-display-001/package.md`
- `/home/workdir/openWEPP/crates/AGENTS.md`

Conditional:

- `/home/workdir/openWEPP/docs/standards/AGENTS.md`
- `/home/workdir/openWEPP/docs/standards/mechanical-refactor-authoring-guide.md`
- `/home/workdir/openWEPP/docs/standards/code-quality-refactor-authoring-guide.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/AGENTS.md`
- `/home/workdir/openWEPP/docs/decisions/0021-module-coverage-closure-thresholds.md`

On-demand:

- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests/core_types.rs`

## Execution Instructions

1. Record baseline line counts, lint suppression census, coverage, and CRAP
   metrics before production decomposition.
2. Run focused runtime-input tests before production edits:
   `cargo test -p openwepp-hillslope-orchestrator runtime_inputs::tests -- --nocapture`.
3. Add targeted `Display`/`code()` characterization before production
   decomposition.
4. Extract cohesive internal display-formatting groups from `fmt` into private
   helpers in the same file. Preserve each format string, argument order, and
   `self.code()` call.
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

- Focused runtime-input characterization fails before production edits.
- A necessary change would alter error codes, display text, typed variant
  semantics, guard behavior, thresholds, aliases, symbols, public API, or
  contract authority.
- Required tooling is unavailable and no package-conforming fallback can record
  evidence truthfully.
