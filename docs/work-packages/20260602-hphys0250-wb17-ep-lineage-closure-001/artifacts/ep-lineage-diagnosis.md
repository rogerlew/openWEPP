# Ep Lineage Diagnosis

Status: complete

Evidence mode: static + ran

Static diagnosis:

- HPHYS0249 left `Ep` unchanged after WB17 layer-storage work, indicating the
  defect was upstream of WB13 publication or root-uptake magnitude.
- Static inspection found three missing lineage pieces:
  1. PL runtime activation was suppressed before scheduler execution.
  2. Established perennial `jdplt=0` slots were not treated as active.
  3. Initial live-canopy `cancov` was not assimilated into live PL state for
     no-growth established perennial H1-style management.

Ran diagnosis:

- H1 probe root: `/tmp/hphys0250_h1_probe_20260602T175631Z`.
- H1 now emits `Ep` on `1461/1461` rows with sum `92.6715228119` mm and
  max `1.0096567604` mm.
- H1 baseline has `Ep` sum approximately `3036.91` mm and max `7.78` mm,
  so magnitude remains substantially under baseline.
- H1 trace root: `/tmp/hphys0250_trace_probe_20260602T180043Z`.
- Trace summary shows PL state is now present: `pl_cancov=0.9`,
  `pl_lai=11.874843642`, `pl_vdmt=0.164470363785`, and `pl_rtd` is
  nonzero throughout the traced first year.
- Trace summary shows `Etp`/`UPi` are nonzero, `Ui`/`Ep` are nonzero,
  but `Ws` drops as low as `0.0037894343827`, indicating remaining
  under-`Ep` is dominated by root-uptake/water-stress magnitude and storage
  availability lineage.

Conclusion:

- Closed: missing scheduler/PL/canopy/publication lineage that produced zero
  `Ep` in H1-style cases.
- Open: baseline-authoritative `swu.for` uptake/stress/storage availability
  magnitude, likely coupled to layer storage, root weighting, and aggregate
  water-balance state, not comparator ingestion.
