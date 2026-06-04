# Contract Implementation Evidence

Status: completed
Evidence mode: static

Static: `docs/specifications/unit-governance.md` now identifies
`tools/release/check_sc_unit_compliance.sh` as the SC contract unit readiness
guard.

Implemented authority surface:

- Missing `Variables and Units` sections are lint failures.
- Missing `Symbol Alias Map` sections are lint failures.
- Missing `Symbol`/`Units` columns and missing alias `Units check` columns are
  lint failures.
- Empty or placeholder unit declarations are lint failures.
- Missing or unparseable executable registry source is a fail-closed lint
  failure.
- Alias `Units check` cells must mention executable registry units for symbols
  registered to the same owning `SC-*` contract.
- Registered boundary/API and publication aliases must appear in the owning
  contract's `Symbol Alias Map`.
- Registered boundary-symbol entries must be declared in the owning
  contract's `Variables and Units` section.
- `Variables and Units` coverage must include canonical registered symbols;
  alias-only coverage is rejected.

Supporting workflow documentation:

- `tools/release/README.md` lists the new guard and records current HOLD
  posture for full-contract runs.

Ran: not applicable for this artifact; contract-derived tests are recorded in
`contract-test-implementation-evidence.md`.
