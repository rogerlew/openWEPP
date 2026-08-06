# Fixture And Forcing Custody

Status: `frozen before result execution`

The exact source fixture-manifest and observation hashes are in
`protocol-freeze.json`. Sources are copied to the ignored result namespace and
never mutated.

Snowbird handling is fixed:

1. copy canonical `snotel_snowbird_ut`;
2. verify canonical staged `p8.cli` SHA-256
   `10c1ede130f697ccec01a4fb076d937213f0699e2f6c100492c7a4ef28ec11a7`;
3. replace only the copy with
   `tests/fixtures/snotel_observed/snotel_snowbird_ut/development/precip_x1p2155576/p8.cli`;
4. verify derivative SHA-256
   `c673145ee7fd41e71e3f2e21c529fba2d12691abd5f0f055444e621fb0b80afb`;
5. record the copied-fixture manifest and exact runfile climate consumer; and
6. use the same derivative for Snowbird control and paired runs.

The transformation is precipitation-only, factor `1.2155576`, output
resolution `0.1 mm`, `ROUND_HALF_UP`, with non-precipitation fields preserved as
defined by the existing derivative manifest. Classification remains
`DEVELOPMENT_ONLY`; no forcing-truth or validation claim is available.
