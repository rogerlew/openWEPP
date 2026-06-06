# HPHYS0298 Disposition

Status: hold

Evidence mode: static+ran

Static:

- HPHYS0298 was scoped to partition snow/`RM` residual source for the nine
  H1/H7/H39 target windows, not to compensate downstream water-balance
  consumers.
- Retrospective review `artifacts/review_claude_hrsnow_unit_artifact.md`
  identified the historical HPHYS0298 `hrsnow` verdict as a
  depth-vs-water-equivalent comparator artifact.
- HPHYS0299 supersedes the HPHYS0298 direct winter-forcing migration inference
  with corrected depth-vs-depth `hrsnow` authority.
- No openWEPP production physics code was changed.
- Production parity remains incomplete.

Ran:

- Full H1..H39 semantic suite through package runner:
  `.venv/bin/python docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/hphys0298_paired_lineage_partition.py --run-root /tmp/hphys0298_full_20260605T000000Z`
- Review-fix regeneration:
  `.venv/bin/python docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/hphys0298_paired_lineage_partition.py --run-root /tmp/hphys0298_full_20260605T000000Z --skip-full-suite --skip-targeted-traces`
- Workspace gates listed in `artifacts/gate-results.md`.
- Dual review and dual verification artifacts listed in
  `artifacts/review-disposition.md`.

## Verdict

`HOLD`.

HPHYS0298 completed its historical diagnostic-output objective but its headline
all-window `OPENWEPP-DEFECTIVE @ hrsnow` verdict is superseded and
non-authoritative for production migration. The paired ledger compared
pinned-baseline snowfall-depth symbol `hrsnow` against openWEPP
`snow_hourly_snowfall_water_equiv_sum_m`, a water-equivalent accounting
surface. The resulting near-10x ratio is a unit artifact, not proof that
openWEPP mispartitioned hourly precipitation.

Use HPHYS0299 corrected depth-vs-depth `hrsnow` evidence as the authority for
any continuation. HPHYS0298 remains useful as historical evidence that the
residuals are real, but it must not be used to justify winter hourly forcing
migration, WB13/WB17/WB18/WB19 compensation, or ADR conclusions by itself.

## Target-Window Partition

| Verdict | Count |
| --- | --- |
| `OPENWEPP-DEFECTIVE` | 9 |

| First Divergent Cut-Point | Count |
| --- | --- |
| `hourly-forcing` | 9 |

First divergent symbols:

- H1 first-2013: `hrsnow`
- H1 spring-2014: `hrsnow`
- H1 spring-2016: `hrsnow`
- H7 first-2013: `hrsnow`
- H7 spring-2014: `hrsnow`
- H7 spring-2016: `hrsnow`
- H39 first-2013: `hrrain`, `hrsnow`
- H39 spring-2014: `hrsnow`
- H39 spring-2016: `hrsnow`

## Closure Rationale

- Baseline observe instrumentation proved release, observe-off, and observe-on
  identity for H1/H7/H39.
- The corrected classifier checks hourly rain/snow forcing before raw melt and
  before negative-melt correction.
- Missing required openWEPP trace fields now fail closed as `trace-gap` /
  `UNRESOLVED`; no zero-fill or downstream fallback can create closure.
- `WB13` `RM` is no longer reconstructed from routed melt and rain when
  publication is missing.
- Per-symbol source provenance is published in
  `artifacts/paired-lineage-ledger.json`.

## Follow-On Recommendation

Do not scaffold a winter hourly snow/rain forcing migration from HPHYS0298
alone. Continue from HPHYS0299 and later corrected ledgers, where canonical
`hrsnow` is paired to an openWEPP snowfall-depth surface. Re-localize remaining
residuals from the corrected Total-Soil/drainage and snow/`RM` evidence chain.

Do not address the HPHYS0298 residual by changing WB13, WB17, WB18, or WB19
storage consumers unless a later package proves those surfaces are the first
divergent cut-point after corrected unit/provenance evidence closes upstream
surfaces.
