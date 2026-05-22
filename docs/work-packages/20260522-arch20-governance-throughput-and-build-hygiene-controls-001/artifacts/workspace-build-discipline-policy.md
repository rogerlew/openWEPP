# Workspace Build-Discipline Policy

Evidence mode: `Static`
Status: `complete`

## Source Finding Linkage

- [DIRECT] `docs/work-packages/20260522-arch14-claude-architecture-review-disposition-001/artifacts/disposition-register.md` (`CRF-009`: documented build-output discipline policy required).
- [DIRECT] `docs/work-packages/20260522-arch14-claude-architecture-review-disposition-001/artifacts/claude-review-findings-register.md` (`CRF-009`: nested `target/` hygiene drift).
- [DIRECT] `find /home/workdir/openWEPP -type d -name target` (current snapshot includes workspace root plus crate/worktree-local target directories).

## Policy Scope

This policy governs local developer and agent execution behavior for Rust build
and test commands in `/home/workdir/openWEPP`.

## Normative Command Discipline

1. `BD-001`: Required closure gates for code-touch packages MUST run from
   workspace root `/home/workdir/openWEPP`.
2. `BD-002`: Crate-local `cargo` commands MAY be used for fast iteration, but
   they MUST NOT be used as closure gate evidence.
3. `BD-003`: Gate command lines in artifact evidence MUST be exact and
   reproducible.
4. `BD-004`: Shell interpolation MUST NOT be used for CLI subprocess contract
   checks; explicit arg arrays are required per ADR-0004 posture.

## Normative Build-Output Hygiene

1. `BH-001`: Workspace canonical build output directory is
   `/home/workdir/openWEPP/target`.
2. `BH-002`: `crates/*/target` directories are prohibited for closure runs and
   SHOULD be removed when discovered.
3. `BH-003`: `.worktrees/*/target` directories are allowed for isolated worker
   worktrees, but they MUST remain inside `.worktrees/` and MUST NOT be cited
   as canonical workspace gate outputs.
4. `BH-004`: Any package that reports build hygiene state MUST include the
   `find ... -name target` evidence snapshot used for its claim.

## Required Commands for Code-Touch Closure

1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo deny check`

## Optional Cleanup Procedure

Use only when cleanup is intended and safe:

```bash
find /home/workdir/openWEPP/crates -type d -name target -prune -print
```

Then remove listed crate-local directories explicitly (no wildcard mass-delete)
after confirming they are non-authoritative artifacts.

## Compliance Failure Handling

- Any code-touch closeout that lacks workspace-root gate evidence is `HOLD`.
- Any closeout that hides failing gates behind partial crate-local runs is
  `HOLD`.
- Build-hygiene drift discoveries are low-severity operational findings unless
  they invalidate reported gate evidence.
