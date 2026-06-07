# Contract Implementation Evidence

Status: complete

Evidence mode: Static.

## Contract Amendment

Static: `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
was amended from `contract_version: 38` to `39`.

Added invariant:

- `INV-RUNOFFPART-027`: WB12/WB14 must apply the Eq. [4.3.2] top-two-layer
  upper-storage condition before same-pass infiltration publication. When
  WB18/percolation has already published same-pass `wb12_infiltration`, WB14 must
  consume that producer value rather than recomputing infiltration from a later
  storage state.

Added guard-map row:

- runtime WB12/WB14 same-pass infiltration cap using top-two
  `wb18_perc_theta` versus `wb18_perc_ul`
- typed hard error on malformed storage symbols
- rainfall-excess residual instead of over-infiltration when storage
  availability is exhausted

## Authority

Static: the amendment stays within `SC-RUNOFFPART-001` runoff-partition authority
and uses physical storage availability plus the existing WB18/WB14 producer
boundary. No comparator magnitude target was added.
