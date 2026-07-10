# Line Count Governance

Evidence label: Static/Ran.

Status: `EXECUTED-WARN`

Ran:

`wc -l crates/openwepp-input-contract/src/parsers/management.rs tests/integration/infile_management_parser_contract.rs tests/integration/infile_management_yaml_contract.rs`

Observed counts:

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-input-contract/src/parsers/management.rs` | `2960` | WARN: above `2000`, below `3000` blocker |
| `tests/integration/infile_management_parser_contract.rs` | `1317` | Test file, outside production `.rs` blocker |
| `tests/integration/infile_management_yaml_contract.rs` | `316` | Test file |

Baseline:

- Target production file started at `2851` lines.

Disposition:

- Production target remains below the `3000` line blocker.
- The package added characterization in integration tests instead of appending
  tests inside the production module.
- The target remains a large parser module. Follow-up CQR or parser seam work
  should split production sections into submodules before future work risks
  crossing the `3000` line blocker.
