# ADR-0002: Clean-room model is kernel-mirror port

**Status:** Accepted
**Date:** 2026-05-11
**Deciders:** Roger Lew, Claude Code

## Context
A Rust reimplementation of WEPP could be authored under several clean-room models:

- **(A) Pure clean-room from science contracts** — agents read only contract specs and oracle vectors, never F90 source. Cleanest provenance; blocked on wepp-palimpsest science-contract completeness, which is mostly empty as of 2026-05.
- **(B) Kernel-mirror port** — agents may read both the corresponding wepp-palimpsest `.f90` kernel and its science contract once the upstream kernel has reached `active` status with published oracle vectors. Faster than (A); preserves the wepp-palimpsest discipline.
- **(C) Translation-aided greenfield** — agents read legacy F90 freely. Fastest; re-imports the regression coefficients and patch lineage the wepp-palimpsest WB-3x program is explicitly trying to escape.

## Decision
**Model (B).** openWEPP Rust kernels port from wepp-palimpsest F90 kernels only after the upstream kernel has reached `active` status with oracle vectors. Agents reference both the F90 source and the science contract during a port.

Implication: openWEPP's kernel cadence is downstream of wepp-palimpsest's kernel cadence. The hillslope soil and plant kernels currently lack upstream kernel status; openWEPP cannot port them until upstream completes.

## Consequences
- openWEPP progress is gated on wepp-palimpsest kernelization progress. Visible in work-package dependency chains.
- Empirical multi-coefficient regressions are rejected per the wepp-palimpsest `WB33-C-20` process-based kernel preference rule, carried forward.
- Every kernel constant must trace to a hydraulic citation in the science contract. "The F90 has this number" is not a sufficient justification.
- Patches and inline tunings in the legacy F90 do not transit to Rust; the science contract is the authoritative specification.
