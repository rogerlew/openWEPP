# Low Target-Selection Review A

Evidence class: **Static independent review**

Status: `PASS`.

Reviewed HEAD: `83f73e3dcaa3330785f89d08e626717f409d0fba`

Review A reproduced the committed Medium-final residual as 13 raw rows across
12 modules. It recommends 11 eligible rows across 10 actionable modules and
two exact `R-OBSERVABILITY` rows across two `DISPOSITIONED-NO-ACTION`
candidates. The latter remain denominator-retained debt and require Review B
concurrence. No `X-*`, `R-INFRASTRUCTURE`, `R-LOW-COMPLEXITY-PRODUCTION`, or
`R-IRREDUCIBLE-CRAP` disposition is supported.

The Low plan's provisional L-12 maximum is stale. The authoritative
Medium-final row for `WatershedNetworkFrameError::fmt` is CRAP 70.539, not
90.246. This review uses the final artifact and current source.

## Classification Ledger

| ID | Exact symbol and current lines | Raw metric | Recommendation | Denominator treatment | Semantic and consumer basis |
| --- | --- | --- | --- | --- | --- |
| L-01 | `FrostParseError::fmt`, `frost.rs:159-209` | CC 7, coverage 0%, CRAP 56.000 | `E-PRODUCTION` | aggregate + floor + CRAP | Although the formatter omits the separate `FROST-E-*` identifier, `parse_legacy_frost_sidecar` places its rendered text in `HillslopeCliError::ParseFailure.detail`; the real hillslope runner therefore publishes it. It is consumer-visible error behavior, not isolated observability. |
| L-02 | `PhosphorusParseError::fmt`, `phosphorus.rs:144-205` | CC 9, coverage 0%, CRAP 90.000 | `E-PRODUCTION` | aggregate + floor + CRAP | Every arm publishes the stable `PHOS-E-*` contract identifier and variant-specific fields. ADR-0021 makes machine-readable error codes eligible even though no current runner handoff was located. |
| L-03 | `PmetparaParseError::fmt`, `pmetpara.rs:230-302` | CC 12, coverage 0%, CRAP 156.000 | `E-PRODUCTION` | aggregate + floor + CRAP | Every arm publishes the stable `PMET-E-*` contract identifier. `parse_legacy_pmetpara_sidecar` forwards the rendered text through the public hillslope CLI error surface. |
| L-04 | `TcrParseError::fmt`, `tcr.rs:179-241` | CC 10, coverage 0%, CRAP 110.000 | `E-PRODUCTION` | aggregate + floor + CRAP | Every arm publishes a stable `TCR-E-*` identifier plus parser/domain or cross-file context. That is machine-readable contract behavior and cannot use the observability exception. |
| L-05 | `WeppUiParseError::fmt`, `wepp_ui.rs:134-177` | CC 6, coverage 0%, CRAP 42.000 | `E-PRODUCTION` | aggregate + floor + CRAP | Every arm publishes a stable `WUI-E-*` identifier. `parse_wepp_ui_sidecar` forwards the rendered text through `HillslopeCliError::ParseFailure.detail`. |
| L-06 | `HbpAdapterError::fmt`, `hbp.rs:115-172` | CC 7, coverage 0%, CRAP 56.000 | `E-PRODUCTION` | aggregate + floor + CRAP | All arms publish stable `HBP-E-*` codes and byte/magic-policy values. The adapter is a real HBP boundary, and its typed outcomes are exercised by parser/bridge and watershed CLI paths. |
| L-07 | `SidecarAdapterError::fmt`, `sidecar.rs:203-264` | CC 9, coverage 25%, CRAP 43.172 | `E-PRODUCTION` | aggregate + floor + CRAP | All arms publish stable `LSB-E-*` codes. `HillslopeCliError::SidecarAdapter` retains this error as a source, and subprocess tests assert the nested code in CLI output. |
| L-08 | `MeteorologyError::fmt`, `error.rs:34-71` | CC 7, coverage 0%, CRAP 56.000 | `R-OBSERVABILITY` | aggregate retained; reviewed floor/CRAP exception | The whole function only renders `BoundaryError` or variant fields. It has no stable machine code, state mutation, validation, branch selection, serialization, or publication effect. Repo consumers match typed variants or remap failures without consuming this text; no subprocess or text parser was located. All arms share this pure role, so no mixed-purpose split is hidden. |
| L-09 | `SnowbenchError::fmt`, `snowbench.rs:137-168` | CC 8, coverage 27.273%, CRAP 32.619 | `E-PRODUCTION` | aggregate + floor + CRAP | Every arm publishes a stable `SNOWBENCH-E-*` identifier. `openwepp-snowbench` converts this error to a string, emits it to stderr, and exits nonzero, making exact error selection and rendering real subprocess behavior. |
| L-10 | `read_canopy_series`, `snowbench_coe_melt.rs:315-398` | CC 13, coverage 45%, CRAP 41.117 | `E-PRODUCTION` | aggregate + floor + CRAP | This accepted-input parser decides the exact header, arity, numeric conversion, day ordering, unit interval, duplicate-date rejection, and nonempty cardinality before constructing the canopy map consumed by CoE melt replay. Parser grammar/cardinality and accepted-input state are always eligible. |
| L-10 | `parse_forcing_line`, `snowbench_coe_melt.rs:400-495` | CC 21, coverage 59.341%, CRAP 50.643 | `E-SCIENCE` | aggregate + floor + CRAP | In addition to grammar, timestamp, finite/domain, and cardinality decisions, the function computes rain, snow water, snowfall depth, radiation energy, cloud fraction, and dew point used by the melt model. Those unit conversions and physical forcing projections make the whole mixed-purpose function science-tier eligible. |
| L-11 | `SymbolAliasRegistryError::fmt`, `symbols.rs:66-120` | CC 9, coverage 0%, CRAP 90.000 | `R-OBSERVABILITY` | aggregate retained; reviewed floor/CRAP exception | The whole function only renders fields already selected by typed registry validation/lookup. It contains no machine code, alias selection, state change, validation, or control effect. Repo consumers use successful mappings or match typed variants; no rendered-string parser, publication field, or subprocess consumer was located. |
| L-12 | `WatershedNetworkFrameError::fmt`, `network_frame.rs:56-110` | CC 11, coverage 21.053%, CRAP 70.539 | `E-PRODUCTION` | aggregate + floor + CRAP | Every arm publishes a stable `WSHEDFRAME-E-*` code, including runtime, routed-state, groundwater-authority, and terminal-publication failures. The watershed CLI wraps the rendered error in `CLIWAT-E-019`, so it is real subprocess and publication-boundary behavior. |

