# CQR06 Numeric Equivalence

Evidence class: Static + Ran

Scope: behavior-preserving private helper extraction only.

Numeric/procedural preservation checks:

- Public entrypoints and public helper signatures unchanged.
- No WB19 or WB14 constants changed.
- No unit-conversion helper paths changed.
- No writeback field names changed.
- Lateral/drainage lane ordering preserved.
- Lateral caps still use potential, available pool, and realized withdrawal.
- Drainage caps still use potential, remaining drainage capacity, and available
  pool.
- WB14 ksat-adjustment regimes still dispatch by `solwpv` `9001`, `>=9002`,
  and `9003` floor behavior.

Focused behavioral evidence:

```bash
cargo test --test wb19_lateral_drainage_physics_kernel_contract
```

Result before edits: `15 passed; 0 failed`.

Result after edits: `15 passed; 0 failed`.

Residual risk:

- This package does not claim new science or comparator equivalence evidence.
  It relies on existing WB19/WB14 contract tests plus full workspace gates for
  behavior preservation.
