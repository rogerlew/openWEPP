# Implementation

Static: the first narrow compile proved that the existing isolated executor
fixture is the canonical reusable construction but its test module and helper
methods are private to `executor.rs`. The package authority was amended before
that file is edited to permit only crate-scoped `#[cfg(test)]` visibility.
Duplicating the complete fixture was rejected as drift-prone. Production code
and behavior remain protected.

Static: the correction exposes the existing executor test module and four
fixture items at `pub(crate)` only under `#[cfg(test)]`. The verifier
characterization now constructs a real isolated committed source range, valid
package authority, terminal plan, LIGHT receipt, READY audit, and HEAVY receipt.
It clones that admitted envelope for the existing identity/context/HEAVY/source
error-order assertions. The manually synthesized ambient-head audit was
removed.

Static: the production prefix of `executor.rs` has identical before/after
SHA-256
`eb481c992b73419ce76fe8beff7e437c9a06b805db3e47d2673ef0bf68386098`.
No production verifier bytes changed.
