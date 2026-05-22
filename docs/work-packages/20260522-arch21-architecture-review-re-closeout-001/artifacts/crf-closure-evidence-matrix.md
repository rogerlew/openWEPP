# CRF Closure Evidence Matrix

Status: `complete`
Evidence mode: `Static + Ran`

| finding_id | severity | target_wp | current_status | closure_class | direct_evidence_links | hold_blocker | notes |
|---|---|---|---|---|---|---|---|
| `CRF-001` | high | `ARCH15` | `closed` | `implementation_closed` | `docs/work-packages/20260522-arch15-typed-kernel-state-and-unit-boundary-seam-001/artifacts/arch15_disposition.md`<br>`docs/work-packages/20260522-arch14-claude-architecture-review-disposition-001/artifacts/disposition-register.md` | `no` | Typed kernel seam migration remains mandatory and is preserved. |
| `CRF-002` | high | `ARCH15` | `closed` | `implementation_closed` | `docs/work-packages/20260522-arch15-typed-kernel-state-and-unit-boundary-seam-001/artifacts/arch15_disposition.md`<br>`docs/work-packages/20260522-arch14-claude-architecture-review-disposition-001/artifacts/disposition-register.md` | `no` | Unit-boundary seam wiring remains mandatory and is preserved. |
| `CRF-003` | high | `ARCH16` | `closed` | `implementation_closed` | `docs/work-packages/20260522-arch16-scheduler-hot-path-surface-optimization-001/artifacts/arch16_disposition.md` | `no` | Hot-path clone/allocation closure evidence recorded in ARCH16 disposition. |
| `CRF-004` | medium | `ARCH15` | `open_follow_on` | `contract_alignment_pending` | `docs/work-packages/20260522-arch15-typed-kernel-state-and-unit-boundary-seam-001/artifacts/arch15_disposition.md`<br>`docs/work-packages/20260522-arch14-claude-architecture-review-disposition-001/artifacts/disposition-register.md` | `no` | Trait-level purity contract decision is still queued as an amendment follow-on. |
| `CRF-005` | high | `ARCH17` | `closed` | `implementation_closed_representative` | `docs/work-packages/20260522-arch17-parser-to-simulation-seam-ownership-and-integration-closure-001/artifacts/arch17_disposition.md` | `no` | Representative parser-to-runtime seam ownership and ingestion closure is implemented and tested. |
| `CRF-006` | high | `ARCH18` | `hold_pending_full_gate_green` | `conditional_closure_blocked_by_workspace_gate` | `docs/work-packages/20260522-arch18-hbp-authority-and-convergence-closure-001/artifacts/arch18_disposition.md`<br>`docs/work-packages/20260522-arch18-hbp-authority-and-convergence-closure-001/artifacts/gate-results.md`<br>`docs/work-packages/20260522-arch21-architecture-review-re-closeout-001/artifacts/gate-results.md` | `yes` | HBP authority/convergence artifacts exist, but ARCH21 replay still has `cargo fmt --check` failure. |
| `CRF-007` | medium | `ARCH19` | `hold_open_follow_on` | `governance_contract_authored_not_executable` | `docs/work-packages/20260522-arch19-run-and-parquet-boundary-contracts-001/artifacts/arch19_disposition.md`<br>`docs/work-packages/20260522-arch19-run-and-parquet-boundary-contracts-001/artifacts/run-boundary-contract-authority.md`<br>`docs/work-packages/20260522-arch19-run-and-parquet-boundary-contracts-001/artifacts/parquet-boundary-contract-authority.md` | `no` | `RUN-HOLD-*` and `PRQ-HOLD-*` remain explicitly open in ARCH19. |
| `CRF-008` | medium | `ARCH20` | `closed` | `governance_closed` | `docs/work-packages/20260522-arch20-governance-throughput-and-build-hygiene-controls-001/artifacts/arch20_disposition.md` | `no` | Throughput/WIP/SLA governance controls are authored and dispositioned `GO`. |
| `CRF-009` | low | `ARCH20` | `closed` | `governance_closed` | `docs/work-packages/20260522-arch20-governance-throughput-and-build-hygiene-controls-001/artifacts/arch20_disposition.md` | `no` | Workspace build-discipline controls are authored and dispositioned `GO`. |
| `CRF-010` | medium | `ARCH17` | `closed_with_follow_on` | `representative_scope_closed` | `docs/work-packages/20260522-arch17-parser-to-simulation-seam-ownership-and-integration-closure-001/artifacts/arch17_disposition.md` | `no` | Implemented seams are closed; exhaustive parser-family coverage remains a follow-on scope. |

## ARCH21 Gate Replay Impact

Ran: `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo deny check`.

- `cargo fmt --check`: fail (format drift in `tests/integration/infile_hbp_parser_contract.rs`).
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo test --workspace`: pass.
- `cargo deny check`: pass (warnings only).
