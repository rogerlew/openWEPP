# Gate results

Status: PASS
Evidence mode: Ran

Final current-source delegated run:

| Gate | Result |
| --- | --- |
| `cargo fmt --check` | PASS, exit 0, 1.97 s |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS, exit 0, 13.56 s |
| `cargo nextest run --workspace --profile full` | PASS, run `8121f433-422d-4964-927d-d23b1a7ec2fb`, 1,747/1,747 passed, 3 skipped, 171 binaries, 593.28 s |
| `cargo deny check` | PASS, exit 0, 7.76 s |
| package/contract/spec `markdown-doc lint` | PASS, 33 files, 0 errors/warnings, 0.02 s |
| `git diff --check` | PASS, exit 0, 0.02 s |

Raw logs and SHA-256:

- `cargo-fmt.log`: `be122f62948406aedf5e7e5badbf425ee61c1b7818e72343ce9202966e259748`
- `cargo-clippy.log`: `94c6033bc8846aecc4e27cd50c7212c5296304392abf7eb749e27b947d7b3c86`
- `cargo-nextest-full.log`: `f73fc4a45a0176c4b2cc22bdd97b56d3c5c49ca2687eda9401fc9b170afdc4aa`
- `cargo-deny.log`: `931ee38fed9a5462d9845b8aad9c8e8aba6c16a0b3f3280550c354de4c6942f5`
- `markdown-doc.log`: `7e9f19b0bc2c25c66d36edec5f5e8e36ccccaacca64a1b6f3745e39caa005c1f`
- `git-diff-check.log`: `cc0631fc6fe9c409ded18ec7a2f856aeaed4f5bdf464bbebdafa55c3d18019e7`
