# Gate Results

Status: complete

Evidence mode: Ran

Ran:

- `cargo fmt --check`
  - Result: pass.
- `cargo test --test hphys0314_adr0017_snow_rm_reclassification_contract -- --nocapture`
  - Initial result: fail (`4 passed; 1 failed`) because `gate-results.md`
    was still a queued placeholder during closeout conversion.
- `cargo test --test hphys0313_snowpack_settling_carry_recursion_contract -- --nocapture`
  - Result: pass (`6 passed; 0 failed`).
- `cargo test --test adr0017_comparator_distrust_ratification_contract -- --nocapture`
  - Result: pass (`4 passed; 0 failed`).
- `bash tools/release/check_authority_suite_antievasion.sh`
  - Result: pass.
- `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture`
  - Result: pass (`2 passed; 0 failed`).
- `markdown-doc lint --path docs/work-packages/20260606-hphys0314-adr0017-snow-rm-reclassification-route-ledger-001 --path docs/work-packages/README.md --path docs/specifications/science-contracts/index.md --path docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md --path docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - Result: pass (`27 files validated, 0 errors, 0 warnings`).
- `git diff --check`
  - Result: pass.

Pending:

- None.

Final rerun:

- `cargo test --test hphys0314_adr0017_snow_rm_reclassification_contract -- --nocapture`
  - Result: pass (`5 passed; 0 failed`).
