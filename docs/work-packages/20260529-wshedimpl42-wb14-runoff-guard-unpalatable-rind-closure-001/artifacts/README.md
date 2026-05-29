# WSHEDIMPL42 Artifacts

Status: completed  
Evidence mode: Static + Ran

Artifact set for WSHEDIMPL42 execution.

Outcome summary:
- WB14 runoff-guard blocker (`HKERNEL-WB14-RUNOFF-E-003`) is closed for the
  unpalatable-rind hillslope cohort (39/39 hillslopes pass).
- Watershed closure is still blocked by two follow-on gaps:
  - `CLIWAT-E-010` (`pw0.imp` uses `jpond=0`, rejected by impoundment parser).
  - `CLIWAT-E-017` / `HBP-E-002` (hillslope `H*.hbp` outputs are ASCII pass
    text, not binary HBP shards required by watershed intake).
