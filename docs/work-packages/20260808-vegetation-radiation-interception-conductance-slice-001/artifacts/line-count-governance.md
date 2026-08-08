# Line-Count Governance

Status: `queued`

Evidence mode: `not-run`

Inventory touched `.rs` files. Files at or above 2000 lines are `WARN` and need
decomposition rationale plus follow-on split intent. Files at or above 3000
lines require refactor before closure unless a generated/fixture exception has
explicit owner and sunset. Record reviewer disposition.
