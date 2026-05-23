# PL13A Kernel Profile Compliance Checklist

Status: `complete`
Evidence mode: `Static + Ran`

Reference profile: `docs/specifications/science-contracts/kernel-process-contract-profile.md`

1. Canonical `SC-*` authority updated for kernel-affecting continuity: `met`
- Static: `SC-PLANT-001` updated to `contract_version: 7`.

2. Alias continuity governance captured in canonical contract surfaces: `met`
- Static: `SC-PLANT-001` alias map amended with projected slot/crop
  continuity rows and `GAP-PLANT-007` closure row.

3. Canonical registry implementation aligned with documented authority: `met`
- Static: `openwepp-sim-contract` canonical registry now includes PL projected
  slot/crop alias templates and schedule drift reconciliation rows.

4. No silent alias substitution posture preserved: `met`
- Static: alias-registry authority explicitly states typed failures for missing
  aliases and prohibits silent substitution.

5. Parallel ownership constraints with PL13 respected: `met`
- Static: no edits to PL13 runtime kernel implementation files.

6. Verification tests executed for amended alias behavior: `met`
- Ran: `cargo test --test sim_contract_symbol_alias_registry -- --nocapture`
  passed.

7. Required repository validation gates executed: `met`
- Ran:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
