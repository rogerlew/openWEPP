# WBVAL02 Artifacts

Status: complete

Evidence mode: mixed `Static:` and `Ran:`

Static:

- Package closed as validated invalid upstream input for
  `WBVAL02-CLIM-RUNTIME-E-017-RADBOUND`.
- Canonical authority, contract amendments, test changes, validation, review,
  verification, disposition, and handoff are recorded in this directory.

Ran:

- Before-state six-hillslope reproduction confirmed the original hourly
  `CLIM-RUNTIME-E-017` failures.
- After-state six-hillslope validation confirmed typed source-symbol
  `radly=486` evidence for all six hillslopes.
- Targeted and package-scoped Rust tests passed; broader workspace gate result
  is recorded truthfully in `gate-results.md`.

Closure checks:

- `HOLD` legitimacy: not a `HOLD`; invalid upstream input is typed and
  contract-backed.
- Envelope adequacy: the in-envelope correction is typed evidence at the
  SIMIMPL28 daily radiation seam, not downstream compensation.
- Protected-boundary integrity: the upstream input boundary is named in
  `worker-handoff.md`; no in-scope climate fix was deferred.
