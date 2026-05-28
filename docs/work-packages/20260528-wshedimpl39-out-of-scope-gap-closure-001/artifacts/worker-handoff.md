# WSHEDIMPL39 Worker Handoff

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-28

## Static
- Canonical contracts/index updated for WSHEDIMPL39 scope:
  - `SC-ROUTE-001` v41 (`GAP-ROUTE-005` closed)
  - `SC-SYSTEM-001` v62 (`GAP-SYSTEM-002` closed,
    `GAP-SYSTEM-001` promotable-with-risk)
  - `SC-IMPOUND-001` v12 (`GAP-IMPOUND-003` closed)
  - `docs/specifications/science-contracts/index.md` notes refreshed
- Watershed runfile contract updated with required
  `[inputs.applicability]` selector table and explicit `CLIWAT-E-040`
  fail-closed semantics.
- Watershed CLI runtime intake now validates required applicability selectors
  before dispatch and fails closed on missing/disallowed declarations.
- Contract-derived watershed CLI applicability tests added and passing.
- Downstream EROD11/EROD12 gap-posture tests updated for new statuses.
- Required validation gate stack passed.

## Immediate Next Actions
1. If governance requires full closure beyond promotable-with-risk posture,
   open follow-on package to disposition remaining companion
   promotable-with-risk rows explicitly.
2. If GO readiness is accepted at current posture, proceed with normal commit
   and queue progression.

## Ran
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract` -> pass
- `cargo test -p openwepp --test erod11_alias_boundary_ownership_contract --test erod12_cross_domain_contract_closure_contract` -> pass
- `cargo fmt --check` -> pass
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- `cargo test --workspace` -> pass
- `cargo deny check` -> pass
