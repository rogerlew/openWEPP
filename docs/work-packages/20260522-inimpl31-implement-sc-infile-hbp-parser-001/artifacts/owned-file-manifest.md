# Owned File Manifest — INIMPL31

## Code and Build Wiring
- `/home/workdir/openWEPP/crates/openwepp-input-contract/Cargo.toml`
  - Added parser dependency for HBP decode path (`flate2`).
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/mod.rs`
  - Exported `hbp` parser module.
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/hbp.rs`
  - HBP parser implementation and typed strict/compat behavior.
- `/home/workdir/openWEPP/Cargo.toml`
  - Added integration test registration for HBP parser contract.
  - Added test-only dependency used by deterministic schema2 fixture synthesis.
- `/home/workdir/openWEPP/Cargo.lock`
  - Dependency lock update from parser/test wiring.

## Tests
- `/home/workdir/openWEPP/tests/integration/infile_hbp_parser_contract.rs`
  - Added deterministic schema1/schema2 fixture synthesis and typed behavior coverage.

## Specifications and Contracts
- `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/hbp-file.spec.md`
  - Canonical HBP input-surface specification.
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-HBP-001.md`
  - Parser contract with guard map, error taxonomy, and boundary mapping.
- `/home/workdir/openWEPP/docs/specifications/wepp-input-files/input-surface-registry.md`
  - Registered active HBP parser surface.
- `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/README.md`
  - Scope note includes HBP spec.

## Work-Package Governance
- `/home/workdir/openWEPP/docs/work-packages/README.md`
  - Added queued/authorized package entry for INIMPL31.
- `/home/workdir/openWEPP/docs/work-packages/20260522-inimpl31-implement-sc-infile-hbp-parser-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-inimpl31-implement-sc-infile-hbp-parser-001/prompts/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-inimpl31-implement-sc-infile-hbp-parser-001/prompts/active/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-inimpl31-implement-sc-infile-hbp-parser-001/prompts/active/inimpl31_kickoff_agent_prompt.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-inimpl31-implement-sc-infile-hbp-parser-001/prompts/archived/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-inimpl31-implement-sc-infile-hbp-parser-001/artifacts/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-inimpl31-implement-sc-infile-hbp-parser-001/artifacts/worker-handoff.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-inimpl31-implement-sc-infile-hbp-parser-001/artifacts/owned-file-manifest.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-inimpl31-implement-sc-infile-hbp-parser-001/artifacts/inimpl31_disposition.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-inimpl31-implement-sc-infile-hbp-parser-001/artifacts/review_agent_a.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-inimpl31-implement-sc-infile-hbp-parser-001/artifacts/review_agent_b.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-inimpl31-implement-sc-infile-hbp-parser-001/artifacts/verification_agent_a.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-inimpl31-implement-sc-infile-hbp-parser-001/artifacts/verification_agent_b.md`
