# CQR18 Public API Surface Parity Report

Status: closed.

Static: production edits are private helper extraction in HBP parser validation
code. No public API change is authorized or made.

Parity findings:

- `validate_payload` remains `pub(super)` with the same signature:
  `fn validate_payload(data: &[u8], layout: &Layout, entry: &DirectoryEntry)
  -> Result<PayloadValidationResult, HbpParseError>`.
- `PayloadValidationResult` remains `pub(super)` and still exposes only
  `latest_event_payload`.
- No exported HBP parser types, options, warnings, path resolution values, or
  error enums were changed.
- No module declarations, public re-exports, Cargo dependencies, feature flags,
  or binary format constants were changed.
- Added tests exercise the public `parse_hbp_from_bytes` entrypoint rather than
  exposing private helpers.

Conclusion: public API surface parity is preserved.
