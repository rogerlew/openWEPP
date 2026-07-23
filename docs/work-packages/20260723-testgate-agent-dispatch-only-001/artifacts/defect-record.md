# TGD-001 Defect Record

Status: `CLOSED`

Cause: `GATE-AUTOMATIC-DISPATCH-PACKAGE-ANCHOR-MISMATCH`

Ran: automatic push run `30020900413` reached forest1 content-gate execution
and failed before any expensive gate node completed. The hosted verifier and
aggregate then failed closed.

Static: automatic push admission cannot reliably select a package scaffold
that was created after the push event's prior remote head. Stable multi-commit
pushes can therefore lack an active base-commit package anchor even when the
exact pushed head carries intent trailers.

Correction boundary: remove automatic push execution; require explicit
post-push agent dispatch with the active scaffold commit as `base_ref` and the
exact package path as `intent_package`; preserve every downstream execution and
attestation guard.

Correction commit:
`5b287d523408e5a45b5a689326c19e18fc32ad11`.

Ran: the exact correction commit passed dual implementation review, dual
terminal verification, focused workflow and policy contracts, YAML parsing,
Markdown lint, formatting, policy binding, diff hygiene, and canonical package
admission. No TESTGATE run or expensive gate was executed.
