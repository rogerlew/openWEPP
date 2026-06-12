# characterization openwepp multi ofe

Status: complete for increment M-A characterization

Evidence mode: Ran + Static

## Evidence

Ran:
- `tools/owcmp/owcmp env --manifest tools/owcmp/suites/wa-cascades-mofe-ksflag0.json`
  - PASS, using `/home/workdir/openWEPP/.venv/bin/python`.
  - Confirmed `/wc1/runs/ar/arboreal-dendrite`, 36 legacy `H*.wat.dat` files, 36 legacy `H*.plot.dat` files, and watershed structure.
- `cargo build -p openwepp-runner --bin openwepp-cli-hill`
  - PASS.
  - Binary SHA-256: `9faa768c0eb0897c347434fad1020a41aed0b02fe782606420043c549da8b174`.
- `cargo build -p openwepp-runner --bin open_wepp_runner`
  - PASS.
  - Binary SHA-256: `09705191091d1b7fbe0a09f7adf3f25ca09d46ed5f39a5d67722cbfb169ec0e0`.
- Generated TOML runner files in isolated temp lane only:
  - Source inputs copied from `/wc1/runs/ar/arboreal-dendrite/wepp/runs`.
  - Temp lane: `/tmp/openwepp_mofe01_ma`.
  - Runfile generation used `.venv/bin/python` and `open_wepp_runner.make_hillslope_run`.
- Ran all H1-H36 locally with `target/debug/openwepp-cli-hill --policy compat --legacy-sidecar-discovery`.
  - No comparator subagent was used.
  - No production source files were edited.
  - No legacy `/wc1/.../wepp/output` files were overwritten.

Static:
- Repository HEAD: `f4c162e45d853805b127eb08d269f7c3b7215d0a`.
- Current runner publication policy for successful hillslope outputs is `single-row-canonicalized-hillslope-aggregate`, recorded in the H8 manifest.

## Cohort inventory

The active H-surface cohort contains 36 hillslopes and 271,808 legacy WAT rows:

| OFE count | Hillslopes | Count |
| --- | --- | ---: |
| 1 | H8, H15, H19, H20, H22, H23, H28 | 7 |
| 2 | H11, H13, H16, H33, H36 | 5 |
| 3 | H6, H12, H14, H29, H30 | 5 |
| 4 | H9, H25, H32 | 3 |
| 5 | H1, H2, H3, H4, H5, H7, H10, H17, H18, H21, H24, H26, H27, H31, H34, H35 | 16 |

`pw0.slp` declares 15 slope segments, but there is no `pw0.wat.dat` in `/wc1/runs/ar/arboreal-dendrite/wepp/output`. The watershed far-point is inventory-only for M-A WAT closure.

## Current openWEPP execution matrix

| OFE count | Hillslopes | Passed | Failed | Output/manifests |
| --- | ---: | ---: | ---: | --- |
| 1 | 7 | 7 | 0 | 7 manifests, 7 WAT parquets, 7 plot parquets, 7 HBP files |
| 2 | 5 | 0 | 5 | none |
| 3 | 5 | 0 | 5 | none |
| 4 | 3 | 0 | 3 | none |
| 5 | 16 | 0 | 16 | none |

Current as-is status:
- 1-OFE surfaces execute to completion.
- Every multi-OFE surface fails before output publication.
- H1-H33 except the passing 1-OFE set, plus H35-H36, fail at `runoff_reconciliation` on `sim_day_index=2`, `calendar_year=2000`, `julian_day=2`, with `HKERNEL-WB14-RUNOFF-E-003` and `DOMAIN_VIOLATION`.
- H34 fails at `runoff_reconciliation` on `sim_day_index=1`, `calendar_year=2000`, `julian_day=1`, with `HKERNEL-WB14-RUNOFF-E-001` and `MISSING_REQUIRED_INPUT`.

This means the current M-A break is pre-publication MOFE execution, not a completed multi-OFE WAT closure mismatch.

## Passing current WAT characterization

The seven passing current outputs are all 1-OFE surfaces. Each has 2,192 WAT rows, OFE set `{1}`, and sim days 1-2,192.

| Metric across passing current WAT rows | Value |
| --- | ---: |
| Total passing rows | 15,344 |
| `UpStrmQ` max abs | 0.0 mm |
| `SubRIn` max abs | 0.0 mm |
| `QOFE - Q` max abs | 0.0 mm |
| `latqcc` max | 14.467652115341204 mm |
| WAT-row diagnostic closure max abs | 76.99999999999999 mm |

The WAT-row diagnostic used:

`RM + Irr + UpStrmQ + SubRIn - Q - Ep - Es - Er - Dp - latqcc - Tile - delta(Total-Soil + frozwt + Snow-Water)`.

This is a diagnostic only. It is not a science-contract closure authority because the current publication path does not expose successful multi-OFE rows and still publishes the single-row aggregate policy for successful hillslopes.

## Current MOFE seams

Static source inspection shows why current multi-OFE behavior is not yet a closed inter-OFE route:

- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs:30-45`
  - Enables MOFE hourly carry arrays when `contributor_ofe_count > 1`.
- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs:461-472`
  - Seeds `wb12_runon_input = 0.0` and `wb12_runoff_carryover = 0.0` for each hillslope run.
  - There is no current upstream OFE transfer injected across element boundaries at this seed point.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs:627-660`
  - `resolve_runoff_carryover_input` can consume MOFE upstream hourly carry, then `wb12_runoff_carryover`, then `wb12_runon_input`.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_runoff_reconciliation.rs:268-326`
  - `runon_input` participates in the runoff partition and solver closure.
  - This is the primary hydrology seam to close once upstream OFE carry is populated.
- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs:956-995`
  - Current WAT publication reads `SubRIn`, hard-codes `UpStrmQ = 0.0`, and sets `QOFE = Q`.
- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_publication.rs:214-282`
  - The manifest records MOFE carry totals and the publication OFE policy.

## M-A conclusion

As-is openWEPP cannot yet characterize completed multi-OFE WAT rows on the arboreal-dendrite cohort because all 29 multi-OFE H surfaces fail in `runoff_reconciliation` before publication. The immediate executable closure target for the next increment is to make multi-OFE hydrology run through day 2 and then through the full 2,192-day cohort while preserving explicit inter-OFE `UpStrmQ`/`SubRIn`/`QOFE` publication semantics.
