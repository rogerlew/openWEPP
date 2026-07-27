# Kickoff

Scope: local repository assurance-lifecycle engineering; flat-file reads and
edits only; no external connectivity.

Execution mode: package-end-to-end.

Required reading: use `artifacts/required-reading-map.md`.

Required-reading budget: 96,605 core bytes, `OK`.

Task: close `ASSURANCE-CANOPY-README-IDENTITY-001` through a typed,
review-invalidating report-source transaction, exact validation, and
CAL04B-NATIVE hold lift.

Constraints: never edit generated hashes directly; never fabricate or carry
human review authority to changed bytes; preserve fail-closed transaction and
source validation.

Subagent requirement: REQUIRED. This prompt explicitly authorizes subagent
spawning/delegation to two read-only assurance reviewers, the
`comparator_suite_runner` for heavy full-workspace execution, and two read-only
terminal verifiers. Outputs are compact verdicts, counts, hashes, receipt IDs,
and artifact paths; write access remains with the primary executor.

Autonomy: execute all package phases without user intervention unless a proven
hard boundary blocks safe DRAFT custody.
