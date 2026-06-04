# Gate Results

Status: completed/HOLD
Evidence mode: ran

Static: focused HPHYS0279 gates pass. Default full-contract SC unit lint is
intentionally HOLD because it now exposes current remediation inventory rather
than hiding existing gaps.

Ran:

- `cargo test --test hphys0279_sc_unit_compliance_lint_contract -- --nocapture`:
  pass, 9 tests.
- `python3 -m py_compile tools/release/check_sc_unit_compliance.py`: pass.
- `tools/release/check_sc_unit_compliance.sh`: fail/HOLD with 227 findings.
- `markdown-doc lint --path docs/specifications/unit-governance.md --path tools/release/README.md --path docs/work-packages/20260603-hphys0279-sc-contract-unit-compliance-lint-001`:
  pass, 25 files.
- `cargo fmt --check`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo deny check`: pass; existing warnings for duplicate `getrandom`,
  `hashbrown`, `twox-hash`, and unmatched license allowances `ISC` and
  `Unicode-DFS-2016`.
- `cargo test --workspace`: fail/HOLD only on
  `simimpl18_contract_requires_cold_day_partition_zero_rm_and_runtime_snow_storage`
  and `simimpl18_contract_requires_multi_day_storage_state_mutation`, both with
  `HKERNEL-WB11-ET-E-003`.

Default lint inventory:

| Code | Count | Meaning |
| --- | ---: | --- |
| `SCUNIT-E-001` | 20 | missing `Variables and Units` section |
| `SCUNIT-E-004` | 4 | registered symbol unit mismatch in `Variables and Units` |
| `SCUNIT-E-005` | 20 | missing `Symbol Alias Map` section |
| `SCUNIT-E-008` | 10 | alias `Units check` omits registry unit |
| `SCUNIT-E-009` | 66 | registered contract symbol absent from `Variables and Units` |
| `SCUNIT-E-011` | 107 | registered boundary/publication alias absent from `Symbol Alias Map` |

Persisted inventory:

- `artifacts/sc-unit-compliance-findings.json`
- `artifacts/sc-unit-compliance-findings.txt`
