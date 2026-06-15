# CQR14 Behavior Equivalence

Static: public release API signatures are unchanged:

- `lint_release_directory`
- `write_release_sidecar_for_binary`
- `validate_release_sidecar`

Static: preserved behavior includes candidate filtering, sidecar-path
construction, binary-role classification, sidecar schema validation, stable
`ReleaseLintError` variants, error payload paths/names, HBP pair parity, JSON
field names, hash generation, timestamp behavior, and release report payload.

Ran: focused characterization before production refactor:
`cargo test -p openwepp-runner release::`, `10` passed.

Ran: focused release tests after production refactor:
`cargo fmt --check && cargo test -p openwepp-runner release::`, `10` passed.

Static: no public API, schema, stable error ID, alias, symbol, unit, parser, or
output formula behavior was changed.
