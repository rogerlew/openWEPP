# Test Reconciliation Record

Evidence: Static + Ran
Date: 2026-06-10

## Reconciled Test

File: `tests/integration/hphys0202_profile_fc_wp_lineage_contract.rs`

Change: updated the contract authority section test so historical HPHYS0202 and
HPHYS0206 system rows are expected through:

- `SC-SYSTEM-001` Binding Exposure Index rows,
- conserved `INV-SYSTEM-027`, and
- `contracts/provenance/SC-SYSTEM-001-provenance.md`.

No behavior assertions were changed.

## Focused Verification

```console
cargo test --test hphys0202_profile_fc_wp_lineage_contract hphys0202_package_and_contract_authority_sections_exist
```

Result: passed (`1 passed; 0 failed`).

The same test also passed during `cargo test --workspace` before that workspace
run failed later in unrelated HPHYS0298 coverage.
