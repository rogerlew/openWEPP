# CQR26 Behavior Equivalence

Status: complete.

Static: no production Rust file was modified.

Static: protected surfaces are therefore unchanged:

- public and crate-visible signatures;
- runtime symbols, aliases, and units;
- formulas and float expression order;
- typed guard IDs, errors, and domain checks;
- parser compatibility and publication behavior;
- writeback ordering and science-contract behavior.

Ran: before and after LCOV/CRAP metrics match for the target file.

Ran: full workspace tests passed after metric capture.
