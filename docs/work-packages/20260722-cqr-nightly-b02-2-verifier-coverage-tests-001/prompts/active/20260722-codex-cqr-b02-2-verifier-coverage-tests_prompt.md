# CQR B02-2 Verifier Coverage Tests Prompt

Execution mode: package-end-to-end.

Autonomy: continue through in-scope characterization, decomposition, metrics,
review, verification, and disposition without intervention; stop only at a
declared hold boundary.

Required reading:

Core: root and crate/work-package instructions, the package, nightly ExecPlan,
mechanical/CQR guides, ADR-0021, and prompt wording guidance.

Conditional: testing/gate selection standards for terminal reconciliation.

On-demand: the target module and adjacent verifier fixture consumers.

Required-reading budget: 205,498 bytes, WARN because the binding governance and
target source exceed the preferred quick-intake size; map:
`artifacts/required-reading-map.md`.

Task: execute the package for
`crates/openwepp-gate-planner/src/verifier_coverage_tests.rs` end-to-end.
Characterize `replace_string` before behavior-preserving decomposition and
reduce it and every extracted helper to CRAP at most 30.

Subagent requirement: REQUIRED: spawn `comparator_suite_runner` for any heavy
gate selected by the terminal plan and for the final batch qualification. This
prompt explicitly authorizes subagent spawning/delegation to comparator,
closure-runner, review, and verification subagents. Outputs are package-local
review/verification artifacts and retained logs; write access is read-only
unless a bounded implementation correction is explicitly assigned.

Do not push, deploy, switch branches, manually dispatch TESTGATE, run HEAVY on
the parent, or rerun unchanged expensive evidence.
