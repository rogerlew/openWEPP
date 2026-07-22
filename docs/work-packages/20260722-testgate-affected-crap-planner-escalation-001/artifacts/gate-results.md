# Gate Results

Ran: exact focused planner regressions pass 3/3 in 39.531 seconds: root
measurement escalation, ordinary production affected scope, and Cargo graph
production/reverse classification.

Ran: planner all-target Clippy passes with warnings denied. Formatting and diff
hygiene pass.

Ran: the pre-commit full planner target stopped at its intentional exact-clean-
checkout test because the correction was uncommitted; this is not claimed as a
passing target. The exact unchanged target will be rerun after the correction
commit creates the required clean checkout.

Ran: the first clean target attempt exposed RTR-036 at 117/153 before fail-fast
cancellation. After its one-line correction, the changed-head target exposed
RTR-037 and one stale planner expectation at 131/153. Both failures are retained;
neither target was repeated unchanged.

Ran: the next changed-head target exposed RTR-038 at 135/153. Source audit found
and corrected the one adjacent remaining receipt fixture in the same package;
the failed target was not repeated unchanged.
