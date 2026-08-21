# Authority review B — 2026-08-20

Review base: `464cd506ad2fa789cc68a22e969646be639b50df`, with the prospective
Version 6/V11 Version 25/transaction Version 14 amendment present as uncommitted
authority artifacts. Reviewer B did not author the amendment and made no
production-Rust or protected-wire edit.

Verdict: **FAIL / authority corrections required before release**.

The fixed `600000000 ns` (0.6 s) boundary is explicit model policy. The text
correctly distinguishes it from coupled-time chronology, forbids rounding or
retrying at the minimum, admits the exact boundary, and orders a below-domain
failure before Newton. The independent oracle passes 4/4 and both JSON files
parse. Those facts do not close the receipt and restart authority below.

## Findings

### `LSE-SUPPORT-B-001` — accepted-slab identity is absent (blocking)

The receipt schema contains only a duration-like `requested_support_ns`. It
does not bind support start/end ticks, parent transaction, segment, accepted
slab ID/ordinal, or the coupled-time duration receipt. Two different slabs
with the same duration can therefore reuse the same purported LSE admission
receipt. This is incompatible with the V11/transaction statement that an
accepted slab *carries* the receipt and makes the receipt insufficient to
prove exact support rather than duration equality.

Required correction: bind parent transaction, segment, accepted slab
ID/ordinal, exact `[start_ns,end_ns)`, and the supplied coupled-time
`duration_s_bits`; require `end-start == requested_support_ns` with checked
integer arithmetic and exact equality to the admitted coupled-time slab.

### `LSE-SUPPORT-B-002` — receipt digest is not derivable (blocking)

The schema carries `receipt_sha256`, but neither the contract, schema, vectors,
nor reference calculator defines its domain separator, canonical field order,
encoding, or rule for zeroing/omitting the digest during derivation. The
reference calculator tests only the scalar threshold and never constructs or
poisons a receipt. Independent implementation and reconstruction are therefore
not possible.

Required correction: freeze one canonical preimage and domain-separated digest
algorithm, add a positive receipt vector with exact expected digest, and add
wrong support, duration bits, model/config/state/tolerance/numerical policy,
minimum, slab identity, and digest poisons.

### `LSE-SUPPORT-B-003` — rollback/restart population is asserted, not closed (blocking)

Version 6 says below-domain rejection returns no candidate and leaves all
owners byte-identical. The V11 amendments additionally promise fresh-restart
equivalence. The supplied vector suite has only four threshold outcomes; it
contains no beginning/staged/committed owner bytes, attempt/accepted cursor,
receipt chronology, restart checkpoint, or pre-/post-rejection comparison.
It therefore cannot show that a rejected attempt consumes no chronology, that
no support receipt is persisted for rejection, or that restoration retries the
same unaccepted support without replay or minimum-step substitution.

Required correction: specify that rejection creates no accepted receipt and
mutates neither accepted clock/controller chronology nor any staged/committed
owner; add exact before/after owner, cursor, scheduled/reduction/publication,
and restart bytes plus fresh-object restoration poisons.

### `LSE-SUPPORT-B-004` — policy evidence scope is not reproducible (blocking)

The narrative claims a frozen covered/open-mineral/litter, wet/dry, capacity,
V10 and actual-stack sweep, but the retained evidence gives only a command and
summary observations for 1 ns, 0.6 s, 0.601 s, and a non-monotone 0.06 s
neighbourhood. It does not retain the fixture matrix, exact support population,
identities, outputs, or raw result artifact needed to audit the stated domain.
Because the policy is intentionally conservative this is not a demand to infer
a sharper threshold; it is a demand to substantiate the declared covered
population.

Required correction: retain the exact fixture/support matrix and command output
with identities and acceptance results, including every named surface/wetness/
capacity class and the exact-minimum case.

## Protected-boundary result

No amendment diff touches coupled-time production/wire files, DirectV10
restart V1, coupled-time restart V2, or persisted-restart code. The prose also
explicitly protects V10 behavior and those wires. This boundary is **PASS** for
the reviewed tree; the requested receipt and restart additions must remain
additive and must not amend those bytes.

Evidence executed:

- support oracle: `4/4` PASS;
- schema and vector JSON parsing: PASS;
- schema SHA-256: `1a77f6425580469019b8b2eaa585a7e03eb9011bf260fb1a1f0721a4c6e52970`;
- vector SHA-256: `83135c7e0711a133ca6a13215d52a4b607044b046844beb92b6148c7ab5456b2`;
- reference SHA-256: `4d971cac5e3e8306e7f66285b544c2d520869112d23bbba8491b5bafe591bbca`.

All four findings require explicit disposition and correction before Review B
can issue PASS.

## Superseding re-review — 2026-08-20

Verdict: **HOLD / prior B findings remain open**.

The correction adds absolute support plus parent/segment/slab fields and is a
useful step, but it does not yet make the receipt implementable or the claimed
rollback/domain evidence executable.

