# Kickoff

Scope: local repository gate-policy engineering; flat-file reads/edits only;
no external connectivity.

Execution mode: package-end-to-end.

Task: close `TESTGATE-ASSURANCE-HISTORICAL-ROOT-001` without moving historical
registry roots or weakening authority checks.

Subagent requirement: REQUIRED. This prompt explicitly authorizes subagent
spawning/delegation to a bounded implementation worker, two read-only
reviewers, a comparator runner, and two terminal verifiers. Outputs are compact
findings, counts, run IDs, and artifact paths.
