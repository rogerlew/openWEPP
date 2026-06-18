# PERFARCH03 Disposition

Evidence class: Static + Ran.

Status: complete 2026-06-18.

Verdict: **GO - branch floor clears <=5x and <=10x.**

## Decision

PERFARCH03 finds no physics-floor blocker in the measured WB11 runoff branch.
The fully array-native branch hot loop measured `0.959423 us/OFE-day`, which is:

- below the `193 us/OFE-day` <=5x budget;
- below the `386 us/OFE-day` <=10x budget;
- about `146.8x` faster than the current logical production kernel on the same
  synthetic branch.

This is a GO for the full array-authoritative migration program, not a claim
that production H2637 is already solved. The prototype is artifact-local and no
production runtime path was changed.

## Required follow-on

Revive the ADR-0023 direction under a new execution package that migrates real
production phases to array-authoritative inputs, state, and outputs. The follow-
on package must preserve these PERFARCH03 constraints:

- no logical payload construction inside migrated hot loops;
- no per-phase `from_logical_payload` or equivalent logical materialization;
- boundary seed/materialize only at temporary validation boundaries, measured
  separately until removed;
- exact identity fixtures for every migrated branch;
- H2637 timing and RSS evidence after each staged migration rung.

The first migration package should start with WB11 runoff because PERFARCH03
already proves that branch's array-native floor and output write set.

## Non-goals closed by this package

PERFARCH03 does not authorize:

- broad production edits without a new migration package;
- treating the PERFARRAY02 `817.810 us/OFE-day` input-only pilot as the floor;
- claiming full H2637 <=5x or <=10x until all migrated phases are measured
  through the real endpoint.

## Gate results

| Gate | Result |
|---|---|
| Prototype `cargo fmt` | PASS |
| Prototype `cargo check` | PASS |
| Prototype `cargo clippy -- -D warnings` | PASS |
| Prototype release build | PASS |
| Branch bit-identity validation | PASS |
| Release timing run | PASS |
| Array-only `perf record/report` | PASS |
| Source line-count check | PASS, prototype `src/main.rs` is 969 lines |

Workspace Rust gates were not rerun because PERFARCH03 adds only an artifact-
local prototype outside the production workspace and documentation artifacts. A
full migration package must run the workspace gates when it changes production
Rust.
