# Line-Count Governance

Status: QUEUED
Evidence mode: not-run.

Populate before closure if Rust files are edited.

Required checks:
- Identify touched `.rs` files and line counts.
- `2000+` lines: WARN disposition.
- `3000+` non-exempt files: refactor before closure or approved exception with
  owner/sunset metadata.
