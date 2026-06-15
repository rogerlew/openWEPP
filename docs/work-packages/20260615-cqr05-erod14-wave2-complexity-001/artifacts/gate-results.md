# CQR05 Gate Results

Evidence: Ran.

Focused characterization:

| Command | Phase | Exit | Result |
| --- | --- | ---: | --- |
| `cargo test --test erod14_wave2_multiofe_enrichment_kernel_contract` | before | 0 | 14 passed |
| `cargo test --test erod14_contract_authority_closure_contract` | before | 0 | 2 passed |
| `cargo test --test erod14_wave2_multiofe_enrichment_kernel_contract` | after | 0 | 14 passed |
| `cargo test --test erod14_contract_authority_closure_contract` | after | 0 | 2 passed |

Metric commands:

| Command | Exit | Result |
| --- | ---: | --- |
| `cargo llvm-cov --workspace --ignore-run-fail --json --output-path .../coverage_before.json` | 0 | baseline captured |
| `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path .../lcov_before.info` | 0 | baseline LCOV captured |
| `cargo crap --workspace --lcov .../lcov_before.info --min 0 --format json --output .../crap_before.json` | 0 | baseline CRAP captured |
| `cargo llvm-cov --workspace --ignore-run-fail --json --output-path .../coverage_after.json` | 0 | final coverage captured |
| `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path .../lcov_after.info` | 0 | final LCOV captured |
| `cargo crap --workspace --lcov .../lcov_after.info --min 0 --format json --output .../crap_after.json` | 0 | final CRAP captured |

Required closure gates:

| Command | Exit | Result |
| --- | ---: | --- |
| `cargo fmt --check` | 0 | pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 | pass |
| `cargo test --workspace` | 0 | pass |
| `cargo deny check` | 0 | pass: advisories ok, bans ok, licenses ok, sources ok |

Warnings:

- `cargo crap` reported unmatched LCOV source-file warnings for unrelated
  source paths. The target production file was included in both before and
  after CRAP artifacts.
- One existing workspace test is ignored under coverage runs; this was not
  introduced by CQR05.
