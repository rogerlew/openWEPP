# PL14S Legacy Comparison Suite Design

Status: `completed`
Evidence mode: `Static`

## Static

### Design objective
Provide a reusable, provenance-aware comparison suite for Tier-A hillslope
water-balance semantic parity investigations where strict raw diff signals are
retained for structure checks but semantic diagnostics are first-class outputs.

### Design scope
- Included
  - Legacy baseline replay (`wepp_260430_hill`) from fixture runs.
  - Strict raw compare lane for `.dat` candidates.
  - Semantic compare lane for `.dat` and `.parquet` candidates.
  - Provenance manifest with command/binary/tool/output checksums.
  - Investigation-grade semantic diagnostics (row deltas, per-column stats,
    top divergent keys).
- Excluded
  - Erosion/sediment parity surfaces.
  - Watershed/hourly promotion claims.

### Suite components
- `tools/legacy_comparison_suite/run_pl14s_legacy_suite.py`
  - orchestrates baseline replay, strict compare (when applicable), semantic
    compare, and provenance publication.
  - enforces hard guards for ambiguous baseline-output discovery and malformed
    semantic report payloads.
- `tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
  - normalizes input surfaces (`.dat` 20/25-column variants and parquet WB13
    projections).
  - emits semantic report schema `pl14s-semantic-wat-v1`.
  - hard-fails on duplicate `(OFE,J,Y)` keys to prevent silent row overwrite.
- `tools/legacy_comparison_suite/configs/pl14s_wat_tolerances.json`
  - default and per-column semantic tolerance profile used for investigation.

### Data model and report shape
- Row-key authority
  - `(OFE, J, Y)` per-row identity used for set-delta and divergence scoring.
- Semantic report required sections
  - `comparison.semantic_pass`
  - row-set deltas (`only_baseline_*`, `only_candidate_*`)
  - column-coverage diagnostics (`shared_columns`, `baseline_only_columns`,
    `candidate_only_columns`)
  - investigation diagnostics (`investigation_columns_used`,
    `investigation_columns_missing`, `top_divergent_rows`)
  - per-column tolerance verdicts (`column_stats`)
- Provenance report additions
  - suite schema marker `pl14s-legacy-suite-v1`
  - candidate format capture (`.dat` or `.parquet`)
  - strict comparator required/skipped posture
  - semantic summary extracted from validated semantic report

### Guard behavior summary
- Strict comparator
  - required for `.dat` candidates;
  - explicitly recorded as skipped for parquet candidates.
- Semantic comparator
  - required for all candidate formats;
  - missing required semantic sections is an execution failure.
- No silent defaults
  - duplicate row keys hard-fail;
  - ambiguous baseline output discovery hard-fail;
  - provenance must expose strict lane skipped/executed posture.

### Reuse posture for successor packages
- Reusable as the default PL14S/PL15S investigation harness for hillslope WB13
  parity lanes.
- Consumer packages can tighten tolerance profiles or investigation columns
  through config updates without changing orchestration flow.

## Ran
- Phase A design artifact is static-by-scope.
- Executed replay/comparator evidence using this design is recorded in:
  - `pl14s-comparator-run-provenance-manifest.md`
  - `pl14s-tier-a-semantic-parity-delta-report.md`
