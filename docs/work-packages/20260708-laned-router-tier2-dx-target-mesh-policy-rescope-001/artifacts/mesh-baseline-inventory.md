# Mesh Baseline Inventory

Status: QUEUED
Evidence mode: not-run.

Populate during T2R-B.

Required inventory:
- Active plain mesh builder call sites.
- Current fixed-cell constants and defaults, including the separate shadow
  lane constant.
- Per-member OFE lengths.
- Current effective `dx = ofe_length_m / cells_per_ofe`.
- Current time-step caps; they remain fixed across this package's mesh ladder.
- Candidate cell counts for the package ladder.
- Exact release binary provenance for baseline runs.
- Baseline timings, counters, closures, and output hashes. Baseline is a judged
  rung, not reference truth.
