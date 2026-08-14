# Canonical JSON Portability Correction

Status: `implementation corrected / focused validation and fresh review pending`

Evidence class: `Ran + Static`

Rust deserialization of the frozen positive configuration reconstructed the
same field values but produced digest `c8896610...` when `serde_json` emitted
`1e-7` and `1e-8`. The independent authority oracle and frozen positive
instance use `1e-07` and `1e-08`, producing `45a5d141...`.

The correction binds the representation already used by the frozen authority:
lowercase exponent,
explicit sign and at least two exponent digits, applied only to JSON number
tokens outside strings. It changes no binary64 value, physics, solver
tolerance, branch, state field, model definition or schema byte. Tests poison
an exponent-like string to prove it is not rewritten and validate the complete
positive configuration/state instances.

During that validation, Rust exposed a distinct generator defect. The strict
positive state was projected from the open tile to the forest tile and then
retained the owner-envelope beginning-state digest. Its embedded `662de618...`
digest therefore did not describe its final bytes. The generator now computes
the strict-state digest only after the complete projection. The corrected
embedded and independently reconstructed digest is
`6ff22f0d72b6c4fdad3c0d8a0b2947571191e48213635609af8f3b951c07abf1`.

The corrected generator is `9278be79...`; the regenerated vector fixture is
`9f171b0f...`. The LSE and V8 definitions, six schemas, exact joint core,
equations, numerical algorithms, physical values, accepted candidates and
failure branches are unchanged. No model identity, solver tolerance, physics
tolerance or bounded normalization was introduced.

An exact recursive comparison against the historical `3f1cf8ee3` fixture found
four changed JSON leaves and no others:

- the strict positive state's embedded digest;
- the same state digest inside the coupled-transaction beginning state;
- the dependent strict-state instance digest;
- the dependent coupled-transaction instance digest.

No forcing, state value, request, authorization, finalized use, candidate,
ledger, diagnostic, failure, poison or reconstructed physical operand changed.
