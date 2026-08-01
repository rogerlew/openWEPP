# Deterministic Regeneration Evidence

Status: `PASS`

Evidence class: `Ran`

The frozen tool's `--analysis-only` route was run twice against the retained
WAT/trace population without executing model cells. SHA-256 manifests for
`qualification-results.json`, `cell-qualification.csv`,
`runtime-qualification.md`, and all four SVG/Markdown figure pairs were
byte-identical. Both reductions retained `acceptance_passes=true`.

The two exact output manifests and their comparison are retained in
`determinism-comparison.md`.
