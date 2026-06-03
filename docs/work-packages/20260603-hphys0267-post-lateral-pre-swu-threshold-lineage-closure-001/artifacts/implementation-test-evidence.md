# Implementation and Test Evidence

Status: completed
Evidence mode: Static + Ran

Static:

- `crates/openwepp-runner/src/hillslope/mod.rs` extends the opt-in HPHYS trace
  schema with WB18 `fc`, WB19 `coca`, WB19 `frzw`, derived WB19 `drfc`, and
  derived WB19 `fzdrfc` per layer.
- Derived trace fields use existing run-state values and do not change
  production hydrology, ET, lateral-flow, snow, or storage physics.
- Focused tests assert trace-row capture and JSON serialization for the new
  threshold fields.
- `hphys0267_diagnostics.py` classifies H1/H7/H39 first-divergence rows using
  pre-lateral, post-lateral, and post-SWU trace rows plus comparator context.

Ran:

- Focused Rust trace tests passed as recorded in `gate-results.md`.
- Diagnostic Python compile passed as recorded in `gate-results.md`.
- H1/H7/H39 targeted trace classification passed and produced
  `/tmp/hphys0267_20260603T162040Z/reports/hphys0267_threshold_lineage_classification.md`.
- Full H1..H39 semantic metrics ran and produced
  `/tmp/hphys0267_20260603T162040Z/reports/hillslope_semantic_summary.md`.

Production patch decision: blocked. The evidence did not prove an in-scope
baseline-authoritative production defect.
