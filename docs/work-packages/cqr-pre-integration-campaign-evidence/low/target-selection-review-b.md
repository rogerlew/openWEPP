# Low Target-Selection Review B

Evidence class: **Static independent review**

Status: `PASS` with three proposed `R-OBSERVABILITY` dispositions.

Review B reproduced the fixed Low ledger's 13 residual rows across 12 modules
at source commit `83f73e3dcaa3330785f89d08e626717f409d0fba`. It classifies 10 rows in
nine modules as `E-PRODUCTION` and proposes `R-OBSERVABILITY` for the three
remaining rows in L-01, L-08, and L-11. No `X-*`, `R-INFRASTRUCTURE`,
`R-LOW-COMPLEXITY-PRODUCTION`, or `R-IRREDUCIBLE-CRAP` disposition is supported.
The proposed retained-denominator exceptions require independent Review A
agreement; any disagreement defaults the affected row to `E-PRODUCTION`.

## Row Classification

| ID | Symbol and exact source span | SHA-256 | Review B disposition | Binding evidence |
| --- | --- | --- | --- | --- |
| L-01 | `FrostParseError::fmt`, `frost.rs:159-210` | `3620917b175a69c557c18616943ad069460fe76d783d8c1ab1bb76ef9bc093e1` | `R-OBSERVABILITY` | The formatter emits prose only. Machine identity is separately supplied by `contract_error_id` at lines 144-156; the hillslope CLI wraps the prose as human-facing `ParseFailure.detail` under outer code `CLIHILL-E-010`. Frost contract tests assert variants and `contract_error_id`, not exact formatter text. No source or test parses or compares this prose. |
| L-02 | `PhosphorusParseError::fmt`, `phosphorus.rs:144-206` | `7b431f8f56b49ab77085fdcf9e021967c23f9f5ebf9395303547b594f865ddd7` | `E-PRODUCTION` | Every arm emits the stable `PHOS-E-*` identity returned by `contract_error_id`; this is machine/public error behavior, irrespective of whether a current runner path formats every variant. `infile_phosphorus_parser_contract.rs` exercises the typed identities. |
| L-03 | `PmetparaParseError::fmt`, `pmetpara.rs:230-303` | `f4b09d61143b080bfecbaa37fe0b7a75099400069d378c06e50864a2b932dbb1` | `E-PRODUCTION` | Every arm emits a stable `PMET-E-*` identity and the runner converts the error to CLI detail. `infile_pmetpara_parser_contract.rs` exercises the typed identities. |
| L-04 | `TcrParseError::fmt`, `tcr.rs:179-242` | `ec271acae336c536a3f4e1af54dbb03da788f9a085493c41f6cb2211d3cd62e4` | `E-PRODUCTION` | Every arm emits a stable `TCR-E-*` identity returned by `contract_error_id`. `infile_tcr_parser_contract.rs` exercises the typed identities. |
| L-05 | `WeppUiParseError::fmt`, `wepp_ui.rs:134-178` | `dc63e32c219d842ff4e92ed1d04c68315b974331464d7fd3609a02bb28d4ae2a` | `E-PRODUCTION` | Every arm emits a stable `WUI-E-*` identity and the runner converts the error to CLI detail. `infile_weppui_parser_contract.rs` exercises the typed identities. |
| L-06 | `HbpAdapterError::fmt`, `hbp.rs:115-172` | `7f6d8e45277cf567397039aaf0fcce6b8f544ddacac6ae13e3dc8434ed5954d1` | `E-PRODUCTION` | Every arm calls `self.code()` and emits `HBP-E-001` through `HBP-E-006`; the machine identity is part of the formatter itself. Unit tests at `hbp.rs:314-359` cover typed codes, and `infile_hbp_parser_contract.rs:1624-1632` verifies the parser-to-bridge code boundary. |
| L-07 | `SidecarAdapterError::fmt`, `sidecar.rs:203-264` | `b53dbcb4eeac0dabcf3ed3464debd6be2ce988b6a0225fe8c150e2c048ed14b7` | `E-PRODUCTION` | Every arm calls `self.code()` and emits `LSB-E-001` through `LSB-E-008`. `HillslopeCliError::SidecarAdapter` retains the typed source, and `cli01_runner_contract_derived_tests.rs:431-453` verifies that the real CLI error includes `LSB-E-007`. |
| L-08 | `MeteorologyError::fmt`, `error.rs:34-72` | `216b42dd308bd50c55a84091d0e629be275c3aaf06e7ca49a1d63d6a5eaf5c06` | `R-OBSERVABILITY` | The formatter emits human-readable diagnostics only: no code API, validation, state mutation, error-priority decision, or control effect occurs in the function. Located consumers match/map typed variants or discard the diagnostic; tests assert variants and numerical behavior, not exact text. No machine parser or exact-string consumer was found. |
| L-09 | `SnowbenchError::fmt`, `snowbench.rs:137-168` | `3e834d7bca3f3e91cd5e7826cce433c3cd8f6b765d0a7f8f65bbb04d337b79e7` | `E-PRODUCTION` | The formatter emits stable `SNOWBENCH-E-001` through `SNOWBENCH-E-007` identities. `openwepp-snowbench.rs:10-13` publishes errors to stderr and command adapters at lines 193, 217, 245, 267, and 344 convert these errors to the public CLI string. |
| L-10 | `read_canopy_series`, `snowbench_coe_melt.rs:315-398` | `d754099ca9c3b4ed1b93c20546e7ec9d12b25b34288ca8761d27cef0e5f32487` | `E-PRODUCTION` | Hand-authored parser decisions enforce exact header/cardinality, numeric and sequential day indices, unit interval, duplicate-date rejection, and nonempty input. `run_coe_melt_snowbench` consumes its map at lines 241-243; `snowdensity10_3_1a_per_day_cancov.rs` and `snowdensity05g_harness_fidelity_rerun.rs` bind the real CoE-melt path. |
| L-10 | `parse_forcing_line`, `snowbench_coe_melt.rs:400-495` | `d754099ca9c3b4ed1b93c20546e7ec9d12b25b34288ca8761d27cef0e5f32487` | `E-PRODUCTION` | Hand-authored grammar, cardinality, timestamp/hour, numeric-domain, and fail-closed checks feed accepted-input rain/snow depth, radiation, cloud, and dew-point transformations. `read_coe_melt_forcing` calls it at line 305 and the result is consumed by `group_daily_forcing` and CoE-melt simulation at lines 240-243. |
| L-11 | `SymbolAliasRegistryError::fmt`, `symbols.rs:66-121` | `13a475dc0c7376072b91a48f9eaded2f36925022533def98fe296dc98e8fc9cd` | `R-OBSERVABILITY` | The formatter emits prose only and has no machine code, state, validation, or control effect. `sim_contract_symbol_alias_registry.rs` asserts typed variants and registry behavior, not exact formatter text; no production or test consumer parsing/comparing the text was found. Assertions for similarly named `BoundaryUnitRegistryError` are a different type and are not evidence against this disposition. |
| L-12 | `WatershedNetworkFrameError::fmt`, `network_frame.rs:56-111` | `534381932420245750ec9ac032a750479a8921fbda99491f0e636c6e1ce2b79b` | `E-PRODUCTION` | The formatter emits `WSHEDFRAME-E-001` through `WSHEDFRAME-E-010`, including nested runtime identity. The watershed CLI publishes it under `CLIWAT-E-013` and `CLIWAT-E-019` at `openwepp-cli-watershed.rs:379-390,570-587`; `network_frame.rs:1989-1996` checks the nested formatted code and `wshedw5_typed_watershed_runtime_contract.rs` exercises typed variants. |

## Recommendation

Accept L-02 through L-07, both L-10 rows, L-09, and L-12 as actionable
`E-PRODUCTION` work. If Review A independently agrees with all exact exception
records, accept L-01, L-08, and L-11 as `R-OBSERVABILITY`; they remain in the
aggregate denominator but receive the reviewed per-function floor/CRAP
exception authorized by ADR-0021. That leaves **10 actionable rows in nine
modules** and **three no-action modules**. If either reviewer disputes any
exception or identifies a machine/exact-string consumer, restore that row to
`E-PRODUCTION` without further adjudication.
