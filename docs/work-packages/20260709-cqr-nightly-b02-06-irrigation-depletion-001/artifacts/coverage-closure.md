# ADR-0021 Coverage Closure

Tier: science. The parser projects immutable irrigation schedule state to the
runtime scheduler under `SC-INFILE-IRRIGATION-DEPLETION-001`; therefore 90%
line and region coverage plus a 75% per-function region floor apply.

The test-first public contract cases bind each `IRD-E-*` error, strict/compat
datver/nozzle/depsrg/zero-start behavior, system-specific row grammar, period
dates, initialization/continuation, cross-file/topology constraints, and typed
warnings to the real parser. The contract guard map G-IRD-001 through G-IRD-012
is exercised only where this parser owns the stated behavior; no exclusion is
used.

Final coverage is 92.329% production lines and 91.633% regions. All target
functions meet the 75% region floor.
