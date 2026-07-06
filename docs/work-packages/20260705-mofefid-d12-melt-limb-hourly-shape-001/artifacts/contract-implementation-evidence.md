# Contract Implementation Evidence

Status: **COMPLETE**.

Static:

- `SC-OFEROUTE-001` `contract_version: 22`.
- Added required-input and branch/guard rows for the D12 source-shape limb.
- Added D12 `INV-OFEROUTE-012` disposition note with H2637 split counters.
- Updated `OBL-OFEROUTE-P-006` to remove melt-limb disposition from the
  remaining activation preconditions.
- No new unit conversion was introduced. Existing hourly depth limbs are
  normalized as weights; the active routing depth-to-rate conversion remains
  the pre-existing `/3600 s` seam helper.
