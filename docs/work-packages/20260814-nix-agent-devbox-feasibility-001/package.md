# Establish The Nix Agent Development Box And Prove Cutover Feasibility

Status: `ACTIVE / ow-dev-01 intake complete / waiting for forest gate release`

Date: `2026-08-14`

Package ID: `20260814-nix-agent-devbox-feasibility-001`

Branch: `infra/nix-agent-devbox-feasibility`

Plan class: `Developer-infrastructure implementation and comparative feasibility`

## Objective

Make `ow-dev-01` a reproducible, concurrency-safe openWEPP agent development
box using the Nix package manager, then compare it fairly with `forest` and cut
primary development over as soon as the current work on `forest` lands and the
evidence shows that `ow-dev-01` is operationally better. Retain `forest` as a
high-memory or heavy-gate fallback until a later explicit retirement decision.

## Current Baseline And Cutover Trigger

- Branch base: `af9a989063aa8751dfadb14c442e1b360653658c`.
- `ow-dev-01`: 10 physical/16 logical i7-13620H CPUs, 32 GiB RAM, encrypted
  NVMe-backed 503 GiB `/tmp`, and encrypted 1.3 TiB `/workdir`.
- `forest`: 24 physical/48 logical Xeon E5-2697 v2 CPUs and 125 GiB RAM.
- At intake, `forest` was running an active openWEPP full-workspace nextest
  campaign. No benchmark, worktree inspection, cache mutation, checkout
  mutation, or competing heavy job is permitted there until that work lands
  and `forest` is confirmed idle.
- The comparison commit is the exact terminal commit containing the current
  `forest` work after it lands. Record it before preparing either benchmark
  checkout. Both machines must use that commit and the same `flake.lock`.
- Cutover is evidence-triggered, not date-triggered. If the acceptance criteria
  pass, make `ow-dev-01` the primary agent-development host immediately after
  preserving the benchmark record and verifying a clean operational checkout.

## Scope

In scope:

- a pinned multi-user Nix development environment expressed as a repository
  flake and lock file;
- Rust, rustfmt, Clippy, cargo-nextest, cargo-deny, sccache, mold/Clang, Python,
  Git LFS, native build dependencies, and benchmark/diagnostic tools;
- optional direnv integration that fails explicitly when prerequisites are
  missing;
- shared Cargo download and sccache locations on `/workdir`;
- isolated Cargo target directories and worktrees per agent/task;
- absolute, checkout-external `/tmp` scratch directories per gate run;
- local concurrency admission rules for focused and heavy Cargo work;
- identical, non-destructive feasibility tests on both machines;
- an evidence-bound cutover and rollback checklist.

Out of scope:

- application/science behavior changes;
- changing validation authority or weakening any package gate;
- benchmarking `forest` while another agent or heavy Cargo process is active;
- deleting, cleaning, or reusing another agent's checkout, target directory,
  sccache, Cargo home, logs, or `/tmp` tree;
- pushing this branch, opening a PR, retiring `forest`, or changing production
  runtime defaults without separate user direction.

## Intended Write Set

- `flake.nix`, `flake.lock`, and an optional `.envrc`;
- bounded developer-environment configuration or scripts under `tools/dev/`;
- `.gitignore` only for new untracked local-environment outputs;
- this package tree;
- `docs/ROADMAP.md` and `docs/work-packages/README.md` for lifecycle state.

No Rust production source or science/authority contract is in the intended
write set.

## Concurrency Model To Prove

The normal package workload is bursty rather than uniformly parallel:

1. one implementation agent performs edit/check loops;
2. several reviewers perform read-heavy inspection and focused validation;
3. at most two focused Cargo compilations run concurrently;
4. exactly one comparator/heavy Cargo campaign runs at a time;
5. every active worktree receives its own `CARGO_TARGET_DIR`;
6. agents share Cargo downloads and sccache, not target directories;
7. full-workspace Clippy, nextest, release builds, and other heavy gates are
   serialized per host.

Initial tuning candidates are 8, 12, and 16 Cargo/nextest worker slots on
`ow-dev-01`. The selected value must consider focused-loop latency, sustained
CPU frequency, memory pressure, and total throughput rather than shortest
heavy-run time alone.

## Phase Plan

### Phase 0 — Preserve And Record The Host Baseline

- [x] Confirm encrypted ext4 `/tmp` and `/workdir` mounts.
- [x] Confirm the daily systemd tmpfiles cleanup timer.
- [x] Override `/tmp` retention from the Ubuntu 10-day default to 3 days on
  `ow-dev-01`; verify configuration precedence and perform a dry run only.
- [x] Add a tracked host-setup check that reports, but does not silently mutate,
  required mount, permission, cleanup, disk-space, and timer state.

### Phase 1 — Implement The Pinned Nix Development Shell

- [x] Install multi-user Nix on `ow-dev-01` using the official installer and
  record the installed Nix version and daemon state.
- [x] Add and lock the minimal openWEPP development flake.
- [x] Prove `nix develop` supplies every canonical developer and gate command.
- [x] Keep user authentication, secrets, and machine-specific credentials out
  of the flake and repository.
- [x] Record closure size and verify the Nix store leaves adequate root-disk
  headroom.

### Phase 2 — Implement Agent Isolation And Rust Optimization

- [x] Define stable `/workdir` locations for Cargo home, sccache, worktrees,
  per-task target directories, and benchmark results.
