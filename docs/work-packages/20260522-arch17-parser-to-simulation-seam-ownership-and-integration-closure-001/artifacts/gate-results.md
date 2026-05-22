# Gate Results

Evidence mode: `Ran`
Status: `complete`

## Required Gates

1. `cargo fmt --check`
- result: pass

2. `cargo clippy --workspace --all-targets -- -D warnings`
- result: pass

3. `cargo test --workspace`
- result: pass
- includes ARCH17 additions:
  - `parser_runtime_seam_integration` (2 tests)
  - `workspace_integration_ownership_acceptance` (2 tests)
  - hillslope runtime adapter unit tests (2)
  - watershed runtime adapter unit tests (2)

4. `cargo deny check`
- result: pass (`advisories ok, bans ok, licenses ok, sources ok`)
- note: existing `license-not-encountered` warnings in `deny.toml` allowances remain non-failing and pre-existing to ARCH17 scope.

## Additional Execution Notes
- Initial clippy run failed on numeric cast lints in watershed adapter; corrected with lossless conversions and explicit `ChannelCountOutOfRange` typed error.
- Re-ran formatter and clippy after fix; all gates pass.
