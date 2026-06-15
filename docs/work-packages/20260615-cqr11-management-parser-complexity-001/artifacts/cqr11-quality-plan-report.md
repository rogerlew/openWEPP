# CQR11 Quality Plan Report

Static: scoped quality target is CRAP/cyclomatic-complexity burn-down for the
current target function in
`crates/openwepp-input-contract/src/parsers/management.rs`.

Static: protected boundaries are public parser API, parser compatibility,
typed errors, error IDs, field names, count/cardinality behavior, aliases,
units, parser-output shape, runtime-projection meaning, and kernel-facing
behavior.

Status: complete-with-warnings.

Static: live CQR target selected from before CRAP was
`parse_yearly_perennial`, not a stale snapshot row.

Static: closure criteria were limited to behavior-preserving CRAP/cyclomatic
decomposition for the scoped target and any new helpers.

Ran: before CRAP confirmed `parse_yearly_perennial` at CRAP `1406.0`, CC
`37.0`, and coverage `0.0`.

Ran: after CRAP confirmed `parse_yearly_perennial` at CRAP `4.0`, CC `4.0`,
and coverage `100.0`.

Ran: all new perennial helpers were below CRAP `10`.

Gate Evidence Non-Deferral:

- before metrics were captured before production refactor;
- characterization coverage was added and run before production refactor;
- production changes were private decomposition only;
- after LCOV/CRAP was rerun after the final clippy cleanup;
- required Rust closure gates were run after implementation.

WARN: target-file coverage remains below the science-tier threshold and
pre-existing out-of-scope CRAP rows remain above `30`.
