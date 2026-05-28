# CLIM17 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260528-clim17-breakpoint-climate-baseline-parity-burndown-001/package.md
- /workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md
- /workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md
- /workdir/openWEPP/docs/specifications/science-contracts/index.md
- /workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md
- /workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md
- /workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md
- /workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md
- /workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md
- /workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md
- /workdir/openWEPP/docs/work-packages/20260522-clim01-wepp-forest-climate-model-discovery-and-specification-001/artifacts/climate-implementation-wp-queue.md
- /workdir/openWEPP/docs/work-packages/20260522-clim04-breakpoint-runtime-port-and-policy-reconciliation-001/package.md
- /workdir/openWEPP/docs/work-packages/20260522-clim04-breakpoint-runtime-port-and-policy-reconciliation-001/artifacts/clim04_disposition.md
- /workdir/openWEPP/docs/work-packages/20260523-clim07-climate-comparator-and-closure-evidence-001/artifacts/clim07_disposition.md
- /workdir/openWEPP/docs/work-packages/20260523-clim08-climate-governance-disposition-closeout-001/artifacts/clim08_disposition.md
- /workdir/wepp-forest_260430_baseline/src/brkpt.for
- /workdir/wepp-forest_260430_baseline/src/stmget.for
- /workdir/wepp-forest_260430_baseline/src/idat.for

Files:
- docs/work-packages/20260528-clim17-breakpoint-climate-baseline-parity-burndown-001/**
- docs/work-packages/README.md
- docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md
- docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md
- docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md
- docs/specifications/science-contracts/contracts/SC-WATBAL-001.md
- docs/specifications/science-contracts/index.md
- crates/openwepp-input-contract/src/parsers/climate.rs
- crates/openwepp-climate-runtime-adapter/src/lib.rs
- crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs
- crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs
- tests/integration/infile_climate_parser_contract.rs
- tests/integration/parser_runtime_seam_integration.rs
- tests/integration/clim07_climate_comparator_and_closure_contract.rs
- tests/fixtures/infile/climate/**

Task: execute CLIM17 end-to-end to identify breakpoint-climate parity gaps
against `/workdir/wepp-forest_260430_baseline`, implement contract-first
closure, and produce disposition-grade evidence anchored to
`/wc1/runs/un/unpalatable-rind`.

Constraints: contract-first sequencing; canonical SC authority; baseline
provenance (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`); typed guards; no
silent defaults.

Mandatory sequencing constraints:
- Do not modify production kernel/runtime/parser code until:
  1. contract updates are implemented,
  2. contract-derived tests are implemented, and
  3. pre-implementation contract-gate evidence is recorded.
- For this legacy migration package, canonical `SC-*` files are authority and
  must be amended first when closure requires changes.
- Do not add fallback wrappers that mask missing required dependencies.
- Do not add silent clamps/defaults for breakpoint domain violations.
- Maintain dual review/dual verification artifacts as hard gates.
- Correctness over completion: unresolved parity-critical gaps remain `HOLD`.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update all required artifacts through disposition, including
contract/test implementation evidence, gate results, dual review/verification,
and worker handoff.
