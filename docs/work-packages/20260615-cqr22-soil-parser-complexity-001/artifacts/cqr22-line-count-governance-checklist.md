# CQR22 Line-Count Governance Checklist

Status: complete.

Ran: before line counts:

```text
1359 crates/openwepp-input-contract/src/parsers/soil.rs
609 docs/work-packages/README.md
697 docs/work-packages/cqr-burndown-execplan.md
```

Ran: after line counts before package commit:

```text
1485 crates/openwepp-input-contract/src/parsers/soil.rs
609 docs/work-packages/README.md
697 docs/work-packages/cqr-burndown-execplan.md
```

Static: touched Rust target file remains below the `3000` line governance
ceiling.

Ran: suppression census:

```text
crates/openwepp-input-contract/src/parsers/soil.rs:7:    clippy::too_many_lines
```

Static: the `too_many_lines` entry is the pre-existing crate-level allow list,
not a new target-specific suppression.