- [x] Define unique absolute `/tmp` scratch allocation outside every checkout.
- [x] Add collision/ownership checks that reject a target directory already
  owned by another active task.
- [x] Benchmark Cargo incremental compilation against
  `CARGO_INCREMENTAL=0` plus shared sccache.
- [x] Benchmark the system linker against Clang plus mold without committing a
  global optimization before evidence exists.
- [x] Select bounded Cargo/nextest concurrency and a one-heavy-run-per-host
  admission mechanism.

### Phase 3 — Freeze The Landed Comparison Identity

- [ ] Wait for the current `forest` package to reach a truthful terminal commit
  and for its agent to release the checkout and heavy-run resources.
- [ ] Record the exact landed Git commit, `flake.lock` hash, Git LFS state, and
  required external fixture identities.
- [ ] Prepare fresh isolated benchmark checkouts on both machines without
  using or mutating the active development checkout or its target/cache state.
- [ ] Verify both shells report identical Rust, Cargo, nextest, deny, linker,
  Python, and native dependency versions.

### Phase 4 — Run The Comparative Feasibility Matrix

- [ ] Measure a clean dependency build with empty target and sccache disabled.
- [ ] Measure a fresh-worktree build with a warm shared sccache.
- [ ] Measure warm no-change and representative incremental edit/check loops.
- [ ] Measure affected-crate Clippy and focused nextest.
- [ ] Measure `cargo build --release -p openwepp-runner --bins`.
- [ ] Measure workspace strict Clippy and full nextest.
- [ ] Measure the realistic concurrency mix: one heavy gate, two isolated
  focused Cargo jobs, and read-heavy repository inspection.
- [ ] Run at least three measured repetitions after preparation; do not drop OS
  caches, alter governor/power policy asymmetrically, or benchmark a busy host.
- [ ] Record wall/user/system time, maximum RSS, CPU frequency/throttling,
  disk I/O, swap activity, sccache statistics, test counts, exit status, and
  exact command/environment identity.

### Phase 5 — Decide And Cut Over

- [ ] Prefer `ow-dev-01` when it materially improves median focused-loop
  latency and remains responsive during the realistic concurrency mix, all
  commands are correct, no thermal/memory failure occurs, and its full-gate
  throughput is operationally acceptable.
- [ ] If accepted, designate `ow-dev-01` as primary, preserve `forest` as an
  idle-only heavy/high-memory fallback, and run the first real package with the
  new isolation policy.
- [ ] Record rollback conditions: repeated OOM, thermal collapse, cache or
  target collisions, nonreproducible Nix shell, or unacceptable full-gate
  throughput.
- [ ] Reconcile roadmap/catalog state and archive the active prompt.

## Validation And Evidence

- `nix flake check` and a clean `nix develop` smoke test;
- canonical focused openWEPP checks selected from repository guidance;
- the exact comparative benchmark matrix above;
- configuration checks proving unique target and scratch directories;
- a no-interference audit for `forest`;
- a benchmark report with raw machine-readable samples and a concise decision;
- an operational cutover smoke package on `ow-dev-01`.

Benchmark performance never substitutes for correctness. Every compared Cargo
or nextest command must exit successfully with the same selected tests and
result meaning before its timing is admissible.

## Delegation

No subagent delegation is required to scaffold or implement the environment.
If later independent benchmark review is desired, add explicit authorization
before delegation; do not infer it from this package.

## Exit Criteria

Close only when the Nix environment is reproducible, agent worktrees and Cargo
targets are collision-safe, `/tmp` cleanup and capacity checks are operational,
the same landed commit has valid measurements on both idle machines, the
cutover decision follows the declared evidence, and the first real
`ow-dev-01` package completes without environment-caused failure.

## Decision Log

- Decision: use a 3-day age policy for the dedicated NVMe `/tmp` and retain the
  daily systemd cleanup timer. Rationale: openWEPP evidence records slow scratch
  and target accumulation plus prior ENOSPC failure; three days preserves
  short-lived investigation artifacts while bounding abandoned agent trees.
  Date/Author: 2026-08-14 / Codex at user direction.
- Decision: defer all `forest` benchmarks until the active work lands and the
  host is idle. Rationale: benchmark validity and noninterference both require
  isolation from the current full-workspace campaign. Date/Author: 2026-08-14
  / Codex at user direction.
- Decision: enable `nix-command` and `flakes` in the multi-user daemon
  configuration before flake implementation. Rationale: Nix 2.35.2 installed
  successfully but left both features disabled, so the planned locked
  development shell could not execute. The daemon-store query and a pure
  evaluation passed after the bounded configuration change. Date/Author:
  2026-08-14 / Codex at user direction.
- Decision: serialize heavy commands with `tools/dev/heavy`, default heavy
  Cargo/nextest work to 8 workers, and retain 16 workers only as an explicit
  idle-host override. Rationale: the corrected 8+4+4 concurrent intake improved
  both focused-job latencies relative to 12+4+4 for only a 0.58-second heavy
  workspace penalty; a nonblocking host-wide lock also prevents accidental
  overlap across worktrees. Date/Author: 2026-08-14 / Codex at user direction.
- Observation: the Phase 3 readiness poll still found the protected `forest`
  package running full-workspace nextest, so comparison checkout preparation
  remains intentionally deferred. Date/Author: 2026-08-14 / Codex.
