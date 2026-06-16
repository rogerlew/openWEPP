# CQR36 Kickoff Prompt

Execute CQR36 against
`crates/openwepp-input-contract/src/parsers/watershed_impoundment.rs`.

Reduce the live `parse_impoundment` target and any extracted helpers to CRAP
`<= 30` using behavior-preserving private helper extraction. Preserve public
parser API, stable error IDs, strict/compatibility behavior, branch arity,
typed guards, parsed output shapes, and downstream runtime semantics.
