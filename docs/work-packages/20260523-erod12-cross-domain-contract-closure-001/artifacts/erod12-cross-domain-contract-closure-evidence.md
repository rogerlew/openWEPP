# EROD12 Cross Domain Contract Closure Evidence

Status: `completed`
Evidence mode: `Static + Ran`

Static:
- Canonical cross-domain ownership and guard-closure authority was added in
  required companion contracts:
  - `SC-SED-001`
  - `SC-HYDRAULICS-001`
  - `SC-ROUTE-001`
  - `SC-RUNOFFPART-001`
  - `SC-WATBAL-001`
  - `SC-SYSTEM-001`
- Wave-0 cross-domain blocker rows from EROD10-AH-002 are now explicitly
  dispositioned to `closed` in canonical `SC-*` files:
  - `GAP-SED-003`
  - `GAP-HYD-003`
  - `GAP-ROUTE-003`
  - `GAP-RUNOFFPART-004`
- Non-Wave-0 governance/compatibility holds remain explicit and are not
  silently down-classified:
  - `GAP-ROUTE-005`
  - `GAP-RUNOFFPART-003`
  - `GAP-WATBAL-002`
  - `GAP-SYSTEM-001`
- Registry notes in `docs/specifications/science-contracts/index.md` were
  updated to reflect EROD12 closure posture.

Ran:
- Verified canonical addenda and gap-row posture by repository inspection
  commands (`rg`, `sed`).
- Executed contract-derived integration test:
  `cargo test --test erod12_cross_domain_contract_closure_contract`.
