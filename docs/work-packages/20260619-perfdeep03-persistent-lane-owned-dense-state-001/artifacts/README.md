# Artifacts

Status: scaffolded 2026-06-19 (pending Codex execution).

Required deliverables:

- `perfdeep03-ownership-refactor.md` - the from-temporary-mirror → lane-owned persistent frame refactor;
  the true-boundary materialization set; the dirty-slot tracking; proof of no per-day temporary frame /
  no full-frame seed/flush loop in phase execution.
- `perfdeep03-endpoint.md` - **the load-bearing gate**: real H2637 endpoint vs PERFDEEP01 669.97 s + RSS.
  Opt-in dense path must be measurably faster (target ~407–450 s / ~43–50×).
- `perfdeep03-identity.md` - H2637 output identity (`.hbp`/`.wat` byte-identical, `pass` Arrow-equal) +
  round-trip zero-mismatch preserved.
- `perfdeep03-gate-results.md` - workspace Rust gates + `cargo deny` + markdown + determinism.
- `perfdeep03_disposition.md` - CONTINUE (measured win → expand island / default activation) or §7
  falsification (still ≥ baseline → re-profile).