- `LSE-SUPPORT-B-001` remains open. The schema omits the promised slab ordinal.
  More importantly, `parent_transaction_id` is constrained as positive decimal
  text although coupled-time `ParentTransactionId` is a canonical 64-character
  lowercase SHA-256 digest. Segment and slab accept arbitrary nonempty strings,
  and `support_start_ns` accepts leading-zero aliases. These fields cannot join
  the production accepted-slab receipt exactly.
- `LSE-SUPPORT-B-002` remains open. There is still no domain separator,
  canonical preimage/field order, digest omission/zeroing rule, exact positive
  receipt body, expected digest, or digest poison. The reference oracle tests
  only threshold/support subtraction and does not authenticate a receipt.
- `LSE-SUPPORT-B-003` remains open. The `rollback` and `fresh_restore` rows are
  outcome labels without owner bytes, accepted clock/controller cursor,
  scheduled/reduction/publication state, checkpoint bytes, or independently
  reconstructed continuation. They do not prove byte-identical rollback or
  fresh-object equivalence.
- `LSE-SUPPORT-B-004` remains open. The retained sweep records only the
  `v10_actual covered forest` fixture. The other profile entries are merely
  `declared`, without exact inputs, identities, commands, or results. This does
  not support the claimed policy coverage across the enumerated domain.

JSON parsing and the scalar oracle pass, and protected coupled-time/DirectV10
restart files remain untouched. Review B cannot authorize promotion.

## Final B regression — 2026-08-20

Verdict: **HOLD**.

`LSE-SUPPORT-B-001` is mostly corrected in the schema, including canonical
64-hex identity shapes, slab ordinal, and canonical decimal ticks. One reference
bug remains: `validate()` applies the no-leading-zero decimal rule to the three
64-hex identities, so a legitimate digest beginning with `0` is rejected even
though the schema admits it. Decimal canonicality must apply only to ordinal
and tick/duration fields.

`LSE-SUPPORT-B-002` remains blocking because the contract and executable KAT
disagree. The contract requires
`SHA256("OPENWEPP_LSE_SUPPORT_ADMISSION_V1\\0" || canonical_json)`, but
`receipt()` and `validate()` hash only the JSON bytes. The frozen baseline
digest `1fe1a105...6be184` is the unprefixed result; the contract-defined result
is `ddbcf496...ba0f0`. Thus the reported 12/12 is not a KAT for the authority.

`LSE-SUPPORT-B-003` remains open. The only added rollback operand is a
three-field label (`parent_cursor`, one owner digest, `candidate_present`). It
does not freeze the complete beginning/staged/committed owner set, controller
and accepted chronology, scheduled/reduction/publication state, checkpoint
bytes, or a fresh-object continuation comparison.

`LSE-SUPPORT-B-004` remains open. The sweep-results artifact still identifies
only `v10_actual covered forest`; no sweep rows are fixture-keyed, and the
profile continues to mark the additional cases merely `declared`. It does not
substantiate exact-minimum success across the contract's enumerated fixture
domain.

Executed evidence: reference reports 12/12; baseline validates against the
JSON Schema. Those passes expose rather than cure the domain-tag contradiction.
Protected coupled-time and restart boundaries remain unchanged.

## Final correction-pass regression — 2026-08-20

Verdict: **HOLD**.

The domain-prefixed baseline digest is now correct: the Python oracle and the
frozen baseline both produce `ddbcf496a02558308aa34b1df961944f4cf2ae863cdccd35eb30593d804ba0f0`,
and the baseline is schema-valid. The following precise blockers remain:

1. The oracle still applies its decimal no-leading-zero rule to
   `parent_transaction_id`, `segment_id`, and `accepted_slab_id`. Canonical
   SHA-256 identities may legitimately begin with zero; the schema admits
   them, while the oracle rejects them. Schema and semantic admission disagree.
2. The claimed binding-poison population is incomplete. It does not mutate
   model version/definition, configuration, beginning-state, tolerance policy,
   minimum policy, support start, or a canonical-but-wrong slab ordinal. Those
   are receipt-bound fields under `INV-LANDSURFACEENERGY-117`, so each requires
   an identity poison rather than reliance on a self-generated baseline.
3. Rollback/restart evidence remains a three-field summary, not the complete
   owner/clock/controller/receipt/scheduled/reduction/publication/checkpoint
   before/after bytes or a fresh-object continuation. `LSE-SUPPORT-B-003`
   therefore remains open.
4. The sweep still contains results for only `v10_actual covered forest`.
   Other fixture-profile entries remain `declared` without fixture-keyed inputs
   and results. `LSE-SUPPORT-B-004` therefore remains open against the
   contract's explicit multi-fixture coverage claim.

Reference 12/12 and baseline schema validation pass, but PASS is not authorized
until the schema/oracle disagreement and evidence gaps above are corrected.

