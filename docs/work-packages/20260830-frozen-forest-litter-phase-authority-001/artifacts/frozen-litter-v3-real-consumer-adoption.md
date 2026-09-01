# Frozen-litter V3 real-consumer adoption

## Current disposition

The p61/native consumer now retains the accepted V3 fixed-final candidate and
runtime candidate, constructs unified hydrology without another LSE solve, and
projects the retained potential/final vegetation operands into the existing
vegetation/BGC complete envelope. The V3 branch no longer calls the legacy
snow-free physical constructor.

The original receipt-free-soil HOLD is corrected. After all non-soil owners are
accepted on the unpublished candidate, the V3 consumer independently rebuilds
the accepted V2 soil operand set from the frozen physical candidates and
ingress receipts, exact-orders and aggregates it, verifies the complete-owner
projection's prepared-owner and restart bytes, and installs one successor from
the authoritative beginning. The next support therefore reads the installed
transaction rather than reusing the first support's transaction.

The same adoption path now also binds the LSE support receipt to the exact
staged owner representation: the initial successor support joins the retained
legacy beginning, while later supports join the native V3 resident. The V3
water protocol publishes the exact aggregate of the accepted named liquid- and
ice-vapor debits; it no longer substitutes the legacy aggregate carrier.

The increment remains `HOLD` for terminal real-consumer closure. The unchanged
p61 fixture advances past the former receipt-free-soil refusal and now reaches
`176400000000000..178200000000000 ns`, where the shared real-hydrology receiver
rejects a retained-surface enthalpy credit whose checked binary64 addition does
not change the resident high-term bits. That shared receiver source is outside
the terminal V3 adoption write set, so this increment preserved the guard and
did not force an ULP, discard energy, or relax closure.

## Superseded failing custody tuple

Temporary diagnostic capture was removed immediately after classification.

- resident owner: transaction `41`, predecessor `40`, support
  `0..1_800_000_000_000 ns`, state
  `3faeddfbec5630cc032873a2ab42c21ac60b8c2d7696313e21f262077ab499ca`,
  receipt chain
  `413d0c3891a0dc81e7601b3e9b32bda7030a8703f1dddab16361f5c556358222`;
- rejected prepared beginning: transaction `41`, predecessor `40`, support
  `1_800_000_000_000..3_600_000_000_000 ns`, with the same state and receipt
  chain.

The strict receipt-free guard was correct: transaction 41 was reserved for the
first support and could not be reused for the successor support. The corrected
consumer now reconstructs and installs that segment's accepted soil successor
exactly once before preparing the next support.

## Evidence run

- focused V3 projection, adoption, publication-retention, rollback, V2-soil,
  persisted-restart, and runner-seed selector: 47/47 passed, nextest run
  `0bc94a6f-2aa2-493d-916d-d889cf6486f8`;
- focused V3 finalized-use regression: 1/1 passed, nextest run
  `441fbce8-0820-4da8-8372-5b4830f7ec86`;
- restart authority selector: 2/2 passed, nextest run
  `502debc6-392a-4d3c-83dc-c0d732394ff9`;
- V14 real-consumer authority binding: 1/1 passed, nextest run
  `1f67c1b9-c721-41cd-9a96-ff6b3dc317fd`;
- surface-liquid custody plus real-hydrology authority suites: 24/25 passed,
  nextest run `ddb078e0-9827-48bd-9182-53a2177f805a`; the sole failure is an
  unrelated stale registry prose assertion requiring `v13 retains the exact
  60-second adaptive floor and conservative routing`;
- unchanged p61 with a 64 MiB test-thread stack advanced through the former
  first/second-support soil refusal and reached the later shared retained
  enthalpy rejection in 82.2 seconds, nextest run
  `6bc0889e-7da1-47db-b802-c9b7638ba379`;
- four-crate `cargo check` passed; the only warning is a concurrent unchanged
  dead-code warning in active open-snow source outside this increment;
- workspace `cargo fmt --all -- --check`, owned-file `git diff --check`, and
  production diagnostic scans passed.

One broader source-introspection test remains independently malformed:
`candidate_boundary_is_pre_ingress_and_crate_private` searches its complete
source file for `pub struct V3MultiTileAcceptedFixedFinalCandidate`, but that
same forbidden string occurs in the assertion itself. It failed after 17
otherwise passing tests in run `72fee3c7-1a5f-46b7-9a69-8fcf10758a0a`; the
source is outside the terminal adoption write set and was not edited here.
The combined LSE authority run also retains one unrelated V12 index-prose
failure requiring `v12 retains exact centered interior differences`; its V14
real-consumer failure was corrected by binding both unchanged fixture drivers
to the successor model and phase-receipt tags exercised by their shared
production seed.

## Remaining handoff

Adjudicate the shared retained-LSE enthalpy representation at
`credit_retained_receipt_group` without weakening independent closure. The
current contract permits neither a forced-ULP credit nor discarded enthalpy;
if a successor exact carry is required, it needs authority and ownership
outside this terminal increment. Then rerun unchanged p61 and the native forest
consumer. The terminal V3 soil/restart/publication correction itself is ready
for review from the evidence above.
