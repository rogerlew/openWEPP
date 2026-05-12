# ADR-0001: License is CC0-1.0

**Status:** Accepted
**Date:** 2026-05-11
**Deciders:** Roger Lew, Claude Code

## Context
WEPP originated as USDA-ARS public-domain software. The wepp-palimpsest re-authoring program (LCR-01..LCR-07) standardized on CC0-1.0 for `.f90` kernels. openWEPP is a fresh Rust reimplementation and needs a license posture set before code lands.

## Decision
openWEPP is licensed CC0-1.0. SPDX identifier `CC0-1.0` is required in per-crate `Cargo.toml` metadata and in per-file headers where convention warrants.

Dependency license posture: no viral copyleft. `cargo deny` denies GPL / AGPL / LGPL by allow-list exclusion. MPL-2.0 (weak per-file copyleft) is also excluded by default; a specific MPL-2.0 dependency may be added with documented justification once a real need arises.

## Consequences
- Maximum reusability; aligns with the wepp-palimpsest CC0 posture and preserves the USDA public-domain provenance.
- No attribution requirement; this is appropriate for science software and matches the upstream convention.
- `cargo deny` is a hard CI gate. Adding a copyleft dependency requires a deliberate, documented exception.
- Rust ecosystem norm is MIT / Apache-2.0 dual. Most Rust crates are CC0-compatible.
