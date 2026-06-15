# CQR11 Kernel Profile Compliance

Status: complete.

Static: CQR11 is parser-affecting because management parser output feeds
runtime projection and kernel-facing management state.

Static: the edit did not change science contracts, units, aliases, symbols,
runtime projection formulas, parser public APIs, or kernel invocation paths.

Static: the production change preserves parser field meanings and output shape
for `YearlyPerennialData` and `YearlyPerennialGrazingCycle`.

Static: no provisional, surrogate, or heuristic process-physics math was added.

Static: no `unsafe`, dependency change, fallback wrapper, default-and-proceed
behavior, broad `Box<dyn Error>`, production `unwrap`, or production `expect`
was introduced.

Ran: parser characterization, workspace clippy, workspace tests, and
after-metrics gates passed; see `gate-results.md`.

Disposition: compliant for current CQR scope.
