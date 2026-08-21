# Child 2C Science Review B — independent contract/governance review

Evidence mode: `Static + Ran`

Review scope was limited to the current working tree, the five amended
contracts, their receipt schemas, vectors, reference model, package evidence,
and the new contract test. I did not use another review artifact.

## Disposition

**HOLD**

The focused vector/oracle test passes, but the package does not yet close the
canonical schema, independent conservation/anti-tautology, typed-failure,
restart/rollback custody, profile, or terminal-gate obligations.

## Findings

### BLOCKER — Required review, verification, and terminal custody gates are
not complete

The evidence was updated to `executed / review pending`, but
`artifacts/gate-results.md:3-15` still records Science review A/B and
Verification A/B as `NOT RUN` and exact diff/docs integrity as `IN PROGRESS`.
`artifacts/pre-implementation-contract-gate.md:3-16` remains `HOLD / review
gate pending`; `artifacts/worker-handoff.md:3-10` remains queued/not-run;
`artifacts/exact-diff-reconciliation.md:3-9` remains queued/not-run; and
`artifacts/owned-file-manifest.md:3-18` remains queued. The package itself is
still queued with progress incomplete (`package.md:3,270-282`). This cannot
be a complete disposition under the Gate Evidence Non-Deferral Rule.

Proposed disposition: complete the two independent reviews and verifications,
disposition every finding, reconcile the exact source/write set and protected
Child 2B identity, and publish the bounded handoff only after those artifacts
are current.

### BLOCKER — Receipt schemas do not close the canonical receipt authority

The contract claims are materially richer than the package schemas:

- `SC-VEGETATIONTRANSACTION-001:497-516` requires trial state, typed support
  and exposure joins, flux lineage, reciprocal longwave, independent ledgers,
  owner candidates, and complete-owner custody. The carrier schema only has
  opaque IDs and two generic `turbulent_flux_ids`
  (`artifacts/carrier-receipt-schema.json:6-34`); it has no typed flux mapping,
  ledger operands, owner map, support-policy content, or conditional
  requirement for `accepted_event_receipt_id`.
- `SC-COUPLEDTIME-001:749-758` requires candidate/tolerance/error and retry
  evidence. The event schema has no tolerance-policy values or digest, no
  per-candidate error/ledger records, no event ordinal or mutation set, and
  only constrains candidate ticks to be unique rather than canonically sorted
  (`artifacts/event-boundary-receipt-schema.json:6-48`). It cannot validate
  the two neighboring predicates, candidate digest semantics, or tie-break
  correctness.
- Neither schema is referenced by the canonical contracts or exercised by
  `tests/integration/snow_stage3_shared_carrier_authority_contract.rs`; that
  test never reads either schema (`:42-134`). The gate evidence's statement
  that schemas merely parse (`artifacts/gate-results.md:10`) is not schema
  conformance or receipt-closure evidence.

Proposed disposition: define the closed canonical receipt/linked-ledger
schemas and identity preimages, add conditional/event-receipt joins and
complete-owner fields, and add valid/invalid JSON-schema fixtures plus direct
schema validation to the contract test.

### BLOCKER — The oracle remains tautological and does not prove physical
closure

The positive carrier vector still has only arbitrary temperatures, humidity,
conductances, and atmospheric longwave (`carrier-boundary-vectors.json:14-36`)
and no duration, SWE/liquid/cold-content state, latent-heat conversion, snow
mass availability, or integrated snow/energy ledger. The reference model
recomputes its residual directly from the same flux values it emits
(`reference_model.py:27-64`), so the zero residual is self-consistency, not an
independent reconstruction. It still tests one canopy surface, not all V11
canopy surfaces/areas and the reciprocal canopy-side longwave ledger required
by `SC-SNOWENERGY-001:1263-1279`.

The newly added conservation case does not cure this: `conservation()` derives
each result directly from the same input operands (`reference_model.py:109-140`)
and the declared `diagnostic_melt_alias_kg_m2` is never read
(`carrier-boundary-vectors.json:197-224`). Event selection likewise accepts
caller-supplied `candidate_errors` and never recomputes a terminal snow state
or event ledger (`reference_model.py:77-106`). This fails the independent
reconstruction obligations in `SC-SNOWENERGY-001:1281-1309`,
`SC-VEGETATIONTRANSACTION-001:513-516`, and the package anti-tautology gate.

