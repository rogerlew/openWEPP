# Numerics Policy

## Parity target
**Semantic parity, not bit-for-bit.** See [../decisions/0003-parity-semantic-not-bit.md](../decisions/0003-parity-semantic-not-bit.md).

Semantic parity is defined per state surface in the corresponding science contract, with named tolerance bounds. The oracle harness diffs trajectories against named tolerances.

## Within-config determinism
A single openWEPP run with a single thread of execution and a pinned RNG seed must be bit-reproducible run-to-run on the same target. Cross-thread and cross-platform bit reproducibility are not required.

## Floating-point types
- Default to `f64` for state-bearing arithmetic.
- `f32` is permitted only for I/O surfaces or where a science contract names it explicitly.
- No implicit promotion or demotion across kernel boundaries.

## Reduction order
Pinned per kernel. Where summation order materially affects results (per the wb-fuzzing evidence in wepp-palimpsest), the kernel documents its reduction strategy in code and the contract.

## FMA
Off by default. Enabled per kernel only with documented justification in the science contract.

## RNG
- Stochastic kernels use `rand_chacha::ChaCha20Rng` (or a successor pinned per release).
- Seed sourcing and propagation are part of the `.run` schema.
- The same seed produces the same trajectory across runs on the same target.

## Open work
The above are commitments. Specific kernel-level numerics decisions are made in each kernel-port work package, citing this policy.
