# ADR-0010: Provenance model is explicitly non-clean-room direct port

**Status:** Accepted
**Date:** 2026-05-11
**Deciders:** Roger Lew, Codex

## Context

openWEPP had language describing a "clean-room kernel-mirror" approach. That is
not the intended operating model for this project.

The intended model is direct, transparent migration from upstream
`wepp-palimpsest` Fortran kernels into Rust with contract and oracle controls.
This policy must be explicit and discoverable in openWEPP governance.

## Decision

1. openWEPP is explicitly **not** a clean-room rewrite.
2. Kernel ports may directly read and map upstream Fortran source, associated
   science contracts, and oracle vectors.
3. Upstream `wepp-palimpsest` kernels remain the authoritative behavioral source
   until replaced by accepted openWEPP routines under parity gates.
4. Provenance is documented through attribution headers and ADR/contract links,
   not by source-isolation constraints.
5. Historical clean-room wording is superseded and retained only as archive
   context.

## Consequences

- Contributor guidance is simpler and aligns with current execution reality.
- Port velocity improves because source access is explicit rather than implied.
- Review focus remains on parity, contracts, and numerical behavior, not
  artificial source-separation rules.
- Governance documents must avoid ambiguous clean-room language unless referring
  to a narrowly scoped replacement class.
