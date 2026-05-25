# simimpl18-contract-implementation-evidence

Status: complete
Evidence mode: static
Date: 2026-05-25

## Static
- Canonical contract amendments applied before production edits:
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
  - `INV-SNOWFREEZE-011` added for day-key cold partition closure and runtime
    SWE publication authority (no static `snow.options.ssd` leakage).
  - Guard map, obligations, invalid-state rows, CLIM05 addendum, and revision
    history updated (`contract_version: 5`).
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - `INV-WATBAL-026` and `INV-WATBAL-027` added for runtime-derived
    rain/snow/storage publication and multi-day storage mutation closure.
  - Guard map, invalid-state rows, producer obligations, boundary disposition,
    SIMIMPL18 addendum, and revision history updated (`contract_version: 34`).
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
  - `INV-CLIMATE-009` added for explicit full-span precipitation parity policy
    authority under baseline-year adaptation.
  - Guard map, invalid-state rows, producer/consumer obligations, boundary
    disposition, SIMIMPL18 addendum, and revision history updated
    (`contract_version: 11`).
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
  - `INV-SYSTEM-026` added for baseline-year policy + full-span keyed
    precipitation comparability governance.
  - Guard map, invalid-state rows, producer obligations, boundary disposition,
    SIMIMPL18 addendum, and revision history updated (`contract_version: 23`).
- Registry synchronization:
  - `docs/specifications/science-contracts/index.md` updated for
    `SC-CLIMATE-001`, `SC-SNOWFREEZE-001`, `SC-SYSTEM-001`, and
    `SC-WATBAL-001` SIMIMPL18 authority notes and reviewed date alignment.

## Ran
- not-run
