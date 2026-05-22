# Specifications

This directory defines openWEPP specification authority and source hierarchy.

## Authority model

openWEPP contracts are authored top-down for openWEPP behavior.

Contract derivation order:
1. WEPP technical references (including `references/50201000`)
2. peer-reviewed literature invariants
3. physical/common-sense invariants
4. static legacy code inspection (secondary evidence)

Legacy static-code provenance defaults to the pinned baseline defined in
[ADR-0012](../decisions/0012-legacy-wepp-260430-baseline-anchor.md):
`/workdir/wepp-forest_260430_baseline`
(`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`).

Exception: binary pass serialization (`H<hillslope_id>.hbp`) remains based on
`/workdir/wepp-forest` HBP contract/implementation authority per ADR-0012.

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

Canonical WEPP input-file specification location:
- [wepp-input-files/README.md](wepp-input-files/README.md)
- OpenWEPP-owned canonical specs: `docs/specifications/wepp-input-files/specs/`
- Specification authoring procedure:
  `docs/specifications/wepp-input-specification-authoring-procedure.md`
- Parser contract data-model/propagation requirements:
  `docs/specifications/wepp-input-files/parser-contract-requirements.md`
- Parser input-surface registry:
  `docs/specifications/wepp-input-files/input-surface-registry.md`

Canonical subsystem specification location:
- [subsystems/README.md](subsystems/README.md)
- Canonical subsystem specs: `docs/specifications/subsystems/<subsystem>/`

Current chapter-to-contract scaffold:
- `docs/work-packages/20260520-sci01-50201000-process-contract-mapping/artifacts/50201000-chapter-process-contract-map.md`

Authoring workflow:
- [science-contract-authoring-procedure.md](science-contract-authoring-procedure.md) (required dual-agent review, disposition, and fix verification gate)
- [wepp-input-specification-authoring-procedure.md](wepp-input-specification-authoring-procedure.md) (required WEPP input specification authoring workflow and coverage/completeness gates)
- [wepp-input-file-parser-contract-authoring-procedure.md](wepp-input-file-parser-contract-authoring-procedure.md) (required parser-contract authoring workflow with dual-agent review and verification)

Promotion workflow:
- Work-package artifacts are draft and evidence surfaces.
- Stable normative specs must be promoted into canonical locations under
  `docs/specifications/`.
- Each promoting work package must document source artifact -> canonical file
  mapping in its disposition.

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
