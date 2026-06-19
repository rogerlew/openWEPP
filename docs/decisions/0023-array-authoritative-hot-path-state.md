# ADR-0023: Array-authoritative hot-path state

**Status:** Accepted
**Date:** 2026-06-18 UTC
**Deciders:** Roger Lew (operator ratification), Codex (draft and implementation)
**Builds on:** [ADR-0022](0022-indexed-runtime-surface-representation.md)
**Work package:**
`docs/work-packages/20260618-perfmig01-wb11-runoff-array-authoritative-production-migration-001/`

## Context

ADR-0022 introduced a run-scoped indexed representation as a read mirror beside
the existing logical `HillslopeWritebackSurface`. PERFIDX04 showed the read
mirror can improve indexed reads, but PERFIDX05 showed writeback and guard work
regresses when logical and indexed representations are both maintained as normal
hot-path authority. PERFIDX03 also showed that full logical export at the kernel
seam erases much of the benefit of making the indexed representation locally
authoritative.

PERFIDX06 measured the H2637 endpoint at 73.12x legacy no-UI. The profile spread
across symbol-keyed map machinery as a class, leaving no narrow write-side
id-table lever.

PERFARCH02 built a standalone floor prototype that reused the real writeback
contract types and compared current logical writeback/guard evaluation against a
dense `SymbolId` candidate. The candidate was bit-identical on the prototyped
success path, fail-closed on invalid payloads, preserved message-id class, and
measured about 49.9x faster for the prototyped writeback/guard surface.

PERFARCH03 then measured a fully array-native WB11 warm-rain runoff branch floor:
`0.959423 us/OFE-day` for the combined array hot loop, validated to exact numeric
`to_bits()` identity against the current production branch output set. That
clears the <=5x and <=10x viability budgets for the measured branch and
authorizes the production migration program.

## Decision

The openWEPP hot path will move from logical-map authority plus indexed mirror to
dense array authority keyed by `SymbolId`.

`HillslopeWritebackSurface` remains a logical contract and publication surface.
It is not the daily mutable kernel-loop store. Logical maps may be materialized
from dense state at input, output, diagnostics, tests, and explicit compatibility
boundaries.

Kernel writeback payloads may use id-backed forms, or kernels may write directly
to dense slots where ownership permits. Finite/domain guard semantics,
fail-closed behavior, and diagnostic message ids must remain equivalent to the
current logical writeback path. Logical symbol names are resolved on failure or
publication paths, not on the normal success hot path.

The migration is staged. Each production rung must name its migrated branch or
phase boundary, prove exact identity for every flipped flow, and measure endpoint
effect before authorizing the next rung.

## Consequences

Positive:

- removes the PERFIDX03 per-day full-export seam from migrated hot paths;
- removes the PERFIDX05 normal-path logical plus indexed dual-write cost;
- keeps the ADR-0022 registry, sorted-id, and fail-closed foundation;
- aligns runtime state with the fixed-array legacy performance model without
  clean-room claims or physics changes.

Negative:

- changes kernel writeback payload shape for migrated paths;
- touches scheduler lane authority, kernel request/response contracts, guard
  helpers, transfer helpers, and publication boundaries;
- requires staged identity and timing gates before any broad migration.

## Required gates

- bit-identical logical materialization for every flipped flow;
- same-machine H2637 timing compared with PERFIDX06 for production rungs;
- no normal-path logical plus array dual-write inside migrated hot loops;
- no daily full `BTreeMap` export at migrated kernel seams;
- standard Rust and Markdown gates for touched scopes.

## Non-decisions

This ADR does not activate irrigation, change science contracts, alter HBP or
parquet schemas, or remove logical surfaces from public/reporting boundaries.
