# CRAP After

Ran: `cargo crap --path crates/openwepp-gate-planner --lcov /tmp/cqr-b03s1-main.lcov --format json --output /tmp/cqr-b03s1-main-crap.json` produced report SHA-256 `afd036e14c757380daa81fbac074c1fc1fa1deb9679ea661d4f96caf71751081`.

| Function | CC | Coverage | CRAP |
| --- | ---: | ---: | ---: |
| `validate_package_chain_command` | 4 | 100% | 4 |
| `package_chain_command_inputs` | 4 | 100% | 4 |
| `plan_request` | 4 | 100% | 4 |
| `planning_context` | 4 | 100% | 4 |
| `package_authority_fields` | 3 | 100% | 3 |
| `package_authority` | 4 | 100% | 4 |
| `read_package_authority` | 2 | 100% | 2 |
| `reconstruct_package_authority` | 3 | 100% | 3 |

Ran: every target and extracted helper is at most 30; the focused report emitted no missing-LCOV warning for `main.rs`.
