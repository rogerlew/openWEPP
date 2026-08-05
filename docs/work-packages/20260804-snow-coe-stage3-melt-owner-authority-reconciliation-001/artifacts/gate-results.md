# Gate Results

Status: executed / focused pass / canonical assurance hold

Evidence mode: Ran

Working directory: `/home/workdir/openWEPP`.

Heavy-suite source identity:
`37442718f97b53912561f0b7bb907e9d1f905f23`.

## Passing Gates

- `cargo fmt --all -- --check`: PASS in 2.590 s.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  PASS in 17.411 s.
- `cargo nextest run --workspace --profile frost`: PASS, 358/358, with
  1917 skipped, in 558.186 s.
- `cargo test --doc --workspace`: PASS, zero doctests, in 6.030 s.
- `cargo nextest run --test snow_surface_eb03_contract`: PASS, 11/11.
- `.venv/bin/python -m pytest -q <package>/tools/test_verify_authority_decision.py`:
  PASS, 2/2.
- `.venv/bin/python <package>/tools/verify_authority_decision.py`: PASS,
  47/47 frozen, authority, chronology, contract, test, source-identity, and
  prompt checks.
- `cargo nextest run --workspace --profile quick --filterset
  'test(.*) - test(/assurance_v2_/)' --no-run`: PASS filter-expression
  validation. This is not reported as a correctness run.

Heavy logs are retained under
`target/snow_coe_stage3_melt_owner_authority_reconciliation/`.

## Expected Fail-Closed Assurance Results

- `cargo nextest run --workspace --profile quick`: FAIL, 87 run, 53 pass,
  34 fail, 40 skip, then fail-fast. Every failure is assurance drift caused by
  the amended `SC-SNOWFREEZE-001` source hash.
- `cargo nextest run --workspace --profile full`: FAIL, 2230 run, 2149 pass,
  81 fail, 33 skip. All failures are in `openwepp-assurance`,
  `assurance_v2_*`, or assurance dossier assembly tests; no kernel or unrelated
  behavioral test failed.

The locked assurance identity still names the pre-amendment contract hash.
Updating it here would falsely present an unreviewed scientific report as
current and is outside this package's authority. `ASSURE-06` must refresh and
review the report against the admitted authority before quick/full can pass.

## Diagnostics

- `bash tools/release/check_sc_unit_compliance.sh`: 150 repository findings.
  The exact predecessor `4c205c3c4f84a1f900710caefe3334dd69797ec3`
  produces the same 150 findings, so 21N adds no global unit-linter debt.
- `bash tools/release/check_science_contract_admission.sh --base-ref
  4c205c3c4f84a1f900710caefe3334dd69797ec3 --head-ref
  d0931911`: expected refusal because the amended contracts retain their
  lifecycle `draft`/`in_review` posture. This was not counted as a pass.
- `cargo deny` was not selected: no manifest, lockfile, or dependency changed.
- Authority-suite anti-evasion gates were not selected: no external-authority
  suite posture, cohort fixture, or required-case binding changed.
