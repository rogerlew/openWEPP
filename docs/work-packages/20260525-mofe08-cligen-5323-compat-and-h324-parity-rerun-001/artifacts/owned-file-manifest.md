# owned-file-manifest

Status: complete
Evidence mode: mixed (Static + Ran)

Static:
- Manifest lists MOFE08-authored/updated files in openWEPP plus required
  cross-repo guidance update file.

Ran:
- Reconciled against staged worktree content.

openWEPP package/index:
- `docs/work-packages/README.md`
- `docs/work-packages/20260525-mofe08-cligen-5323-compat-and-h324-parity-rerun-001/package.md`
- `docs/work-packages/20260525-mofe08-cligen-5323-compat-and-h324-parity-rerun-001/prompts/README.md`
- `docs/work-packages/20260525-mofe08-cligen-5323-compat-and-h324-parity-rerun-001/prompts/active/README.md`
- `docs/work-packages/20260525-mofe08-cligen-5323-compat-and-h324-parity-rerun-001/prompts/active/mofe08_kickoff_agent_prompt.md`
- `docs/work-packages/20260525-mofe08-cligen-5323-compat-and-h324-parity-rerun-001/prompts/archived/README.md`
- `docs/work-packages/20260525-mofe08-cligen-5323-compat-and-h324-parity-rerun-001/artifacts/*`

openWEPP contract/spec/code/test:
- `docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md`
- `docs/specifications/wepp-input-files/specs/climate-file.spec.md`
- `crates/openwepp-input-contract/src/parsers/climate.rs`
- `tests/integration/infile_climate_parser_contract.rs`
- `tests/fixtures/infile/climate/datver_5_323.cli`

external guidance repo:
- `/workdir/jimf-cligen532/README.md`
