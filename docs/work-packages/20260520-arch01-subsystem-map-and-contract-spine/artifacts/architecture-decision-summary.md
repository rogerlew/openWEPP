# Architecture Decision Summary (ARCH-01)

Status: complete
Date: 2026-05-20 UTC
Evidence mode: `Static`
Ran evidence: none in this kickoff execution

## Decision Outcomes

1. Subsystem boundaries SS-01..SS-09 are explicit and internally consistent for contract-first implementation kickoff (`[INFERENCE][Static]`).
2. Orchestrator boundaries are process-separated with HBP shard handoff (`SS-03 -> SS-04`) and no shell interpolation policy (`[DIRECT][Static]`, ADR-0004).
3. Kernel routines are governed as contracted units via routine descriptors/lifecycle interfaces (`[DIRECT][Static]`, routine-interface-v1, ADR-0008 proposed).
4. Acceptance logic is tiered: Tier-A deltas gate promotion; Tier-B deltas drive investigations (`[DIRECT][Static]`, ADR-0011).
5. Legacy source inspection remains secondary provenance evidence and not sole acceptance authority (`[DIRECT][Static]`, ADR-0011).
6. Legacy stdin `.run` plus `.txt` sidecar compatibility is retained initially as an explicit bridge into typed run/config state with hard-error handling for missing/ambiguous prerequisites (`[DIRECT][Static]`, ADR-0011, `docs/contracts/README.md`).

## Promotion Candidate (Follow-On ADR Scope)

Promote ARCH-01 outputs into one follow-on ADR that codifies:

- the SS-01..SS-09 boundary contract,
- state-surface ownership responsibilities,
- invariant severity routing,
- comparator tier triage workflow,
- legacy `.run` + sidecar bridge constraints and deprecation path.

## Implementation Readiness

This kickoff produced code-ready architecture artifacts without requiring kernel implementation (`[DIRECT][Static]`).

## Phase 2 Code-Ready Blueprint (Deferred Coding Path)

This package satisfies `package.md` Phase 2 via the deferred-coding path: explicit file/module targets plus acceptance checks (`[DIRECT][Static]`).

| Target file/module (planned) | Responsibility | Acceptance checks (planned) |
|---|---|---|
| `crates/openwepp-input-contract/src/run_bridge.rs` | Dual-mode `.run` ingestion bridge (schema-first + legacy sidecar compatibility normalization). | Missing/ambiguous sidecar prerequisites return typed errors; no silent defaulting (`INV-CONTRACT-005`). |
| `crates/openwepp-state/src/surfaces.rs` | Canonical typed state-surface structs and units manifests. | State surfaces compile with explicit units metadata and no implicit numeric demotion paths (`INV-CONTRACT-003`, `INV-NUM-001`). |
| `crates/openwepp-kernel-interface/src/routine_descriptor.rs` | Routine descriptor validation and lifecycle/contract checks. | Descriptor validation rejects missing required fields and unresolved units manifests (`INV-CONTRACT-003`). |
| `crates/openwepp-orchestrator-hill/src/lib.rs` | Hillslope orchestration skeleton + HBP handoff boundary wiring. | HBP handoff path is explicit and typed; subprocess argument construction uses explicit arg arrays only (`INV-ORCH-001`, `INV-ORCH-002`). |
| `crates/openwepp-invariants/src/daily_balance.rs` | Tier-A daily water-balance invariant enforcement surface. | Tier-A daily balance closure checks produce hard-fail signals on unresolved closure violations (`INV-CLOSE-001`, `INV-PARITY-001`). |
| `crates/openwepp-replay-comparator/src/tier_policy.rs` | Tiered comparator disposition routing. | Tier-A unresolved deltas block promotion; Tier-B deltas produce investigation records without auto-reject (`INV-PARITY-001`, `INV-PARITY-002`). |
| `tests/integration/run_bridge_contract.rs` | Contract tests for dual-mode `.run` compatibility bridge. | Explicit failing cases for missing sidecars and selector ambiguity; typed error IDs asserted. |
| `tests/integration/tier_a_daily_balance_gate.rs` | Tier-A gate behavior test. | Unresolved Tier-A delta triggers blocking decision metadata. |
| `tests/integration/tier_b_investigation_path.rs` | Tier-B investigation behavior test. | Tier-B delta emits investigation metadata and remains non-blocking absent Tier-A violations. |

Global gate checks for the first implementation slice:

1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. Comparator disposition metadata present for both Tier-A and Tier-B test fixtures.
5. Cross-cutting invariant bindings (`INV-NUM-001`, `INV-NUM-002`, `INV-PROV-001`) asserted in the invariant-to-surface crosswalk.

Recommended follow-on work-package sequence:

1. SS-01 + SS-02 scaffolding (dual-mode run ingestion bridge + typed state skeleton).
2. SS-05 routine interface crate skeleton with descriptor validation.
3. SS-03 hillslope orchestrator skeleton (no physics implementation yet) with HBP output wiring.
4. SS-06 invariant-check surface for one Tier-A daily water-balance slice.
5. SS-07 replay/comparator stub with tiered disposition metadata support.

## Open Risks and Gaps

- `references/50201000` corpus is now present in this workspace (synced 2026-05-20); remaining work is chapter-level invariant extraction and citation normalization.
- ADR-0008 and ADR-0009 remain Proposed; subsystem implementation should treat them as guiding policy until accepted or revised.
- No `Ran` evidence was produced in this kickoff, so gates depending on executable behavior remain pending follow-on implementation packages.
