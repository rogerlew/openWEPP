# Authority Review A — LSE Version 6 positive-support amendment

Reviewer role: independent time/numerics and supported-domain authority review
Reviewed: `SC-LANDSURFACEENERGY-001@6`, `SC-VEGETATION-001@25`,
`SC-VEGETATIONTRANSACTION-001@14`, the support schema/vectors/reference,
representation evidence, tiny-support blocker, and lifecycle index.
Disposition: **HOLD**

The architecture is sound: coupled-time keeps nanosecond chronology while the
physical LSE solver may reject a declared domain before Newton. The fixed
`600000000 ns` rule is an admission boundary rather than a duration floor,
the exact-minimum and one-tick-below sides are clear, and protected wires are
explicitly preserved. Promotion is blocked by the findings below.

## Findings

### `LSE-A-001` — Critical — receipt omits exact slab support identity

The schema persists only `requested_support_ns`, not absolute `start_ns` and
`end_ns`, parent transaction, segment, accepted slab, or coupled-time receipt
identity. Different slabs with the same duration can reuse one receipt.

Required: bind parent/segment/slab and half-open support; independently derive
duration ticks and binary64 bits; poison cross-parent, cross-segment,
cross-slab, shifted-support and same-duration substitution.

### `LSE-A-002` — Critical — oracle tests only integer comparison

The reference script neither builds/validates the closed receipt, derives its
digest, converts ticks to exact duration bits, nor tests configuration/state/
tolerance/numerical-policy joins. `LSEB-E-042` has no executable population.

Required: independently construct canonical receipts and error precedence;
add duration-bit, minimum-policy, identity/policy, digest-reframe,
unknown-field and malformed-integer poisons.

### `LSE-A-003` — Critical — supported domain exceeds frozen evidence

Evidence identifies one covered actual forest fixture and summarized 0.6/0.601
second passes. It claims a broader covered/open-mineral/litter, wet/dry,
capacity and actual-stack sweep without a raw case ledger, fixture/state/
forcing digests, per-case outcomes, closure, iterations or backtracks.

Required: narrow Version 6 to the exact frozen profile, or provide a
machine-readable population for every admitted surface/branch/configuration at
minimum-1, minimum and minimum+1 plus longer controls, with exact identities,
results, closure and rollback evidence.

### `LSE-A-004` — High — representation conclusion is too broad

The temperature ULP and `C*ulp(T)/1e-6` calculations reasonably show that the
listed storage-temperature lattice lies below 0.6 seconds. They do not define
the complete representational floor of the coupled hydraulic/humidity/
evaporation/Jacobian/residual system, and capacity operands lack provenance.

Required: narrow the conclusion to temperature-storage quantization, bind
capacity operand sources, and inventory other coupled scales as quantified or
explicitly unquantified. The empirical sweep must remain threshold authority.

### `LSE-A-005` — High — acceptance population is incomplete

The four vectors omit receipt-identity poisons, rollback bytes, fresh restore,
repeated/cross-slab use, zero/overflow conversion and the actual-stack branch
matrix. Structural 1 ns must prove no LSE receipt/candidate is created.

Required: add those populations and byte-exact V11 rollback/restart evidence.

### `LSE-A-006` — Medium — lifecycle metadata is inconsistent

LSE `last_reviewed` remains 2026-08-19 although Version 6 is dated 2026-08-20.
Vegetation is Version 25 without a Version 25 change-log row. The transaction
text says “reviewed LSE” while all amendments and disposition remain pending.

Required: correct dates/change logs and say prospective/in-review until dual
review, disposition and dual verification pass.

## Protected boundaries

PASS as written: V10 configuration/behavior, selectors/defaults, coupled-time
V2 bytes and DirectV10 restart V1 bytes remain protected. Corrections must keep
exact hash evidence for them.

## Release condition

`HOLD / AUTHORITY_EVIDENCE_AND_WIRE_IDENTITY_INCOMPLETE`. Close A-001 through
A-006, rerun executable receipt/schema/vector and actual-stack populations,
then obtain two independent verifications.

## Independent rerun after authority corrections

Rerun disposition: **HOLD**.

