# Contract Implementation Evidence

Status: completed
Evidence mode: static

Static: canonical unit governance authority was authored and linked from
contract governance docs.

## Authored Authority

- Added `docs/specifications/unit-governance.md:1`.
- The standard defines authority order at
  `docs/specifications/unit-governance.md:24`.
- Canonical internal units are enumerated at
  `docs/specifications/unit-governance.md:40`.
- Registry requirements are defined at
  `docs/specifications/unit-governance.md:63`.
- `BoundaryValue::scalar` policy is defined at
  `docs/specifications/unit-governance.md:83`.
- Conversion policy is defined at
  `docs/specifications/unit-governance.md:104`.
- Publication metadata policy is defined at
  `docs/specifications/unit-governance.md:123`.
- Contract and work-package gates are defined at
  `docs/specifications/unit-governance.md:134` and
  `docs/specifications/unit-governance.md:151`.

## Linked Governance

- `docs/specifications/science-contract-authoring-procedure.md:26` now links
  unit governance as a complementary authority.
- `docs/specifications/science-contract-authoring-procedure.md:248` adds
  normative unit-governance rules.
- `docs/specifications/science-contracts/kernel-process-contract-profile.md:17`
  links unit governance.
- `docs/specifications/science-contracts/kernel-process-contract-profile.md:83`
  requires a unit-governance map in kernel contracts.
- `docs/specifications/science-contracts/index.md:14` and
  `docs/specifications/science-contracts/index.md:24` make unit governance
  discoverable from the contract registry.

Ran: not-run; implementation was documentation/governance authoring only.
