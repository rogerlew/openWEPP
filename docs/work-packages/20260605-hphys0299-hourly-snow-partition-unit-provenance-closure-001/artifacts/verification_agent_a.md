# Verification Agent A

Status: complete

Evidence mode: static + ran

Verifier: Heisenberg (`rust_code_reviewer`)

Initial verification result: FAIL.

Checked items:

- Negative-melt `next_action`: code fixed; `next_action_for` only retains
  corrected negative-melt authority for `LEGACY-DEFECTIVE` plus material
  negative baseline raw melt.
- Trace validation: code fixed; `validate_trace_fields` covers missing,
  non-map, non-numeric, and non-finite payloads before classification.
- WATBAL guard map: fixed; `INV-WATBAL-074` present in the guard map.
- Corrected ledger mapping: fixed; all `hrsnow` provenance rows map only to
  `snow_hourly_snowfall_depth_sum_m`.

Finding:

- MEDIUM: test coverage did not assert `missing` or `non-numeric` validation
  cases, and did not directly assert the regenerated `OPENWEPP-DEFECTIVE`
  negative-melt ledger row routes to follow-on rather than legacy-defective
  acceptance.

Disposition: fixed.

- Added static test assertions for `"missing"`, `"non-numeric"`, and the
  regenerated H7 first-2013 negative-melt ledger route.
- Ran:

```text
cargo fmt
cargo fmt --check
cargo test --test hphys0299_hourly_snow_partition_unit_provenance_contract
markdown-doc lint --path docs/work-packages/20260605-hphys0299-hourly-snow-partition-unit-provenance-closure-001 --path docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md --path docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md --path docs/specifications/science-contracts/contracts/SC-WATBAL-001.md --path docs/work-packages/README.md --format json
cargo clippy --workspace --all-targets -- -D warnings
```

Result: pass. Focused HPHYS0299 test now reports `5 passed`.
