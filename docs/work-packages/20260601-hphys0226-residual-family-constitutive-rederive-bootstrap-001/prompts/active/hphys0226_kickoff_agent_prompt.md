Scope: local repository science-contract/kernel task; flat-file reads/edits
only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0226-residual-family-constitutive-rederive-bootstrap-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0225-wb19-available-pool-authority-closure-001/artifacts/worker-handoff.md`
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
- `docs/work-packages/20260601-hphys0226-residual-family-constitutive-rederive-bootstrap-001/**`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/external-authority/registry.yaml`
- `docs/specifications/external-authority/suites/cas_l4_subhyd_lateral_saturated_thickness_response_001.md`
- `tests/fixtures/constitutive/cas_l4_subhyd_lateral_saturated_thickness_response_001/*`
- `tests/integration/hphys0226_wb19_lateral_saturated_thickness_response_contract.rs`
- `tests/integration/auth06_fixture_provenance_hash_enforcement_contract.rs`
- `Cargo.toml`

Task: execute HPHYS0226 end-to-end by bootstrapping constitutive re-derivation
authority for open coupled residual families through a required Level-4 WB19
behavioral gate (`lateral_saturated_thickness_response`) and linked
contract-derived test evidence.

Constraints: contract-first sequencing; canonical SC authority updates before
test or runtime-seam assertions; typed hard-fail guard posture; no silent
defaults; no heuristic/proxy process-physics substitutions.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
