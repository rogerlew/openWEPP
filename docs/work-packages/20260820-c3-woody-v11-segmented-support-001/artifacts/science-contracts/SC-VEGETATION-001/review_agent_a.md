# Contract Review Agent A — Time, Numerics, And Chronology

Status: complete / `HOLD`

Date: 2026-08-20

Reviewed exact commit: `5f4d3902065c316309785cc47ac63f766884bbd1`

Evidence class: `Static + Ran`

Scope: V11 time/numerics, exact full-support compatibility authority, temporal
operator semantics, segment/event/restart chronology, and independent
vector/reference anti-alias coverage. Production Rust was not reviewed or
edited.

## Findings

### A-001 — Critical — Required chronology population is mostly unevaluated

The contract requires start/end events, zero-remainder receiver skip,
mid-parent restart, consecutive segmented parents, distinct segment forcing,
and the full poison set (`SC-VEGETATION-001.md:2619-2623`; `package.md`,
Acceptance Population). The frozen vectors contain only support coverage,
scalar debits, one generic rejected attempt, duplicate scheduled receipt, and
one replay flag (`segmented-support-vectors.json:5-18`). The calculator has no
event, participant-set, segment-state, forcing, receipt/digest, checkpoint,
restore, consecutive-parent, or publication model
(`reference_calculator.py:35-60`). Its 600+1200 and 1200+600 cases therefore
cannot observe execution order. Passing 22/22 is self-consistency over a much
narrower model, not the required authority population.

Required disposition: expand frozen vectors and the independent calculator to
execute every required positive and poison chronology with alias-separating
forcing/state/identity operands before authority verification.

### A-002 — Critical — V11 event-transition authority is not implementable

The amendment classifies an event only as “regime/participant and admitted
custody change” (`SC-VEGETATION-001.md:2553`) and later mentions events in
coverage/finalization/restart. It does not define a V11 event input/capability,
precondition, deterministic same-tick order, beginning/ending V11 and owner
digests, transfer ledger, event ordinal, accepted receipt, replay key, failure
rollback, or zero-remainder skip rule. `execute_v11_segment` accepts only an
accepted slab. Importing generic coupled-time rules does not decide which V11
state/custody transitions are legal. This leaves event-boundary restart and
start/end event acceptance non-derivable.

Required disposition: define the V11 event-transition API and receipt/ledger
join, or explicitly define V11 as an inactive byte-identical participant at
events and bind all mutation to a named adopter authority; then add the
required event and replay vectors.

### A-003 — High — Restart wire omits contract-required continuation state

The contract requires current segment/slab/event/participants and
publication/reduction state (`SC-VEGETATION-001.md:2597-2601`), but the closed
restart schema has no explicit fields for those values and no publication or
reduction field (`v11-restart-schema.json:6-19`). A base64 coupled-time blob is
not sufficient authority for adopter-owned reduction/publication state, and
the schema does not specify canonical blob encodings or prove that hashes bind
decoded content. Consequently a mid-parent restore can lose an accepted peak,
publication buffer, or active V11 regime while still satisfying this schema.

Required disposition: make all adopter-owned continuation facts explicit and
closed, specify canonical encodings/hash relationships, and add fresh-object,
pre/post-event, mid-parent, abort, rejected-attempt-absence, and replay vectors.

### A-004 — High — Temporal classification leaves phenology/GSI chronology ambiguous

The table places phase/timers/GSI in sequential state while prose says named
GSI receipts are scheduled once and phenology edge selection runs per physical
segment (`SC-VEGETATION-001.md:2551-2560`). It does not enumerate which imported
V10 GSI calculation, prior-GSI update, threshold edge, timer increment,
preparation/deployment, and material effects occur per slab versus once per
parent/calendar boundary. A literal import of the V10 per-transaction path can
run a daily edge repeatedly when a parent has multiple slabs.

Required disposition: publish an operation-level phenology/GSI ledger with
receipt and state-update order, then test multi-slab and retry cases that would
fail if any scheduled action executes per segment.

### A-005 — High — Resource/reference arithmetic does not bind exact ordered semantics

The reference uses `math.fsum` over all debits after support validation
(`reference_calculator.py:53`), which is neither an ordered per-segment staged
inventory update nor a specified production binary64 operation sequence. It
does not model NH4/NO3 identity, current staged inventory, receipt lineage, or
ending-owner reconstruction. Distinct wrong implementations can therefore
pass the same expected total.

Required disposition: freeze exact amount representation and arithmetic/order,
advance each typed inventory sequentially, independently reconstruct parent
beginning minus ending, and add non-associative/identity-swapped aliases.

### A-006 — Medium — Migration vectors do not exercise the stated rounding authority

The migration rule requires exact rational conversion, ties-to-even, and exact
bit roundtrip (`SC-VEGETATION-001.md:2518-2522`), but the eight vectors include
no halfway tie with even/odd neighbors, one-bit neighbors around an admitted
cadence, large representable cadence/overflow boundary, or a positive duration
that rounds to zero. The calculator and a production implementation could
share common boundary mistakes without detection.

Required disposition: add independently derived tie, neighbor, zero-rounded,
and checked-range vectors with exact expected bits/ticks.

### A-007 — Medium — Full-support compatibility classification is not yet closed

The prose correctly requires a generated, omission-failing field ledger
(`SC-VEGETATION-001.md:2612-2617`), but the current ledger is a prose category
list with result population deferred. It does not freeze the exact V10/V11
types/paths, recursive field inventory, comparison operation, or exhaustive
identity-difference allowlist. In particular, transaction sequence and
diagnostic/receipt fields mix physical and successor identity concerns.

Required disposition: generate and freeze the exhaustive preimplementation
field classification (including branch/diagnostic handling and exact allowed
identity projections). Actual values may be populated during implementation,
but implementation must not choose what compatibility means.

## Gate evidence

- Ran: strict Binding Exposure Index lint — PASS for both amended contracts.
- Ran: science-contract unit compliance lint — PASS for both contracts.
- Ran: `cargo test --test c3_woody_v11_authority_contract` — 3/3 PASS.
- Ran: independent Python calculator — 22/22 reported PASS, subject to A-001
  and A-005 coverage defects.
- Ran: `git diff --check` — PASS.

## Recommendation

`HOLD`. The high-level successor boundary is sound, but the event/restart API
and authority population are not yet complete enough to constrain an
independent production implementation. No production Rust may begin until the
findings are dispositioned and independently verified.
