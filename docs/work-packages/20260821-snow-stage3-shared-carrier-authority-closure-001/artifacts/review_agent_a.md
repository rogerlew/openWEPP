# Child 2C Science Review A — independent authority review

Decision: **HOLD**

Evidence mode: `Static:` source, contract, schema, vector, and test review of
the current working tree. `Ran:` `python3
docs/work-packages/20260821-snow-stage3-shared-carrier-authority-closure-001/artifacts/reference_model.py`
completed successfully; `git diff --check` passed. No production Rust
implementation was run. No other review artifact was used.

## Finding SCI-A-001 — Blocker: longwave authority uses the turbulent node as a bulk canopy radiator

`SC-SNOWENERGY-001.md:1263-1279` defines the snow longwave boundary with
`sigma*T_ca^4` and the reference model implements the same shortcut at
`artifacts/reference_model.py:42-46`. The only accepted carrier vector has one
canopy surface and supplies no canopy component radiative state
(`carrier-boundary-vectors.json:12-36`). This conflicts with the active V11
authority: `SC-VEGETATION-001.md:2274-2297` requires current-trial component
temperatures, emissive-area weights, rank recurrence, and explicitly rejects a
bulk-canopy-temperature repartition. The Child 2C amendment also says V11
supplies those component temperatures and reciprocal operands
(`SC-VEGETATION-001.md:2833-2838`), but the new equation does not consume them.

Consequences are material: `T_ca` is a turbulent recipient state, not generally
the radiating temperature of leaf, wet-surface, and stem components; component
area/rank contributions are omitted; and the returned
`snow_canopy_longwave_exchange_w_m2` is not independently closed against an
equal-and-opposite component ledger. This is a prohibited bulk-temperature
proxy and can change both snow energy and canopy energy signs/magnitudes.

Proposed disposition: **reject and amend before promotion**. Define the
component/rank longwave equations and common area basis explicitly, retain the
equal-and-opposite snow and V11 operands, and add a fixture with distinct
component temperatures/areas where the bulk-`T_ca` result differs. Re-run
independent review after the authority and oracle are corrected.

## Finding SCI-A-002 — Blocker: event selection is not an independent state or conservation reconstruction

The coupled-time authority requires candidate enumeration, terminal-state and
ledger recomputation at every candidate, and four independently admitted
tolerances (`SC-COUPLEDTIME-001.md:701-718`). The reference model instead loops
over the already supplied `candidate_ticks` and reads four already supplied
errors from `candidate_errors` (`artifacts/reference_model.py:76-92`). It has no
terminal snow/liquid/energy state, no mass or energy operands, no event ledger,
and no candidate-state recomputation. The vectors therefore provide the answer
that determines acceptance rather than an independently reconstructable
physical case (`carrier-boundary-vectors.json:72-81`, `97-99`).

The Rust test only checks the selected tick and a floating-point tie rank for
accepted events (`tests/integration/snow_stage3_shared_carrier_authority_contract.rs:93-100`);
it cannot detect an omitted candidate, fabricated error, wrong terminal state,
or non-closing ledger. This fails the package’s explicit independent
conservation and candidate-enumeration obligations
(`package.md:161-165`, `220-233`).

Proposed disposition: **reject and amend before promotion**. Supply independent
begin/end state and flux operands, compute each candidate’s time/snow/liquid/
energy errors in the reference model, and make the test compare those
reconstructions and rollback identities. If a finite candidate subset rather
than every integer tick is intended, the contract must define the authoritative
candidate-generation rule and prove that the supplied list is complete.

## Finding SCI-A-003 — High: event ticks use the wrong wire representation and the receipt schema is not canonical

`SC-COUPLEDTIME-001.md:91-94` makes every tick an unsigned `u128` nanosecond
encoded as a canonical decimal string. The event receipt schema uses JSON
`integer` for every tick and support boundary
(`artifacts/event-boundary-receipt-schema.json:32-38`), the vectors use JSON
numbers (`carrier-boundary-vectors.json:69-79`), and the Python oracle performs
ordinary Python integer arithmetic (`reference_model.py:75-88`). Large valid
ticks can therefore lose identity when passed through a binary64/JSON consumer,
and leading-zero/overflow/noncanonical serialization cases are not exercised.