Proposed disposition: author a result-independent oracle with separately
constructed mass, liquid, vapor, sensible/latent, longwave, cold-content, and
event-time ledgers; consume and reject the diagnostic alias; recompute each
candidate from immutable state; and include multi-surface V11/canopy-side
longwave lineage.

### BLOCKER — Restart, rollback, and complete-owner custody are represented by
sentinel echo, not evidence

Rejected carrier/event cases return the input `before_sha256` unchanged
(`reference_model.py:17-25,68-95`), and the Rust test merely compares that
echo to the vector (`snow_stage3_shared_carrier_authority_contract.rs:88-90`).
No owner state is staged, failed, and rolled back; no seven-owner candidate is
joined; and no restart before/after-event case exists. The one-nanosecond case
has empty participant-support sets and only reports
`positive_physical_successor: false` (`carrier-boundary-vectors.json:139-154`),
so it does not exercise a below-domain physical owner rejection.

This leaves unproven the explicit restart/rollback obligations in
`SC-COUPLEDTIME-001:257-266,290-306`, the complete-owner transaction in
`SC-VEGETATIONTRANSACTION-001:490-516`, and the required restart populations
in `SC-COUPLEDTIME-001:760-766`.

Proposed disposition: replace sentinels with structured immutable beginning
snapshots and staged candidate maps, force failures after staged mutation,
assert byte/digest identity for every owner and receipt, and add restart
before/after event plus fresh-restore equivalence vectors.

### HIGH — New typed failures are not integrated into error contracts or
precedence

`ERR-CT-021` appears only in the Child 2C addendum
(`SC-COUPLEDTIME-001:723-747`), not in the canonical branch/error table or
precedence `ERR-CT-001..020` (`:470-511`). It is overloaded for invalid support
receipts, candidate rejection, and no-candidate retry, although those paths
have different retryability and caller obligations. Likewise, `LSEB-E-043`,
`SNOWENERGY-E-CARRIER-001`, `SNOWENERGY-E-LW-001`,
`SNOWENERGY-E-WIND-001`, `SNOWENERGY-E-REGIME-001`,
`SNOWENERGY-E-SCOPE-001`, `VEG-E-129..132`, and `VEGTXN-E-015..018` are named
in addendum tables but have no canonical typed-failure definitions/precedence
rows (`SC-LANDSURFACEENERGY-001:954-968`, `SC-SNOWENERGY-001:1296-1302`,
`SC-VEGETATION-001:2847-2852`, and `SC-VEGETATIONTRANSACTION-001:506-516`).

Proposed disposition: add each error to the owning branch table with stable
meaning, validation phase, retryability, rollback posture, and precedence;
separate unrecoverable receipt-invalid errors from recoverable no-candidate
retry; add poison vectors for every new variant.

### HIGH — The amendments do not fully satisfy the kernel contract profile

Child 2C symbols are introduced only in appendices. `dt_min_pre`/
`dt_min_post` and event-error units do not appear in the coupled-time variables
table (`SC-COUPLEDTIME-001:70-89`); `H_i`, `V_i`, `R_T`, `R_q`, `T_ca`, `q_ca`,
`g_H`, and `g_q` do not appear in the snow variables/units table
(`SC-SNOWENERGY-001:161-203`). The new support/error/receipt symbols are not
closed in the corresponding alias and unit-governance maps. This conflicts
with the profile's required canonical variables, algorithm intermediates,
unit map, and guard map, even though the checklist's checked status remains
conditional on review (`artifacts/kernel-profile-compliance-checklist.md:3-13`).

There is also lifecycle/evidence drift inside four amended contracts:
frontmatter says `in_review`/`draft`, while body status remains
`approved`/`active` in `SC-COUPLEDTIME-001:4-24`,
`SC-LANDSURFACEENERGY-001:4-23`, `SC-VEGETATION-001:4-25`, and
`SC-VEGETATIONTRANSACTION-001:4-24`. Snow body evidence remains `static`
while its frontmatter advertises `static+independent_oracle+contract_vectors`
(`SC-SNOWENERGY-001:4-26`).

Proposed disposition: integrate Child 2C variables, domains, aliases,
constants, tolerances, branch rows, and guard/error mappings into the
canonical required sections, then synchronize frontmatter, body, registry,
and evidence labels.

### HIGH — Binding Exposure Index closure is incomplete and the lint pass is
overstated

