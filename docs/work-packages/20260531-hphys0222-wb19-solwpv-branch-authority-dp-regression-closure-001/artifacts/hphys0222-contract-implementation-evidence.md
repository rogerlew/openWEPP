# HPHYS0222 Contract Implementation Evidence

Status: completed
Evidence mode: Static

## Objective
Record canonical contract authority amendments for WB19 `solwpv` branch-law
correction and external-authority gate linkage.

## Contract updates
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - `contract_version: 58`
  - Added `HPHYS0222 WB19 solwpv Branch-Authority Correction Addendum`:
    - `fcdep/unsdep` mutation limited to `solwpv < 2006`,
    - explicit no-mutation rule for `solwpv >= 2006` (including `9001+`),
    - required external-authority suite linkage.
  - Updated symbol-map wording for `wb19_watyld` usage to
    `solwpv < 2006` mutation scope.
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
  - `contract_version: 13`
  - Added HPHYS0222 addendum with the same branch-law correction and
    constitutive gate requirement.
  - Updated symbol-map wording for `wb19_watyld` usage to
    `solwpv < 2006` mutation scope.

## Result
- `MEASURE-HP222-001`: pass.
