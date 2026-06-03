# Verification Agent A

Status: complete
Evidence mode: ran

Ran: targeted verification

- `cargo test --test wb11_storage_projection_kernel_contract hphys0255 -- --nocapture`
  - Result: pass.
- `cargo test --test cli03_runner_contract_derived_tests cli03_mofe04_multiofe_publication_uses_canonicalized_oferow_and_total_area -- --nocapture`
  - Result: pass.

Conclusion

- Targeted HPHYS0255 tests verify both runtime projection semantics and manifest
  provenance.
