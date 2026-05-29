# gate-results

Status: complete  
Evidence mode: Ran

Executed:
- `cargo fmt --all`
- `cargo clippy -p openwepp-runner --all-targets -- -D warnings`
- `cargo test -p openwepp-runner`
- `markdown-doc lint --path docs/contracts/openwepp-runner-contract.md --format plain`
- `markdown-doc lint --path docs/contracts/openwepp-binary-release-contract.md --format plain`
- `markdown-doc lint --path docs/governance/openwepp-release-procedure-draft.md --format plain`
- `markdown-doc lint --path docs/work-packages/README.md --format plain`
- `markdown-doc lint --path docs/work-packages/20260529-relproc02-runner-sidecar-emission-cli-001 --format plain`

Result:
- all listed gates passed.