`E-SCIENCE` on `parse_forcing_line` makes L-10 science tier. The remaining
actionable modules are glue tier unless their module record identifies a
separate contract or conservation authority requiring the stricter tier.

## Source Identity

All twelve target files are unchanged from the Medium measurement source in
production source. Current SHA-256 identities are:

| ID | Source SHA-256 |
| --- | --- |
| L-01 | `3620917b175a69c557c18616943ad069460fe76d783d8c1ab1bb76ef9bc093e1` |
| L-02 | `7b431f8f56b49ab77085fdcf9e021967c23f9f5ebf9395303547b594f865ddd7` |
| L-03 | `f4b09d61143b080bfecbaa37fe0b7a75099400069d378c06e50864a2b932dbb1` |
| L-04 | `ec271acae336c536a3f4e1af54dbb03da788f9a085493c41f6cb2211d3cd62e4` |
| L-05 | `dc63e32c219d842ff4e92ed1d04c68315b974331464d7fd3609a02bb28d4ae2a` |
| L-06 | `7f6d8e45277cf567397039aaf0fcce6b8f544ddacac6ae13e3dc8434ed5954d1` |
| L-07 | `b53dbcb4eeac0dabcf3ed3464debd6be2ce988b6a0225fe8c150e2c048ed14b7` |
| L-08 | `216b42dd308bd50c55a84091d0e629be275c3aaf06e7ca49a1d63d6a5eaf5c06` |
| L-09 | `3e834d7bca3f3e91cd5e7826cce433c3cd8f6b765d0a7f8f65bbb04d337b79e7` |
| L-10 | `d754099ca9c3b4ed1b93c20546e7ec9d12b25b34288ca8761d27cef0e5f32487` |
| L-11 | `13a475dc0c7376072b91a48f9eaded2f36925022533def98fe296dc98e8fc9cd` |
| L-12 | `534381932420245750ec9ac032a750479a8921fbda99491f0e636c6e1ce2b79b` |

