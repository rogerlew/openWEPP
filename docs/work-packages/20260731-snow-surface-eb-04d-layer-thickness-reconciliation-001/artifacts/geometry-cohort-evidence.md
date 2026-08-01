# Geometry Cohort Evidence

Evidence class: `Ran`

The hash-bound corrected-binary replay executes the two exact EB-04A geometry
targets using their frozen fixtures and selectors.

| Target | Former rejected processing day | Final completed day | Result |
| --- | ---: | ---: | --- |
| `harvard_open/S` | 2,643 | 16,436 | complete |
| `marcell_open/LS` | 3,371 | 16,436 | complete |

Each trace contains the exact sequential day indexes `0..16436`, with each
former failure day present once. The maximum independently reconstructed mass
residual is `1.4432899320127035e-15 m`; the maximum physical-depth residual is
`3.3306690738754696e-16 m`. The machine-readable report retains Git HEAD,
build command, binary, source-diff, tool, imported harness, transitive observed
helper tree, EB-04A source-report, fixture, generated-runfile, and trace hashes.
Acceptance pins Git HEAD, binary, `crates/tests` source diff, source report,
imported harness, helper tree, both fixture hashes, selectors, failure days,
and failure classifications to expected authority values; recording a new hash
alone cannot silently admit drift.

The report separately parses the frozen EB-04A typed snapshots and reconstructs
all 5 Harvard and 14 Marcell layers, including the exact rejected fragments.
Corrected former-day layer topology is not falsely equated to that
counterfactual snapshot because the fix changes earlier conservative evolution.

Transient run outputs are under `target/snow_surface_eb04d_replay/`; the
accepted report is `geometry-cohort-replay.json`.
