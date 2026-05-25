Scope: local repository science-contract/kernel migration task; flat-file reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260525-mofe08-cligen-5323-compat-and-h324-parity-rerun-001/package.md
- /workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md
- /workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md
- /workdir/openWEPP/docs/specifications/science-contracts/index.md
- /workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md
- /workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md
- /workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md
- /workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md
- /workdir/openWEPP/docs/work-packages/20260525-mofe06-single-hillslope-semantic-parity-carved-letter-001/artifacts/mofe06_disposition.md
- /workdir/openWEPP/docs/work-packages/20260525-mofe07-carved-letter-slope-soil-parser-compatibility-001/artifacts/mofe07_disposition.md
- /workdir/jimf-cligen532/README.md
Files:
- docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md
- docs/specifications/wepp-input-files/specs/climate-file.spec.md
- crates/openwepp-input-contract/src/parsers/climate.rs
- tests/integration/infile_climate_parser_contract.rs
- tests/fixtures/infile/climate/*
- docs/work-packages/20260525-mofe08-cligen-5323-compat-and-h324-parity-rerun-001/**
- /workdir/jimf-cligen532/README.md
Task: execute package objective end-to-end for declared scope.
Constraints: contract-first sequencing; canonical SC authority; baseline provenance; typed guards; no silent defaults.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Outputs: update package artifacts/disposition for all completed phases.
