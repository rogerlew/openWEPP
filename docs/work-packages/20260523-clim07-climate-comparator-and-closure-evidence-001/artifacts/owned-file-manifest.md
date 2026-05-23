# Owned File Manifest

Status: `completed`
Evidence mode: `Static`

## CLIM07 Write Set
- `Cargo.toml`
- `tests/integration/clim07_climate_comparator_and_closure_contract.rs`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md`
- `docs/specifications/science-contracts/index.md`
- `docs/work-packages/20260523-clim07-climate-comparator-and-closure-evidence-001/package.md`
- `docs/work-packages/20260523-clim07-climate-comparator-and-closure-evidence-001/artifacts/*.md`

## Explicitly Not Owned by This Package
- Pre-existing local changes outside CLIM07 write set (for example ARCH22/EROD11
  work-in-progress files and unrelated modified contracts) were not reverted or
  re-authored by CLIM07.
- Unexpected large local diff observed in
  `crates/openwepp-watershed-orchestrator/src/lib.rs` was intentionally left
  untouched per explicit user instruction to continue.
