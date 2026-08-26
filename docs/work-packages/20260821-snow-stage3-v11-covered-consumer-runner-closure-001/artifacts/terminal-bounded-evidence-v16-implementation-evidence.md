# Terminal bounded evidence implementation-first checkpoint

Status: `EXECUTED / ONE LIVE-FIXTURE HOLD`.

Source base: `80f0af80fdd0b5044fb6c6e800212076b1e2acec`.
Diagnostic source predecessor: `ce58080c06f604ec1c5579db83517c8495c8514c`.
Last qualified physical implementation:
`43cc9bbea2fbf5fe6ab6596cee4162de75cef999`.

## Implemented

Static: the private sealed evidence mode now retains complete successful
`CoveredCarrierPhaseResultV1` values, ordered provider requests/outcomes,
optional iteration-zero comparisons, four live coupling comparisons, coupling
selection reason, complete selected-trial terminal states and ledgers, three
joint identities, the exact five live decision operands, pair-scoped proposed
next duration, and raw floor drafts. `NoEvidence::ENABLED=false` guards DTO
construction and cloning at the generic source sites. Production APIs,
physics, tolerances, the 600 ms floor, controller, acceptance predicate,
publication and owner installation are unchanged.

Static: a post-return validator checks raw provider/iteration request-key
joins, iteration-zero optionality, selected-trial order and half-chain joins,
five-component decision reconstruction, final Retry identity, pair/floor
duration continuity, the 600 ms floor and no provider call after the final
pair. It returns a new immutable validation summary and does not mutate raw
evidence.

## Ran

- `git diff --check`: PASS.
- `nix develop --command cargo fmt --all -- --check`: PASS.
- `nix develop --command cargo check -p openwepp-hillslope-orchestrator --lib`:
  PASS with pre-existing warnings.
- `nix develop --command cargo test -p openwepp-hillslope-orchestrator --lib --no-run`:
  PASS before the final validator correction; focused recompilation after the
  correction passed.
- focused lower capture:
  `capture_mode_retains_rejected_pair_and_separate_floor_admission`: PASS.
- real capture:
  `interior_terminal_event_capture_reproduces_below_carrier_domain`: PASS;
  real result remains `BelowCarrierDomain`.
- V20/V21 structural guards: PASS, 5/5, nextest run
  `dff5bb5d-7c83-4945-bf07-14b1d4af2b9a`.
- affected heavy nextest: 844 passed / 11 failed / 1 skipped, final run
  `3149fbd6-7ecf-4061-8887-0145a9405fcd`; the eleven names and normalized
  signatures exactly match the frozen historical census. Log:
  `/tmp/openwepp-task-logs/openwepp-hillslope-orchestrator-heavy-20260826T004954Z.log`.

## Review disposition

Independent numerical/evidence and Rust/API reviews returned `HOLD`. Remaining
blocking evidence obligations are `CHILD1-TERM-COUPLING-020`'s real alternating
temperature 32-iteration characterization; a validated selected coupling/result
and provider ordinal in the immutable final record; three separately typed
terminal-ingress exclusions; independent identical-beginning
NoEvidence/Capture provider-support and caller-state comparison; and complete
raw one-field poison coverage. The failed-call projection also needs a closed
typed error classification rather than a static string.

The candidate-v21 effectivity/conservation matrix is therefore `NOT RUN`:
enriched capture did not pass its two required sufficiency reviews. Final
v21/v11/v139/v6 numerical and science/ownership reviews were not opened.
Temporal operator and Batch V2 implementation remain prohibited.

## Forward correction checkpoint

Ran: the corrected real capture executes independent identical-beginning
`NoEvidence` and `CaptureEvidence` calls, proves identical physical failure and
ordered provider supports, and preserves caller-owned state. Typed Stage-3
supply, WB14 receipt and input-ingress exclusions are separate. The raw poison
matrix covers provider, iteration, selection, trial, pair and floor
omission/duplication/reorder/substitution, comparison arithmetic, decision
arithmetic and both rejection conjuncts.

Static: post-return validation now reconstructs successful result/request
joins, failed-call zero-iteration custody, closed ordered coupling chains,
selection finality, selected carrier and hydrology joints, all four comparison
tuples, all five decision tuples and the final floor boundary. The production
scaled-error refactor preserves the original NaN behavior.

Ran: focused lower and real captures pass; crate check and test compilation
pass; V20/V21 guards pass 5/5 (`921ef42e-24fa-4b14-8445-098062288373`). The
affected heavy run `79bb978b-1037-4651-a8bc-007b5382f821` is 845 passed / 11
failed / 1 skipped. Authoritative comparison found the exact historical eleven
names and normalized signatures with zero delta. Numerical/evidence review is
`GO`; Rust/API/private-compilation/noninterference review is `GO`.

Overall evidence remains `HOLD` only because `CHILD1-TERM-COUPLING-020` is
characterized through the production comparison functions but not by a real
carrier/provider capture that records `IterationLoopExhausted`. Synthetic
substitution is prohibited. Therefore the candidate-v21 matrix and final
candidate reviews remain unrun; no temporal operator or Batch V2 was changed.
