# Rollback Evidence

Status: `focused Child-2 PASS / Child-4 exhaustive phase matrix pending`

Evidence class: `Static + Ran`

The public Child-2 API takes the production `DirectRunFrame` immutably and
constructs all snapshots, authorizations and ending state in owned clones.
Failures return no candidate. The bounded Child-2 tests exercise request and
snapshot identity rejection, authorization/final-use validation, candidate
debit failure, aggregate reconciliation, and successful return without
mutating the production value. Exhaustive named phase injection through the
later land-surface real consumer remains a Child-4 obligation.

Every case must prove:

- whole-production-value equality for the original run frame;
- exact canonical hydrology-snapshot bytes before and after;
- no retained request, authorization, use or candidate on error; and
- no runner, selector, output or persistent-lane mutation.

Thirteen crate-local cases cover wrong OFE, layer, transaction, basis,
duplicate identity, mixed eligibility, zero supply, partial frost and
finalized-use overrun. Each error path returns no candidate and preserves the
original whole `DirectRunFrame`. The three-case public contract separately
proves success also leaves the production frame unchanged. Independent
hydrology/ownership review returned GO for this declared boundary.
