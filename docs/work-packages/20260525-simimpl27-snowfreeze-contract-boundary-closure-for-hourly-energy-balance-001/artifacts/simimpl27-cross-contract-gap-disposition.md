# SIMIMPL27 Cross-Contract Gap Disposition

Status: complete
Evidence mode: static
Date: 2026-05-25

## Static
- Cross-contract ownership for migration-scope snow/freeze boundary payloads is
  explicit across:
  - `SC-SNOWFREEZE-001`
  - `SC-WATBAL-001`
  - `SC-RUNOFFPART-001`
  - `SC-SOIL-001`
  - `SC-SYSTEM-001`
- Gap reclassification outcome:
  - `GAP-SNOWFREEZE-002`: non-promotable -> promotable-with-risk
    (authority-side naming fixed; runtime emission pending).
  - `GAP-SNOWFREEZE-004`: non-promotable -> promotable-with-risk
    (companion contract ownership explicit; vector depth still pending).
  - `GAP-SNOWFREEZE-005`: remains promotable-with-risk with explicit alias and
    queued timing-validation tests.
- Remaining non-promotable gap outside SIMIMPL27 migration scope:
  - `GAP-SNOWFREEZE-003` (drift activation authority unresolved; drift remains
    inactive in baseline lineage).

## Ran
- not run
