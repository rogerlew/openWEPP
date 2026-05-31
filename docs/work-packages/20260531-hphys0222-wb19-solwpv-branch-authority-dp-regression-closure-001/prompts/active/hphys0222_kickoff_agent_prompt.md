Scope: local repository science-contract/kernel migration task; flat-file reads
and edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in `package.md` sequentially through
disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260531-hphys0222-wb19-solwpv-branch-authority-dp-regression-closure-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/external-authority/README.md`
- `/workdir/openWEPP/docs/specifications/external-authority/suite-schema.md`
- `/workdir/openWEPP/docs/specifications/external-authority/registry.yaml`
- `/workdir/openWEPP/docs/work-packages/20260531-hphys0221-wb19-water-yield-fcdep-coupling-implementation-001/artifacts/worker-handoff.md`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for`

Files:
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/external-authority/registry.yaml`
- `docs/specifications/external-authority/suites/cas_l4_subhyd_solwpv_fcdep_branch_001.md`
- `tests/integration/auth06_fixture_provenance_hash_enforcement_contract.rs`
- `tests/integration/auth07_fc_authority_cohort_contract.rs`
- `tests/integration/auth08_wb19_solwpv_fcdep_branch_constitutive_contract.rs`
- `tests/integration/hphys0221_wb19_water_yield_fcdep_coupling_contract.rs`
- `tests/fixtures/constitutive/cas_l4_subhyd_solwpv_fcdep_branch_001/*`
- `Cargo.toml`
- `docs/work-packages/README.md`
- `docs/work-packages/20260531-hphys0222-wb19-solwpv-branch-authority-dp-regression-closure-001/**`

Task: execute HPHYS0222 end-to-end:
1. correct WB19 branch authority so `fcdep/unsdep` mutation applies only when
   `solwpv < 2006`,
2. capture the branch law in canonical contracts,
3. add external-authority constitutive governance for this law with active
   required/hard-fail suite coverage,
4. run workspace validation gates and close package artifacts.

Constraints:
- Contract-first sequencing:
  1) contracts,
  2) contract-derived tests (+ external-authority suite),
  3) pre-implementation gate evidence,
  4) production edits.
- Canonical SC authority and baseline provenance are mandatory.
- No heuristic or surrogate process-physics substitutions.
- Typed guards only; no silent defaults/clamping.
- Preserve truthful `Static:` vs `Ran:` evidence labels in artifacts.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs:
- updated contracts/tests/production code,
- external-authority suite + fixture lock/provenance + registry integration,
- gate evidence (`fmt`, `clippy`, `test`, `deny`) and final disposition.
