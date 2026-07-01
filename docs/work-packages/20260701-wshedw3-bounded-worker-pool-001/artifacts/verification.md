# Verification

Status: `EXECUTED-COMPLETE-WSHED-W3`

Verification evidence.

## Local Gates

Evidence class: `Ran:`

| Command | Result |
| --- | --- |
| `cargo fmt --check` | `PASS` |
| `cargo clippy --workspace --all-targets -- -D warnings` | `PASS` |
| `cargo test -p openwepp-runner --test watershed_cli_behavior_contract wshedw3 -- --nocapture` | `PASS`: `3 passed`, `0 failed`, finished in `41.58s` after timing hook; earlier focused run also passed after worker-pool fixes. |
| `cargo test -p openwepp-runner --test watershed_cli_behavior_contract -- --nocapture` | `PASS`: `23 passed`, `0 failed`, finished in `69.70s` |
| `cargo build -p openwepp-runner --release --bins` | `PASS` |
| `cargo nextest run --workspace --profile full` | `PASS`: `1283` passed, `1` skipped, `2` slow, finished in `663.594s` |
| `cargo deny check` | `PASS`: `advisories ok, bans ok, licenses ok, sources ok` |
| `sha256sum -c input-manifest.sha256` from `tests/fixtures/watershed/carnivorous-adobo` | `PASS`: all `175` fixture files validated after the `radly` clamp |
| `git diff --check` | `PASS` |
| `markdown-doc lint --path docs/work-packages/20260701-wshedw3-bounded-worker-pool-001 --path docs/ROADMAP.md --path docs/work-packages/README.md --path tests/fixtures/watershed/carnivorous-adobo/README.md` | `PASS`: `15 files validated`, `0 errors`, `0 warnings` |

## Review Verification

Evidence class: `Static:` plus `Ran:` focused and final gates after fixes.

- Code reviewer findings were accepted and fixed before the final local gate
  run.
- QA reviewer findings were accepted, fixed where in scope, and dispositioned
  before final closure.
- Final code and QA reviewers found no worker-pool correctness blocker after
  the fixture clamp and scaling evidence; both required stale hold docs and
  final gate evidence to be refreshed before complete disposition.

## Scaling Verification

Evidence class: `Ran:`

`artifacts/scaling/carnivorous-adobo-release-scaling-summary.json` records
`status: PASS` for `18` release runs. Each run used committed
`carnivorous-adobo` inputs, generated schema-versioned TOML wrappers, real
`target/release/openwepp-cli-hill` children, and row-equivalence checks against
`jobs1-rep1`.
