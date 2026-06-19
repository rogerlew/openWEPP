# PERFDEEP09 Contract-Test Implementation Evidence

Status: complete.
Evidence class: Ran.

Focused regression added:

- `pl12_contract_conformance_rejects_unexpected_indexed_perennial_symbol`
  in `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/boundaries.rs`.

The test injects `pl_decomp_slot_0001_crop_0001_digest_0002` while
`ncycle=1` and verifies:

- halted phase is `DecompositionTransition`;
- message id is `HS-DECOMP-E-008`;
- boundary class is `DomainViolation`.

Focused commands:

```text
cargo test -p openwepp-hillslope-orchestrator pl12_contract_conformance
cargo test -p openwepp-hillslope-orchestrator decomposition
```

Both passed before the final H2637 gate.