## Hardened-oracle Review B — 2026-08-20

Verdict: **HOLD; receipt identity/digest findings closed, evidence findings remain**.

The hardened oracle closes `LSE-SUPPORT-B-001` and `LSE-SUPPORT-B-002`: it
accepts leading-zero 64-hex identities, enforces canonical decimal fields,
joins independently supplied receipt identity, rejects digest-valid reframing,
and matches the domain-prefixed baseline KAT
`ddbcf496a02558308aa34b1df961944f4cf2ae863cdccd35eb30593d804ba0f0`.
The baseline is schema-valid and the oracle reports 15/15.

Two exact blockers remain:

1. `LSE-SUPPORT-B-003` is not closed by the constructed checkpoint. It contains
   one state digest, not complete owner IDs plus canonical bytes/digests. It
   omits controller policy/checkpoint and event chronology. "Fresh restore" is
   JSON encode/decode equality of that reduced dictionary, not reconstruction
   of a fresh continuation followed by the same unaccepted slab.
2. `LSE-SUPPORT-B-004` remains open. Loading seven profile labels is not fixture
   execution. The sweep artifact still names only `v10_actual covered forest`;
   six profile rows have no keyed input, exact-minimum result, or raw evidence.
   Retain results for every claimed fixture or narrow the claim prospectively.

No production or protected restart/coupled-time wire change was observed.

## Narrowed-domain Review B — 2026-08-20

Verdict: **HOLD / one residual blocker**.

`LSE-SUPPORT-B-004` is CLOSED. Version 6 now admits only the executed V11
actual covered-forest adopter; the profile marks open-mineral, litter, wet/dry,
and other surfaces out of scope/non-admitted pending a separate authority
cycle. The sweep evidence and contract claim now agree.

`LSE-SUPPORT-B-003` remains open. The amendment still promises exact all-owner
rollback and fresh-restore equivalence. The oracle checkpoint contains one
anonymous state digest rather than the complete owner manifest with canonical
owner bytes/digests, omits controller policy/checkpoint and event chronology,
and proves only JSON roundtrip of that reduced dictionary. It neither restores
a continuation nor executes and compares the same unaccepted slab. Close this
with the complete checkpoint/suffix evidence already named by the V11 restart
authority, or narrow this amendment to pre-Newton no-candidate/no-clock-mutation
authority and defer fresh-restore equivalence to implementation acceptance.

Receipt identity, digest, poison, minimum-domain, fixture-scope, and protected-
wire checks otherwise PASS.

## Complete-checkpoint Review B — 2026-08-20

Verdict: **HOLD / restart chronology remains internally inconsistent**.

The seven owner envelopes and additional controller/event/scheduled/reduction/
publication/outbox fields close the prior shape omission, but the constructed
checkpoint is not an admissible chronology:

- `accepted_until_ns` is `0`, while `accepted_receipts` already contains the
  baseline receipt for `[0,600000000)`. An accepted receipt cannot exist beyond
  the accepted cursor. The separately frozen vector snapshot instead has an
  empty accepted-receipt list, so vector and oracle disagree.
- `suffix_operations` then executes the same 600 ms support, making the fixture
  ambiguous between replaying an already accepted receipt and executing the
  unaccepted first slab.
- The two suffix digests are not independent beginning-versus-restored
  executions. Both hash the same checkpoint JSON plus the same literal suffix
  string; equality follows from JSON roundtrip and cannot detect a wrong state
  transition, owner mutation, receipt append, or cursor advance.

Required correction: choose one consistent boundary. For a pre-first-slab
checkpoint, keep cursor `0` and accepted receipts empty, then execute a modeled
suffix independently from beginning owners and from a freshly decoded
checkpoint and compare ending cursor, seven owner bytes/digests, and receipt
chronology. Alternatively checkpoint after the slab at `600000000` and execute
a distinct later suffix. Do not derive both expected and restored results from
the checkpoint under test.

All other Review B findings remain closed; oracle execution itself reports
15/15 and protected boundaries remain unchanged.

## Final corrected Review B — 2026-08-20

Verdict: **PASS**.

`LSE-SUPPORT-B-003` is CLOSED. The checkpoint consistently records the accepted
600 ms prefix at cursor `600000000`, seven identified beginning and staged
owner envelopes with derived state digests, controller policy, accepted receipt
chronology, and empty event/scheduled/reduction/publication/outbox collections.
The uninterrupted path advances frozen beginning owners through prefix plus
suffix; the restored path advances freshly decoded staged owners through suffix
only; the independently constructed seven-owner endings compare exactly.
Rejection leaves checkpoint bytes unchanged.

All `LSE-SUPPORT-B-001..004` findings are CLOSED. Oracle 15/15, schema/KAT,
explicit 0.6-second policy, narrowed covered-forest scope, and protected-wire
checks PASS. Review B authorizes disposition and independent verification;
production remains gated on that cycle and implementation acceptance.