The schema also does not require `parent_end_tick > parent_start_tick`, does not
require candidates to be sorted, and does not bind candidate bounds or their
error data to `candidate_digest`. That is inconsistent with the closed receipt
requirements and canonical sorted candidate list in
`SC-COUPLEDTIME-001.md:749-758`.

Proposed disposition: **reject and amend before promotion**. Use canonical
decimal-string ticks in the receipt/vectors/oracle, validate `u128` range and
checked half-open support relations, enforce sorted unique candidates, and
make the digest cover the complete candidate/error payload.

## Finding SCI-A-004 — High: `ERR-CT-021` is referenced but absent from the canonical error contract

The new event amendment returns `ERR-CT-021 EventBoundaryNoCandidate`
(`SC-COUPLEDTIME-001.md:720-727`, `742-747`), and the model/vectors repeat that
error (`reference_model.py:93-94`, `carrier-boundary-vectors.json:136-192`).
However, the canonical branch table stops at `ERR-CT-020`, explicitly defines
precedence only through `ERR-CT-020`, and omits the new stable alias
(`SC-COUPLEDTIME-001.md:470-510`). Thus a no-candidate failure has no defined
global precedence or boundary/API alias and can drift against existing
`MinimumStepExhaustion`, `EventTransition`, or `AtomicAcceptance` handling.

Proposed disposition: **reject and amend before promotion**. Add the new error
to the branch table, stable aliases, precedence ordering, serialization/error
schema, and an overlapping-trigger poison vector. The test must assert the
canonical variant, not only the literal string.

## Finding SCI-A-005 — High: the accepted carrier oracle does not require the sealed exposure/wind authority

The contract requires an exposure-projected wind at the admitted virtual
geometry and rejects raw 10 m wind, fixed attenuation, and hidden floors
(`SC-SNOWENERGY-001.md:1215-1230`, `1258-1261`). The accepted vector contains
no wind, exposure receipt, transfer height, or geometry at all
(`carrier-boundary-vectors.json:14-21`). The model only rejects the exact token
`wind_operand == "raw_10m"` and otherwise accepts missing or arbitrary wind
authority (`reference_model.py:18-25`); its sensible/vapor conductances are
hand-supplied values, not derived or joined to an exposure receipt. The receipt
schema has only an unconstrained `exposure_receipt_id`
(`carrier-receipt-schema.json:23-28`).

This means the positive case can pass without the required sealed precondition,
and fixed attenuation or a hidden-floor path is not distinguished from a valid
exposure projection. It does not prove the package’s prohibited-proxy boundary.

Proposed disposition: **reject and amend before promotion**. Add a typed
exposure/transfer input with source identity, geometry, units, and derived
conductance lineage; reject missing, raw, fixed-attenuation, and hidden-floor
variants; and add a positive fixture whose conductance is traceable to the
sealed receipt.

## Finding SCI-A-006 — High: support aggregation is numerically sketched but not receipt-authenticated or tested end-to-end

The reference model does compute `max(pre_participant_supports)` and
`max(post_participant_supports)` (`reference_model.py:69-70`), matching the
intended rule in `SC-COUPLEDTIME-001.md:681-697`. But the vectors carry anonymous
arrays of untyped integers rather than ordered active participant IDs and
individual support receipts (`carrier-boundary-vectors.json:73-79`, `91-97`).
The event schema has only the two aggregate numbers and no participant set or
support-receipt binding (`event-boundary-receipt-schema.json:14-18`), while the
carrier schema has only unconstrained receipt-ID strings
(`carrier-receipt-schema.json:23-25`).

The integration test does not assert either aggregate support result or the
successor-admissibility flag; for accepted events it checks only the selected
tick and tie rank (`tests/integration/snow_stage3_shared_carrier_authority_contract.rs:93-100`).
Consequently inactive-owner inclusion, participant omission, swapped support
units, and below-domain successor behavior are not caught. The listed vectors
show boundary-shaped numbers, but do not prove a complete active-participant
receipt or an exact physical-support admission.

Proposed disposition: **amend and re-review**. Represent each active
participant and its admitted minimum support/receipt explicitly, preserve the
derived maximum and participant digest, add exact-minimum and one-nanosecond
physical successor cases, and assert all aggregate/rollback outputs.

