# Execute Gate Planner Governance Authority Alignment

Scope: local repository governance engineering; flat-file reads/edits and
focused local validation only; no external systems or network actions.

Execution mode: package-end-to-end.

Required reading: use
`artifacts/required-reading-map.md`.

Files: only the declared write set in `package.md`.

Task: execute roadmap Order 1 end-to-end. Apply ADR-0043 to operative guidance,
guards, policy identity, and frozen package statuses. Preserve independently
binding correctness, science, assurance, quality, security, review, and Harvard
requirements.

Constraints: direct manual landing; no planner/TESTGATE admission; no planner
executable, CI, CAL, science, kernel, publication, or protected-data changes.

Subagent requirement: this prompt explicitly authorizes subagent
spawning/delegation to two independent read-only reviewers and two independent
read-only verifiers for the scopes named in `package.md`; outputs are compact
findings/verification with exact paths; write access is read-only. No heavy
runner or comparator is selected.

Autonomy: execute all phases through disposition without requesting additional
direction unless hard-blocked.
