# CQR21 Quality Plan Report

Static: scoped quality target is CRAP/cyclomatic-complexity burn-down for the
current target function in `crates/openwepp-climate-runtime-adapter/src/lib.rs`.

Static: protected boundaries are public API, typed guards, stable error IDs,
aliases, runtime symbols, units, parser compatibility, formulas, float
expression order, and science-contract behavior.

Ran: live baseline target identity from `crap_before.json`:

```text
SharedClimateRuntimeInputError::fmt  line 195  CC 19.0  coverage 0.0  CRAP 380.0
```

Static: target-file baseline coverage was:

```text
lines 507/657 77.17%
functions 23/25 92.00%
```

Static: closure approach was focused characterization of every
`SharedClimateRuntimeInputError` error-code/display branch followed by private
message-format helper extraction.

Status: complete pending package commit and push.
