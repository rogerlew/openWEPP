# ADR-0031: Delete the compatibility runtime - single-authority terminal state

**Status:** Accepted
**Date:** 2026-06-30 UTC
**Deciders:** Roger Lew, Codex
**Supersedes:** the `--compatibility-runtime` seam-retention clause of
[ADR-0030](0030-r7-terminal-contract-and-compatibility-runtime-deletion.md)
**Amends:** [ADR-0025](0025-array-native-hillslope-day-frame.md)
**Ratification provenance:** accepted by
`20260630-compatibility-runtime-full-deletion-001`.
**Evidence:** the typed seed-authority re-architecture
(`20260630-typed-day-zero-seed-computation-001/`, the single-authority cutover at
`45852c25`), the prior `20260630-compatibility-runtime-deletion-001`, and the
direct-publication RSS packages.

## Context

ADR-0030 retained the explicit `--compatibility-runtime` replay/comparator seam
"for now... while the direct runtime and RSS work settle," and anticipated a
later full-deletion package that removes replay needs.

Those conditions are now met. The typed seed-authority re-architecture is
complete: production direct is seeded from a typed `DirectProductionSeedAuthority`
with no symbol-map seed surface, persistent lane symbol-map state, registry, or
hot tables in production setup. H2637 remains byte-identical,
`cargo nextest run --workspace --profile full` passed, direct production reports
`compatibility_edge_invocations=0`, RSS is run-length-flat, and direct production
is the no-env default for all supported hillslope surfaces.

Re-examined, the seam's residual value as a cheap replay oracle does not hold:

- **No consumer.** The only thing reaching the symbol-map runtime is the
  deprecated `--compatibility-runtime` flag and legacy scheduler tests. There is
  no `openwepp-replay` binary and no diagnostics path uses it.
- **Not a correctness authority.** The old symbol-map runtime is openWEPP's own
  prior implementation, not legacy WEPP, not the science contracts, and not the
  byte-identity fixtures. A/B against it only shows two openWEPP implementations
  agree; the typed one is now authoritative.
- **It decays.** As the typed runtime gains new physics, the frozen symbol-map
  runtime diverges by design, making the comparator useless for future changes.
- **Rollback is better served by release and git history** than a live in-tree
  runtime flag.

The seam's migration-validation purpose is spent. Retaining it now carries cost:
a stale alternate runtime, maintenance load, and silent-divergence risk without
durable benefit.

## Decision

Supersede ADR-0030's seam-retention clause and authorize full deletion of the
symbol-map runtime, reaching the single-authority terminal state:

- Delete `scheduler.rs`, `day_frame.rs`, and the carrier types
  (`HillslopeWritebackSurface`, `HillslopeKernelRequest`, `KernelWritebackPayload`,
  `SymbolRegistry`, and their production/seed paths) plus their legacy scheduler
  tests and residual vestigial symbol-map plumbing in production setup.
- Remove the `--compatibility-runtime` runtime selection and the `Compatibility`
  executor.
- The typed `DirectRunFrame` becomes the sole runtime representation.
  Symbol-keyed types survive only where genuinely required by intake/output
  serialization edges, never as an executable alternate runtime.
- Rollback for the deletion is the prior release or git commit recorded in the
  release notes, not a live runtime flag. This satisfies the rollback spirit of
  the array-native activation rule: a rollback path exists, but it need not be
  in-tree executable code.

### Execution scope (2026-06-30): runtime selection removed; carrier-type deletion is a separate program

This package executed the self-contained half of the decision: the public
`--compatibility-runtime` selector and the `Compatibility` executor are **removed**,
so the symbol-map runtime is **unreachable** from any production input, user flag, or
topology. The single-authority terminal state is achieved for production **runtime
selection** (direct-only).

The carrier types (`HillslopeWritebackSurface`, `HillslopeKernelRequest`,
`KernelWritebackPayload`, `SymbolRegistry`) are **not** the compatibility runtime's
private code — they are the **kernel-invocation boundary**, referenced at ~1007 sites
across ~56 source files (notably ~20 hydrology-kernel files and ~20 runner files) plus
`openwepp-kernel-contract`. Deleting them is therefore not a unit deletion but a
re-typing of the entire kernel request/writeback interface across the hydrology
physics. That **kernel-boundary typing is a separate, deliberately-scoped program**,
not this package; forcing it through 1007 references would violate the deletion
discipline. Per §0 of the array-native spec, the carrier types accordingly **survive
for now as the kernel-invocation interface**, with their elimination deferred to that
program (the remaining symbol-map-free frontier, and a likely contributor to the
`<=5x` ideal). `scheduler.rs`/`day_frame.rs` deletion rides with that same untangling.

## Consequences

Positive:

- True single-authority for production **runtime selection**: the typed frame is the
  only reachable runtime; the symbol-map runtime is unreachable.
- Removes the stale alternate-runtime *selection* and its silent-fallback risk.
- Advances the ADR-0025 terminal state; the symbol-map-free codebase (carrier-type
  deletion = kernel-boundary typing) is the remaining backlogged frontier.

Negative / costs:

- No live `--compatibility-runtime` A/B oracle remains. Diffing against the old
  runtime requires checking out the prior release or rollback commit.
- A future `openwepp-replay`, if built, must be direct-native.
