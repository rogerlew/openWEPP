# REFACTOR024 Modularization Plan Report

Evidence class: Static

Planned split:

- Root file: declare child modules only.
- `support.rs`: fixture seed, helper functions, shared imports and re-exports.
- `contract_gates.rs`: CLIM06/SIMIMPL33 conformance tests and early
  FDHP01 Dh/Dj tests.
- `fine_layer.rs`: fine-sublayer, shadow seam, dispatch, C1b, and C2 exchange
  tests.
- `thermal_front.rs`: lower-front, top-thaw, heat-flow depth, warm thaw, and
  hard-fail tests.
- `publication.rs`: SIMIMPL32/FQ4 publication, lineage, layered-store, and
  cross-contract seam tests.

Movement rule:

- Move contiguous function/test blocks only.
- Add `use super::support::*;` to test modules.
- Add `pub(super)` to support items that sibling modules need.
- Do not edit assertions, fixture numeric values, formula constants, or message
  identifiers.
