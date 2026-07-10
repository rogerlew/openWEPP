# Characterization

The target's public type surface is governed by the kernel-writeback and
unit-safe-boundary contracts. New local tests pin, without changing production
code:

- all `BoundaryValue` variant scalar/unit-label mappings and typed constructor
  rejection classes;
- all phase and consumer-adapter text labels plus classification predicates;
- writeback payload and response field preservation;
- dense-view, legacy-slot, and indexed fallback precedence; and
- state/flux scalar, series, and grid hot-symbol lookup identity.

Ran: `cargo nextest run -p openwepp-kernel-contract --profile quick` — `33`
passed, `0` skipped.

These tests are the behavior oracle. They assert typed errors rather than merely
`is_err`, preserving `BoundaryError::{NonFinite,BelowMinimum,AboveMaximum}`
semantics required by the unit-safe contract.
