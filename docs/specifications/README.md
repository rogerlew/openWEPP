# Science Contracts

This directory defines openWEPP science-contract authority and source hierarchy.

## Authority model

openWEPP contracts are authored top-down for openWEPP behavior.

Contract derivation order:
1. WEPP technical references (including `references/50201000`)
2. peer-reviewed literature invariants
3. physical/common-sense invariants
4. static legacy code inspection (secondary evidence)

Legacy contracts and legacy runtime behavior are valuable references, but not
automatic authority for acceptance decisions in all execution surfaces.

Stable references use:

```
SC-<DOMAIN>-<NNN>#INV-<DOMAIN>-<NNN>
```

Canonical science-contract location:
- [science-contracts/README.md](science-contracts/README.md)
- Canonical `SC-*` files: `docs/specifications/science-contracts/contracts/`
- Canonical registry: `docs/specifications/science-contracts/index.md`

Current chapter-to-contract scaffold:
- `docs/work-packages/20260520-sci01-50201000-process-contract-mapping/artifacts/50201000-chapter-process-contract-map.md`

Authoring workflow:
- [science-contract-authoring-procedure.md](science-contract-authoring-procedure.md) (required dual-agent review, disposition, and fix verification gate)

## openWEPP usage

- Architecture and module work may proceed before one-for-one legacy
  re-kernelization.
- Kernel work packages must cite the governing openWEPP contract invariants.
- If a legacy surface is under-specified, document it as a contract gap and
  resolve via top-down contract authoring in this repo.
- Legacy behavior deltas are triaged using the comparator confidence tiers from
  [ADR-0011](../decisions/0011-architecture-first-top-down-science-contracts.md).

## Tolerance bounds

Semantic-parity tolerance bounds for openWEPP comparator harnesses are part of
the contract. Per [../decisions/0003-parity-semantic-not-bit.md](../decisions/0003-parity-semantic-not-bit.md), contracts without explicit tolerance bounds block acceptance.
