# AUTH10 Disposition

Status: completed  
Evidence mode: Static + Ran  
Decision: GO

Static:
- AUTH09 review follow-on closure delivered:
  1. Level-3 WB19 suite provenance/path mismatch resolved (active metadata no
     longer points at stale `cas_l4_*` roots/hashes).
  2. Direct-theta FC cohort behavior de-inverted by removing
     expectation-pinned discrepancy acceptance.
  3. FC direct-theta cohort authority promoted to Level-4
     `required`/`hard-fail` constitutive posture with explicit threshold
     enforcement.
- Authority, registry, suite spec, fixture sidecars, and contract-derived test
  posture are coherent after the promotion.

Ran:
- Full workspace gate stack passed:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`

Residual risk:
- Historical AUTH07 package artifacts retain prior `cas_l5_*` references as
  point-in-time evidence. Current canonical authority surfaces are AUTH10
  `cas_l4_*`.
