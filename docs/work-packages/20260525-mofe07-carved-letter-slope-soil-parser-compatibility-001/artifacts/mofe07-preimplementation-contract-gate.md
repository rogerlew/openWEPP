# MOFE07 Pre-Implementation Contract Gate

Status: complete
Evidence mode: Ran

Gate sequence A (before initial parser production edits):
1. `cargo test -p openwepp --test infile_slope_parser_contract compatibility_mode_accepts_shared_geometry_multi_ofe_form -- --nocapture`
- Result: expected failure
- Error: `TokenParseError ... expected integer, got '0.0000'`

2. `cargo test -p openwepp --test infile_soil_parser_contract compatibility_accepts_quoted_7778_soil_header_form -- --nocapture`
- Result: expected failure
- Error: `SOL-E-006 ... expected 9 token(s), found 15`

Gate sequence B (before per-OFE restrictive compatibility parser edits):
3. `cargo test -p openwepp --test infile_soil_parser_contract compatibility_accepts_quoted_7778_with_per_ofe_restrictive_rows -- --nocapture`
- Result: expected failure
- Error: `SOL-E-006 ... expected 9 token(s), found 3`

Gate result:
- PASS: contract-first sequencing preserved; production parser edits followed
  failing evidence capture for each scoped compatibility addition.