## Public Consumers And Existing Tests

| IDs | Evidence located |
| --- | --- |
| L-01 | `infile_frost_parser_contract` exercises strict/compat success, arity, token, range, prefix, and missing-file behavior. The runner's `parse_legacy_frost_sidecar` is the real publication handoff. |
| L-02 | `infile_phosphorus_parser_contract` exercises header, required/optional absence, record count, numeric, finite, range, and fanout behavior and asserts contract IDs. |
| L-03 | `infile_pmetpara_parser_contract` exercises header, count, duplicate, tokenization, arity, range, sidecar absence, and lookup precedence and asserts contract IDs. The hillslope runner is the real error-text consumer. |
| L-04 | `infile_tcr_parser_contract` exercises open precedence, optional absence, prefix, domain, relational, cross-file, curve, token, and record-count paths and asserts contract IDs. |
| L-05 | `infile_weppui_parser_contract` exercises sentinel, mode closure, open precedence, soil compatibility, and missing-surface behavior and asserts contract IDs. The hillslope runner consumes the text. |
| L-06/L-07 | Bridge unit tests exercise strict/compat magic and sidecar policies. `cli01_runner_contract_derived_tests` verifies `LSB-E-007` through the real runner error surface; watershed CLI tests exercise HBP boundary failures. |
| L-08 | Meteorology tests `solver_reports_non_convergence`, `domain_errors_reject_nonphysical_inputs`, and `absolute_zero_guard_rejects_nonphysical_temperature` exercise typed public failure behavior. They do not establish a machine consumer of `fmt`, which is why the exact formatter remains the proposed exception. |
| L-09/L-10 | `snowdensity05e_melt_adjudication`, `snowdensity05g_harness_fidelity_rerun`, and `snowdensity10_3_1a_per_day_cancov` execute the real CoE replay and canopy handoff. `openwepp-snowbench` is the subprocess consumer. Current tests do not close the invalid-input/error-priority matrix of the two private parsers. |
| L-11 | `sim_contract_symbol_alias_registry` exercises construction, template validation, duplicate/ambiguity selection, missing lookup, and successful reverse lookup using typed variants. No test or production consumer parses `fmt` output. |
| L-12 | `network_frame` unit tests exercise routing, duration, publication, state, and error-source paths. `watershed_cli_behavior_contract` executes the real typed frame/publication consumer, and `openwepp-cli-watershed` publishes nested frame errors as `CLIWAT-E-019`. |

## Findings And Disposition

- `accepted`: the Medium-final residual is exactly 13 rows / 12 modules, with
  no current production-source drift.
- `accepted`: eleven rows default to eligibility because they publish stable
  error identities, feed real CLI consumers, decide accepted parser inputs, or
  perform scientific forcing projection.
- `accepted-pending-review-b`: only `MeteorologyError::fmt` and
  `SymbolAliasRegistryError::fmt` meet the exact `R-OBSERVABILITY` definition.
  Each exception is symbol-and-line bounded, retains the aggregate denominator,
  and waives no underlying typed validation or lookup behavior.
- `rejected`: formatter-name, error-module, diagnostic-host, low-priority, or
  current low-coverage status is not an exclusion rationale.
- `rejected`: `R-LOW-COMPLEXITY-PRODUCTION` cannot pre-disposition any live row
  because every raw target currently exceeds CRAP 30.
- `rejected`: no generated, nondefault-configuration, delegating-main,
  impossible, infrastructure-origin, or irreducible decision-table evidence
  exists for these rows.

Review A therefore recommends implementation records for L-01 through L-07,
L-09, L-10, and L-12. L-08 and L-11 may become
`DISPOSITIONED-NO-ACTION` only if Review B independently accepts both exact
observability dispositions and the tranche evidence records the retained raw
rows. Any disagreement defaults the disputed row to `E-PRODUCTION`.
