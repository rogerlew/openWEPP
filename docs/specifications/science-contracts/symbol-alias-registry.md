# Symbol Alias Registry

Status: `in_review`
Evidence mode: `Static`

## Purpose

Define canonical WEPP/wepp-forest symbol authority and explicit openWEPP
boundary-name aliases.

Implementation authority:
`crates/openwepp-sim-contract/src/symbols.rs`

## Authority Rules

- Canonical WEPP/wepp-forest symbols remain authoritative contract keys.
- openWEPP boundary names are aliases only.
- Reverse alias lookup must resolve to exactly one canonical symbol.
- Silent alias substitution is prohibited; missing aliases are typed failures.

## Registry Validation Rules

`SymbolAliasRegistry::new(...)` enforces:

1. non-empty canonical symbols,
2. non-empty boundary aliases,
3. no duplicate `(canonical, alias)` rows,
4. no ambiguous alias reuse across canonical symbols,
5. valid template-token usage,
6. non-empty registry.

## Supported Template Tokens

- `{ofe}`: one-or-more digits, non-zero (`1..`)
- `{idx4}`: exactly four digits (`0001..9999`)

These tokens are used across legacy-like aliases (for example
`ofe{ofe}_xinput_{idx4}`) and PL projected slot/crop aliases
(for example `pl_decomp_slot_{idx4}_crop_{idx4}_ncycle`).

## PL13A Alias Continuity Closure (PL09-GAP-007)

Closed continuity classes:

- schedule drift closure: boundary `conset`/`drset` now maps to canonical
  `conseq`/`drseq`.
- projected PL slot/crop families from PL10/PL11 now have canonical alias
  continuity templates for schedule/growth/decomp surfaces.
- indexed perennial projected families (`cutday/gday/gend/animal/bodywt/area/
  digest`) now have canonical alias continuity templates.

Scoped exception class (explicitly non-canonical structural metadata):

- scheduler topology/projection metadata symbols (for example
  `pl_schedule_slot_count`, `pl_schedule_rotation_years`,
  `pl_order_decomp_before_soil`) remain boundary-structural controls rather
  than canonical science variables.

## Representative Canonical Map (PL Surfaces)

| canonical symbol | representative boundary alias templates |
| --- | --- |
| `lanuse` | `pl_schedule_ofe{ofe}_lanuse`, `pl_schedule_slot_{idx4}_crop_{idx4}_lanuse` |
| `itype` | `pl_schedule_slot_{idx4}_crop_{idx4}_itype`, `pl_growth_slot_{idx4}_crop_{idx4}_itype` |
| `imngmt` | `pl_schedule_slot_{idx4}_crop_{idx4}_imngmt`, `pl_growth_slot_{idx4}_crop_{idx4}_imngmt`, `pl_growth_ofe{ofe}_imngmt_seed` |
| `tilseq` | `pl_schedule_slot_{idx4}_crop_{idx4}_tilseq` |
| `conseq` | `conset`, `conset_{idx4}`, `ofe{ofe}_conset_{idx4}`, `pl_schedule_slot_{idx4}_crop_{idx4}_conset` |
| `drseq` | `drset`, `drset_{idx4}`, `ofe{ofe}_drset_{idx4}`, `pl_schedule_slot_{idx4}_crop_{idx4}_drset` |
| `jdplt` | `pl_growth_slot_{idx4}_crop_{idx4}_jdplt` |
| `jdharv` | `pl_growth_slot_{idx4}_crop_{idx4}_jdharv` |
| `jdstop` | `pl_growth_slot_{idx4}_crop_{idx4}_jdstop` |
| `rw` | `pl_growth_slot_{idx4}_crop_{idx4}_rw` |
| `mgtopt` | `pl_growth_slot_{idx4}_crop_{idx4}_mgtopt`, `pl_decomp_slot_{idx4}_crop_{idx4}_mgtopt` |
| `resmgt` | `pl_decomp_slot_{idx4}_crop_{idx4}_resmgt` |
| `jdherb/jdburn/jdslge/jdcut/jdmove` | `pl_decomp_slot_{idx4}_crop_{idx4}_{root}` |
| `fbrnag/fbrnog/frcut/frmove` | `pl_decomp_slot_{idx4}_crop_{idx4}_{root}` |
| `ncut/ncycle` | `pl_decomp_slot_{idx4}_crop_{idx4}_{root}` |
| `cutday/gday/gend/animal/bodywt/area/digest` | `pl_decomp_slot_{idx4}_crop_{idx4}_{root}_{idx4}` |

## Lookup Surfaces

- canonical -> aliases: `aliases_for_canonical(...)`
- boundary alias -> canonical: `canonical_for_boundary_alias(...)`

Missing symbols are explicit typed errors:

- `CanonicalSymbolNotFound`
- `BoundaryAliasNotFound`

## Test Linkage

Covered by:

- `tests/integration/sim_contract_symbol_alias_registry.rs`
