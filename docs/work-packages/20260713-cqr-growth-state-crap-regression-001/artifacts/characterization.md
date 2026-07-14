# Characterization

Evidence mode: Static.

Fresh target coverage is `97.43589743589743%`, so ADR-0021 cover-first closure
is already met and this package does not add test-only coverage.

Existing in-module tests exercise the relevant contract branches:

- `intval_zero_perennial_root_cap_uses_saturated_branch_before_division`
  proves exact-zero `rtmmax` bypasses division and publishes the root-depth
  envelope.
- `r5d_annual_growth_phase_computes_mutates_downstream_shadow_and_r4n_context`
  exercises the annual incremental root branch and validates the published
  state consumed by R4N.
- `r5d_perennial_growth_phase_supports_grazing_after_annual_phase_identity`
  exercises the ordinary positive-cap perennial growth path and its published
  state identity.
- The growth input guard test accepts zero perennial `rtmmax` and rejects
  negative `rtmmax`.

The refactor oracle is exact existing test behavior plus unchanged expression
text/order in the extracted block. No characterization assertion may be
deleted, weakened, or replaced.
