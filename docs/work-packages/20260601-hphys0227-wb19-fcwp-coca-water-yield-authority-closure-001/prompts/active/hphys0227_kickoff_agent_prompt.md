Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0227-wb19-fcwp-coca-water-yield-authority-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0226-residual-family-constitutive-rederive-bootstrap-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/correctness-authority-model.md`
- `/workdir/openWEPP/docs/specifications/external-authority/registry.yaml`
- `/workdir/openWEPP/docs/governance/correctness-reanchoring-keep-condemn-map.md`

Files:
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-hphys0227-wb19-fcwp-coca-water-yield-authority-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/external-authority/registry.yaml`
- `docs/specifications/external-authority/suites/cas_l4_subhyd_watyld_fcwp_consistency_001.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `tests/fixtures/constitutive/cas_l4_subhyd_watyld_fcwp_consistency_001/*`
- `tests/integration/hphys0227_wb19_fcwp_coca_watyld_authority_contract.rs`
- `tests/integration/auth06_fixture_provenance_hash_enforcement_contract.rs`
- `tests/integration/auth08_wb19_solwpv_fcdep_branch_constitutive_contract.rs`
- `tests/integration/hphys0219_wb19_coca_threshold_contract.rs`
- `tests/integration/hphys0221_wb19_water_yield_fcdep_coupling_contract.rs`
- `tests/integration/hphys0224_wb19_withdrawal_soilwater_cap_contract.rs`
- `tests/integration/hphys0225_wb19_layer_pool_withdrawal_cap_contract.rs`
- `tests/integration/hphys0226_wb19_lateral_saturated_thickness_response_contract.rs`
- `tests/integration/wb19_lateral_drainage_physics_kernel_contract.rs`
- `Cargo.toml`

Task: execute HPHYS0227 end-to-end to close WB19 FC/WP + COCA water-yield
authority by enforcing `avfca` theta-lineage (`thetfc_####`) and FC-store
consistency (`wb18_perc_fc_#### = (thetfc_####-thetdr_####)*dg_####`) with
required Level-4 constitutive gating.

Constraints: contract-first sequencing; canonical SC authority updates before
runtime implementation edits; typed hard-fail guard posture; no silent
defaults; no heuristic/proxy process-physics substitutions.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
