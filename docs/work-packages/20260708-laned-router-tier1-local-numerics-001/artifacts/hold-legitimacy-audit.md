# Hold Legitimacy Audit

Status: `EXECUTED-HOLD-APPROXIMATION-ENVELOPE`

Hold blocker: the `Re^0.45` approximation candidate lacks a ratified
bounded-error envelope.

Why legitimate:

- Rev 47 explicitly forbids an approximate Hirsch branch without coefficient
  provenance, input range, and max absolute/relative error tests.
- The package landed all other Tier 1 numerics and met the H2637 timing target
  without this approximation.
- Adding a heuristic approximation would violate the no-unbounded-fast-math
  rule and weaken branch fidelity.

Minimal follow-on: create a small approximation-envelope package that derives
and tests a minimax/vector-ready replacement for `Re^0.45`, or explicitly
close the candidate as not worth further risk.