- `LSE-A-001`: **partially closed**. Parent, segment, slab ID and absolute
  support fields now exist. Slab ordinal is still absent despite V11 claiming
  it is bound; `support_start_ns` admits noncanonical leading-zero text; and no
  executable validator proves exact joins or same-duration substitution.
- `LSE-A-002`: **open**. The reference remains a six-assert integer threshold/
  subtraction script. It never constructs the schema object, derives or checks
  `receipt_sha256`, reconstructs binary64 duration bits, or exercises model,
  configuration, state, tolerance and numerical-policy identities. The new
  JSON poison rows are declarations, not executed evidence.
- `LSE-A-003`: **open**. The new fixture profile lists seven labels but contains
  no configuration/state/forcing/policy digests or per-fixture results. The
  sweep file still contains results only for `v10_actual covered forest`, yet
  claims coverage across the entire declared profile. Either execute and bind
  every listed fixture or narrow the policy to that one frozen fixture.
- `LSE-A-004`: **open**. Capacity rows remain unbound to fixture/source operands,
  and the evidence still calls the result an independent storage-lattice floor
  for the broader coupled system rather than the narrower temperature-storage
  observation.
- `LSE-A-005`: **open**. Rollback/fresh-restore and identity mutations were added
  to JSON, but the oracle does not consume that file or reconstruct owner/
  checkpoint bytes. Structural 1 ns still does not executable-prove absence of
  an LSE receipt/candidate.
- `LSE-A-006`: **open**. LSE `last_reviewed` is still 2026-08-19, Vegetation
  Version 25 still lacks a Version 25 change-log row, and transaction Version
  14 still says “reviewed LSE” while disposition remains pending.

The corrections improve the prospective wire shape but do not yet constitute
executable contract evidence. Review A cannot authorize promotion.

## Third independent rerun

Executed `lse_support_admissibility_reference.py`: reported `12/12`. Independently
validated the baseline against the JSON Schema: PASS. Lifecycle date and the
Version 25 change-log row are corrected. Verdict remains **HOLD** for these
release-blocking residuals:

1. The receipt validator authenticates internal self-consistency but accepts no
   expected parent/segment/slab/ordinal/support context. Every identity poison
   merely changes a field without recomputing the digest, so it proves digest
   tamper detection—not rejection of a coordinated, digest-valid receipt from
   another slab. `LSE-A-001` therefore remains open until validation joins an
   independently supplied expected slab domain and includes rehashed
   cross-domain poisons.
2. Receipt/duration/digest execution materially closes `LSE-A-002`, but the
   oracle does not enforce lowercase-hex syntax itself and does not consume the
   vector file. This is secondary to item 1.
3. `lse-support-fixture-profile.json` still provides labels only. The only raw
   sweep remains explicitly `v10_actual covered forest`; no open-mineral,
   separate litter, wet, or dry fixture has identity-bound results. The file's
   final statement that the decision is fail-closed “across the declared
   fixture profile” is unsupported. `LSE-A-003` remains open: narrow policy to
   the swept fixture or run/bind each declared fixture.
4. No rollback snapshot artifact is present in the package tree. Rollback and
   fresh restore remain two declarative vector rows; the oracle does not load
   them or compare staged/committed owner/checkpoint bytes. `LSE-A-005` remains
   open.
5. Transaction Version 14 still says “reviewed LSE” before disposition and
   verification. This residual portion of `LSE-A-006` remains open.

`LSE-A-004` is acceptable only as the narrowed temperature-storage
representation observation; it cannot expand the empirical fixture domain.

## Fourth independent rerun after oracle hardening

Executed `lse_support_admissibility_reference.py`: **15/15**. Independently
validated the baseline against the Draft 2020-12 JSON Schema: **PASS**. The
expected-domain join now rejects digest-valid coordinated reframing of parent,
segment, slab, ordinal, support, configuration, and beginning-state identity;
the canonical digest preimage, decimal encoding, model/policy/minimum guards,
and checkpoint rollback/roundtrip assertions close the prior executable-wire
and rollback findings.

Verdict remains **HOLD** on one exact evidence contradiction:

