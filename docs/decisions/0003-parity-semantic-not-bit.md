# ADR-0003: Parity target is semantic, not bit-for-bit

**Status:** Accepted
**Date:** 2026-05-11
**Deciders:** Roger Lew, Claude Code

## Context
The oracle for openWEPP correctness is the wepp-palimpsest binary. A parity contract could require bit-for-bit reproduction (which demands identical FP order, FMA settings, summation strategy across compilers, OSes, and CPU vendors) or semantic parity (closure correctness, trajectory shape, physically-meaningful tolerance bounds at named state surfaces).

Bit-for-bit parity across the Rust / GFortran boundary is impractical: different compilers, intrinsic libraries, and reduction orders produce different last-bit results even when the algorithm is the same. The wb-fuzzing program in wepp-palimpsest has already demonstrated FP-order sensitivity inside the legacy codebase itself.

## Decision
Parity target is **semantic**, not bit-for-bit.

Semantic parity is defined per state surface in the corresponding science contract, with named tolerance bounds. Default bounds and per-domain overrides are specified in [../numerics/README.md](../numerics/README.md) and per-kernel science contracts.

## Consequences
- The oracle harness is a tolerance-aware diff, not a byte diff.
- Cross-core / cross-platform bit-for-bit reproducibility is not a release gate. Within a single deterministic configuration (single thread, pinned RNG seed) runs must remain bit-reproducible run-to-run on the same target.
- Reduces compiler / CPU lock-in; openWEPP can compile under any LLVM-supported target without per-target parity tuning.
- Requires that science contracts specify tolerance bounds explicitly. Kernel ports without a tolerance bound in the contract block on contract authoring.
