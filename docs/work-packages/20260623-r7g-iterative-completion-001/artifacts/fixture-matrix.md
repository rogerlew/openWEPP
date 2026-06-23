# Fixture Matrix

Status: HOLD-R7G-FROST-STATEFUL-SUBSOLVER-REQUIRED.

| Family | Fixture/Evidence | Status | Residual Risk |
| --- | --- | --- | --- |
| sidecar-only snow presence | `r7g_snow_sidecar_presence_is_not_active_snow_coupling` | pass | Low for activation predicate. |
| runtime-SWE active snow | `r7g_runtime_swe_activates_snow_without_sidecar_presence` | pass | Low for activation predicate. |
| thermally active wet snow | H2637 lane 1 day 13 endpoint loop passed after typed snow partition | pass | Medium; output parity still impacted by frost and downstream snow state deltas. |
| active frost | H2637 WAT `frozwt`/`frdp` parity deltas plus day-5 frost traces after active-frost endpoint remediation | hold | High; producer executes with zero compatibility edges, but stateful fine/shadow carry requires a new frost sub-solver and current performance is not proven green. |
| breakpoint climate | H2637 full run uses breakpoint climate and completed direct endpoint | pass for endpoint | Parity held downstream of frost/snow. |
| PMET | H2637 full run completed direct endpoint | pass for endpoint | No separate PMET parity fixture in this package. |
| irrigation when enabled | Not exercised by H2637 evidence | not covered | Requires follow-up fixture if R7G scope continues beyond H2637. |
| multi-OFE transfer ratios | H2637 19-OFE run completed; direct counters zero | pass for endpoint | WAT/PASS transfer columns differ after frost/snow divergence. |
| nonzero erosion | `r7g_committed_zero_upstream_erosion_qout_feeds_downstream_erod14_qin`; H2637 EROD14 handoff blocker closed | pass | Does not prove final HBP byte parity. |
| management transitions | Existing direct-runtime R5/R7 tests in `direct_runtime --lib` | pass focused | No new package-specific management fixture added. |
