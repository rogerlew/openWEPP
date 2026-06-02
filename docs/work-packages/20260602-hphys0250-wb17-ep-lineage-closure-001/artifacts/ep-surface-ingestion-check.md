# Ep Surface Ingestion Check

Status: complete

Evidence mode: static + ran

Static:

- WB13 publication now consumes flux-surface `Ep` after post-WB19
  `PlantRootUptake`; stale state-surface `Ep` no longer shadows final root
  uptake in the HPHYS0250 test vector.
- HPHYS0245 trace rows include post-phase `ep_m`, `ui_m`, `upi_m`,
  `etp_m`, and `ws` fields for ingestion diagnosis.

Ran:

- H1 probe root: `/tmp/hphys0250_h1_probe_20260602T175631Z`.
- Candidate H1 `Ep`: nonzero `1461/1461`, sum `92.6715228119` mm,
  max `1.0096567604` mm.
- Full 39 suite root: `/tmp/hphys0250_20260602T175731Z`.
- Full-suite `Ep`: `0/39` pass, fail count `56230`, mean abs diff mean
  `1.683413925`, max abs diff `7.778431644` at H13 day/key
  `[1, 202, 2015]`.

Conclusion:

- The semantic comparator now sees nonzero candidate `Ep`; ingestion is no
  longer the primary defect.
- The continuation focus should be `swu.for`-equivalent actual uptake/stress
  magnitude and water-availability lineage.
