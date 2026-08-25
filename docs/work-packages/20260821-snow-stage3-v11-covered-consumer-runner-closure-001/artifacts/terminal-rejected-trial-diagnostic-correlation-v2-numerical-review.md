# Terminal rejected-trial diagnostic correlation V2 numerical review

Evidence class: `Static`

Reviewed SHA-256:
`f4a7ff15127fdfd5068f16126f440a57a25026b44a5c610f175dfab30417cc5c`.
The hash matched. I made no source or authority edits and did not read or
communicate with the other reviewer.

Recommendation: **HOLD**.

## Static evidence

- `snow_stage3_v11_terminal_execution.rs` owns each complete
  `CoveredCarrierPhaseResultV1` in the provider closure and currently retains
  phase results by ending-joint digest.
- `stage3_solver/evaluation.rs` owns the 0..32 coupling replay, computes the
  convergence predicate, and returns only the selected transition, flux and
  ending joint.
- `stage3_solver/terminal_event.rs` owns the full/two-half composition, the
  five-component scaled-error reduction, retry proposal and subsequent
  pre-provider `BelowCarrierDomain` check.
- The last admissible rejected pair is therefore reachable only by carrying a
  selected provider-call identity through both intervening layers. A fixed-size
  key plus caller-local arena can do this without carrying complete receipts
  through the numerical solver and without changing the physical decision.

## Findings

1. **Critical — selected/discarded record finalization is not closed.** A
   provider evaluation record is required to contain
   `selected-for-trial` and its final record digest, but the provider closure
   cannot know selection when it appends the record. Selection is known only
   after the coupling loop compares the newly computed preview with its prior
   hint. The authority neither permits nor specifies a post-selection arena
   finalization operation, nor defines how such finalization changes the
   record digest, arena index entry and already-returned key digest. Resolving
   this during implementation would invent custody semantics; leaving every
   appended record unselected would violate the exactly-one selected rule.

2. **Critical — the canonical role vocabulary does not map the actual call
   graph.** On every rejected adaptive retry, the existing solver labels the
   coarse request `Retry`, not `Full`; its pair mates remain `Half1` and
   `Half2`. The V2 text simultaneously requires each pair to contain a `full`
   record, requires role equality between trial and carrier record, and gives
   `retry` a separate tag, without defining whether "full" is a pair position
   distinct from the provider's actual role. In addition, the live enum has
   `BracketLower` and `BracketUpper`, while the closed wire offers only
   `root`, `retry`, and `event_root` for nonordinary calls. The ordered
   non-pair chronology therefore cannot encode every actual retained call
   without inventing role translation or dropping calls.

3. **Major — pair-to-floor chronology is ambiguous.** The 1.875-second pair
   produces `reject_retry` and proposes a 0.9375-second next full support. The
   next loop iteration then returns `BelowCarrierDomain` before any provider
   call because two 0.9375-second halves would be below 600 ms. The authority
   says the final pair record ending in `BelowCarrierDomain` binds the prior
   triple, but also gives every pair one decision tag and one typed
   result/error. It does not state that the prior pair remains
   `reject_retry`, while the prefix separately owns the later pre-provider
   error, nor define a distinct floor-decision record. Encoding
   `BelowCarrierDomain` on the prior pair would misstate the existing numerical
   decision; encoding only `reject_retry` leaves the claimed final cardinality
   underspecified.

4. **Major — exact error reconstruction fields remain numerically
   underdefined.** The record requires a signed raw difference and a winning
   component tag, but does not define whether raw difference is
   `full-refined` or `refined-full`, or the deterministic winning-component
   rule when scaled values tie. The current implementation computes only
   absolute normalized values followed by a chained maximum and retains no
   winner tag. These conventions affect canonical bytes and the claimed
   independent reconstruction and must be frozen before implementation rather
   than chosen by it.

5. **Major — the complete-receipt adapter escape hatch is not yet a closed
   schema.** Required forcing, topology, prescribed/rate/generated, `q_ss`,
   hydrology and WB14 evidence is physically reachable from the provider
   inputs and `CoveredCarrierPhaseResultV1`/carrier envelope. However, the
   authority defers every receipt lacking released canonical bytes to adapter
   field lists that will be frozen later in an implementation-intent diff.
   Because those lists determine whether the required numerical operands are
   actually complete, the reviewed V2 hash does not itself close the evidence
   wire sufficiently to authorize implementation without further schema
   judgment.

The proposed dual-channel architecture materially fixes the mini-gate's
original reachability problem, and fixed-size identities are sufficient in
principle. The findings above concern the exact canonical and cardinality
rules needed to implement that architecture without inventing numerical
authority. Revise and re-review the authority before any source implementation.
