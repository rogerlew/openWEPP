# R7H Closure And Activation Gates

Status: executed-held.

Final disposition: `HOLD-R7H-H2637-DIRECT-PERFORMANCE-AND-PROTECTED-PARITY`.

Package type: Defect-Closure ExecPlan / closure-gate execution / activation
guard.

Roadmap item: `docs/ROADMAP.md`, R7G winter-column follow-up item 7,
"Closure and activation gates."

## Objective

Rerun the R7G closure matrix after the ADR-0026 snow/frost winter-column
migration and decide whether direct production may advance to R7H release
readiness or direct-default activation. This package may only activate the
direct production runtime as the normal/default path after all closure gates
pass on current code.

## Required Reading

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `crates/AGENTS.md`
- `docs/ROADMAP.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/architecture/coupled-frost-sub-solver-specification.md`
- `docs/decisions/0026-stateful-winter-column-sub-solver.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/work-packages/20260624-r7g-consumer-cutover-deletion-001/package.md`
- `docs/work-packages/20260623-r7g-iterative-completion-001/package.md`

## Scope

- Execute current-code H2637 direct default-candidate, explicit direct, default
  compatibility, and explicit rollback runs when the local H2637 fixture is
  available.
- Compare protected public outputs: HBP, WAT, PASS, loss, plot, and manifest
  checksum/metadata parity.
- Verify direct runtime counters, especially
  `compatibility_edge_invocations=0`.
- Run source/static gates for no winter hot-path compatibility/symbol-surface
  authority after snow/frost bridge deletion.
- Add or tighten focused tests only when a current closure gate lacks in-repo
  coverage.
- If every closure gate passes, promote the runtime selection/default
  activation path with explicit rollback evidence.
- If any closure gate fails, do not activate by default; close with a named
  `HOLD-R7H-*` blocker and package-local worker handoff.

## Out Of Scope

- Process-physics formula changes.
- Heuristic snow/frost retuning to force parity.
- Deleting compatibility rollback, replay, diagnostic, or shadow modes.
- R7H release documentation beyond the gate disposition required to decide
  whether R7H may proceed.

## Terminal Rule

This package has exactly two honest terminal states:

1. `COMPLETE-R7H-CLOSURE-ACTIVATION-GATES`: H2637 direct default candidate and
   explicit direct complete within the `<=10x` legacy budget, protected outputs
   are byte/Arrow identical to compatibility, manifest metadata parity passes,
   direct counters report `compatibility_edge_invocations=0`, winter hot-path
   source scans find no compatibility/symbol-surface authority, snow/frost
   anti-alias fixtures and independent operand reconstruction pass, rollback
   remains explicit and proven, and default activation is made safe or handed to
   R7H release readiness with a green gate packet.
2. `HOLD-R7H-<SPECIFIC-GATE>`: a current closure gate fails or is unavailable
   for a concrete reason, default activation remains disabled, and
   `artifacts/worker-handoff.md` names the first implementation correction
   required to clear the blocker.

The package must not claim complete activation while any protected parity,
performance, no-compatibility, reconstruction, anti-alias, or rollback gate is
red or unrun.

## Closure Gates

- H2637 direct default candidate: exits `0`, `compatibility_edge_invocations=0`,
  and wall time is `<=10x` the recorded legacy reference.
- H2637 explicit direct: exits `0`, direct manifest/publication provenance, and
  same protected output checksums as direct default candidate.
- H2637 default compatibility and explicit rollback: exit `0`, scheduler
  publication provenance, identical protected outputs to each other.
- Compatibility versus direct protected outputs: HBP byte identity; WAT/PASS/
  plot Arrow identity; loss JSON identity; manifest output checksum and
  metadata parity except intentional runtime-selection provenance.
- Source/static winter hot-path gate: no production references to
  `DirectFrostRunoffSurface`, `BoundarySymbol`, `BoundaryValue`,
  `HillslopeWritebackSurface`, or `HillslopeKernelRequest` in the direct
  winter-column hot path, excluding named compatibility/test/comparator
  adapters.
- Focused in-repo gates: default activation policy, explicit rollback,
  no-compatibility runtime counters, deleted bridge source scans, anti-alias
  fixture coverage, and independent snow/frost operand reconstruction.
- Final Rust/workspace gates if production code changes:
  `cargo fmt --check`; `cargo clippy --workspace --all-targets -- -D warnings`;
  `cargo test --workspace`; `cargo deny check`; `git diff --check`.

## Progress

- [x] Scaffold package.
- [x] Read governing roadmap/specification/ADR/work-package context.
- [x] Build current release CLI for same-binary H2637 matrix.
- [x] Run current H2637 direct default-candidate endpoint loop.
- [x] Implement in-envelope closure corrections exposed by H2637 direct:
  no-material frost storage delta suppression and stale coarse frozen-layer
  clearing.
- [x] Compare protected outputs against the retained compatibility capture for
  blocker characterization. Current-code compatibility rerun was skipped after
  the direct performance gate failed.
- [x] Run focused in-repo closure/source tests.
- [x] Decide activation/hold disposition.
- [x] Record review, verification, line-count governance, and worker handoff.
- [x] Update catalog/roadmap/specification links as needed.

## Final Disposition

Executed-held. The direct default-candidate H2637 run now reaches endpoint with
`compatibility_edge_invocations=0`, `scheduler_kernel_executed=false`, and
`publication_source=direct-publication-frame`, but it misses the performance
gate (`113.53 s` observed versus the `91.2 s` `<=10x` legacy budget). A retained
compatibility-capture comparison also remains red for HBP, WAT, PASS, loss, and
plot, so protected-output parity is not activation-ready.

Default activation remains disabled. R7H release readiness/direct-default
activation must not proceed until the follow-up closes
`HOLD-R7H-H2637-DIRECT-PERFORMANCE-AND-PROTECTED-PARITY`.
