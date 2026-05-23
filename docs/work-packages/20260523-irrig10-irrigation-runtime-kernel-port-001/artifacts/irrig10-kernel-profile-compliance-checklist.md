# IRRIG10 Kernel Profile Compliance Checklist

Status: `completed`
Evidence mode: `Static + Ran`

## Checklist

1. Canonical `SC-*` files updated: `pass`
- `SC-IRRIG-001`, `SC-RUNOFFPART-001`, `SC-WATBAL-001`, `SC-CLIMATE-001`, index updated.

2. Required schema sections present in canonical authority: `pass`
- IRRIG10 addenda include required surfaces, deterministic rules, guard codes,
  and contract-test vectors.

3. Algorithm steps and branch behavior updated for changed runtime behavior: `pass`
- Runtime code implements fixed-date-first schedule priority, depletion fallback,
  typed event normalization, and coupled runoff/storage handling.

4. Guard/error mapping aligned with code: `pass`
- Missing/non-finite/domain failures map to typed WB14/WB12 guard codes.

5. Test-vector obligations reflected in tests + evidence: `pass`
- IRRIG10 integration tests implemented and passing.
- Pre-implementation gate recorded prior to production edits.

6. Required package gates executed: `pass`
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
