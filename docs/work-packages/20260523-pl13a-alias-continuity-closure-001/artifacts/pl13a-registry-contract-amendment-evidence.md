# PL13A Registry Contract Amendment Evidence

Status: `complete`
Evidence mode: `Static + Ran`

## Canonical Registry Amendment

Static:
- `crates/openwepp-sim-contract/src/symbols.rs`
  - Added PL projected slot/crop alias continuity templates.
  - Added schedule naming drift continuity (`conset -> conseq`,
    `drset -> drseq`).
  - Added indexed perennial alias templates for projected decomp families.

## Science Contract Amendment

Static:
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
  - `contract_version` updated from `6` to `7`.
  - Symbol alias map expanded for PL slot/crop projected families.
  - Added `GAP-PLANT-007` closed row documenting PL13A continuity closure.
  - Added revision-history entry for PL13A amendment.

## Alias Registry Authority Amendment

Static:
- `docs/specifications/science-contracts/symbol-alias-registry.md`
  - Updated status/evidence posture.
  - Added template-token authority section (`{ofe}`, `{idx4}`).
  - Added PL13A continuity closure summary and scoped structural exception.
  - Added representative PL projected canonical mapping table.

## Registry Index Note Amendment

Static:
- `docs/specifications/science-contracts/index.md`
  - Updated `SC-PLANT-001` note to include PL13A closure and explicit
    exception posture.

## Executable Validation Evidence

Ran:

```bash
cargo test --test sim_contract_symbol_alias_registry -- --nocapture
cargo test --workspace
```

Result:
- Alias registry integration tests pass (`13 passed, 0 failed`).
- Workspace tests pass with updated alias authority behavior.
