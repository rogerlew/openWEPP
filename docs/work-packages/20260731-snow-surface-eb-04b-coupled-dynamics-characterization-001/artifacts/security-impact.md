# Security Impact

Evidence: `Static + Ran`

No secrets, network access, authentication, dependency, unsafe Rust, fixture
mutation, or external side effect is authorized. The tool reads local retained
evidence and writes only beneath the package and `target/`.

The sole subprocess is an explicit argument-array `git diff --binary -- crates
tests` identity read. There is no shell interpolation or user-controlled
command construction. Security gate: `PASS`.
