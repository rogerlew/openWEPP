# CQR20 Quality Plan Report

Static: scoped quality target is CRAP/cyclomatic-complexity burn-down for the
current target function in
`crates/openwepp-hillslope-orchestrator/src/runtime_inputs/05_projection_helpers.rs`.

Static: protected boundaries are public API, typed guards, stable error IDs,
aliases, runtime symbols, units, parser compatibility, formulas, float
expression order, and science-contract behavior.

Ran: live baseline target identity from `crap_before.json`:

```text
project_annual_extension_controls  line 585  CC 29.0  coverage 25.0  CRAP 383.796875
```

Static: target-file baseline coverage was:

```text
lines 599/796 75.25%
functions 44/48 91.67%
```

Static: closure approach was focused characterization followed by private
helper extraction for annual extension projection branches only.

Status: complete pending package commit and push.
