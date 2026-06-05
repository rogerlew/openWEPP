# Gate Results

Status: completed

Evidence mode: ran

Ran:

- `.venv/bin/python docs/work-packages/20260605-hphys0301-h39-forcing-melt-term-producer-closure-001/artifacts/hphys0301_h39_forcing_release_lineage.py --run-root /tmp/hphys0300_full_20260605T155527Z --artifact-dir docs/work-packages/20260605-hphys0301-h39-forcing-melt-term-producer-closure-001/artifacts`
  - Result: pass.
  - Route: `h39-rain-release-lineage-reclassified-hold`.
  - Raw-rain delta: `-16.476986 mm`.
  - Released-plus-post-rain delta: `-0.237193 mm`.
- `cargo fmt --check`
  - Initial result: failed due formatting in the new focused test.
- `cargo fmt`
  - Result: pass; formatted the new test.
- `cargo fmt --check`
  - Result: pass.
- `cargo test --test hphys0301_h39_forcing_melt_term_producer_contract`
  - Result: pass; 3 passed, 0 failed.
- `wctl doc-lint --path docs/work-packages/README.md`
  - Result: pass; 1 file validated, 0 errors, 0 warnings.
- `wctl doc-lint --path docs/work-packages/20260605-hphys0301-h39-forcing-melt-term-producer-closure-001`
  - Result: pass; 0 files validated, 0 errors, 0 warnings under current doc-lint include rules.
- `wctl doc-lint --path docs/specifications/science-contracts/index.md`
  - Result: pass; 0 files validated, 0 errors, 0 warnings under current doc-lint include rules.
- `wctl doc-lint --path docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
  - Result: pass; 0 files validated, 0 errors, 0 warnings under current doc-lint include rules.
- `wctl doc-lint --path docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - Result: pass; 0 files validated, 0 errors, 0 warnings under current doc-lint include rules.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Result: pass.
- `cargo deny check`
  - Result: pass; existing warning-only duplicate crate and `license-not-encountered` output remains non-failing.
- `cargo test --workspace`
  - Result: pass.

Static:

- HPHYS0300 full H1..H39 metrics were carried forward into this package because HPHYS0301 made no production code edit.
