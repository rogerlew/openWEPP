# Low Raw-To-Actionable Ledger

Evidence class: **Ran + Static**

Source commit: `83f73e3dcaa3330785f89d08e626717f409d0fba`.
Fresh production census: 13 rows across 12 modules. Fixed Low cohort: 13 raw
rows across 12 modules. Actionable cohort: 11 rows across 10 modules. Two exact
rows are dual-reviewed, denominator-retained `R-OBSERVABILITY` dispositions.

Review B proposed a third exception for L-01. Review A rejected it because the
real hillslope runner publishes `FrostParseError::fmt` through
`HillslopeCliError::ParseFailure.detail`; the binding disagreement rule
therefore classifies L-01 as `E-PRODUCTION`. L-10 defaults to science tier
because `parse_forcing_line` performs physical forcing and unit projections.

| ID | Function | Line | CC | Coverage % | CRAP | Class | Treatment |
| --- | --- | ---: | ---: | ---: | ---: | --- | --- |
| L-01 | `FrostParseError::fmt` | 160 | 7 | 0.000 | 56.000 | `E-PRODUCTION` | cover stable runner-visible error detail |
| L-02 | `PhosphorusParseError::fmt` | 145 | 9 | 0.000 | 90.000 | `E-PRODUCTION` | cover embedded `PHOS-E-*` identity |
| L-03 | `PmetparaParseError::fmt` | 231 | 12 | 0.000 | 156.000 | `E-PRODUCTION` | cover embedded `PMET-E-*` identity and runner detail |
| L-04 | `TcrParseError::fmt` | 180 | 10 | 0.000 | 110.000 | `E-PRODUCTION` | cover embedded `TCR-E-*` identity |
| L-05 | `WeppUiParseError::fmt` | 135 | 6 | 0.000 | 42.000 | `E-PRODUCTION` | cover embedded `WUI-E-*` identity and runner detail |
| L-06 | `HbpAdapterError::fmt` | 116 | 7 | 0.000 | 56.000 | `E-PRODUCTION` | cover embedded `HBP-E-*` boundary identity |
| L-07 | `SidecarAdapterError::fmt` | 204 | 9 | 25.000 | 43.172 | `E-PRODUCTION` | cover embedded `LSB-E-*` identity and CLI source chain |
| L-08 | `MeteorologyError::fmt` | 35 | 7 | 0.000 | 56.000 | `R-OBSERVABILITY` | reviewed retained exception; `DISPOSITIONED-NO-ACTION` |
| L-09 | `SnowbenchError::fmt` | 138 | 8 | 27.273 | 32.619 | `E-PRODUCTION` | cover embedded `SNOWBENCH-E-*` CLI identity |
| L-10 | `read_canopy_series` | 315 | 13 | 45.000 | 41.117 | `E-PRODUCTION` | cover grammar, order, domain, duplicate, and cardinality |
| L-10 | `parse_forcing_line` | 400 | 21 | 59.341 | 50.643 | `E-SCIENCE` | cover grammar plus rain/snow/radiation/cloud/dew-point projection |
| L-11 | `SymbolAliasRegistryError::fmt` | 67 | 9 | 0.000 | 90.000 | `R-OBSERVABILITY` | reviewed retained exception; `DISPOSITIONED-NO-ACTION` |
| L-12 | `WatershedNetworkFrameError::fmt` | 57 | 11 | 21.053 | 70.539 | `E-PRODUCTION` | cover embedded `WSHEDFRAME-E-*` CLI identity |

The start filter is byte-identical to Medium final, SHA-256
`9501960825bc75401c8bb98c2ccf353fe128f5f5baaee103a32737e00306bb93`.
No new, removed, or source-drifted identity exists at Low activation. No
`X-*`, `R-INFRASTRUCTURE`, `R-LOW-COMPLEXITY-PRODUCTION`, or
`R-IRREDUCIBLE-CRAP` disposition is accepted.
