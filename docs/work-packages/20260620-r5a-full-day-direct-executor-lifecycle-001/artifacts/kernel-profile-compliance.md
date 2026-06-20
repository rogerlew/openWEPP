# Kernel Profile Compliance

Static:

R5A is kernel-affecting because runtime projection controls kernel-adjacent
execution surfaces, but it does not change canonical process math.

Checklist:

- Canonical contract amendment required: no.
- Contract-derived process-vector tests required: no new process formulas.
- Typed guard behavior changed: no.
- Unit metadata or conversion behavior changed: no.
- Public binding exposure changed: no.
- Runtime default activation changed: no.
- Publication authority changed: no.

Required evidence moves to implementation tests, no-compatibility scan,
default-disabled counter tests, and H2637 default-disabled timing.
