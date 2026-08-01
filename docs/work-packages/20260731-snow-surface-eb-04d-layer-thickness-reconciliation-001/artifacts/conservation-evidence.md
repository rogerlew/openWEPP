# Conservation Evidence

Status: `PASS`

Evidence class: `Ran`

The real-consumer captured-fragment vectors retain mass, physical depth,
liquid, refrozen mass, cold content, density after the authorized daily
compaction/cap, and settle state after its one-day increment. A targeted
partial-sublimation vector additionally conserves a represented `5e-10 m` SWE
remainder and its coupled liquid/refrozen/cold-content state.
A separate Stage 3 target-trim vector leaves the same represented remainder
while preserving every coupled layer operand under the authorized proportional
mass-removal transformation.
The across-layer vector removes one complete `2e-6 m` layer, continues the
remaining `5e-10 m` request below the aggregate residual tolerance, and retains
the next `2e-9 m` layer at `1.5e-9 m`; liquid, refrozen mass, and cold content
all scale to `0.75`. Any final negative floating correction uses the same
coupled-state scaling rather than bare ice-mass subtraction.

The package independently parses and reconstructs the frozen EB-04A typed
snapshots (5 Harvard layers and 14 Marcell layers). The corrected replay then
reconstructs each emitted daily aggregate from serialized layer vectors, not
producer aggregate fields.

Across both complete 16,437-day trajectories:

- maximum absolute layer-SWE versus runtime-SWE residual:
  `1.4432899320127035e-15 m`;
- maximum absolute layer-depth versus runtime-depth residual:
  `3.3306690738754696e-16 m`;
- both original rejection boundaries pass; and
- both processes exit successfully.

Wrong-unit filtering is distinguished by the exact captured values. Tolerance
inflation is rejected at the exact next representable value above `1e-9 m`,
while equality remains the unchanged acceptance boundary and the typed error
is preserved outside it.
