# HPHYS0277 Climate Radiation Physical Flux Guard Artifacts

Status: completed/HOLD
Evidence mode: mixed static-and-ran

Static: artifact index for HPHYS0277 contract-first implementation, validation,
dual review, dual verification, and disposition evidence.

Ran: local contract, unit, lint, targeted H1/H7/H39, and full H1..H39 diagnostic
commands are recorded in `gate-results.md` and `implementation-test-evidence.md`.
`cargo test --workspace` remains HOLD for the known SIMIMPL18/WB11 ET domain
violation outside the HPHYS0277 write set.

## Artifact Map

- `baseline-provenance-map.md`: baseline source lineage and derived hourly
  radiation bound.
- `contract-implementation-evidence.md`: canonical `SC-CLIMATE-001` and index
  amendments.
- `contract-test-implementation-evidence.md`: red/green contract-derived test
  evidence.
- `pre-implementation-contract-gate.md`: pre-code red gate.
- `implementation-test-evidence.md`: implementation summary and validation.
- `targeted-h1-h7-h39-radiation-guard-metrics.md`: focused H1/H7/H39 metrics.
- `full-39-suite-metrics.md`: full H1..H39 diagnostic summary.
- `review_agent_a.md`, `review_agent_b.md`: independent review artifacts.
- `verification_agent_a.md`, `verification_agent_b.md`: independent
  verification artifacts.
- `disposition.md`: final package disposition and remaining HOLD scope.
