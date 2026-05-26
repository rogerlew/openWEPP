# Gate Results

Status: complete
Evidence mode: ran
Date: 2026-05-26

## Ran
- `cargo test -p openwepp --test erod14_wave2_multiofe_enrichment_kernel_contract --no-run`
- `cargo test -p openwepp --test erod14_wave2_multiofe_enrichment_kernel_contract`
- `cargo test -p openwepp --test erod14_wave2_multiofe_enrichment_kernel_contract -- --ignored --nocapture`

## Result Summary
- Build/no-run: pass.
- Default run: pass (`6 passed; 0 failed; 5 ignored`).
- Ignored run: expected fail (`0 passed; 5 failed`) due missing route-branch
  publication symbols.
