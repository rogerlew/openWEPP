# Line-Count Governance

Status: queued

Evidence mode: not-run.

Run before closure:

- Identify touched `.rs` files.
- Record line counts.
- Files at or above 2000 lines require a WARN disposition and split intent.
- Files at or above 3000 lines block closure unless explicitly approved as a
  generated/fixture exception with owner and sunset plan.
