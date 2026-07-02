# Line-Count Governance

Status: `queued`

Evidence mode: `not-run`

Record `.rs` line counts after implementation. Files at or above 2000 lines
are `WARN`; files at or above 3000 non-exempt lines block closure until split
or explicitly exempted with owner and sunset plan.
