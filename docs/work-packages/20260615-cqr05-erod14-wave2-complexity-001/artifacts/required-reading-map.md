# CQR05 Required Reading Map

Evidence: Static.

Read before implementation:

- `AGENTS.md`
- `crates/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/standards/module-test-enhancement-authoring-guide.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `tests/integration/erod14_wave2_multiofe_enrichment_kernel_contract.rs`

Relevant authority notes:

- `SC-SED-001` owns EROD14 Wave-2 sediment/enrichment behavior.
- `SC-SYSTEM-001` records runtime boundary obligations for MOFE and EROD14
  state exchange.
- ADR-0021 identifies `run_erod14_wave2` as a CRAP/cyclomatic-complexity
  backlog target and binds science-tier coverage thresholds.
