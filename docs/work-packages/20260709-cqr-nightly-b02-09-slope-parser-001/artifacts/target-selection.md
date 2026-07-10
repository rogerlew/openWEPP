# Target Selection - CQR Nightly Batch 02, Target 09

Baseline source:

- LCOV: `/tmp/openwepp-cqr-nightly-new-isolated.lcov`
- CRAP JSON: `/tmp/openwepp-cqr-nightly-new-isolated-crap.json`

Selected module:

- `crates/openwepp-input-contract/src/parsers/slope.rs`

Baseline high-CRAP rows:

| Function | Line | CC | Coverage | CRAP |
|---|---:|---:|---:|---:|
| `SlopeParserError::fmt` | 157 | 13 | 0.0000% | 182.0000 |
| `parse_slope_str` | 259 | 23 | 71.7647% | 34.9078 |

The next-highest rows are below the CRAP threshold: `parse_ofe_shape` at
26.4450 and `derive_distance_mode` at 24.4290.
