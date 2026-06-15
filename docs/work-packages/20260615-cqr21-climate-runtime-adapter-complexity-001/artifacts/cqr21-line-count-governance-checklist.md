# CQR21 Line-Count Governance Checklist

Status: complete.

Ran: before line counts captured before production edits:

```text
crates/openwepp-climate-runtime-adapter/src/lib.rs  1039
docs/work-packages/README.md                         605
docs/work-packages/cqr-burndown-execplan.md          691
```

Ran: after line counts:

```text
crates/openwepp-climate-runtime-adapter/src/lib.rs  1140
docs/work-packages/README.md                         605
docs/work-packages/cqr-burndown-execplan.md          691
```

Static: no touched non-exempt Rust file is at or above `3000` lines.

Ran: suppression census:

```text
before target-file suppressions:
lib.rs:194: #[allow(clippy::too_many_lines)]

after target-file suppressions:
none
```
