# Contract Disposition

Status: `EXECUTED`
Evidence mode: `Static`

Amended contract:

- `SC-ROUTE-001`: `contract_version` 48 -> 50.
- Changed `REF-ROUTE-CH13-PEAKIN` fallback wording from "any contributor lacks
  hourly surfaces" to "no contributor carries hourly surface authority".
- Tightened `INV-ROUTE-005` to an all-hourly or no-hourly inlet rule.
  Complete minor-1 inlets consume `V_h`/`S_h`; all-no-hourly inlets keep the
  triangular fallback; partial, malformed, mixed hourly/non-hourly inputs or
  hourly hillslope inputs with dependency nodes lacking channel-hourly surfaces
  fail closed.
- Added `SC-ROUTE-001` rev 50 as a profile-only Binding Exposure Index closure
  mapping active route/channel authority to existing `INV-ROUTE-*` bindings; no
  process-physics authority changed.

Contracts read and not amended:

- `SC-INFILE-HBP-001` already binds minor-1 field layout, structural parser
  validation, and run-level intake closure.
- `SC-SED-001` already binds hourly sediment mass and closure via
  `INV-SED-014`/`INV-SED-016`.
- `SC-OFEROUTE-001` already binds active Lane D routed-hydrograph production
  authority and baseflow/export prerequisites.
- `SC-RUNOFFPART-001` was not touched; no runoff-partition source-shape policy
  changed.

Profile/BEI disposition:

- No new process equation or unit conversion was added.
- `INV-ROUTE-005` remains the binding exposure for the changed runtime
  projection.
- `.venv/bin/python tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`:
  PASS, 6 binding exposure rows fully consolidated.
- `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`:
  PASS.
- `bash tools/release/check_unit_registry.sh`: PASS, 21 tests.
- Contract-derived tests landed in
  `tests/integration/wshedw5_typed_watershed_runtime_contract.rs`.
