# Correctness review — publication observer boundary

Reviewer: `/root/observer_correctness` (independent correctness/authority
review). Scope: adopted publication-observer continuation; exact detached source
starting at tree `d354ccc6ba947c6b0a9e3eed1cdedd0417ae1d5c2896288efbc828beca2f873b`;
SC-COUPLEDTIME-001 INV-COUPLEDTIME-032 / OBL-COUPLEDTIME-015; trusted append
callers, observer meaning, and the current four-file source delta. This is an
interim HOLD pending frozen-source runtime and reconstruction evidence.

**Static:** I read the adopted authorization, package/review instructions, prior
terminal correctness and QA reviews, the affected contract amendment, the
trusted append/install implementation and audit, all real trusted-support
callers, the NativeMixedPhase positive/refusal/late-rollback sequence, and the
current delta against the retained terminal reconstruction. **Ran:** no command
in this interim review is claimed as reviewer-run behavioral evidence.

## Findings and accepted fixes

- **Resolved blocking — the proposed test-level observation boundary changed
  record meaning and missed real callers.** Observation after
  `commit_selected_publication_and_take_staged_ending` would have changed the
  inherited record from one successful local history append into a later
  committed-ending claim, lost a record when later enclosing work rolls back,
  and failed to cover every real append path. The accepted source instead
  observes in
  `DirectV10RealConsumerShadow::push_validated_accepted_publication_support`
  only after `AcceptedPublicationHistoryV1::push_validated_support` returns.
  The ordinary/deferred, covered-owner-finalization, and snow-free-reseal callers
  all use that wrapper. The snow-free caller no longer bypasses it. Direct inner
  calls are confined to capability tests.

- **Resolved blocking — serialization failure could have been counted and
  emitted as success.** `accuracy_observation_wire` reports conversion failure
  as JSON `Null`; wrapping that value in another JSON object still encodes
  successfully. The accepted fix materializes the support value first, treats
  `Null` as an observation error, emits no success record, and increments no
  success count.

- **Resolved blocking — output failure could unwind an already successful model
  transition.** The initial moved observer used `println!`, which may panic on
  output failure. The accepted fix uses fallible stdout write and flush, does
  not propagate their error, and increments success only after both complete.
  A failed or partial measurement therefore cannot become a valid exact-one
  evidence record or alter the append result.

- **No static blocker in the reviewed cut.** The installer no longer performs
  observer serialization. The wrapper reads the just-installed immutable
  `Arc` by borrow, does not clone the support/capability, and does not rerun
  validation, operand/receipt reconstruction, or physics. It filters on the
  actual installed support's retained parent replay and preserves the original
  record version, boundary text, and support projection. The audit-scope query
  and counter make external placement observable. Failed inner append returns
  before observation.

## Residual risk and missing tests

- Terminal acceptance still requires a frozen source identity and non-vacuous
  runtime evidence. Required evidence includes: unchanged source guard PASS;
  validation-once and forbidden-work counters with zero append-time
  serialization/reconstruction/clone/full scan; enabled positive exactly one
  complete record with outside-scope/error counters zero; disabled and refused
  paths with no record; an injected measurement failure that preserves the
  append/model result while failing the measurement; and NativeMixedPhase N12
  showing one local-append record even though the later final-owner-join fault
  rolls back the enclosing outer roots.

- The positive stream must pass the unchanged independent parent reconstruction,
  exact-one/malformed/corruption controls, and associated donor/credit/install
  correspondence. Location or ordering changes need explicit binding; prior
  output cannot be relabeled as a current-source execution.

- The no-feature/both-feature control populations, affected four-mode
  compilation/membership, format, quality attribution, source custody, and
  retained C1-C8 reuse remain execution/evidence obligations. Strict Clippy,
  broader A-001/RQ1, original-input E008, scientific/conservation/restart
  qualification, production adoption, and cadence remain separate.

## Interim verdict

**HOLD pending terminal runtime evidence and same-reviewer follow-up.** The
current placement and failure isolation satisfy the static capability boundary
and preserve the local-append meaning. No source correctness defect remains in
the reviewed cut, but static inspection cannot approve the required observer
record or feature-control disposition.
