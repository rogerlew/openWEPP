# Review Disposition

Review: `review-codex.md` (Codex, 2026-07-01). Merge correctly withheld on
the red clippy gate. All four findings **accepted** and fixed:

| # | Finding | Action |
|---|---|---|
| DC01-CX-001 | seven `usize as f64` casts fail `-D warnings` | replaced with `f64::from(u32::try_from(...))` / `DC01_HOUR_BIN_COUNT_F64` const; clippy `-D warnings` now 0 across both crates (Ran) |
| DC01-CX-002 | `erod14_qin_clamped_events` not reset | added to `reset_direct_runtime_audit_counters` |
| DC01-CX-003 | acceptance criterion 1 under-tested | `dc01_dry_runon_day_still_infiltrates` added: zero-intensity day infiltrates nothing; +runon infiltrates with exact infiltration+excess split; suite green |
| DC01-CX-004 | diag env selector left as unmanaged behavior switch | **deleted** (`OPENWEPP_DC01_DIAG_INTERVAL_BASIS` + appended-basis fn); its decomposition result is recorded in the ExecPlan and is reproducible from history |

Post-fix gates (Ran): clippy `-D warnings` 0; orchestrator suite green;
single-OFE byte-identity re-verified (marcell); H2637 exit 0 with
`runvol_pct_precip` re-measured (cast rewrites are value-identical).
