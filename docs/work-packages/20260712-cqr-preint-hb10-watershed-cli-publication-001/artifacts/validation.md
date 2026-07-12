# Terminal Validation

Evidence class: **Ran + Static**

| Gate | Result | Evidence |
| --- | --- | --- |
| Full runner | PASS | `205/205` |
| Complete watershed CLI | PASS | `29/29` |
| Private companion | PASS | `5/5` |
| Six fixed rows | PASS | All CRAP at most 30. |
| Strict floors | PASS | Area, publication area/residual/carry totals, CRFRAC and groundwater all exceed 75%. |
| Retained floors | PASS | 18 rows accepted independently by reviewers A/B in `retained-floor-ledger.md`. |
| Format/Clippy/diff | PASS | Workspace format; runner all-target `-D warnings`; clean diff check. |

Fixed rows (CC/coverage/CRAP): `run` 11/75.472%/12.786;
`hillslope_area_m2_from_source_runfile` 5/83.333%/5.116;
`parse_watershed_runfile` 13/80.328%/14.287;
`validate_manifest_publication_metadata` 10/76.190%/11.350;
`validate_manifest_per_ofe_wb13_publication_policies` 6/50%/10.500;
`validate_manifest_mofe_hourly_carry_metadata` 9/73.171%/10.564.

| Item | SHA-256 |
| --- | --- |
| Production source | `ee899e075d1a3b4112770e5e2dd8120ff3fc1d5ed859bbf0c822f8630e1c6ebe` |
| Companion tests | `3b5a9365a784d765f195831636a5eb6282f6387349a88ebaf9adf804519ec0af` |
| JSON | `707475e21726d755e0c1eddb807cb3eed28246fc39239a567ce01dd24b1ff8ad` |
| LCOV | `c9575e7d45787f4d4ebd0c6cfb2b0146f0717b5be6d7d4bbb2fa9b71568cc79f` |
| CRAP | `1f1e3590acd7093cf856a9af8871e3c949ad9a93a3dad4f6e811938f0e814fe2` |

Line governance: production source `2,656` lines, WARN below the 3,000-line
blocker. Test-only characterization resides in the authorized companion.
