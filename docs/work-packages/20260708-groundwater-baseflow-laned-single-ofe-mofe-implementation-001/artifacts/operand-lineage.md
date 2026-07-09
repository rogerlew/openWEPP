# Operand Lineage

Status: `QUEUED`

Before implementation, record every conservation-sensitive operand.

| Operand | Units | Source | Destination | Alias rejected | Status |
|---|---|---|---|---|---|
| `D_i` groundwater recharge | `m^3 day^-1` volume over daily timestep | Direct deep-percolation producer | Groundwater recurrence | generated `gwdsv`; `latqcc`; surface runoff | queued |
| `S_i` groundwater storage | `m^3` | Groundwater carry | Next-day recurrence and diagnostics | soil water storage; snow storage | queued |
| `Qb_i` / `gwbfv` | `m^3 day^-1` volume over daily timestep | Groundwater recurrence | HBP/pass/watershed export ledger | `latqcc`; `cbase`; active surface source | queued |
| `Qs_i` / `gwdsv` | `m^3 day^-1` volume over daily timestep | Groundwater recurrence | Deep-seepage export/publication ledger | current soil deep percolation; `latqcc` | queued |
| `latqcc` | existing owning units | `SC-SUBHYD-001` lateral export | Lane D bypass/export ledger | `gwbfv`; `gwdsv`; `cbase` | queued |
| `ui_SCrunf` | active-router source depth/volume | return-flow exfiltration seam | Lane D surface source series | `gwbfv`; `gwdsv`; `latqcc` | queued |
