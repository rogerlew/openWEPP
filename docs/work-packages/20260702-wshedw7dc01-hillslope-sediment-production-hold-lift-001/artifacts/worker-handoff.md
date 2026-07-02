# Worker Handoff

Status: `queued`

Evidence mode: not-run

First actionable item: close defect `WSHED-W7-HOLD-001`.

Seed evidence from W7:

- Legacy source `/wc1/runs/in/insensible-aliquot/wepp/output/H1.loss.dat`
  reports nonzero soil loss.
- Current openWEPP probes for multi-OFE hillslopes `1`, `21`, `172`, `297`,
  `333`, `390`, and `437` produced `tdet=0`, `tdep=0`, and `sedcon_*=0` across
  pass parquet rows despite `erod14_wave2_enabled=true`.
- W7 did not change hillslope sediment physics and held before fixture adoption.
