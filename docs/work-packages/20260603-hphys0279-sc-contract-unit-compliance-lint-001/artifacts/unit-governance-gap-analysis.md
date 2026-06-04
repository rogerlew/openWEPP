# Unit Governance Gap Analysis

Status: completed/HOLD
Evidence mode: ran

Static: HPHYS0279 closes the tooling gap where `SC-*` unit-governance
requirements were human-reviewed only.

Closed:

- `SC-*` contract lint command exists.
- Fixture tests prove compliant and non-compliant snippets are distinguished.
- Registry unit cross-checks are contract-scoped to avoid false positives from
  reused legacy symbols.
- Release tooling README documents the command and current HOLD posture.

Current full-contract residual inventory is persisted in
`sc-unit-compliance-findings.json` and `sc-unit-compliance-findings.txt`.

| Code | Count | Meaning |
| --- | ---: | --- |
| `SCUNIT-E-001` | 20 | missing `Variables and Units` section |
| `SCUNIT-E-004` | 4 | registered symbol unit mismatch in `Variables and Units` |
| `SCUNIT-E-005` | 20 | missing `Symbol Alias Map` section |
| `SCUNIT-E-008` | 10 | alias `Units check` omits registry unit |
| `SCUNIT-E-009` | 66 | registered contract symbol absent from `Variables and Units` |
| `SCUNIT-E-011` | 107 | registered boundary/publication alias absent from `Symbol Alias Map` |

Highest-count contracts:

| Contract | Findings |
| --- | ---: |
| `SC-WATBAL-001` | 49 |
| `SC-SNOWFREEZE-001` | 47 |
| `SC-CLIMATE-001` | 36 |
| `SC-SOIL-001` | 29 |
| `SC-EVAP-001` | 11 |
| `SC-SUBHYD-001` | 10 |

Infile contract family:

- 20 `SC-INFILE-*` contracts currently lack both `Variables and Units` and
  `Symbol Alias Map` sections under this lint.

Ran:

- `tools/release/check_sc_unit_compliance.sh --format json`: fail/HOLD with
  227 findings.
