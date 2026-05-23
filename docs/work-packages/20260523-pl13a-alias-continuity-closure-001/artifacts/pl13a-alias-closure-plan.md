# PL13A Alias Closure Plan

Status: `complete`
Evidence mode: `Static + Ran`

## Closure Objective

Close `PL09-GAP-007` by reconciling projected PL runtime boundary symbols to
canonical WEPP symbols, with explicit disposition for any intentionally
non-canonical structural symbols.

## Inventory and Disposition Matrix

| closure_id | continuity surface | disposition | evidence |
|---|---|---|---|
| `PL13A-CL-001` | Schedule naming drift: `conset` continuity to canonical `conseq` | `closed` | Canonical alias rows added in `openwepp-sim-contract` registry + reverse-lookup tests |
| `PL13A-CL-002` | Schedule naming drift: `drset` continuity to canonical `drseq` | `closed` | Canonical alias rows added in `openwepp-sim-contract` registry + reverse-lookup tests |
| `PL13A-CL-003` | PL11 slot/crop projected families (`schedule`, `growth`, `decomp`) missing canonical alias continuity | `closed` | `SymbolAliasEntry` rows added for projected slot/crop families in canonical registry |
| `PL13A-CL-004` | Indexed perennial projected families (`cutday/gday/gend/animal/bodywt/area/digest`) missing slot/crop template continuity | `closed` | Indexed alias templates added and reverse lookup validated |
| `PL13A-EX-001` | Non-canonical scheduler structural metadata symbols (topology/order controls) | `exceptioned` | Explicitly documented as boundary-structural, not canonical science variables |

## Execution Sequence

1. Update canonical registry implementation authority in
   `crates/openwepp-sim-contract/src/symbols.rs`.
2. Add integration tests for alias-set presence and reverse-lookup closure in
   `tests/integration/sim_contract_symbol_alias_registry.rs`.
3. Amend canonical authority docs:
   - `docs/specifications/science-contracts/symbol-alias-registry.md`
   - `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
   - `docs/specifications/science-contracts/index.md`
4. Run required validations and package gates.

## Result

`PL09-GAP-007` closure target is met with explicit exception posture for
non-canonical structural scheduler metadata. No unresolved continuity row was
left silently deferred.
