# Worker Handoff

Status: executed-held.

## Handoff

R7D3 lifted the previous fatal direct-production blocker:
`HOLD-R7D2-DIRECT-WB14-R4K-INFILTRATION-PRODUCER-AUTHORITY-ABSENT`.
Direct WB14/R4K now produces nonzero infiltration/depression operands, feeds
R4A/WB18/R4N, and H2637 direct production executes to completion with
`compatibility_edge_invocations=0`.

Do not claim R7D complete. H2637 parity remains blocked at
`HOLD-R7D3-DIRECT-MOFE-DYNAMIC-CARRY-TRANSFER-ABSENT`.

First R7D4 code action:

1. Add a typed same-day upstream-to-downstream transfer producer in
   `DirectFrameExecutor` after each lane's R4O/R4L outputs are known and before
   the next downstream lane's R3A/R4J spans execute.
2. Publish current-lane R4O `hourly_lateral_carry_m` and R4L/R4O
   `hourly_saturation_carry_m` into the current lane's direct transfer output
   buffers.
3. Copy those arrays with the declared area ratio into the downstream lane's
   transfer input buffers as `ui_LfUrf` and `ui_SUrunf`.
4. Make R4J consume typed dynamic transfer buffers, not only constructor
   `DirectRunonCarryInputs`.
5. Add anti-alias tests proving downstream lane `UpStrmQ` and `SubRIn` change
   when upstream current-lane carry arrays change, and rerun H2637 parity.

Residual evidence to reuse:

- Direct H2637: `elapsed=192.90 rss_kb=643724`.
- Default H2637: `elapsed=637.63 rss_kb=227352`.
- Direct manifest carry totals: current/upstream `0.0`.
- Default manifest carry totals: current/upstream `0.2205447764353141`.
- PASS first row `runvol`: default `107.13682236123434`, direct `0.0`.
