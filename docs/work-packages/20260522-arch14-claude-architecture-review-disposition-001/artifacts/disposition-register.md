# ARCH14 Disposition Register

Static: disposition decisions based on external review normalization plus repository source inspection.
Ran: none for closure-by-implementation in ARCH14.
Status: finalized for governance; implementation closure pending follow-on packages.

Decision states:
- `accept`: finding is valid; remediation required.
- `amend`: core concern accepted with corrected scope wording.
- `reject`: finding not accepted.
- `defer`: valid but intentionally sequenced later with explicit risk acceptance.

| finding_id | severity | decision | rationale_summary | required_evidence_for_close | closure_owner | target_follow_on_wp | status |
|---|---|---|---|---|---|---|---|
| `CRF-001` | high | `accept` | Kernel seam currently uses stringly state/writeback surfaces; must migrate to typed kernel state surfaces with canonical symbol alias mapping continuity. | ADR/contract update replacing string-keyed seam maps; compile-time typed seam APIs; integration tests proving deterministic writeback across hillslope/watershed seam. | `ARCH/KERNEL owners` | `ARCH15` | `HOLD` |
| `CRF-002` | high | `accept` | Unit-boundary crate exists but is not wired at kernel seam; ARCH direction requires unit-safe boundary wiring at this seam. | Kernel/orchestrator seam types use `openwepp-unit-boundary` value objects for unit-critical interfaces; failing compile-time tests for mixed-unit misuse; seam contract update. | `ARCH/KERNEL owners` | `ARCH15` | `HOLD` |
| `CRF-003` | high | `accept` | Clone-heavy request surfaces in hot loops are a structural performance risk and can mask future scalability regressions. | Hot-path redesign decision and benchmarks demonstrating reduced clone/allocation pressure while preserving deterministic ordering and closure semantics. | `ORCHESTRATOR owners` | `ARCH16` | `HOLD` |
| `CRF-004` | medium | `amend` | Purity concern is valid; disposition is to reconcile trait mutability semantics with architecture docs and ratify one contract stance. | Explicit purity ADR/contract note defining whether kernels are `&self`/stateless or stateful with constrained mutable internals; docs/trait alignment proof. | `ARCH/KERNEL owners` | `ARCH15` | `OPEN` |
| `CRF-005` | high | `accept` | Parser contracts and orchestrators need explicit seam ownership and integration closure package instead of implicit coupling assumptions. | Parse-to-simulation seam contract, ownership table, and integration tests connecting parser outputs to runtime/orchestrator consumers. | `INPUT + ORCHESTRATOR owners` | `ARCH17` | `HOLD` |
| `CRF-006` | high | `amend` | Two HBP paths are not inherently wrong (parser vs compatibility adapter), but authority/convergence and provenance policy must be explicit and testable. | HBP authority matrix (parser vs adapter responsibilities), divergence tests, and ADR-0012-compliant provenance pinning records with exact `/workdir/wepp-forest` SHA. | `HBP/LEGACY-BRIDGE owners` | `ARCH18` | `HOLD` |
| `CRF-007` | medium | `accept` | Top-level `.run` and parquet boundaries need explicit contract sequencing to preserve architecture-first completeness. | Dedicated `.run` and parquet contract packages with acceptance criteria and downstream ownership mapping. | `CONTRACT authors` | `ARCH19` | `OPEN` |
| `CRF-008` | medium | `defer` | Governance/throughput balancing is valid but should follow resolution of high-severity seam/integration blockers. | Measured package-throughput rubric, WIP limits, and closure SLA policy with retrospective evidence. | `GOVERNANCE owners` | `ARCH20` | `OPEN` |
| `CRF-009` | low | `amend` | Nested `target/` directories are real hygiene drift, but impact is low and mainly operational. | Documented build-output discipline policy and optional cleanup script/process guidance; no architecture gate dependency. | `WORKSPACE maintainers` | `ARCH20` | `OPEN` |
| `CRF-010` | medium | `amend` | Visibility concern stands, but "re-export orphan crates" wording is not directly evidenced; remediation focuses on explicit integration ownership criteria. | Integration ownership matrix naming required consumer/producer contracts and no-orphan acceptance checks for root aggregation workflows. | `ARCH + ORCHESTRATOR owners` | `ARCH17` | `OPEN` |

## Mandatory Direction Statement

openWEPP is moving to typed kernel state surfaces and unit-boundary wiring at the kernel seam. `CRF-001` and `CRF-002` are locked as required remediation tracks and are not dispositioned as `reject`.

## HOLD Rule

Unresolved high-severity findings (`CRF-001`, `CRF-002`, `CRF-003`, `CRF-005`, `CRF-006`) remain `HOLD` until their closure evidence is produced by follow-on packages.
