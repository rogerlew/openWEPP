# Gate Evidence

Status: `PASS-LOCAL`

Ran:

| Gate | Result |
| --- | --- |
| `git diff --check` | PASS |
| `cargo fmt --check` | PASS |
| focused Nextest public verifier test | PASS, 1 passed |
| `cargo clippy -p openwepp-gate-planner --all-targets -- -D warnings` | PASS |
| fresh affected adjudicated CRAP | PASS, raw 0, actionable 0 |

The affected CRAP run took about 48 minutes because the `affected` profile ran
184 gate-planner tests, including repeated exact workspace-inventory
reconstruction. This is confirmed efficiency debt: the gate caught the target
before TESTGATE dispatch, but its latency is not yet a fast feedback loop.

Exact-diff pre-heavy admission and terminal TESTGATE qualification remain
pending.
