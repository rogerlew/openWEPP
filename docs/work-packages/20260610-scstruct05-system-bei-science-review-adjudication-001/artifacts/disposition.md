# SCSTRUCT05 Disposition

Evidence: Static + Ran
Date: 2026-06-10
Status: `executed-HOLD`

## Disposition

SCSTRUCT05 is closed as `executed-HOLD`. It reduced the SC-SYSTEM core by
relocating three historical profile-lineage addenda, resolved 16 of 27 BEI rows,
and converted the remaining 11 rows into narrower named science gates.

## Conservation

- `INV-*` / `OBL-*` rows added: none.
- `INV-*` / `OBL-*` rows removed: none.
- Binding obligations weakened: none.
- Kernel/runtime files edited: none.
- Comparator re-tiering: none.

## Final BEI State

- Total BEI rows: 27.
- Historical/superseded relocated rows: 3.
- Mapped-to-existing rows: 13.
- Deferred narrower HOLD rows: 11.

## Gate Disposition

`cargo fmt --check`, focused test reconciliation, `cargo deny check`, BEI default
lint, and `git diff --check` passed. `cargo clippy --workspace --all-targets -- -D
warnings` failed in unrelated `owcmp_cli_contract.rs`. `cargo test --workspace`
failed later in unrelated `hphys0298_paired_lineage_partition_contract.rs`.

Strict BEI lint remains `PASS-DEFERRED` with exit `1`, expected for executed-HOLD.
