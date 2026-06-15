# CQR22 CRAP Before

Status: complete.

Ran: before CRAP command:

```bash
cargo crap --workspace --lcov docs/work-packages/20260615-cqr22-soil-parser-complexity-001/artifacts/lcov_before.info --min 0 --format json --output docs/work-packages/20260615-cqr22-soil-parser-complexity-001/artifacts/crap_before.json
```

Ran: ranked before rows for
`crates/openwepp-input-contract/src/parsers/soil.rs`:

```text
parse_policy_row                      line 648   CC 29.0  coverage 26.041666666666668  CRAP 369.2180435745803
parse_soil                            line 291   CC 34.0  coverage 52.459016393442624  CRAP 158.2116476709504
parse_layer_row                       line 765   CC 69.0  coverage 73.95833333333334   CRAP 153.0822855631509
parse_ofe_block                       line 443   CC 35.0  coverage 69.16666666666667   CRAP 70.908521412037
SoilErrorCode::as_str                 line 42    CC 10.0  coverage 33.33333333333333   CRAP 39.62962962962964
SoilDatver::numeric                   line 112   CC 8.0   coverage 40.0                CRAP 21.823999999999998
tokenize_whitespace_and_quotes        line 1197  CC 16.0  coverage 73.01587301587301   CRAP 21.029966366323134
parse_restrictive_layer               line 976   CC 11.0  coverage 66.66666666666666   CRAP 15.481481481481488
SoilDatver::from_raw                  line 124   CC 12.0  coverage 80.48780487804879   CRAP 13.069746521379548
validate_common_extended              line 1007  CC 10.0  coverage 100.0               CRAP 10.0
parse_ofe_header_tokens               line 1160  CC 8.0   coverage 72.72727272727273   CRAP 9.298271975957926
parse_policy_tokens                   line 615   CC 7.0   coverage 65.51724137931035   CRAP 9.009102464225673
parse_binary_flag                     line 1292  CC 5.0   coverage 60.0                CRAP 6.6000000000000005
SoilDatver::layer_arity               line 179   CC 5.0   coverage 85.71428571428571   CRAP 5.072886297376093
validate_non_negative                 line 1031  CC 3.0   coverage 47.82608695652174   CRAP 4.2782115558477845
maybe_parse_ofe_restrictive_row       line 593   CC 4.0   coverage 94.11764705882352   CRAP 4.003256665988195
validate_positive                     line 1055  CC 3.0   coverage 62.5                CRAP 3.474609375
validate_percent                      line 1072  CC 3.0   coverage 62.5                CRAP 3.474609375
validate_fraction_unit                line 1089  CC 3.0   coverage 62.5                CRAP 3.474609375
single_token                          line 1275  CC 2.0   coverage 62.5                CRAP 2.2109375
tokens_exact                          line 1139  CC 2.0   coverage 100.0               CRAP 2.0
LineCursor::next_line                 line 1337  CC 2.0   coverage 100.0               CRAP 2.0
LineCursor::current_line_number       line 1347  CC 1.0   coverage 0.0                 CRAP 2.0
LineCursor::peek_line                 line 1351  CC 2.0   coverage 100.0               CRAP 2.0
parse_i32                             line 1106  CC 1.0   coverage 33.33333333333333   CRAP 1.2962962962962963
parse_usize                           line 1117  CC 1.0   coverage 33.33333333333333   CRAP 1.2962962962962963
parse_f64                             line 1128  CC 1.0   coverage 33.33333333333333   CRAP 1.2962962962962963
LineCursor::new                       line 1319  CC 1.0   coverage 87.5                CRAP 1.001953125
SoilErrorCode::fmt                    line 58    CC 1.0   coverage 100.0               CRAP 1.0
SoilParserError::new                  line 72    CC 1.0   coverage 100.0               CRAP 1.0
SoilParserError::fmt                  line 88    CC 1.0   coverage 100.0               CRAP 1.0
SoilDatver::requires_policy_row       line 168   CC 1.0   coverage 100.0               CRAP 1.0
SoilDatver::requires_restrictive_footer line 172 CC 1.0  coverage 100.0               CRAP 1.0
SoilParserOptions::default            line 198   CC 1.0   coverage 100.0               CRAP 1.0
approx_eq                             line 1309  CC 1.0   coverage 100.0               CRAP 1.0
```

Ran: target-file before LCOV:

```text
lines 665/1023 65.00%
functions 35/42 83.33%
```

Static: `cargo crap` emitted the repo-wide warning that 126 source files had no
matching LCOV entries. The target file had matching LCOV entries, matching
prior CQR package evidence posture.
