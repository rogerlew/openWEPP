# Implementation

Static: the first narrow compile proved that the existing isolated executor
fixture is the canonical reusable construction but its test module and helper
methods are private to `executor.rs`. The package authority was amended before
that file is edited to permit only crate-scoped `#[cfg(test)]` visibility.
Duplicating the complete fixture was rejected as drift-prone. Production code
and behavior remain protected.
