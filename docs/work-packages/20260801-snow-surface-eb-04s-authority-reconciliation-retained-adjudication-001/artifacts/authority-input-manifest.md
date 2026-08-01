# Authority Input Manifest

Evidence mode: `Static`.

Phase A is restricted to these four files:

1. `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md`
2. `docs/specifications/unit-governance.md`
3. `crates/openwepp-unit-boundary/src/lib.rs`
4. `docs/work-packages/20260731-snow-surface-eb-04e-corrected-population-runtime-qualification-001/artifacts/prospective-qualification-protocol.md`

Forbidden during Phase A: every path containing
`20260801-snow-surface-eb-04r`, `target/snow_surface_eb04r_factorial`,
`factorial-results`, `execution-attempt`, `terminal-frozen-protocol-audit`, or
observation data. The authority tool enforces this fixed whitelist and records
the exact input hashes.
