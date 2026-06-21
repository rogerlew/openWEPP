# Scope Selection

Status: implemented.
Evidence mode: Static.

Selected scope: close `HOLD-R6-DIRECT-PUBLICATION-FRAME-ABSENT` by creating the
run-bound direct publication frame and direct projection consumers needed before
R6 output-family cutover.

Not selected:

- default activation;
- public production writer cutover;
- compatibility adapter deletion;
- output schema/unit/metadata changes;
- process physics changes.

Rationale: the current blocker is structural. Public output cutover cannot be
honest until a real direct publication frame exists and downstream consumers
read it.

Implemented scope stayed within this boundary: direct frame and direct
projection consumers now exist behind explicit shadow opt-in, while production
writers remain compatibility-backed pending R6 cutover.
