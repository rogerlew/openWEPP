# PERFDEEP05 Gate Results

Evidence class: Ran + Static.

## Rust Gates

| Gate | Result |
|---|---|
| `cargo fmt --check` | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS |
| `cargo test -p openwepp-hillslope-orchestrator perfdeep05 -- --nocapture` | PASS, `4` tests |
| `cargo test --workspace` | PASS |
| `cargo deny check` | PASS, `advisories ok, bans ok, licenses ok, sources ok` |

## Release Build

| Gate | Result |
|---|---|
| `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` | PASS |

Final release binary SHA-256 from final endpoint manifests:

```text
6833a30b57ef7a96b409437a656b91037e9db7e0a3a77b24471bcdaf299a07a6
```

## Line-Count Governance

The new dense-transfer branch initially pushed
`execute_ofe_sequence_with_kernel_internal` over the enforced clippy
`too_many_lines` threshold. The branch was extracted to
`apply_next_transfer_input_to_lane`, and the strict clippy gate passed without
lint suppression.

## Markdown Gates

| Gate | Result |
|---|---|
| `markdown-doc lint --path docs/work-packages/20260619-perfdeep05-lane-dense-transfer-authority-sync-removal-001 --format plain` | PASS, `10` files validated, `0` errors, `0` warnings |
| `markdown-doc lint --path docs/work-packages/20260619-perfdeep05-lane-dense-transfer-authority-sync-removal-001 --path docs/work-packages/README.md --path docs/ROADMAP.md --path docs/architecture/array-native-runtime-specification.md --format plain` | PASS, `13` files validated, `0` errors, `0` warnings |
