# Heavy Attempt 04: Wrapper Preflight Failure

Evidence class: Ran.

Attempt:
`/home/workdir/openWEPP-quality-history/20260724-order3-local-attempt4-0uwQOs`.

The external wrapper opened `transition-command.log` inside the new attempt
root before invoking the observatory. The observatory correctly rejected the
nonempty root with exit `2`.

No admission, inventory, profile, merge, CRAP, or publication phase ran. The
changed-condition attempt used sibling-only wrapper capture and a fresh root;
attempt 4 was never resumed.
