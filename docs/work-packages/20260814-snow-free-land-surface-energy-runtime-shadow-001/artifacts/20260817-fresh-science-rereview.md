# Fresh Science Re-review

Evidence class: `Static + Ran`

Verdict: `PASS for bounded runtime correction / Child-3 terminal HOLD retained`

The first review pass held because the litter test exercised only a scalar
helper. That accepted finding was remediated. The final exact-byte review
confirmed that the test constructs a projected tile with immutable beginning
litter water `W_l=2.0 kg m-2 tile`, configured top-soil conductivity
`1.1 W m-1 K-1`, and litter thickness `0.04 m`; executes
`ValidatedV8RuntimeInputProjection::solver_ready_tiles`; independently
reconstructs `0.1 + 0.03*W_l/(1000*dz_l)`; observes the actual solver-ready
surface and soil-node operands; proves non-aliasing; and changes only the
beginning litter store to prove surface conductivity changes while the soil
node remains bit-identical.

The review also confirmed each LSE tile's configured VIS/NIR albedos are the
sole E01--E03 lower boundary, heterogeneous covered optics execute through the
strict endpoint, and `SnowFreeForcing` albedo is not consumed or equality-gated
as ground optics.

Ran by reviewer: focused projection test 1/1 PASS. No material science or
ownership findings remain in the bounded runtime correction. V3/V5 protected
bytes are untouched. Child 3 remains HOLD on the separately scoped
contract-first oracle reconciliation and subsequent clean full-workspace gate.
