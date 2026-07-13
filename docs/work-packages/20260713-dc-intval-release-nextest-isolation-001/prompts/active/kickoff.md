# Close INTVAL-REL-001 End To End

Scope: local openWEPP release-harness engineering; no external connectivity.

Execution mode: `package-end-to-end`.

Task: close defect `INTVAL-REL-001` exactly as bounded in `package.md`. Add the
source guard first, replace the stale threaded workspace libtest release lane
with canonical full nextest isolation, and run the package's exact pinned-input release
command through required authority and stability. Do not alter production,
H2637 assertions, fixtures, thresholds, or skip any lane.

Subagent requirement: **REQUIRED**. This prompt explicitly authorizes subagent
spawning/delegation to `comparator_suite_runner`, two reviewers, and two
verifiers for the scopes and bounded artifact writes in `package.md`.

Autonomy: execute all phases through correction or a legitimate named boundary;
do not stop at intermediate diagnosis.
