# Gate Results

Status: QUEUED. Evidence mode: Static.

| Gate | Result | Evidence |
|---|---|---|
| Clean starting tree | NOT RUN | Run at execution start. |
| Markdown/doc lint | NOT RUN | Run after scaffold and final updates. |
| Contract/profile/BEI checks | NOT RUN | Required for touched `SC-*` contracts during execution. |
| SC unit compliance | NOT RUN | Required for touched contracts. |
| Focused implicit/friction/source-memory tests | NOT RUN | Required during implementation. |
| Focused Lane-D / `ofe_routing` tests | NOT RUN | Required during implementation. |
| Case-4 full-hybrid oracle ladder | NOT RUN | Required before disposition. |
| H2637 active hybrid timing/profile | NOT RUN | Required with exact release-binary provenance. |
| Solve-cost counter before/after evidence | NOT RUN | Required before ratification claim. |
| Fidelity/timing ratification audit | NOT RUN | Required before promotion claim. |
| Protected-output byte identity | NOT RUN | Required if default/off surfaces are touched. |
| Authority anti-evasion guard | NOT RUN | Required if required-case binding or external-authority posture is touched. |
| `git diff --check` | NOT RUN | Required before disposition. |
| `cargo fmt --check` | NOT RUN | Required before disposition if code touched. |
| `cargo clippy --workspace --all-targets -- -D warnings` | NOT RUN | Required before disposition if code touched. |
| `cargo nextest run --workspace --profile full` | NOT RUN | Required before disposition if code touched. |
| `cargo deny check` | NOT RUN | Required before disposition. |
| `.rs` line-count governance | NOT RUN | Required if Rust files are touched. |
