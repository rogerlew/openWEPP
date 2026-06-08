# REFACTOR019 Line-Count Governance Checklist

Status: complete
Evidence mode: Static/Ran

Static:
- `.rs` threshold requirements:
  - 2000+ lines: WARN and requires explicit decomposition rationale
  - 3000+ lines: must be refactored before closure unless explicit generated/fixture exception applies
- Pre-refactor large file was 2110 lines (single WARN-only boundary).
- Post-refactor files are split across one facade plus five module files.

Ran:
- 2026-06-08T22:50:27Z: post-refactor line counts:
  - `hydrology_phase_storage_erosion.rs`: 5
  - `hydrology_phase_storage_reconciliation.rs`: 160
  - `hydrology_phase_erod13.rs`: 246
  - `hydrology_phase_erod14.rs`: 648
  - `hydrology_phase_erod19.rs`: 612
  - `hydrology_phase_peak_runoff.rs`: 466
  - Total across group: 2137
- Post-refactor longest file: 648 (<2000; <3000), so WARN/requirement satisfied with explicit decomposition rationale completed.
