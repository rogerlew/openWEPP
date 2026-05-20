# ADR-0011: Architecture-first delivery with top-down science contracts

**Status:** Accepted
**Date:** 2026-05-20
**Deciders:** Roger Lew, Codex

## Context

openWEPP needs a delivery strategy that can move forward without waiting on
full legacy kernel decomposition.

The legacy implementation remains useful, but commonblock mutation structure and
runtime/tooling fragility make it unsuitable as a sole architectural template or
gold behavioral oracle for all execution surfaces.

## Decision

1. openWEPP adopts an **architecture-first** implementation strategy:
   - define typed state, module boundaries, orchestration flow, and contracts
     first;
   - implement kernels against these contracts instead of waiting for
     one-for-one legacy re-kernelization completion.
2. openWEPP science contracts are authored **top-down** from:
   - WEPP technical references (including the `references/50201000` corpus),
   - peer-reviewed literature invariants,
   - physical/common-sense invariants (for example: melt cannot exceed
     available snowpack),
   - static legacy code inspection used as secondary evidence for discovered
     state transitions and ordering.
3. Legacy binary comparison is a **flagging mechanism for investigation**, not a
   universal acceptance oracle.
4. Comparator confidence tiers are explicit:
   - higher confidence: single OFE and daily water-balance surfaces;
   - lower confidence: hourly and watershed surfaces, where comparator deltas
     trigger investigation rather than automatic rejection.
5. openWEPP remains explicitly non-clean-room. Legacy source may be read
   directly for static analysis and provenance mapping.
6. openWEPP supports an initial backward-compatibility bridge for legacy
   stdin-driven `.run` plus `.txt` sidecar flags/inputs while architecture
   and contracts mature.
   - Compatibility support is explicit and validated.
   - Missing/ambiguous sidecar requirements are hard errors.
   - No silent fallback behavior is allowed.

## Consequences

- openWEPP can progress core Rust architecture and module contracts immediately.
- Science authority moves to explicit contracts and citations instead of
  inherited runtime behavior alone.
- Legacy comparisons remain valuable, but interpreted by confidence tier.
- Reviews and promotion gates emphasize:
  - contract correctness and invariant satisfaction,
  - internal closure and conservation behavior,
  - comparator deltas triaged by confidence tier.
