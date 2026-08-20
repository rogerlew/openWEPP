# Implementation and test evidence

Status: BLOCKED before production implementation

Evidence mode: Static + Ran

Production implementation: NOT RUN. No production runtime file changed.

Ran before the blocker was found:

- `nix develop -c cargo fmt --all`: PASS.
- focused terminal numerics and terminal-receiver contract binaries: PASS,
  12/12.
- real shared-WB14 variable-duration nonlinear oracle tests: PASS, 2/2.
- `git diff --check`: PASS before contract checkpoint commit `83cf6eb8e`.

Static consumer tracing after that checkpoint found the closure blocker in
`openwepp-vegetation`: carbon and final tile receipts require
`interval_s == configuration.dt_s`; occupancy/T10 execution consumes that
nominal duration; LSE projections repeat the equality. The actual scheduler,
restart, rollback, rain, cross-midnight, routing, and noninterference runtime
scenarios are therefore NOT RUN and cannot be reported as passed.
