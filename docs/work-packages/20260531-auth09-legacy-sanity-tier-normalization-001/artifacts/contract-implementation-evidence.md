# AUTH09 Contract Implementation Evidence

Status: completed  
Evidence mode: Static

Static:
- Added canonical Level-3 legacy/sanity authority tier to
  `docs/specifications/correctness-authority-model.md`.
  - Authority ranking now distinguishes:
    - Level-3 legacy/sanity suites (non-blocking investigation),
    - Level-4 constitutive suites (blocking for touched families),
    - Level-5/Level-6 validation tiers.
- Updated
  `docs/specifications/external-authority/suite-schema.md` authority-level enum
  to `3|4|5|6` and documented Level-3 semantics.
- Updated external-authority framework entrypoint
  `docs/specifications/external-authority/README.md` to codify Level-3 legacy
  posture and preserve Level-4+ legacy-free acceptance criteria.
- Re-tiered and renamed the WB19 branch suite in:
  - `docs/specifications/external-authority/registry.yaml`
  - `docs/specifications/external-authority/suites/cas_l3_subhyd_solwpv_fcdep_branch_001.md`
- Updated canonical SC addendum references to the new Level-3 suite ID:
  - `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
  - `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- Updated package index authorization stream:
  - `docs/work-packages/README.md`
