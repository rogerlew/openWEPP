# Gate Results

Status: complete
Evidence mode: ran
Date: 2026-05-26

- `cargo fmt --all`: pass
- `cargo clippy -p openwepp-hillslope-orchestrator -p openwepp-runner --all-targets -- -D warnings`: pass
- `cargo test -p openwepp --test erod14_wave2_multiofe_enrichment_kernel_contract`: pass (`14 passed; 0 failed`)
- `cargo test -p openwepp --test cli03_runner_contract_derived_tests cli03_mofe03 -- --nocapture`: pass (`2 passed; 0 failed`)
