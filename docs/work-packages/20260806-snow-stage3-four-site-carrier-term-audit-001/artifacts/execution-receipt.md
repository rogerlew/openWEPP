# Execution Receipt

Status: `PASS execution and retained verification / FAIL carrier screen`

Evidence mode: `Ran`.

- Exact execution HEAD: `3ee1bac3ee849fbe00b517d1d227140f87fedc2a`.
- Command: `.venv/bin/python .../run_carrier_term_audit.py --expected-head
  3ee1bac3ee849fbe00b517d1d227140f87fedc2a`.
- Release build: `cargo build --release -p openwepp-runner --bin
  openwepp-cli-hill`, exit `0`.
- Retained binary SHA-256:
  `4ffe2f9cb24e9e4b14ec13bac3387754cd27c656d5b86197ad59ebd7c7fa6021`.
- Retained protocol SHA-256:
  `83d472c565f432c5e379825ad17a28fb1ea652389b8bf4af9cc7296bc3f6a992`.
- Four control plus four paired runs: PASS in `147 s`.
- Exact runfile climate and PASS/loss/WAT consumers: PASS for all eight lanes.
- WAT and HBP control/paired byte identity: PASS at all four sites.
- Retained verification: PASS by independent reparse/reconstruction.
- Retained manifest: `108` files, SHA-256
  `c77af18a3ae4e718f80b0a67e3af8a65830c65634b08ba5ac2d3c8db19377b0f`.
- Raw result JSON SHA-256:
  `d027fe11b156a839c27ff699f80c001a5391f56e35634b2074f5f0813581b6e9`.
- Raw execution receipt SHA-256:
  `050baca2aba5a1c2feb4d5a2b0b8de71ff1d168081cc72c6e21dff7eec2a5d78`.
- Site table SHA-256:
  `ec9fdfc3561d0aa42e9555debf6fe7c550355f3a4ab60070a3dae3f06fd218e7`.
- Water-year table SHA-256:
  `a61a87373408ae6e8749fe060943ceddc165a3898eade4608d3f1dac652893b7`.

Raw custody remains under ignored
`target/snow_stage3_four_site_carrier_term_audit_v2/`.
