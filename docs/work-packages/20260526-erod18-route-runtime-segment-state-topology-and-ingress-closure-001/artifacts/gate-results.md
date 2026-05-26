# Gate Results

Status: complete
Evidence mode: ran
Date: 2026-05-26

## Ran
- `cargo fmt --all`
- `cargo test -p openwepp --test erod14_wave2_multiofe_enrichment_kernel_contract`
- `cargo test -p openwepp --test erod14_wave2_multiofe_enrichment_kernel_contract -- --ignored --nocapture`
- `cargo test -p openwepp --test cli03_runner_contract_derived_tests cli03_mofe03 -- --nocapture`

## Result Summary
- Formatting: pass.
- EROD14/EROD18 targeted integration suite: pass (`10 passed; 0 failed; 4 ignored`).
- Ignored EROD17 branch-family suite: expected fail (`0 passed; 4 failed`) pending EROD19.
- CLI03 MOFE03 runner seam tests: pass (`2 passed; 0 failed`).
