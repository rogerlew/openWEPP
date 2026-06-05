# Owned File Manifest

Status: complete

Evidence mode: static

Static:

## openWEPP Files Touched

- `Cargo.toml`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `tests/integration/hphys0298_paired_lineage_partition_contract.rs`
- `docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/package.md`
- `docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/**`

## Intentionally Not Touched

- `crates/openwepp-hillslope-orchestrator/src/**`
- `crates/openwepp-runner/src/**`
- `tools/legacy_comparison_suite/**`
- `docs/work-packages/README.md`

## Diagnostic External Worktree

- Detached baseline diagnostic worktree:
  `/tmp/hphys0298_wepp_forest_obs`
- Source authority:
  `/workdir/wepp-forest_260430_baseline` at
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- Patch artifact:
  `docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/baseline-observe-instrumentation.patch`
- Pinned baseline worktree cleanup status: clean.
- Detached diagnostic worktree cleanup command:
  `git -C /workdir/wepp-forest_260430_baseline worktree remove --force /tmp/hphys0298_wepp_forest_obs`

## Scope Note

No production Rust physics code was modified. The only implementation changes
inside openWEPP are canonical contract text, a contract-derived integration
test, and package-local diagnostic/evidence artifacts.
