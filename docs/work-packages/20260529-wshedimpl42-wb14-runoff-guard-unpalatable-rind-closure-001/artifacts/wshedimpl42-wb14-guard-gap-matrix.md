# WSHEDIMPL42 WB14 Guard Gap Matrix

Status: completed  
Evidence mode: Static + Ran

## Gap Matrix
| Gap ID | Scope | Evidence | Disposition |
| --- | --- | --- | --- |
| `WSHEDIMPL42-GAP-001` | Breakpoint hyetograph cardinality seeding preferred stale `ninten` over active-day `nbrkpt` during breakpoint mode. | Static: `crates/openwepp-runner/src/hillslope/mod.rs` (`seed_wb11_runtime_surface_inputs`) previously selected `ninten` whenever present. Ran: pre-fix replay showed `HKERNEL-WB14-RUNOFF-E-003` across all hillslopes in `/tmp/wshed_parity_probe_20260529T044701Z/hillslope_batch_status.tsv`. | closed |
| `WSHEDIMPL42-GAP-002` | Watershed rerun intake rejected `pw0.imp` from unpalatable-rind because `jpond=0` violates parser domain guard. | Ran: `/tmp/wshedimpl42_unpalatable_20260529T143937Z/logs/watershed.stderr.log` => `CLIWAT-E-010 ... IMP-E-004 ... jpond ... expected >= 1`. | open (follow-on) |
| `WSHEDIMPL42-GAP-003` | Watershed rerun intake rejected hillslope pass files because `H*.hbp` outputs are ASCII daily pass text, not binary HBP shards. | Ran: `/tmp/wshedimpl42_unpalatable_20260529T143937Z/logs/watershed_retry.stderr.log` => `CLIWAT-E-017 ... HBP-E-002: bad magic`; `file /tmp/wshedimpl42_unpalatable_20260529T143937Z/hillslope_output/H1.hbp` => `ASCII text`. | open (follow-on) |
