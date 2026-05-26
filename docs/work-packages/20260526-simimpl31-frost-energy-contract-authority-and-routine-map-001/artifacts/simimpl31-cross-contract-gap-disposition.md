# SIMIMPL31 Cross-Contract Gap Disposition

Status: complete
Evidence mode: static+ran
Date: 2026-05-26

## Static
- Cross-contract producer/consumer ownership for frost runtime payloads remains
  explicit across:
  - `SC-SNOWFREEZE-001`
  - `SC-SOIL-001`
  - `SC-RUNOFFPART-001`
  - `SC-WATBAL-001`
  - `SC-SYSTEM-001`
- Gap reclassification outcome:
  - `GAP-SNOWFREEZE-002`: authority ambiguity closed; remains
    promotable-with-risk until SIMIMPL32 frost-hourly tests and runtime
    implementation follow-ons are complete.
  - `GAP-SNOWFREEZE-004`: remains promotable-with-risk with explicit
    cross-contract ownership, but executable frost-hourly comparator evidence
    is still pending SIMIMPL32 and SIMIMPL35.
- Remaining non-promotable gap outside SIMIMPL31 migration scope:
  - `GAP-SNOWFREEZE-003` (drift activation authority unresolved).

## Ran
- `rg -n "GAP-SNOWFREEZE-00[2-5]" docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `rg -n "frost\.runtime_|SC-SOIL-001|SC-RUNOFFPART-001|SC-WATBAL-001|SC-SYSTEM-001" docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
