# HPHYS0203 Contract Implementation Evidence

Status: completed  
Evidence mode: Static

## Canonical contract amendments
- Static: `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - `contract_version: 48 -> 49`.
  - Added `### HPHYS0203 Physics-Robustness Validation Addendum` requiring
    robustness vectors for profile, soil-water aggregate, and subsurface-loss
    publication families; conservation/order/domain/non-finite checks;
    deterministic perturbation vectors; per-family regression fixtures.
- Static: `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
  - `contract_version: 9 -> 10`.
  - Added `## HPHYS0203 Soil-Water Robustness Validation Addendum` for
    `Total-Soil`/`SoilWaterTotal` closure and fail-closed posture obligations.
- Static: `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
  - `contract_version: 7 -> 8`.
  - Added `## HPHYS0203 Subsurface WB13 Robustness Validation Addendum` for
    `latqcc`/`Dp` domain/non-finite guard obligations and regression fixtures.
- Static: `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
  - `contract_version: 69 -> 70`.
  - Added `## HPHYS0203 WB13 Robustness Governance Addendum` defining
    system-level robustness evidence obligations and parity-as-diagnostic
    posture.
- Static: `docs/specifications/science-contracts/index.md`
  - Updated SC registry notes for `SC-WATBAL-001`, `SC-SOIL-001`,
    `SC-SUBHYD-001`, and `SC-SYSTEM-001` with HPHYS0203 obligations.

## Contract versioning hygiene
- Static: touched contract files include revision-history rows for HPHYS0203
  with new version numbers and provenance notes.
