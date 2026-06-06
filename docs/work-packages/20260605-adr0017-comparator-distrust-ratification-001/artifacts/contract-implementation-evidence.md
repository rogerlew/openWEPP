# Contract Implementation Evidence

Status: complete

Evidence mode: Static

Static: ADR0017 was ratified as accepted governance and propagated into the
canonical governance surfaces:

- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
  now has `**Status:** Accepted` and names this package as ratification.
- `docs/decisions/0016-promote-260430-baseline-as-canonical-comparator-and-abandon-kernel-rewrite.md`
  records ADR0017 as an accepted amendment.
- `docs/decisions/README.md` lists ADR0017 as `Accepted`.
- `docs/specifications/correctness-authority-model.md` adds normative
  ADR0017 comparator-flag adjudication.
- `docs/specifications/science-contract-authoring-procedure.md` requires the
  ADR0017 comparator-distrust governance gate for comparator/ledger contracts.
- `docs/codex_exec_plans.md` requires comparator/ledger ExecPlans to include
  like-for-like unit/lineage proof, independent correctness authority, peer
  `HARNESS-SURFACE-MISMATCH`, and owned `HOLD`.
- `docs/specifications/unit-governance.md` applies ADR0017 to comparator/ledger
  unit and lineage-stage pairing.
- `docs/specifications/science-contracts/index.md` registers ADR0017 and links
  `SC-SNOWFREEZE-001#INV-SNOWFREEZE-039` and
  `SC-WATBAL-001#INV-WATBAL-087`.

Static: Affected canonical science contracts were amended:

- `SC-SNOWFREEZE-001` version `44` adds `INV-SNOWFREEZE-039`,
  `OBL-SNOWFREEZE-P-018`, a guard-map row, and in-place taxonomy correction for
  HPHYS0296-0298 historical rows.
- `SC-WATBAL-001` version `137` adds `INV-WATBAL-087`,
  `OBL-WATBAL-P-023`, a guard-map row, an ADR0017 water-balance addendum, and
  in-place taxonomy correction for HPHYS0296-0298 historical rows.

Static: No production Rust kernel/runtime file was edited by this package.
