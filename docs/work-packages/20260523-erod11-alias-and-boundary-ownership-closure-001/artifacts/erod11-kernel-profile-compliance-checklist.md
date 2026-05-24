# EROD11 Kernel Profile Compliance Checklist

Status: `completed`
Evidence mode: `Static + Ran`

Reference profile:
`docs/specifications/science-contracts/kernel-process-contract-profile.md`

## Checklist

- [x] Canonical authority updates are implemented in `SC-*` files (not only in
      package-local artifacts).
- [x] Required companion contracts for erosion-lane Wave-0 alias closure are
      amended (`SC-SED-001`, `SC-HYDRAULICS-001`, `SC-ROUTE-001`,
      `SC-WATBAL-001`, `SC-RUNOFFPART-001`).
- [x] Symbol alias map sections now include explicit runtime alias ownership for
      required Wave-0 boundaries.
- [x] Cross-contract producer/consumer ownership is explicit in canonical
      contract addenda.
- [x] Gap-register promotability posture reflects closure of Wave-0 alias
      ambiguity while preserving deferred implementation risk rows.
- [x] Contract-derived integration test is implemented and executed.
- [x] Truthfulness labels (`Static:` / `Ran:`) are present in EROD11 evidence
      artifacts.
- [x] No production erosion kernel physics implementation was introduced in
      EROD11 scope.

Ran:
- `cargo fmt --check`
- `cargo test --test erod11_alias_boundary_ownership_contract`
