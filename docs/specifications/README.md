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
