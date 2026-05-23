# PL13A Canonical Alias Table Diff

Status: `complete`
Evidence mode: `Static + Ran`

## Canonical Registry Delta Summary

Static:
- File updated: `crates/openwepp-sim-contract/src/symbols.rs`.
- Added `41` canonical alias rows for PL projected slot/crop continuity and
  schedule naming drift reconciliation.

## High-Signal Additions

| canonical symbol family | representative new alias templates |
|---|---|
| `lanuse` | `pl_schedule_ofe{ofe}_lanuse`, `pl_schedule_slot_{idx4}_crop_{idx4}_lanuse` |
| `itype` | `pl_schedule_slot_{idx4}_crop_{idx4}_itype`, `pl_growth_slot_{idx4}_crop_{idx4}_itype` |
| `imngmt` | `pl_schedule_slot_{idx4}_crop_{idx4}_imngmt`, `pl_growth_slot_{idx4}_crop_{idx4}_imngmt`, `pl_growth_ofe{ofe}_imngmt_seed` |
| `tilseq` | `pl_schedule_slot_{idx4}_crop_{idx4}_tilseq` |
| `conseq` | `conset`, `conset_{idx4}`, `ofe{ofe}_conset_{idx4}`, `pl_schedule_slot_{idx4}_crop_{idx4}_conset` |
| `drseq` | `drset`, `drset_{idx4}`, `ofe{ofe}_drset_{idx4}`, `pl_schedule_slot_{idx4}_crop_{idx4}_drset` |
| `jdplt/jdharv/jdstop/rw` | `pl_growth_slot_{idx4}_crop_{idx4}_{root}` |
| `resmgt/mgtopt` | `pl_decomp_slot_{idx4}_crop_{idx4}_{root}` (+ growth alias for `mgtopt`) |
| `jdherb/jdburn/jdslge/jdcut/jdmove` | `pl_decomp_slot_{idx4}_crop_{idx4}_{root}` |
| `fbrnag/fbrnog/frcut/frmove` | `pl_decomp_slot_{idx4}_crop_{idx4}_{root}` |
| `ncut/ncycle` | `pl_decomp_slot_{idx4}_crop_{idx4}_{root}` |
| `cutday/gday/gend/animal/bodywt/area/digest` | `pl_decomp_slot_{idx4}_crop_{idx4}_{root}_{idx4}` |

## Authority Surface Diffs

Static:
- `SC-PLANT-001` alias-map updated and `contract_version` bumped `6 -> 7`.
- `science-contracts/symbol-alias-registry.md` rewritten to include PL13A
  continuity closure and scoped structural exception class.
- `science-contracts/index.md` note for `SC-PLANT-001` updated to reflect
  PL13A closure semantics.

## Executable Evidence

Ran:

```bash
cargo test --test sim_contract_symbol_alias_registry -- --nocapture
```

Result:
- `13 passed, 0 failed`.
- New coverage includes:
  - projected PL slot/crop alias-set presence checks
  - reverse lookup for `conset/drset` and indexed slot/crop families
  - invalid-width index reject remains typed (`BoundaryAliasNotFound`)
