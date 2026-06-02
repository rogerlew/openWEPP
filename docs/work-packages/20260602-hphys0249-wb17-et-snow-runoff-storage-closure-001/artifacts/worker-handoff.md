# HPHYS0249 Worker Handoff

Status: complete

Evidence mode: static + ran

Static:

- Keep the WB17 `Es` layer-storage correction and post-WB19 `PlantRootUptake`
  ordering. They are contract-authoritative and covered by tests.
- Do not revert aggregate-storage worsening by restoring scalar ET
  over-withdrawal; that would violate `SC-EVAP-001#INV-EVAP-015`.

Ran:

- Final full-suite root:
  `/tmp/hphys0249_20260602T161254Z_postreview`.
- Runtime success: `39/39`.
- Semantic report success: `39/39`.
- Semantic pass: `0/39`.

Recommended next focus:

1. `Ep` lineage: root-depth/growth activation and crop/runtime `rtd`/`pltol`
   projection, because full-suite `Ep` is unchanged after `swu` support.
2. Snow/runoff timing: `Snow-Water`, `RM`, and `Q` are unchanged and remain
   high-leverage residuals.
3. Aggregate storage: reassess after `Ep` and snow/runoff timing corrections;
   current storage worsening is process-correct fallout from removing the old
   scalar `Es` over-withdrawal.
4. WB19 lateral magnitude: still open but not materially changed by this
   package.
