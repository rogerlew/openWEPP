# V57 bounded-liquid eligibility pre-implementation red

Status: `EXPECTED_RED`

Evidence mode: `Static + Ran`

## Retained canonical blocker

The exact r147 canonical run is retained at
`/tmp/wghl_001d_v56_64m_r147.log`, SHA-256
`1b95e317d65cf831933ef7778f20c7295ef2e590a199b986fcfee8dc97b759fc`.
It failed on exact-floor support `2100..2160 s` after `6:44.93` wall time
with peak RSS `442964 KiB`. The V56 specialization remained zero-charge
ineligible solely because `external_liquid_kg_m2` was
`2.7404676319867775e-15` to `5.480935263973555e-15 kg m^-2 OFE-ground`.
The unchanged legacy root then reached shared budget 63 and V55 found no exact
Q witness.

## Contract-first authority

`SC-SNOWENERGY-001` version 57 binds `REF-SNOWENERGY-WGHL-V57`,
`INV-SNOWENERGY-081`, and `OBL-SNOWENERGY-C-049`. The inclusive
`1.0e-12 kg m^-2` bound is the existing minimum terminal physical snow-closure
scale. It changes eligibility only: the exact liquid/refreeze mass, latent
energy, receipt, replay, finalization, and independent closure operands remain
unchanged. V57 also authorizes a zero-charge same-budget coordinate transition
from the first tolerance-closed legacy root immediately before V55.

## Isolated expected red

Ran after `cargo fmt --all` and `git diff --check`:

`nix develop -c cargo nextest run --test snow_terminal_enthalpy_event_numerics_contract v57_`

Nextest run ID: `df31d505-4d9d-4526-a1f9-043e5b5cd5ec`.

Result: `2 tests run: 1 passed, 1 failed, 56 skipped`, exit `100`.

- PASS: `v57_contract_binds_bounded_liquid_eligibility_without_normalization`.
- EXPECTED RED:
  `v57_bounded_liquid_eligibility_and_post_root_transition_are_required`.

The red names the absent bounded-eligibility helper/constant, post-root
transition, six required behavior vectors, and both temporary r145/r147
diagnostics. No production Rust was edited before this evidence was captured.
