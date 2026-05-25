Scope: local repository parser-compatibility engineering task; flat-file
reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe07-carved-letter-slope-soil-parser-compatibility-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe06-single-hillslope-semantic-parity-carved-letter-001/artifacts/mofe06_disposition.md`

Files:
- `docs/work-packages/20260525-mofe07-carved-letter-slope-soil-parser-compatibility-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `tests/fixtures/infile/slope/**`
- `tests/fixtures/infile/soil/**`
- `tests/integration/infile_slope_parser_contract.rs`
- `tests/integration/infile_soil_parser_contract.rs`
- `crates/openwepp-input-contract/src/parsers/slope.rs`
- `crates/openwepp-input-contract/src/parsers/soil.rs`

Task: execute MOFE07 objective end-to-end for carved-letter slope/soil parser
compatibility closure.

Constraints: contract-first sequencing; canonical SC authority; typed guards; no
silent defaults.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
