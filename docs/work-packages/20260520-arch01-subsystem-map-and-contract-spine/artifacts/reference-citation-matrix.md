# Reference Citation Matrix

Status: complete
Date: 2026-05-20 UTC
Evidence mode: `Static`
Ran evidence: none in this kickoff execution

| Ref ID | Source | Scope | Mapped Subsystems/Invariants | Authority Class | Evidence |
|---|---|---|---|---|---|
| ADR-0011 | `docs/decisions/0011-architecture-first-top-down-science-contracts.md` | Architecture-first strategy, source hierarchy, comparator tiers | SS-06, SS-07, INV-PHYS-001, INV-PARITY-001, INV-PARITY-002, INV-PROV-001 | primary architecture authority | `[DIRECT][Static]` |
| ADR-0004 | `docs/decisions/0004-subprocess-hillslope-orchestration.md` | Subprocess-per-hillslope policy and invocation safety | SS-03, SS-04, INV-ORCH-001, INV-ORCH-002 | primary architecture authority | `[DIRECT][Static]` |
| ADR-0005 | `docs/decisions/0005-parquet-via-wepppyo3-interchange.md` | Parquet interchange ownership | SS-08, INV-IO-001 | primary interface authority | `[DIRECT][Static]` |
| ADR-0006 | `docs/decisions/0006-three-binaries-incl-replay.md` | Binary role decomposition and replay purpose | SS-03, SS-04, SS-07, INV-CONTRACT-004 | primary executable-boundary authority | `[DIRECT][Static]` |
| ADR-0007 | `docs/decisions/0007-openwepp-runner-and-release-governance.md` | Runner ownership, sidecars, explicit engine selection | SS-09, INV-CONTRACT-002, INV-RELEASE-001 | primary governance authority | `[DIRECT][Static]` |
| ADR-0008 | `docs/decisions/0008-routine-lifecycle-and-replacement.md` | Routine lifecycle semantics | SS-05 | proposed (guiding, not accepted) | `[DIRECT][Static]` |
| ADR-0009 | `docs/decisions/0009-network-node-contract-and-extensibility.md` | Typed watershed-node contract and adapter failure rules | SS-04, INV-ORCH-003 | proposed (guiding, not accepted) | `[DIRECT][Static]` |
| ADR-0003 | `docs/decisions/0003-parity-semantic-not-bit.md` | Semantic parity and tolerance-driven comparison | SS-06, SS-07, INV-NUM-001 | primary numerics authority | `[DIRECT][Static]` |
| CONTRACT-RUNNER | `docs/contracts/openwepp-runner-contract.md` | Explicit engine selector, no fallback, failure posture | SS-01, SS-09, INV-CONTRACT-001, INV-CONTRACT-002 | normative contract | `[DIRECT][Static]` |
| CONTRACT-RUN-BRIDGE | `docs/contracts/README.md` (`.run` contract and legacy bridge section) | Initial backward compatibility for legacy stdin `.run` + `.txt` sidecars with explicit validation | SS-01, ST-001, INV-CONTRACT-005 | normative contract guidance | `[DIRECT][Static]` |
| CONTRACT-RELEASE | `docs/contracts/openwepp-binary-release-contract.md` | Binary naming + mandatory sidecars + release lint gate | SS-09, INV-RELEASE-001 | normative contract | `[DIRECT][Static]` |
| CONTRACT-ROUTINE-V1 | `docs/contracts/routine-interface-v1.md` | Kernel routine descriptor and lifecycle interface | SS-05, INV-CONTRACT-003 | draft-normative contract | `[DIRECT][Static]` |
| ARCH-README | `docs/architecture/README.md` | Process/data-flow and kernel-vs-orchestrator boundary | SS-03, SS-04, SS-08 | primary architecture reference | `[DIRECT][Static]` |
| SPEC-README | `docs/specifications/README.md` | Contract authority hierarchy and tolerance requirement | SS-02, SS-06, INV-NUM-001 | primary science-contract governance | `[DIRECT][Static]` |
| NUM-README | `docs/numerics/README.md` | Determinism + floating-point policy | SS-06, INV-NUM-001, INV-NUM-002 | primary numerics governance | `[DIRECT][Static]` |
| LEGACY-HBP | `/home/workdir/wepp-forest/src/wshpas.f90`, `/home/workdir/wepp-forest/src/hbp_mode2_bridge.f90`, `/home/workdir/wepp-forest/src/hbp_legacy_bridge.f90` | Static provenance for HBP boundary behavior | SS-03, SS-04, SS-08 | secondary legacy static evidence | `[DIRECT][Static]` |
| LEGACY-WSH-LAYERING | `/home/workdir/wepp-forest/src/wshdrv.f90`, `/home/workdir/wepp-forest/src/wshrun.f90` | Static provenance for watershed orchestration layering | SS-04 | secondary legacy static evidence | `[DIRECT][Static]` |
| REF-50201000-CORPUS | `references/50201000` (synced from `wepp-forest` on 2026-05-20) | Domain science citations for top-down invariants | GAP-REF-50201000-001 | available corpus; invariant extraction and chapter-level citation mapping pending | `[DIRECT][Static]` |
