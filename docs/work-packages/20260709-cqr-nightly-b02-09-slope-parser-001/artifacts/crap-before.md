# CRAP Before

Command:

`jq -r '.entries[] | select(.file|endswith("crates/openwepp-input-contract/src/parsers/slope.rs")) | [.function, .line, .cyclomatic, .coverage, .crap] | @tsv' /tmp/openwepp-cqr-nightly-new-isolated-crap.json`

Evidence:

- Baseline CRAP JSON SHA-256:
  `5e3abb273dffdc8c8308da0af9f3d85283307b2b2c77f0c9ba00c3a55741f765`
- Rows above 30: 2.
- Maximum CRAP: 182.0000 (`SlopeParserError::fmt`, coverage 0%, CC 13).

Rows above threshold:

| Function | Line | CC | Coverage | CRAP |
|---|---:|---:|---:|---:|
| `SlopeParserError::fmt` | 157 | 13 | 0.0000% | 182.0000 |
| `parse_slope_str` | 259 | 23 | 71.7647% | 34.9078 |
