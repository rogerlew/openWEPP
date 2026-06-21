# Independent Reconstruction Plan

Evidence mode: Static.

R6C must add or run reconstruction that does not call the production direct
projection builder under test.

Required families:

- HBP binary payload values reconstructed from typed publication operands.
- WAT rows reconstructed from direct row operands and metadata.
- PASS rows reconstructed from direct row operands and volume basis.
- Loss JSON reconstructed from typed publication operands and static run
  metadata.

Manifest reconstruction is covered separately by
`manifest-cutover-proof.md`.

## Execution Disposition

Independent reconstruction remains blocked for accepted public outputs because
there are no accepted production direct operands to reconstruct. The next
package must add retained direct producers first, then implement reconstruction
per family before accepting parity.
