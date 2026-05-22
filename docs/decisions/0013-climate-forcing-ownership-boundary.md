# ADR-0013: Climate forcing ownership boundary across hillslope and watershed surfaces

**Status:** Accepted
**Date:** 2026-05-22
**Deciders:** Roger Lew, Codex

## Context
ADR-0004 and ADR-0006 define the production watershed execution boundary as
subprocess hillslope runs that exchange data via HBP shards. CLIM02 also
implemented parser-to-runtime climate seams in both orchestrator crates,
including a watershed climate-assignment seam (`WS-CLIM-SEAM-001`) used by
in-process dispatch/runtime integration tests.

CLIM04 accepted finding `CLIM04-RVW-003` identified an ambiguity: watershed
runtime climate-assignment behavior exists in code, while architecture narrative
describes watershed routing as HBP-driven.

## Decision
1. Cross-binary climate routing authority remains HBP-first:
   - production watershed routing consumes completed hillslope HBP outputs.
   - watershed runtime must not require direct climate parser payloads when HBP
     outputs are the execution boundary.
2. Hillslope climate adaptation authority is canonical for climate-to-kernel
   symbol projection (`HS-CLIM-SEAM-001`).
3. Watershed climate-assignment seam authority is scoped:
   - `WS-CLIM-SEAM-001` is allowed for in-process orchestration surfaces
     (integration probes, parity instrumentation, and shared-adapter migration
     work).
   - `WS-CLIM-SEAM-001` does not supersede HBP as the authoritative
     cross-binary routing contract.
4. Climate-file-to-hillslope assignment ownership belongs to orchestration
   layers upstream of hillslope execution; watershed may carry assignment
   metadata but may not reinterpret or re-own climate physics after HBP handoff.

## Consequences
- CLIM11 resolves the architecture ambiguity in `CLIM04-RVW-003` without
  changing validated runtime code paths.
- CLIM12 shared-adapter extraction must preserve this boundary (single shared
  implementation, unchanged ownership contract).
- CLIM13 typed forcing migration must preserve the same authority split while
  replacing dynamic symbol synthesis.
