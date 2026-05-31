# HPHYS0221 Contract Implementation Evidence

Status: completed
Evidence mode: Static

## Objective
Record canonical `SC-*` authority amendments for WB19 coupled water-yield and
saturated-depth behavior.

## Contract updates
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - `contract_version: 56`
  - Added HPHYS0221 WB19 addendum:
    - `solwpv` branch semantics (`2006` vs non-`2006`),
    - required WB19 porosity symbols (`por_####`),
    - coupled outputs: `wb19_fcdep`, `wb19_unsdep`, `wb19_watyld`,
    - WB19 guard semantics for non-finite / non-positive coupling paths.
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
  - `contract_version: 12`
  - Added WB19 coupling algorithm/guard details and invariant `INV-SUBHYD-015`
    for non-`2006` `watyld` positivity requirement.
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
  - `contract_version: 76`
  - Added WB19 coupled saturated-depth governance addendum.

## Result
- `MEASURE-HP221-001`: pass.
