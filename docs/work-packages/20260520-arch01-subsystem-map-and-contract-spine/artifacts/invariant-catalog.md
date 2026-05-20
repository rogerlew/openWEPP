# Invariant Catalog

Status: complete
Date: 2026-05-20 UTC
Evidence mode: `Static`
Ran evidence: none in this kickoff execution

## Comparator Tier Semantics

- `Tier-A`: invariant applies to higher-confidence surfaces (single OFE + daily water-balance), and unresolved violations/deltas are promotion-gating.
- `Tier-B`: invariant applies to lower-confidence hourly/watershed-integrated surfaces, and unresolved deltas route to investigation by default.
- `Tier-A/B`: invariant is cross-tier and applies to both Tier-A and Tier-B surfaces; escalation follows the target surface tier under comparator policy.

## Active Invariants

| Invariant ID | Statement | Severity | Comparator Tier | Authority | Evidence |
|---|---|---|---|---|---|
| INV-PHYS-001 | Melt depth over a timestep cannot exceed available snowpack depth. | hard-fail | Tier-A | ADR-0011 (example invariant) | `[DIRECT][Static]` |
| INV-PHYS-002 | Storage variables remain non-negative unless a contract explicitly allows signed state. | hard-fail | Tier-A/B | physical/common-sense + ADR-0011 contract model | `[DIRECT][Static]` |
| INV-CLOSE-001 | Closure residual is computed and reported for each audited water-balance surface. | hard-fail | Tier-A/B | `docs/numerics/README.md`, ADR-0003, ADR-0011 | `[INFERENCE][Static]` |
| INV-CONTRACT-001 | Dual-engine boundaries must include explicit engine selector (`legacy_wepp` or `openwepp`). | hard-fail | N/A | `docs/contracts/openwepp-runner-contract.md` | `[DIRECT][Static]` |
| INV-CONTRACT-002 | No silent fallback between engine families or pass-family contracts. | hard-fail | N/A | `docs/contracts/openwepp-runner-contract.md`, ADR-0007 | `[DIRECT][Static]` |
| INV-CONTRACT-003 | Every routine descriptor must include `units_manifest_ref` and required lifecycle/contract fields. | hard-fail | N/A | `docs/contracts/routine-interface-v1.md` | `[DIRECT][Static]` |
| INV-CONTRACT-004 | Replay I/O boundary remains distinct from normal simulation input boundary. | hard-fail | N/A | ADR-0006 | `[DIRECT][Static]` |
| INV-CONTRACT-005 | Legacy `.run` + `.txt` sidecar compatibility mode is explicit; missing/ambiguous sidecar prerequisites are hard errors (no silent fallback/defaulting). | hard-fail | N/A | ADR-0011, `docs/contracts/README.md` | `[DIRECT][Static]` |
| INV-ORCH-001 | Watershed routing consumes completed hillslope HBP shards across a subprocess boundary. | hard-fail | N/A | ADR-0004, ADR-0006, `docs/architecture/README.md` | `[DIRECT][Static]` |
| INV-ORCH-002 | Subprocess invocation uses explicit argument arrays (`std::process::Command`), not shell interpolation. | hard-fail | N/A | ADR-0004 | `[DIRECT][Static]` |
| INV-ORCH-003 | Unknown required watershed `node_kind` or missing adapter is a hard configuration error. | hard-fail | N/A | ADR-0009 | `[DIRECT][Static]` |
| INV-IO-001 | Parquet outputs conform to existing wepppy/wepppyo3 interchange schema governance. | hard-fail | N/A | ADR-0005 | `[DIRECT][Static]` |
| INV-RELEASE-001 | Release binaries must match naming regex and have mandatory JSON sidecars. | hard-fail | N/A | `docs/contracts/openwepp-binary-release-contract.md`, ADR-0007 | `[DIRECT][Static]` |
| INV-PARITY-001 | Tier-A comparator deltas (single OFE + daily water-balance surfaces) block promotion until dispositioned. | investigate-then-gate | Tier-A | ADR-0011 | `[DIRECT][Static]` |
| INV-PARITY-002 | Tier-B comparator deltas (hourly/watershed surfaces) open investigations and do not auto-reject alone. | investigate | Tier-B | ADR-0011 | `[DIRECT][Static]` |
| INV-NUM-001 | Semantic parity requires explicit tolerance bounds per state surface contract. | hard-fail | Tier-A/B | ADR-0003, `docs/numerics/README.md`, `docs/specifications/README.md` | `[DIRECT][Static]` |
| INV-NUM-002 | Within-config deterministic runs (single thread + pinned seed) are required on the same target. | hard-fail | Tier-A/B | `docs/numerics/README.md` | `[DIRECT][Static]` |
| INV-ERROR-001 | Production paths must surface typed errors and avoid silent numeric masking defaults. | hard-fail | N/A | root `AGENTS.md` | `[DIRECT][Static]` |
| INV-PROV-001 | Legacy static inspection is secondary authority for provenance and ordering, not sole acceptance authority. | hard-fail | Tier-A/B | ADR-0011 | `[DIRECT][Static]` |

## Cross-Cutting Invariant-to-Surface Binding

The following invariants are intentionally modeled as cross-cutting and are explicitly bound in the state-surface catalog so they are not dropped:

| Invariant ID | Bound state surfaces | Binding mode |
|---|---|---|
| INV-NUM-001 | `ST-015` (+ tiered numeric surfaces `ST-003`, `ST-007`, `ST-008`) | Cross-tier tolerance-governance binding |
| INV-NUM-002 | `ST-016` | Cross-tier determinism-governance binding |
| INV-PROV-001 | `ST-017` | Cross-tier provenance-authority binding |

## Gap Register

| Gap ID | Gap Statement | Impact | Evidence |
|---|---|---|---|
| GAP-REF-50201000-001 | `references/50201000` corpus is synced locally, but chapter-level invariant extraction and explicit rights classification records are still pending. | Limits immediate promotion of chapter-derived invariant families from placeholder IDs to citation-complete IDs. | `[DIRECT][Static]` |
| GAP-INV-DOMAIN-001 | Domain-level invariant families (`INV-SNOW-*`, `INV-WATBAL-*`, `INV-SOIL-*`) are not yet split into per-variable toleranced contracts. | Blocks kernel-complete acceptance gates for those domains. | `[INFERENCE][Static]` |
