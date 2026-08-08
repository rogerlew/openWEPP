# Assurance Impact

Status: PASS; DRAFT source custody re-adopted, no authority created.

Evidence mode: Static + Ran on 2026-08-08.

Exact source validation proved that the planned `SC-PLANT-001` and
`SC-RESIDUE-001` amendments affect the existing
`native-forest-canopy-phenology-evaluation` DRAFT report. The typed
`adopt-report-source` workflow was used for each already-declared external
`local_content` dependency. Because the operation accepts one external source
at a time and rejects unrelated simultaneous drift, the two sources were
adopted sequentially without changing their final authored content.

| Phase/source | Receipt | Generation |
|---|---|---|
| initial `SC-PLANT-001.md` | `3208ab181e5eb9261a51bb3d8ea63d25c133244b8cf25b6949b4f4eb3a26cc1f` | `92aeea68348b5acb69bc281a495a56b21e1323ba430e4fe4b3d39ef613bf9f07` |
| initial `SC-RESIDUE-001.md` | `b8f0e7ed62428b2e3bfd9a5fb603ca45e9698dd9d4a63b3a6c435f925d81458b` | `78d0a933bc07d82ef29f085b3e02485c0c165a78f0be7eaf291f1c79cd93f750` |
| review remediation `SC-PLANT-001.md` | `bb0bd503a7db0b2211c1810994f464165f98a87cf71bb8fc964cddffda0d4c7e` | `100681154b587d34d29b5c83328016c5d3ed0828290a50eb5714eea9cea57b63` |
| review remediation `SC-RESIDUE-001.md` | `642d054e638f3a6a3301e9af61f1dee6aa6bbadcbd63108145bc9762c980212f` | `f509db04ab810d00a2eded24d32e87189e6ee0c3a2c9650fa28eb80e980cb11f` |
| terminal addendum cleanup `SC-PLANT-001.md` | `c1d9ad5502198d10faef62be994525bf23cb69375114a27275028c0d14d1bcaa` | `22b4fda37bc176ae4fedc04fdeb13900304a13f72eb6c920194e422627af6817` |
| terminal addendum cleanup `SC-RESIDUE-001.md` | `df95b74417166de4ef891f20db27f3b1cad1c0d89be907b7fa582323a21363c6` | `90313e7b476cb5366605a1a708c29b5c2eeb68ecac36f90b00b9160b882c4fd8` |

The typed transaction also canonically re-rendered the existing DRAFT
`report.yaml` manifest (821 additions, 74 deletions) while leaving its semantic
report content and DRAFT/publication posture unchanged. That machine-owned
manifest rewrite is a required transaction output, not authored report prose;
the package write set and terminal reconciliation include this exact path.

Ran after the six-transaction initial, review, and terminal-remediation chain:

- `cargo nextest run --test assurance_v2_source_contract`: PASS, 12/12;
- `target/release/openwepp-assurance validate --all`: PASS, three DRAFT
  reports, zero public reports; and
- source root:
  `f9a47b670543cec4165b4673d99370f9f01f566af8926f4d82bf8c72765eb5c9`.

No finding, approval, realization, release-transfer, or publication authority
was created. The amendment only refreshes machine-owned DRAFT identity custody.