## Finding SCI-A-007 — High: normalized tie-breaking is underdefined and the oracle/test relax deterministic identity

The contract names “lowest combined normalized mass/energy error” but does not
define the combination, operation order, numeric representation, or zero-
tolerance behavior (`SC-COUPLEDTIME-001.md:711-718`). The model chooses an
undocumented sum of three binary64 divisions, substitutes `1.0` for a zero
tolerance, and relies on Python float behavior (`reference_model.py:88-95`).
The expected JSON stores `0.3`, while the run returns `0.30000000000000004`; the
Rust test accepts any difference below `1e-12`
(`tests/integration/snow_stage3_shared_carrier_authority_contract.rs:94-99`).

That is not a deterministic receipt identity across Rust/Python or restart
boundaries. It also leaves ambiguous whether event-time error participates in
the normalized rank and whether mass/energy terms have fixed weights.

Proposed disposition: **amend and re-review**. Specify the exact normalized
score, included terms, units, zero-tolerance rule, binary64/decimal encoding,
operation order, and tie-rank digest. Replace the `1e-12` comparison with a
bit-exact or contract-defined canonical comparison and add equal-score and
nonfinite/negative-error poison cases.

## Finding SCI-A-008 — Medium: receipt schemas do not expose enough operand lineage to enforce exact-once closure

The carrier contract requires complete operand lineage, residuals, current-trial
temperatures, owner/support identities, and independent snow/vapor/liquid/
energy/longwave ledgers (`SC-SNOWENERGY-001.md:1304-1309`). The carrier schema
contains only node values, opaque flux/radiation IDs, one ledger digest, and
owner digests (`carrier-receipt-schema.json:6-33`); it has no units, area basis,
conductance/temperature operands, residuals, or closure components. The event
schema similarly exposes only one selected error tuple while leaving the
candidate payload behind an opaque digest (`event-boundary-receipt-schema.json:16-48`).

Opaque IDs/digests can authenticate bytes only after a separately closed
canonical payload schema exists. In the current artifact set, the reference
model emits no event error fields or receipt identities at all
(`reference_model.py:96-105`), so replay and exact-once conservation cannot be
independently reconstructed.

Proposed disposition: **amend before promotion**. Define the canonical payload
covered by each digest, include or separately schema-bind every authoritative
operand and residual with units/area/interval/owner lineage, and test duplicate,
alias, stale-snow, and partial-owner receipts.

## Independent-oracle conclusion

`Static:` `reference_model.py` imports only Python standard-library modules
(`json`, `math`, and `pathlib`) and contains no `openwepp` import, Rust FFI, or
Rust subprocess call (`reference_model.py:1-12`). The integration test launches
it as a separate `python3` process (`snow_stage3_shared_carrier_authority_contract.rs:49-70`).
Therefore it is genuinely separate from the Rust implementation in the narrow
execution/provenance sense. It is not yet an independent scientific oracle for
the Child 2C claim because it trusts vector-supplied candidate errors and uses
the bulk-`T_ca` longwave proxy identified above. Those are authority failures,
not Rust coupling.

## Positive observations and residual risk

`Static:` For the one supplied carrier fixture, the turbulent sign convention
closes numerically: the model produces snow `+527.625 W m^-2`, canopy
`-376.875 W m^-2`, and reference `+150.75 W m^-2`, with a zero temperature
residual (`reference_model.py:36-41`, `60-62`; vectors
`carrier-boundary-vectors.json:26-36`). The listed unequal-support examples
also select the expected max-support boundary for their supplied candidate
lists.

`Ran:` The reference model completed and `git diff --check` passed. No
production consumer, Rust carrier, restart implementation, or real downstream
LSE/snow path was executed; those remain implementation-gate risks even after
the authority defects above are corrected.

## Final disposition

**HOLD.** The longwave equation/proxy, non-reconstructive event oracle,
unclosed `ERR-CT-021` contract, and missing canonical tick/exposure/support
receipt semantics are closure-blocking. The package is not eligible for
GO-WITH-AMENDMENTS until the proposed corrections are made and independently
re-reviewed.
