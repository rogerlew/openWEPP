# Implementation Test Evidence

Status: complete
Evidence mode: Static + Ran

## Production Changes

Static:
- \ adds \, \, per-hour residual rain-on-snow release tracking, and routed-melt addition after daily signed-melt redistribution.
- \ carries \ through WB12/WB14 partition reconciliation and centralizes the snow partition assembly in \.
- \ bumps HPHYS0245 trace schema to v13, serializes \ and \, and computes snow runtime closure with released rain excluded from physical snowpack loss.
- \ is registered in \ and covers partial retention/release, dense snow with positive raw melt plus released rain, and multi-hour dense-snow release.

## Focused Tests

Ran:
- \
- \
- \
- \

Result: pass.

## Final Workspace Gates

Ran:
- \
- \
- \
- \
- \
- \

Result: pass. \ reported existing duplicate/unmatched-license warnings and exited successfully.

## Target H1/H7/H39 Traces

Ran:
- Release rebuild plus H1/H7/H39 trace captures with \ and \.
- Trace root: \.

Result: pass; traces include schema v13, \, \, WB18 theta storage, and WB13 publication fields.

## Full H1..H39 Semantic Suite

Ran:
- \

Result: runtime 39/39 completed; semantic 39/39 completed; semantic pass 0/39.
hphys0288_winter_rain_snowmelt_partition_contract -- --nocapture`
- `cargo test -p openwepp-runner hphys0288_trace_row_captures_rain_on_snow_release_without_snowpack_loss -- --nocapture`
- `cargo test -p openwepp-runner hphys0245_trace_writer_serializes_jsonl_rows -- --nocapture`

Result: pass.

## Final Workspace Gates

Ran:
- `bash tools/release/check_authority_suite_antievasion.sh`
- `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture`
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`

Result: pass. `cargo deny check` reported existing duplicate/unmatched-license warnings and exited successfully.

## Target H1/H7/H39 Traces

Ran:
- Release rebuild plus H1/H7/H39 trace captures with `OPENWEPP_HPHYS0245_TRACE_PATH` and `OPENWEPP_HPHYS0245_TRACE_MAX_DAYS=1461`.
- Trace root: `/tmp/hphys0288_target_traces_v13_20260604T162402Z`.

Result: pass; traces include schema v13, `snow_hourly_rain_released_sum_m`, `wb12_infiltration_m`, WB18 theta storage, and WB13 publication fields.

## Full H1..H39 Semantic Suite

Ran:
- `python docs/work-packages/20260604-hphys0288-winter-rain-on-snow-melt-partition-magnitude-closure-001/artifacts/hphys0288_diagnostics.py --run-root /tmp/hphys0288_full_release_final_v13_20260604T163204Z`

Result: runtime 39/39 completed; semantic 39/39 completed; semantic pass 0/39.
