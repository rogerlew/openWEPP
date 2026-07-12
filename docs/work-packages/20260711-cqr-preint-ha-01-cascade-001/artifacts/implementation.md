# Implementation

Evidence class: **Ran**

Scaffold commit `c0a75d8e` predates all Rust/test edits. Characterization first
reduced the selected row to iteration CRAP 7. Review B rejected a provisional
arm exclusion, so the existing point-sampling match was moved verbatim into
private `sample_upstream_point` and both arms were directly tested. The caller
closure delegates with the same `upstream`, `time`, and `width_ratio`. No
formula, float grouping, evaluation/accumulation order, error, public API,
handoff selection, or allocation changed.
