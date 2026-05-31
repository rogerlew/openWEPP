# AUTH02 Worker Handoff

Status: completed  
Evidence mode: Static

## Scope
- Handoff to AUTH03 (`20260531-auth03-level4-constitutive-gate-bootstrap-001`).

## Immediate next actions
1. Create first active suite-definition docs under
   `docs/specifications/external-authority/suites/` using the AUTH02 template.
2. Create matching fixture directories under `tests/fixtures/constitutive/`.
3. Add contract-derived integration harness(es) and register in `Cargo.toml`.
4. Amend `SC-WATBAL-001`/`SC-SOIL-001` invariants to reference initial Level-4
   suites and run gate evidence.
