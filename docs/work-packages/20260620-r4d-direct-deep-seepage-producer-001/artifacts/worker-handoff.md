# R4D Worker Handoff

Status: complete.
Evidence mode: Static.

R4D is complete with verdict
`COMPLETE-R4D-DIRECT-DEEP-SEEPAGE-PRODUCER`.

Carry forward boundaries:

- no default activation;
- no publication/schema cutover;
- no scheduler edits;
- no compatibility storage/request/writeback/symbol lookup inside direct
  runtime;
- no full WB18 percolation migration unless the pre-implementation contract
  gate explicitly amends package scope before Rust edits;
- no use of public `Dp`, WB19 `Qd`, ET, snow, precipitation, runoff, R3B
  diagnostic ledger values, or storage residual compensation as direct
  `deep_seepage_m` authority;
- default-disabled H2637 median `<= 676.67 s`.

Recommended next route: R4E direct `subsurface_loss_m` / `Qd` producer.

Rationale:

- R4B now has direct upstream producers for storage initial/precipitation
  (R4C), deep seepage `D` (R4D), and runoff `q` (R4A).
- The remaining explicit R4B storage operands are snow coupling `S`,
  evapotranspiration, subsurface loss `Qd`, and closure tolerance.
- `Qd` is the best next slice because `SC-SUBHYD-001` already carries
  lateral/subsurface authority, and it exercises another non-public direct
  producer without the broader ET or snow/frost lifecycle.

R4E should stay handoff-only unless its pre-implementation contract gate
explicitly proves a safe wider scope. It must not migrate public `Qd`
publication, WB13/WAT schema, scheduler paths, compatibility runtime, or default
activation.
