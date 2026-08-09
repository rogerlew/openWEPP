# Line-Count Governance

Status: `pass / no Rust touched`

Evidence mode: `Static`

Production Rust is outside scope. The terminal changed-path scan must continue
to show no `.rs`, Cargo, runtime, fixture, or test-source path. Because no Rust
appears in the audit diff, line-count and module-size escalation are not
applicable.
