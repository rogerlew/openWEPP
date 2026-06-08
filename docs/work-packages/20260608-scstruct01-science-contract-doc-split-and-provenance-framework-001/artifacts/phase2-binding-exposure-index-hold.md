# Phase 2 Binding Exposure Index Checkpoint

Evidence mode: Static
Status: HOLD - binding exposure precondition failed closed

## What changed

- Added a conservative `Binding Exposure Index` to `SC-WATBAL-001` before the addendum block.
- Added `tools/check_sc_binding_exposure.py` as the static binding-exposure lint.
- Did not relocate any `SC-WATBAL-001` narrative to a provenance sidecar because many addenda still lack precise canonical binding exposure.

## Conservation posture

No `INV-*` or `OBL-*` rows were removed, weakened, or added. No addendum narrative was moved out of the contract core. The index exposes the current consolidation blocker rather than pretending a broad token scrape is a binding map.

## HOLD reason

The live binding set cannot yet be conserved through sidecar consolidation because many addendum sections contain binding language without same-section `INV-WATBAL-*` or `OBL-WATBAL-*` mappings, and some sections reproduce broad invariant/guard material that is not a precise map.

## First actionable follow-through

Close `SCSTRUCT01-WATBAL-BEI-MAPPING`: semantically map each `unpromoted-binding` or `undecidable` index row to existing canonical binding IDs, promote truly unpromoted binding obligations through the flagged review gate, or route them to science-review follow-on. Only then move historical/superseded narrative to a non-binding provenance sidecar.

## Index row counts

- `maps-to-existing-INV`: 6
- `undecidable`: 8
- `unpromoted-binding`: 61
