# Contract-Test Implementation Evidence

Status: `IMPLEMENTED / RED BEFORE PRODUCTION`

Evidence class: `Ran`

Contract-derived tests were added before production changes:

- `growth.rs`:
  - exact zero, zero-to-positive, and structural-only leaf-off vectors;
  - monotonic endpoint and finite exponential saturation vectors;
  - zero/negative/non-finite parameter rejection;
  - checked-sum overflow, checked-product overflow, and positive-`Bt`
    underflow rejection.
- `direct_publication_source_guards.rs`:
  - native `bbb/hmax` validation must precede `ForestCanopyState::advance`;
  - native height projection must follow GSI realization;
  - height must be assigned to the published `DirectGrowthStateSurface`.

Exact red outputs and exit codes are retained in
`preimplementation-contract-gate.md`.
