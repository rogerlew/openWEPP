# Implementation Test Evidence

Status: complete
Evidence mode: static + ran

Static: production implementation

- `crates/openwepp-runner/src/hillslope/mod.rs`
  - Added `storage_lineage_policy` to `HillslopeWb13PublicationProvenance`.
  - Added policy constant `HPHYS0255_STORAGE_LINEAGE_POLICY =
    "single-runtime-wb11-state"`.
  - Populated the policy in `build_wb13_publication_provenance`.

Static: implementation boundary

- No WB11/WB13 storage math changed.
- No static area-weighted storage synthesis was added.
- No per-OFE dynamic hydrology state was introduced.

Ran: targeted closure tests

- `cargo test --test wb11_storage_projection_kernel_contract hphys0255 -- --nocapture`
  - Result: pass (`2 passed`).
- `cargo test --test cli03_runner_contract_derived_tests cli03_mofe04_multiofe_publication_uses_canonicalized_oferow_and_total_area -- --nocapture`
  - Result: pass (`1 passed`).
