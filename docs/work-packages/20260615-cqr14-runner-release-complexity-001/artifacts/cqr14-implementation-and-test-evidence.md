# CQR14 Implementation And Test Evidence

Static: implementation decomposed `lint_release_directory` into private helpers:

- `collect_release_candidate_binaries`
- `lint_release_binaries`
- `lint_release_binary`
- `validate_lint_sidecar`
- `validate_lint_sidecar_role`
- `validate_lint_sidecar_binary_name`
- `lint_sidecar_hbp_supported`
- `required_lint_sidecar_str`
- `validate_release_hbp_pair`

Static: removed the targeted `#[allow(clippy::too_many_lines)]` suppression
from `lint_release_directory`.

Static: added focused release-lint characterization tests for:

- valid candidate discovery and non-candidate filtering;
- no-candidate failure;
- missing sidecar failure;
- invalid binary name before sidecar read;
- sidecar binary-name mismatch;
- watershed/hillslope HBP pair mismatch.

Ran: `cargo test -p openwepp-runner release::` before production refactor,
`10` passed.

Ran: `cargo fmt --check && cargo test -p openwepp-runner release::` after
production refactor, `10` passed.

Ran: final `cargo test --workspace`, passed.
