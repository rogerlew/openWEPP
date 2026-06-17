# PERFIDX04 Verification A

Ran:
- Final anchor cases: OFE1-OFE5, H2637 no-UI, H2637 with UI.
- Byte-equal outputs: HBP, loss JSON, WAT Parquet, PLOT Parquet for every case.
- Logical pass Parquet rows: zero differences in both directions for every case.

Ran:
- `cargo test --workspace`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.

Conclusion:
- Behavior-preserving verification passed.
