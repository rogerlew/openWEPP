# Implementation

Static: commit `761f990b` extracts exact-match string mutation, array traversal,
and object-value traversal into private helpers. It preserves String/Array/
Object dispatch order, exact equality, array order, `object.values_mut()`
value-only traversal, recursive calls, scalar/null no-op behavior, and object
keys. No public API or production verifier logic changes.

Prerequisite RTR-043 correction `11df7e1d` separately moved the module to its
natural test-only path and replaced the inline `include!` wrapper with private
module wiring. That move retained executable tokens; rustfmt reflowed existing
layouts only. RTR-043 durable CLOSED digest is
`db483f16ddcac00825abebbef36a83b315fda30a809c0beec9225c91b411bc29`.
