# Worker Handoff

Status: complete.

## Handoff

R7C final disposition:
`COMPLETE-R7C-PRODUCTION-DIRECT-EXECUTOR-PATH`.

What changed:

- Added explicit opt-in production direct selection:
  `HillslopeRuntimeSelection::DirectProductionExecutor`.
- Added CLI flag `--direct-production-executor`.
- Routed production direct mode through `DirectFrameExecutor` and retained
  direct publication artifacts for output writing.
- Skipped compatibility symbol-registry audit and indexed-shadow diagnostic
  adapter setup for production direct selection.
- Added manifest-visible direct runtime counters for production direct mode.
- Preserved default compatibility behavior and R6J direct-publication cutover
  behavior.

Current proof:

- Focused R7C fixture passes with nonzero direct phase/counter evidence,
  `scheduler_kernel_executed=false`, `publication_source=direct-publication-frame`,
  `skeleton_runs=0`, and `compatibility_edge_invocations=0`.
- H2637 direct production executes all `235961` OFE-days through direct
  counters and records `direct_phase_entries=8494599`.
- Default compatibility remains default and records `direct_runtime_counters=null`.
- Full Rust closure gates, scoped docs lint, and `git diff --check` passed.

Important residual blockers:

- R7D is next for publication producer authority and parity. Direct production
  H2637 output checksums currently differ from compatibility for HBP, PASS, and
  WAT. Do not activate direct mode until R7D closes this.
- R7G must profile and remediate direct production performance and RSS. Current
  same-binary H2637 evidence is direct production
  `753.76 s / 625132 KB` versus default compatibility
  `642.77 s / 228804 KB`.
- R7E default activation remains blocked until direct publication parity,
  rollback behavior, and performance gates are clean.
- R7F compatibility isolation/deletion remains blocked until direct production
  no longer depends on compatibility-adapter authority for output/publication
  evidence and activation gates are ready.

Suggested next package:

- R7D direct publication producer authority. Start by making direct production
  HBP/WAT/PASS/loss/manifest projection parity-clean from executor-owned typed
  publication producers, with static scans proving production direct
  publication does not read compatibility WB13 rows or runtime surfaces as
  direct authority.