The current Child 2C package source is mapped in the LSE index
(`SC-LANDSURFACEENERGY-001:386-390`) but not in the BIEs for coupled time,
snow energy, vegetation, or vegetation transaction
(`SC-COUPLEDTIME-001:666-672`, `SC-SNOWENERGY-001:1084-1103`,
`SC-VEGETATION-001:1438-1457`, `SC-VEGETATIONTRANSACTION-001:240-247`).
Those four contracts therefore do not expose the new package-local binding
residue and its `INV`/`OBL` mappings. In addition, the new LSE row uses
`dual review and verification required`, outside the allowed BIE review-gate
vocabulary in `docs/specifications/science-contract-spec.md`.

The reported `PASS` is not sufficient: `tools/check_sc_binding_exposure.py`
does not validate review-gate vocabulary, and its pass therefore misses this
defect. The package evidence's claim of five consolidated passes
(`artifacts/contract-implementation-evidence.md:28-29`) is too strong.

Proposed disposition: add a Child 2C BIE row to each affected contract (or
explicitly document why a direct core amendment is excluded), use only the
canonical gate vocabulary, strengthen the linter, and rerun normal and strict
BIE checks.

### MEDIUM — Operand lineage and calibration/evidence artifacts are not
complete at the claimed status

`artifacts/operand-lineage.md:7-24` omits the required normalization/
denominator and area/volume-basis columns, gives no per-operand evidence path
or digest, and does not provide the independent ledger reconstruction it says
the fixtures will supply. It is marked `complete / pre-implementation` while
the package's exact-diff and manifest artifacts remain queued. The calibration
matrix now says complete/not applicable but contains only the three orthogonal
status fields (`artifacts/calibration-readiness-matrix.md:3-11`), not the
readiness-obligation rows with evidence paths and structure-backed
`NOT_APPLICABLE` rationales required by `science-contract-spec.md` if this is
the package readiness matrix.

Proposed disposition: complete lineage for every carrier/event operand with
normalization and physical basis, source identity, producer/consumer, and
authoritative/diagnostic status; either add all readiness rows as
`NOT_APPLICABLE` with rationale or explicitly scope the matrix out; reconcile
the status labels.

### MEDIUM — Required event-boundary populations and assertions remain
incomplete

The vectors cover unequal supports and one equal-displacement/lower-error
case, but do not contain an actual equal-displacement/equal-error tie proving
the final earlier-tick rule, an out-of-window candidate, distinct event-time
versus displacement error, an exact active-adopter support acceptance, or a
one-tick-below-support physical rejection. The current event model also does
not emit the required receipt fields or candidate digest. The test's generic
accepted-case assertion compares `result["accepted_event_tick"]` with a
missing expected field for carrier/conservation cases, so it compares two JSON
`null` values instead of proving a tick (`snow_stage3_shared_carrier_authority_contract.rs:93`).
This falls short of `SC-COUPLEDTIME-001:760-766` and the package exit
criterion, despite the narrower coverage assertion at
`snow_stage3_shared_carrier_authority_contract.rs:137-174`.

Proposed disposition: add the missing boundary/tie/poison cases and assert
the full receipt, candidate enumeration, support predicates, tolerance
admission, retry policy, and owner outcome.

## Authority-claim boundary

The package explicitly avoids production activation, calibration, efficacy,
seasonal qualification, and cutover (`package.md:144-151`,
`SC-SNOWENERGY-001:1199-1203,1311-1320`). It also correctly retains an
exposure gap rather than authorizing attenuation. However, `exposure_receipt_id`
is only an opaque string in `carrier-receipt-schema.json:6-34`, while
`SC-SNOWENERGY-001` still records deployed/server exposure as unavailable in
its wind-custody invariant. Until the exposure receipt schema/provider
identity is closed, `AUTHORITY_ADMITTED` must be read only as a contract
interface proposal, not exposure or runtime authority.

## Checks run

- `python3 .../reference_model.py`: passed; current evidence reports 13 result
  records (6 accepted, 7 rejected).
- `cargo nextest run --test snow_stage3_shared_carrier_authority_contract`:
  passed; 3 tests.
- Per-contract `check_sc_binding_exposure.py`: passed, but with the linter
  limitation described above.
- `git diff --check`: passed.
- `check_science_contract_admission.sh --base-ref HEAD --worktree`: returned
  non-admitted because the five changed contracts are intentionally
  `in_review`/`draft`; no release admission was inferred.

Final disposition: **HOLD**. No implementation authority, runtime exposure
authority, calibration claim, or production cutover is granted.
