# Worker Handoff

Status: complete

Evidence mode: Static + Ran

`SNOW-WETCOMPACT-DUP-001` is closed. The next actionable package is 21L,
`SNOW-WARM-MIXED-PREPEAK-LOSS-ENERGY-ATTRIBUTION`, after it is scaffolded with
the corrected-state baseline.

- Canonical Snowbird control:
  `tests/fixtures/snotel_observed/snotel_snowbird_ut/p8.cli`, SHA-256
  `10c1ede130f697ccec01a4fb076d937213f0699e2f6c100492c7a4ef28ec11a7`.
- Development sensitivity lane:
  `tests/fixtures/snotel_observed/snotel_snowbird_ut/development/precip_x1p2155576/p8.cli`,
  SHA-256
  `c673145ee7fd41e71e3f2e21c529fba2d12691abd5f0f055444e621fb0b80afb`.
- Accepted materiality receipt:
  `target/snow_wet_compaction_operand_closure/execution-receipt.json`, SHA-256
  `1cd4aa5fb2110eb0445f57de846e2b65b224e7b0704e00a9d6cff1e3d4ca220a`.
- Accepted materiality result:
  `target/snow_wet_compaction_operand_closure/results/materiality.json`,
  SHA-256
  `25c8150f95d1be81afa7597d93dc271f8df5d82e062c558b231dd1695afab05a`.

21L must run both Snowbird lanes but use the canonical lane for acceptance and
the scaled lane only for input sensitivity. It must not treat empirical CoE
melt-depth terms as measured energy shares or infer forcing truth, calibration,
default suitability, or transferability from the scaled lane. Rebaseline all
loss and timing claims on the corrected operand before attribution. No distinct
follow-up defect remains from 21K.
