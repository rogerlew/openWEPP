# Kickoff: Local CI Timing and Profile Optimization

Execute `docs/work-packages/20260705-local-ci-timing-profile-optimization-001/`.

The task is workflow/tooling and docs only. Do not change kernel behavior.
Implement persistent local nextest timing diagnostics, empirically evaluate
nextest capped group concurrency on the `forest` development machine, and
install concise agent-discoverable guidance for tiered local CI gate selection.

Required evidence:

- script compile/smoke evidence,
- nextest concurrency sweep evidence,
- documentation link/path consistency,
- `git diff --check`.

Subagent authorization: this package explicitly authorizes
spawning/delegating to read-only review subagents for local-CI tooling,
nextest-profile, and documentation-soundness review; expected outputs are
review findings with severity, evidence, and recommended disposition; write
access is read-only.
