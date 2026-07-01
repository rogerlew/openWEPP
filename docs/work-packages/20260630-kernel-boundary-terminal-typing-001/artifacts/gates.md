# Gates

Evidence class: Ran.

| Gate | Status | Evidence |
| --- | --- | --- |
| Rust format | PASS | `cargo fmt --check` exited `0`. |
| Rust clippy | PASS | `cargo clippy --workspace --all-targets -- -D warnings` exited `0`. |
| Full tests | PASS | `cargo nextest run --workspace --profile full --no-fail-fast`: `1221` passed, `1` skipped, `3` slow; elapsed `703.514s`. |
| Dependency policy | PASS | `cargo deny check`: advisories, bans, licenses, and sources all `ok`. |
| Authority anti-evasion | PASS | `bash tools/release/check_authority_suite_antievasion.sh`: `PASS`. |
| Required-suite obligation guard | PASS | `cargo nextest run --test auth11_required_suite_obligation_guards_contract`: `2` passed. |
| No-compatibility carrier scan | PASS | Forbidden executable carrier/runtime names scan under `crates/openwepp-hillslope-orchestrator/src`, `crates/openwepp-runner/src`, and `tests` returns only source-guard test string literals. |
| H2637 endpoint | PASS | Release `openwepp-cli-hill` exited `0`: `1:10.69` wall, `79284 KiB` max RSS, manifest `selected=direct-production-executor`, `compatibility_edge_invocations=0`; HBP/loss/plot/WAT/PASS byte-identical to `/tmp/typed-direct-carrier-identity/base/output`. |
| Docs lint | PASS | `markdown-doc lint --path docs/ROADMAP.md --path docs/architecture/array-native-runtime-specification.md --path docs/work-packages/README.md --path docs/work-packages/20260630-kernel-boundary-terminal-typing-001 --format json`: `9` files scanned, `0` errors, `0` warnings. |
| Docs validate | PASS | Same scoped paths with `markdown-doc validate --format json`: `9` files scanned, `0` errors. |
| Diff whitespace | PASS | `git diff --check` exited `0`. |

The full nextest profile includes the direct-production source guards,
`compatibility_edge_invocations=0` guard, multi-OFE/Wave-2 fixture execution,
and direct-output schema/value tests. The explicit failed subset from the first
full run was rerun after fixes and passed: `hphys0296` snow/RM trace, typed seed
authority guard, `hphys0298` unit guard, Paradigm-2 meltwater-temperature
publication, and `owcmp` env manifest.
