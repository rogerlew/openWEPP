# Gate Results

Status: scaffolded
Evidence mode: Static/Ran

Static:
- package scope and gate set captured in `package.md`
- required commands and outputs to be filled during execution

Ran:
- TBD

Required gates:
- cargo fmt --check
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test -p openwepp-hillslope-orchestrator --tests
- cargo test --workspace
- cargo deny check

