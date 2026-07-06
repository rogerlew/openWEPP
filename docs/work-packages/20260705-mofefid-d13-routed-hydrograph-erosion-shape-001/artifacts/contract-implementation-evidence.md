# Contract Implementation Evidence

Status: **COMPLETE** (Static).

## Contract Revisions

- `SC-SED-001` rev 53:
  - `REF-SED-DC01-SHAPE` narrowed to default/off authority.
  - `REF-SED-LANED-ROUTED-HYDROGRAPH` added for active-routed-water hourly
    shape authority.
  - `hourly_runoff_fraction[h]` and `hourly_sediment_mass_kg[h]` unit rows
    added.
  - `INV-SED-013` now selects the Wave-1 hourly water shape by surface-water
    ownership and fails closed on missing/malformed/non-closing routed shapes.
- `SC-OFEROUTE-001` rev 23:
  - consumer scope/out-of-scope language updated for routed-hydrograph shape
    surfaces without sediment process ownership.
  - Branch/guard row added for routed-hydrograph erosion shape.
  - `INV-OFEROUTE-008`, `OBL-OFEROUTE-P-006`, and `OBL-OFEROUTE-C-004`
    updated.
  - D13 test-vector obligation and
    `OFEROUTE-EROSION-ROUTED-HYDROGRAPH` BEI row added as an active row
    routed to `science-review-follow-on` for the remaining producer/default
    activation work.

## ADR-0036

No ADR text change was required. ADR-0036 already authorizes the paired
hourly `V_h`/`S_h` surfaces and per-hour Wave-1 solve form. D13 changes the
active-routed-water shape authority feeding that existing form.

## Non-Claims

No production/default activation, DC01 disablement, D10 Case-4 acceptance,
D14 profiling, D15 default promotion, D11 friction rework, or D12 melt-limb
change is made by D13.
