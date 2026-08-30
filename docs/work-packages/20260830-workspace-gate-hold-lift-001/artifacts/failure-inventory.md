# Failure inventory

Status: `INTAKE`

Evidence mode: `Static + retained Ran`

Retained current full-profile evidence: 3,628 attempted, 3,503 pass, 96 fail,
29 timeout; log SHA-256
`dbdd682aa9c654f08955f65d7b74addfad999691be21c678ecd6da977f0b35ee`.

Retained current Clippy evidence: exit 101 after two root diagnostics; log
SHA-256
`aac68d695f1d8f2e06f687c01aa199cc25d48f8d708a958763266e4323d11637`.

| ID | Owner/path | Classification | Prospective correction | Focused evidence |
|---|---|---|---|---|
| WGHL-CLIPPY-001A | `crates/openwepp-coupled-time/src/event.rs` | behavior-preserving iterator spelling | replace boolean `filter_map` with equivalent `filter` + `map` | affected crate Clippy/tests |
| WGHL-CLIPPY-001B | `crates/openwepp-biogeochemistry/src/lib.rs` | behavior-preserving local name | rename `used` to a semantically distinct local | affected crate Clippy/tests |

Full-profile family classification is pending delegated baseline/current
inventory comparison. No additional implementation path is authorized yet.
