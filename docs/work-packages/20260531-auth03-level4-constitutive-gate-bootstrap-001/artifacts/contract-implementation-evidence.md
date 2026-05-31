# AUTH03 Contract Implementation Evidence

Status: completed  
Evidence mode: Static

## Scope
- Implement first active Level-4 constitutive authority suites and canonical
  contract amendments for FC/WP + relax-to-FC gating.

## Canonical contract amendments
- Updated `SC-SOIL-001`:
  - added `INV-SOIL-014` (FC/WP constitutive ordering + aggregate bounds),
  - added AUTH03 Level-4 FC/WP authority addendum,
  - updated revision history and contract version.
- Updated `SC-WATBAL-001`:
  - added AUTH03 Level-4 constitutive gate bootstrap addendum linked to
    `INV-WATBAL-006`,
  - updated revision history and contract version.
- Updated `docs/specifications/science-contracts/index.md`:
  - added AUTH03 references in `SC-SOIL-001` and `SC-WATBAL-001` registry notes.

## External-authority suite implementation
- Added active suite registry:
  - `docs/specifications/external-authority/registry.yaml`
- Added suite definitions:
  - `cas_l4_soil_fc_minus33_001`
  - `cas_l4_soil_wp_minus1500_001`
  - `cas_l4_watbal_relax_to_fc_001`

## Contract-first sequence evidence
1. Canonical contracts amended first (`SC-SOIL-001`, `SC-WATBAL-001`).
2. Suite registry + suite definitions authored next.
3. Contract-derived tests + fixtures implemented after authority linkage.
4. No production kernel runtime code edits were required in AUTH03 scope.
