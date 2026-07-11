# Gate results

Status: PASS
Evidence mode: Ran

| Gate | Result |
| --- | --- |
| `cargo fmt --check` | PASS, 2.01 s |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS, 13.44 s |
| `cargo nextest run --workspace --profile full` | PASS, run `3d16d2bb-ad39-43ef-b37e-edb64778b023`, 1,765/1,765 passed, 3 skipped, 171 binaries, 591.67 s |
| `cargo deny check` | PASS, 6.94 s |
| scoped `markdown-doc lint` | PASS, 33 files, 0 errors/warnings, 0.02 s |
| `git diff --check` | PASS, 0.03 s |

Raw logs and SHA-256:

- `cargo-fmt.log`: `bb0d16c7a2bf23597420648cf208b5670787d1bc0e0f2278149e78118078964e`
- `cargo-clippy.log`: `ddf7620148c1b89b811a13510686ed5ab5e7372d86691dcdeccb4e18eb4a7aaa`
- `cargo-nextest-full.log`: `b61d320a7c89da8ae4fc0fb5c4d932b5df0a055613a5fe640b565cf410050ef2`
- `cargo-deny.log`: `e32d8d2a4a77942b5d58dd1384746881225c28b00f53e0fe317d95280ed084af`
- `markdown-doc.log`: `7e9f19b0bc2c25c66d36edec5f5e8e36ccccaacca64a1b6f3745e39caa005c1f`
- `git-diff-check.log`: `01bee08d98cd0cf617c4c78eab914439e44338fde4e0363da1e8168531b387d5`
