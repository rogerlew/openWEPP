# HPHYS0241 Pre-Implementation Contract Gate

Status: complete
Evidence mode: static

Static: canonical contract authority was amended before production-code edits:

- `SC-WATBAL-001` version `70` adds `INV-WATBAL-033` and the HPHYS0241
  MOFE hourly carry-array addendum for `ui_SUrunf`, `ui_SCrunf`, `ui_LfUrf`,
  and `ui_LfCrf`.
- `SC-RUNOFFPART-001` version `25` adds `INV-RUNOFFPART-013` and the
  array-derived runoff/runon carryover anti-shadow rules.
- `SC-SYSTEM-001` version `77` adds `INV-SYSTEM-028` and
  `mofe_hourly_carry` manifest/watershed-intake authority.
- `SC-ROUTE-001` version `44` adds `INV-ROUTE-014` and routing-admission
  continuity gating for MOFE hourly carry metadata.

Static: contract-derived tests were added before production-code edits:

- `tests/integration/wb11_hydrology_kernel_contract.rs` now contains
  HPHYS0241 scheduler vectors for array-derived carryover, copy-forward, and
  negative upstream carry rejection.
- `tests/integration/cli03_runner_contract_derived_tests.rs` now asserts
  hillslope manifests and watershed CLI source expose HPHYS0241 carry metadata.
- `tests/integration/mofe04_publication_contract_authority_closure_contract.rs`
  now checks MOFE hourly carry-array authority in `SC-WATBAL-001` and
  `SC-SYSTEM-001`.
- `tests/integration/mofe05_watershed_contributor_metadata_contract_authority_closure_contract.rs`
  now checks `SC-SYSTEM-001` and `SC-ROUTE-001` HPHYS0241 metadata gates.

Ran: not run at this gate. The required workspace validation commands remain
scheduled after production implementation.
