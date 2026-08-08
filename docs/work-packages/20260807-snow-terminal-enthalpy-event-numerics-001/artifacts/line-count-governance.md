# Line-Count Governance

Status: measured / warning only

Evidence mode: Ran

| Rust file | Before | Current | Disposition |
|---|---:|---:|---|
| `stage3_solver.rs` | 2,868 | 2,996 | WARN; below the 3,000-line block |
| `evaluation.rs` | 1,389 | 1,574 | acceptable |
| `terminal_event.rs` | absent | 603 | new bounded numerical module |
| `00c_day_input_builder_impl.rs` | 2,849 | 2,983 | WARN; below the 3,000-line block |
| `00j_snow_terminal_event_trace.rs` | absent | 953 | new bounded rejecting-consumer module |

The terminal solver and consumer were split into new modules specifically to
avoid placing their full implementations in files already under WARN.
