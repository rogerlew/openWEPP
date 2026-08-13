# fresh-retry

- Commands: `cargo clippy --workspace --all-targets -- -D warnings` (PASS), `tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md` (PASS).
- Nextest partition runs: 16/16 final attempts PASS with hash partitions `1/16` through `16/16`.
- All final partition summaries are clean pass outcomes in their partition logs.
- Infra anomalies occurred due overlapping earlier sequential invocations: duplicate runners and interrupted runs for partitions 6-16, with exits 100 and 0 interleaving in command logs.
- Resolved by final isolated reruns; final gate is PASS.

Absolute log artifacts are under:
`/home/workdir/openWEPP/docs/work-packages/20260813-c3-woody-storage-transfer-phenology-authority-001/artifacts/terminal-runs/fresh-retry`
