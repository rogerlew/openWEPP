Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0225-wb19-available-pool-authority-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0224-cam-wb19-soilwater-authority-closure-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/correctness-authority-model.md`
- `/workdir/openWEPP/docs/specifications/external-authority/registry.yaml`
- `/workdir/openWEPP/docs/governance/correctness-reanchoring-keep-condemn-map.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`

Files:
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-hphys0225-wb19-available-pool-authority-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/external-authority/registry.yaml`
- `docs/specifications/external-authority/suites/cas_l4_subhyd_layer_pool_withdrawal_cap_001.md`
- `tests/fixtures/constitutive/cas_l4_subhyd_layer_pool_withdrawal_cap_001/*`
- `tests/integration/hphys0225_wb19_layer_pool_withdrawal_cap_contract.rs`
- `tests/integration/auth06_fixture_provenance_hash_enforcement_contract.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `Cargo.toml`

Task: execute HPHYS0225 end-to-end to close WB19 available-pool authority drift
by removing legacy max-reconciliation expansion and enforcing layer-derived cap
authority through canonical contracts, required Level-4 suite coverage, and
contract-derived runtime checks.

Constraints: contract-first sequencing; canonical SC authority updates before
runtime edits; typed guards and explicit failures; no silent defaults; no
heuristic/proxy process-physics substitutions in production code.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