- `lse-support-fixture-profile.json` declares seven fixtures but records six as
  `pre-admission profile`; only `v11_actual_stack` is marked executed. The raw
  sweep artifact likewise states `Fixture: v10_actual covered forest` while
  concluding that the 600 ms boundary is fail-closed across the declared
  fixture profile. `SC-LANDSURFACEENERGY-001@6` expressly applies the boundary
  across covered forest, open mineral, litter, wet/dry surface, V10 full
  support, and V11 actual-stack fixtures. Loading seven labels and checking
  their status strings does not supply identity-bound results for the six
  unswept populations. Either narrow the admitted policy/contract to the one
  executed frozen fixture, or execute and bind the required populations with
  fixture configuration/state/forcing/policy identities and per-fixture
  outcomes.

No other Review A blocker remains after this rerun.

## Fifth independent rerun after scope narrowing

Executed the reference oracle: **15/15**. Independently validated the baseline
against the Draft 2020-12 JSON Schema: **PASS**. The contract and fixture
profile now substantially narrow admission to the executed covered-forest V11
actual stack and mark open-mineral, litter, wet/dry, and V10 compatibility as
non-admitted/protected.

Verdict remains **HOLD** because two result-bearing artifacts still assert the
superseded broad conclusion:

- `lse-support-sweep-results.txt` concludes that the boundary is fail-closed
  “across the declared fixture profile,” although its own header records only
  the covered-forest fixture.
- `lse-support-domain-evidence.md` still says the 0.6 s policy provides a
  solver-domain margin for the declared covered/open/litter and wet/dry
  fixture family.

Those statements directly conflict with the narrowed contract and profile.
Replace them with the exact admitted V11 actual covered-forest scope and state
that all other fixture families require a separate authority cycle. With that
artifact reconciliation, Review A has no remaining substantive blocker.

## Sixth independent rerun after artifact reconciliation

Executed the reference oracle: **15/15**. Independently validated the baseline
against the Draft 2020-12 JSON Schema: **PASS**. The sweep, domain evidence,
fixture profile, and contract now consistently limit admission to the executed
V11 actual covered-forest adopter. The prior fixture-scope blocker is closed.

Verdict: **HOLD** on one remaining evidence defect. The claimed independent
uninterrupted-versus-restored suffix comparison is tautological:

```python
uninterrupted = sha256(before + suffix)
restored_suffix = sha256(json.dumps(json.loads(before)) + suffix)
```

Both sides consume the same serialized checkpoint bytes; neither independently
constructs or advances a chronology from frozen parent-beginning owners and
operations. Therefore the equality proves JSON roundtrip stability only, not
fresh-restore continuation equivalence or detection of a validly serialized
but semantically wrong staged checkpoint. In addition, the vector
`rollback_snapshot.accepted_receipts` is empty while the oracle checkpoint
contains the baseline receipt, so the purported frozen checkpoint population
is not the population actually executed.

Close by constructing the uninterrupted result independently from frozen
beginning owners plus admitted prefix/suffix operations, restoring the
checkpoint through a separate validator, advancing the same suffix, and
comparing complete final owner/receipt/event/scheduled/reduction/publication/
outbox bytes. The oracle must load or exactly match the frozen vector snapshot.

## Seventh independent rerun — terminal disposition

Executed the reference oracle: **15/15**. Independently validated the baseline
against the Draft 2020-12 JSON Schema: **PASS**. The frozen checkpoint now has
an accepted 600 ms prefix, seven complete beginning/staged owner envelopes,
the baseline accepted receipt, controller identity, and explicit buffered
chronology. The uninterrupted path advances independently from frozen
beginning owners through prefix and suffix; the restored path advances the
persisted staged owners through the suffix; complete final owner bytes/digests
compare exactly. The rollback vector and executed chronology are aligned.

Receipt identity, digest-valid reframing, fixed 600 ms admission, one-tick-
below rejection, rollback, representation analysis, protected wires, narrowed
covered-forest population, and lifecycle posture satisfy Review A.

**Verdict: PASS.** No Review A finding remains open. Promotion still requires
finding disposition, the second review, and both independent verifications.
