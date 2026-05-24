# WS12 Implementation and Test Evidence

Status: `completed`
Evidence mode: `Static + Ran`

## Static
- Updated impoundment production lane in
  `crates/openwepp-watershed-orchestrator/src/lib.rs`:
  - removed WS10 headroom-retention surrogate computation from
    `run_impoundment_node`.
  - added required coefficient-family reads for node-scoped WS12 symbols:
    `a,b,c,d,e,ha,ht,hlm,a0,a1,a2,l0,l1,l2`.
  - added `require_impoundment_coefficient_scalar` with WS10 guard-family
    continuity for missing/non-finite payloads:
    - missing: `WKERNEL-WS10-IMPOUNDMENT-E-001`
    - non-finite: `WKERNEL-WS10-IMPOUNDMENT-E-002`
  - added continuity-domain guard enforcement for area denominator and stage
    update (`hnext`), mapped to
    `WKERNEL-WS10-IMPOUNDMENT-E-003`.
  - computes impoundment stage by continuity update
    `hnext = h + deltat * (Qi - (Qo + qinf)) / A(H)` where
    `A(H) = a0 + a1 * H^a2`.
- Updated `tests/integration/ws10_watershed_kernel_contract.rs` to seed WS12
  coefficient symbols in the WS10 fixture surface so existing WS10 conformance
  vectors remain runnable under the new required payload set.

## Ran
Executed commands:
```bash
cargo test --test ws10_watershed_kernel_contract
cargo test --test ws12_impoundment_physics_equivalence_contract
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
```

Results:
- `cargo test --test ws10_watershed_kernel_contract`: pass (`4 passed`).
- `cargo test --test ws12_impoundment_physics_equivalence_contract`: pass
  (`4 passed`).
- `cargo fmt --check`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo test --workspace`: **failed** at
  `cli01_contract_conformance_hillslope_run_emits_required_outputs_and_manifest`
  (`tests/integration/cli01_runner_hillslope_integration.rs`) with
  `ReleaseMetadata ... JSON parse EOF` in a debug-sidecar JSON file.
- `cargo deny check`: **failed** due repository-level dependency policy issues
  (`RUSTSEC-2025-0038` on `arrow2`, rejected `BSL-1.0` license entry for
  `xxhash-rust`, duplicate lockfile entries warnings).

## Notes
- WS12 targeted contract vectors are green post-implementation.
- Workspace/deny failures above were observed during the required gate sweep and
  are recorded here without suppression.
