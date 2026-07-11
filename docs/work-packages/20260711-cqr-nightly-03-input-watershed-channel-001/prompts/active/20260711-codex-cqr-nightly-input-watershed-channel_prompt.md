# CQR Nightly Batch 01 Target 03 Kickoff

Scope: local behavior-preserving CQR; flat-file worktree reads/edits only; no
external connectivity. Execution mode: package-end-to-end.

Required reading: Core/Conditional/On-demand tiers in `package.md` and
`artifacts/required-reading-map.md`. Required-reading budget: `124648` local
bytes, `OK`; map: `artifacts/required-reading-map.md`.

Task: execute all phases through reviewed completion or legitimate local hold.
Preserve token/record consumption order, strict/compatibility branching, field
values, warnings, typed errors, public API, and accepted-input behavior.

Coverage closure: characterization edits require ADR-0021 tier, line/region
floors, per-function region-floor disposition, and A-H obligation bindings
before decomposition may close.

Subagent requirement: REQUIRED. This prompt explicitly authorizes subagent
spawning/delegation to comparator/closure, review, and verification roles for
heavy metrics/gates, dual review, and dual verification; outputs are compact
verdicts/log paths; access is read-only unless a bounded fix is assigned.

Autonomy: proceed without user direction unless hard-blocked. Commit scaffold
before implementation and completion/hold before target 04.
