# Gate Results

Ran:
- `python -m tools.hillslope_mofe_daily_closure_audit /wc1/runs/ca/carved-letter/wepp/output/interchange --wepp-id 324 --output-dir /tmp/openwepp_mofe324_semantic_parity/audit_h324` -> pass
- `openwepp-cli-hill --run-dir /wc1/runs/ca/carved-letter/wepp/runs --run-file p324.run ...` -> fail (`CLIHILL-E-010` runfile TOML requirement)
- `openwepp-cli-hill` with generated TOML + original carved-letter inputs -> fail (`CLIHILL-E-010` slope parser)
- `openwepp-cli-hill` with exploratory normalized temp slope -> fail (`CLIHILL-E-010` soil parser `SOL-E-006`)

Static:
- Workspace-wide Rust gates (`cargo fmt/clippy/test/deny`) were not re-run in
  this package because no source code changed.
