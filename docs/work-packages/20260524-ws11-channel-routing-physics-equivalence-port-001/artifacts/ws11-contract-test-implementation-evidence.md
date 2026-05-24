# WS11 Contract-Test Implementation Evidence

Status: `completed`
Evidence mode: `Static + Ran`

## Static
- Added contract-derived integration test target
  - `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
  - registered in `Cargo.toml`:
    - `name = "ws11_channel_routing_physics_equivalence_contract"`
- WS11 vector coverage encoded
  1. `ws11_contract_conformance_executes_ipeak_1_and_2_with_finite_outputs`
  - validates finite/non-negative channel outputs for `ipeak = 1` and
    `ipeak = 2`.
  2. `ws11_contract_conformance_executes_ipeak_3_and_4_with_routed_closure`
  - validates finite/non-negative outputs plus routed closure
    `roff = qpo * durrof` for `ipeak = 3` and `ipeak = 4`.
  3. `ws11_contract_conformance_requires_ipeak_symbol`
  - validates missing `ipeak` must hard-fail with channel guard family
    `WKERNEL-WS10-CHANNEL-E-001`.
  4. `ws11_contract_conformance_rejects_non_finite_ipeak`
  - validates non-finite `ipeak` must hard-fail with
    `WKERNEL-WS10-CHANNEL-E-002`.
  5. `ws11_contract_conformance_rejects_out_of_domain_ipeak`
  - validates out-of-domain `ipeak` must hard-fail with
    `WKERNEL-WS10-CHANNEL-E-003`.
  6. `ws11_contract_conformance_distinguishes_ipeak_branches`
  - validates branch-sensitive routing behavior is not collapsed to a
    single-gain surrogate path.
- Contract linkage
  - `SC-ROUTE-001` WS11 addendum vectors + `INV-ROUTE-006/007`
  - `SC-SYSTEM-001` WS11 addendum vectors + `INV-SYSTEM-005/006`
  - `SC-HYDRAULICS-001` WS11 consumer-coupling vector posture

## Ran
- Command
```bash
cargo test --test ws11_channel_routing_physics_equivalence_contract
```
- Result: **failed** (`2 passed; 4 failed`)
- Failure set captured for pre-implementation gate:
  - `ws11_contract_conformance_requires_ipeak_symbol`
  - `ws11_contract_conformance_rejects_non_finite_ipeak`
  - `ws11_contract_conformance_rejects_out_of_domain_ipeak`
  - `ws11_contract_conformance_distinguishes_ipeak_branches`
