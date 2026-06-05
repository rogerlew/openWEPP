# HPHYS0298 Disposition

Status: hold

Evidence mode: static+ran

Static:

- HPHYS0298 was scoped to partition snow/`RM` residual source for the nine
  H1/H7/H39 target windows, not to compensate downstream water-balance
  consumers.
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

HPHYS0298 completed its diagnostic objective and localized all nine target
windows to the first divergent cut-point `hourly-forcing`. It did not close
production physics because the required correction is upstream of WB13, WB17,
WB18, and WB19.

This is a porting-fidelity defect against an unimpeached pinned-baseline
precipitation-phase partition routine. For this verdict,
`OPENWEPP-DEFECTIVE` means openWEPP failed to reproduce the baseline
`winter.for:410-412` `hrrain`/`hrsnow` partition before the corrected
negative-melt defect family is reached; it does not mean the residual was
accepted solely because openWEPP differed from the baseline.

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

Scaffold the next package as a baseline-authoritative winter hourly snow/rain
forcing partition migration. The write set should target the openWEPP climate
hourly forcing projection consumed by winter snow/freeze logic, with explicit
provenance to `/workdir/wepp-forest_260430_baseline` and the canonical
`SC-SNOWFREEZE-001` / `SC-WATBAL-001` obligations added by HPHYS0298. The
specific baseline lineage to port is
`/workdir/wepp-forest_260430_baseline/src/winter.for:410-412`.

Do not address the HPHYS0298 residual by changing WB13, WB17, WB18, or WB19
storage consumers unless a later package proves those surfaces are the first
divergent cut-point after hourly forcing is corrected.
