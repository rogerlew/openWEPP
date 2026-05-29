Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260529-wshedimpl43-hbp-binary-only-ascii-pass-removal-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-HBP-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/wepp-input-files/specs/hbp-file.spec.md`
- `/workdir/openWEPP/docs/work-packages/20260529-wshedimpl42-wb14-runoff-guard-unpalatable-rind-closure-001/artifacts/wshedimpl42_disposition.md`

Files:
- `docs/work-packages/README.md`
- `docs/work-packages/20260529-wshedimpl43-hbp-binary-only-ascii-pass-removal-001/**`
- `docs/specifications/science-contracts/contracts/SC-INFILE-HBP-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/wepp-input-files/specs/hbp-file.spec.md`
- `crates/openwepp-input-contract/src/parsers/hbp.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-runner/src/release.rs`
- `tests/integration/infile_hbp_parser_contract.rs`
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`

Task: execute package objective end-to-end for declared scope: remove ASCII
pass support fully and enforce binary HBP-only publication/ingestion.

Constraints: contract-first sequencing; canonical SC authority; typed guards;
no silent defaults; no `.pass.dat` fallback/derivation support.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
