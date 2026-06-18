# Proposed ADR-0023 - Array-Authoritative Hot-Path State

Evidence class: Proposed decision record text. Not ratified.

## Status

Proposed by PERFARCH02. Supersedes the hot-path authority portion of ADR-0022
while preserving ADR-0022's `SymbolRegistry`, `SymbolId`, sorted-order, and
logical export requirements.

## Context

ADR-0022 introduced a run-scoped indexed representation as a read mirror beside
the existing logical `HillslopeWritebackSurface`. PERFIDX04 showed the read
mirror can improve indexed reads, but PERFIDX05 showed writeback/guard work
regresses when logical and indexed representations must both be maintained.
PERFIDX03 also showed that full logical export at the kernel seam erases the
benefit of making the indexed representation authoritative too locally.

PERFIDX06 measured the endpoint at 73.12x legacy no-UI on H2637. The profile
spread across symbol-keyed map machinery as a class, leaving no narrow
write-side id-table lever.

PERFARCH02 built a standalone floor prototype that reused the real writeback
contract types and compared current logical writeback/guard evaluation against
a dense `SymbolId` array candidate. The candidate was bit-identical on the
prototyped success path, fail-closed on invalid payloads, preserved message id
class, and measured about 49.9x faster for the prototyped writeback/guard
surface.

## Decision

The openWEPP hot path will move from logical-map authority plus indexed mirror
to dense array authority keyed by `SymbolId`.

`HillslopeWritebackSurface` remains a logical contract and publication surface.
It is not the daily mutable kernel-loop store. Logical maps may be materialized
from dense state at input, output, diagnostics, tests, and explicit compatibility
boundaries.

Kernel writeback payloads will gain id-backed forms, or kernels will write
directly to dense slots where ownership permits. Finite/domain guard semantics,
fail-closed behavior, and diagnostic message ids must remain equivalent to the
current logical writeback path. Logical symbol names are resolved on failure or
publication paths, not on the success hot path.

## Consequences

Positive:

- removes the PERFIDX03 per-day export seam;
- removes the PERFIDX05 dual-write cost;
- keeps ADR-0022 registry/sorted-id foundation;
- aligns runtime state with the fixed-array legacy performance model without
  clean-room claims or physics changes.

Negative:

- changes the kernel writeback payload shape;
- touches scheduler lane authority, kernel request/response contracts, guard
  helpers, transfer helpers, and publication boundaries;
- requires staged identity and timing gates before any broad migration.

## Required Gates

- integrated WB11 array-authoritative pilot before broad production migration;
- bit-identical logical materialization for every flipped flow;
- same-machine H2637 timing compared with PERFIDX06;
- no normal-path logical + array dual-write;
- no daily full `BTreeMap` export at the kernel seam;
- standard Rust and markdown gates for touched scopes.

## Non-Decisions

This ADR does not activate irrigation, change science contracts, alter HBP or
parquet schemas, or remove logical surfaces from public/reporting boundaries.
