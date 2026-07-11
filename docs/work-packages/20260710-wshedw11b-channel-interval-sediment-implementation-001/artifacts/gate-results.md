# Gate Results

Status: `EXECUTED-PASS`

Evidence mode: `Ran` on the frozen final tree, 2026-07-10.

| Gate | Result | Evidence |
|---|---|---|
| `cargo fmt --check` | PASS | `logs/cargo-fmt-check.log` |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | `logs/cargo-clippy-workspace.log` |
| `cargo nextest run --workspace --profile full` | PASS | run `f8ad9edd-774f-40da-b5ce-63a674e12890`; 1,677/1,677, four slow, three configured skips, 591.594 s; `logs/nextest-full.log` |
| `cargo deny check` | PASS | advisories, bans, licenses, sources; `logs/cargo-deny-check.log` |
| `cargo nextest run --workspace --profile erosion` | PASS | run `b7bd6908-80ab-4ae4-b77d-7030c2c12a85`; 312/312, three slow, 1,368 profile skips, 152.278 s; `logs/nextest-erosion.log` |
| W11B hourly/ENDDET selector | PASS | 23/23 |
| typed watershed integration | PASS | 18/18 |
| runner hourly consumer | PASS | 2/2 |
| protected P102 five-class CLI | PASS | 1/1 |
| release build/hash/direct CLI | PASS | `release-binary-provenance.md` and release logs |
| comparator/source delta review | PASS with authorized deltas | `comparator-delta-review.md` |
| `git diff --check` | PASS | current tree |
| scoped Markdown lint | PASS | package 32 files plus catalog, roadmap, and predecessor handoff: 35 files total, zero errors/warnings |

No code, science, consumer, conservation, release, security, or documentation
gate is failed, blocked, or unjustifiably not run.
