# Contract Implementation Evidence

Status: complete

Evidence mode: Static

Static:

HPHYS0320 amended canonical `SC-*` authority before production code edits.

Contract amendments:

- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
  - Version `22`.
  - Added `REF-CLIMATE-WF-WNTTIM-MIN`.
  - Added `INV-CLIMATE-018`.
  - Added guard-map coverage for SIMIMPL28 `wnttim < 1.0` normalization.
  - Added `OBL-CLIMATE-P-013`.
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
  - Version `51`.
  - Added `REF-SNOWFREEZE-LEGACY-WNTTIM-MIN`.
  - Added `INV-SNOWFREEZE-046`.
  - Added guard-map coverage and `OBL-SNOWFREEZE-P-025`.
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - Version `144`.
  - Added `INV-WATBAL-094`.
  - Added guard-map coverage and `OBL-WATBAL-P-030`.
- `docs/specifications/science-contracts/index.md`
  - Registered HPHYS0320 governance pointers for `SC-CLIMATE-001`,
    `SC-SNOWFREEZE-001`, and `SC-WATBAL-001`.

Contract authority states that legacy breakpoint storm starts with
`wnttim < 1.0` normalize to `1.0` before `stmtim` active-interval evaluation.
