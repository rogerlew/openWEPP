# Implementation Evidence

Evidence class: Static + Ran.

SNOWDENSITY-08 added `SC-SNOWFREEZE-001` v87 authority for the gate rerun:
`INV-SNOWFREEZE-061`, `OBL-SNOWFREEZE-P-036`, and the
SNOWDENSITY-08 addendum.

Implementation changes:

- Added `tools/snowfreeze_observed/snowdensity08_gate_rerun.py`.
- Added `tests/integration/snowdensity08_gate_rerun.rs`.
- Added the integration test target in `Cargo.toml`.
- Updated SNOWDENSITY contract-version guards from v86 to v87.
- Updated the SNOWDENSITY-03 `physics_bulk` confinement allowlist for the
  package-specific 08 diagnostic script and guard test.
- Updated `docs/work-packages/README.md` and
  `docs/planning/snow-frost-fidelity-strategy.md`.

Execution:

- Ran `.venv/bin/python tools/snowfreeze_observed/snowdensity08_gate_rerun.py`.
- SNOTEL CoE-bound density rerun generated both fixed boundaries:
  `legacy_coe` and `coe_shortwave_albedo_v1`.
- Non-SNOTEL direct-production rerun generated five comparison reports.
- Compact committed decision artifact:
  `artifacts/snowdensity08_gate_rerun.json`.

Decision:

- SNOTEL density lineage gate: cleared.
- Non-SNOTEL coupled opt-in WAT path: absent.
- Default non-SNOTEL snow-control gate: not passed.
- Frost attribution: not authorized.
- Disposition:
  `COMPLETE-08-SNOTEL-CLEARED-FROST-ATTRIBUTION-BLOCKED`.

Corrective note:

- Initial `cargo test --workspace` exposed an expected allowlist drift in
  `snowdensity03_physics_bulk_offline_contract.rs` because the new
  SNOWDENSITY-08 diagnostic script mentions `physics_bulk`.
- The guard was updated to allow only the new package-specific script/test and
  to require `INV-SNOWFREEZE-061`.
- The full workspace test was rerun after the fix and passed.
