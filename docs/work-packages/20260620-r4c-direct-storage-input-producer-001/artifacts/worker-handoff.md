# R4C Worker Handoff

Status: complete.
Evidence mode: Static.

R4C completed the direct WB12 storage-input producer and storage-module split.

Carry forward boundaries:

- no default activation;
- no publication/schema cutover;
- no scheduler edits;
- no compatibility storage/request/writeback/symbol lookup inside direct
  runtime;
- no use of R3B diagnostic ledger as storage authority;
- default-disabled H2637 median `<= 676.67 s`.

Recommended next route: scaffold a post-R4C producer for one remaining explicit
R4B storage operand. Prefer the operand with the strongest already-authored
contract authority and lowest publication risk. Current candidates:

- ET producer for `evapotranspiration_m`;
- deep-seepage producer for `deep_seepage_m`;
- subsurface-loss producer for `subsurface_loss_m`;
- snow-coupling producer for `snow_coupling_m`.

The next package should preserve R4C boundaries: no default activation, no
publication/schema cutover, no scheduler edits, no compatibility storage/request
/writeback/symbol lookup inside direct runtime, and default-disabled H2637
median `<= 676.67 s`.
