# Characterization Plan

Before extraction, exercise the existing `pre_heavy` unit inventory and the
public TESTGATE executor contract. Add only behavior-characterization cases that
pin current audit identity, check order, error code, and receipt-admission
behavior. No source refactor may rely on unmeasured behavior.

Ran: a broad planner quick baseline was invalidated because the package intake
documentation changed while its exact-checkout verifier fixture was executing.
The fixture failed closed with `GATE-COMMITTED-CHECKOUT-NOT-EXACT`; it does not
establish a target-module behavioral failure. The replacement is a narrower,
clean-worktree `pre_heavy` inventory, not a repeat of the broad suite.

Ran: `cargo nextest run -p openwepp-gate-planner pre_heavy::tests --profile
quick` passed 15/15 in 16.175 seconds from committed head `fdf2c4c1`.
