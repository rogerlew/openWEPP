# CQR33 Quality Plan Report

Static/Ran: package-scoped quality plan for CQR33.

## Target

- Live target from before metrics:
  `WatershedStructureParseError::fmt`
- File:
  `crates/openwepp-input-contract/src/parsers/watershed_structure.rs`
- Before CRAP: `240.0`
- Before cyclomatic complexity: `15.0`
- Before coverage: `0.0%`

## Suppression Census

The target file has an existing crate-level `#![allow(...)]` block including
parser-oriented Clippy exceptions. CQR33 added no new `allow` attributes and no
new suppression comments.

## Refactor Strategy

The chosen strategy is private delegation only:

- add exact display-string and `source()` characterization tests before
  production refactor;
- keep every public parser API, error ID, variant, field, and emitted parse
  structure unchanged;
- move the existing display match into a private
  `WatershedStructureParseError::write_display` helper;
- leave parser control flow, grammar, compatibility mode, token order,
  validation thresholds, and runtime-facing meanings untouched.

## Protected Boundaries

No science-contract authority, parser compatibility, unit, alias, symbol,
output structure, error ID, or public API change is authorized in this package.
