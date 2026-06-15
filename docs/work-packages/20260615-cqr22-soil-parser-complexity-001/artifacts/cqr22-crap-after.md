# CQR22 CRAP After

Status: complete.

Ran: after CRAP command:

```bash
cargo crap --workspace --lcov docs/work-packages/20260615-cqr22-soil-parser-complexity-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr22-soil-parser-complexity-001/artifacts/crap_after.json
```

Ran: ranked after rows for
`crates/openwepp-input-contract/src/parsers/soil.rs`:

```text
parse_soil                         line 292   CC 34.0  coverage 52.459016393442624  CRAP 158.2116476709504
parse_layer_row                    line 781   CC 69.0  coverage 73.95833333333334   CRAP 153.0822855631509
parse_ofe_block                    line 444   CC 35.0  coverage 69.16666666666667   CRAP 70.908521412037
SoilErrorCode::as_str              line 43    CC 10.0  coverage 33.33333333333333   CRAP 39.62962962962964
SoilDatver::numeric                line 113   CC 8.0   coverage 40.0                CRAP 21.823999999999998
tokenize_whitespace_and_quotes     line 1213  CC 16.0  coverage 73.01587301587301   CRAP 21.029966366323134
parse_restrictive_layer            line 992   CC 11.0  coverage 66.66666666666666   CRAP 15.481481481481488
SoilDatver::from_raw               line 125   CC 12.0  coverage 80.48780487804879   CRAP 13.069746521379548
validate_common_extended           line 1023  CC 10.0  coverage 100.0               CRAP 10.0
parse_ofe_header_tokens            line 1176  CC 8.0   coverage 72.72727272727273   CRAP 9.298271975957926
parse_policy_tokens                line 616   CC 7.0   coverage 65.51724137931035   CRAP 9.009102464225673
parse_v9005_policy_row             line 718   CC 8.0   coverage 96.0                CRAP 8.004096
parse_v9002_policy_row             line 668   CC 7.0   coverage 100.0               CRAP 7.0
SoilDatver::layer_arity            line 180   CC 5.0   coverage 85.71428571428571   CRAP 5.072886297376093
parse_v9003_policy_row             line 694   CC 5.0   coverage 95.0                CRAP 5.003125
parse_policy_row                   line 649   CC 5.0   coverage 100.0               CRAP 5.0
parse_binary_flag                  line 1308  CC 5.0   coverage 100.0               CRAP 5.0
maybe_parse_ofe_restrictive_row    line 594   CC 4.0   coverage 94.11764705882352   CRAP 4.003256665988195
parse_lkeff_policy_value           line 773   CC 4.0   coverage 100.0               CRAP 4.0
validate_positive                  line 1071  CC 3.0   coverage 62.5                CRAP 3.474609375
validate_percent                   line 1088  CC 3.0   coverage 62.5                CRAP 3.474609375
validate_fraction_unit             line 1105  CC 3.0   coverage 62.5                CRAP 3.474609375
validate_non_negative              line 1047  CC 3.0   coverage 73.91304347826086   CRAP 3.1597764444809733
parse_burn_code                    line 747   CC 3.0   coverage 100.0               CRAP 3.0
parse_texid_enum                   line 760   CC 3.0   coverage 100.0               CRAP 3.0
single_token                       line 1291  CC 2.0   coverage 62.5                CRAP 2.2109375
tokens_exact                       line 1155  CC 2.0   coverage 100.0               CRAP 2.0
LineCursor::next_line              line 1353  CC 2.0   coverage 100.0               CRAP 2.0
LineCursor::current_line_number    line 1363  CC 1.0   coverage 0.0                 CRAP 2.0
LineCursor::peek_line              line 1367  CC 2.0   coverage 100.0               CRAP 2.0
parse_i32                          line 1122  CC 1.0   coverage 33.33333333333333   CRAP 1.2962962962962963
parse_usize                        line 1133  CC 1.0   coverage 33.33333333333333   CRAP 1.2962962962962963
parse_f64                          line 1144  CC 1.0   coverage 33.33333333333333   CRAP 1.2962962962962963
LineCursor::new                    line 1335  CC 1.0   coverage 87.5                CRAP 1.001953125
SoilErrorCode::fmt                 line 59    CC 1.0   coverage 100.0               CRAP 1.0
SoilParserError::new               line 73    CC 1.0   coverage 100.0               CRAP 1.0
SoilParserError::fmt               line 89    CC 1.0   coverage 100.0               CRAP 1.0
SoilDatver::requires_policy_row    line 169   CC 1.0   coverage 100.0               CRAP 1.0
SoilDatver::requires_restrictive_footer line 173 CC 1.0 coverage 100.0             CRAP 1.0
SoilParserOptions::default         line 199   CC 1.0   coverage 100.0               CRAP 1.0
approx_eq                          line 1325  CC 1.0   coverage 100.0               CRAP 1.0
```

Static: final scoped target `parse_policy_row` CRAP is `5.0`, below the `30`
threshold. All newly extracted helpers are below the `30` threshold.

Static: out-of-scope rows still above CRAP `30` are unchanged rows owned by
future ranked CQR work. They are not CQR22 targets or new helpers.

Static: `cargo crap` emitted duplicate rows for some functions and the same
repo-wide LCOV warning pattern seen before; duplicated target/helper rows
reported identical CRAP values.
