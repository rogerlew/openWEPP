# Review Agent B Rereview: Coupled State and Resource Transaction

Status: `HOLD / one critical evidence finding remains`

Evidence mode: `Static + Ran`

Review scope: independent repeat review of the exact current worktree bytes for
the full coupled owner/resource/schema/C/N/oracle envelope. The historical
`review_agent_b.md` was treated as immutable evidence. This rereview assessed
every original B finding, not only the remediation diff.

## Original Finding Reassessment

| Original finding | Current assessment | Evidence |
|---|---|---|
| `B-CRITICAL-001` | `REMEDIATED` | `SC-VEGETATIONTRANSACTION-001@1` now canonically binds the exact occupancy/layer/resource/amount-basis key, same-layer arbitration, one-time `f_t` conversions, finalized-use-only debit, independent energy reconstruction, and all-owner atomicity ([SC-VEGETATIONTRANSACTION-001.md:44](../../../specifications/science-contracts/contracts/SC-VEGETATIONTRANSACTION-001.md#L44), [SC-VEGETATIONTRANSACTION-001.md:60](../../../specifications/science-contracts/contracts/SC-VEGETATIONTRANSACTION-001.md#L60)). `SC-VEGETATION-001@6` consumes it as `REF-VEGETATION-030` and in `INV-VEGETATION-077` ([SC-VEGETATION-001.md:127](../../../specifications/science-contracts/contracts/SC-VEGETATION-001.md#L127), [SC-VEGETATION-001.md:718](../../../specifications/science-contracts/contracts/SC-VEGETATION-001.md#L718)). `SC-BIOGEOCHEM-001` separately retains exact layer/species NH4/NO3 arbitration authority. |
| `B-CRITICAL-002` | `NOT REMEDIATED` | The fixture now exposes 31 named booleans, but several required vectors/poisons remain tautological or do not instantiate the scenario they claim. See `B-REREVIEW-CRITICAL-001`. |
| `B-HIGH-003` | `REMEDIATED` | The canonical amendment and V2 definition enumerate every occupancy scalar/vector, domain, root-layer cardinality/order, transaction identity rule, serialization participation, and migration requirement ([SC-VEGETATION-001.md:601](../../../specifications/science-contracts/contracts/SC-VEGETATION-001.md#L601), [SC-VEGETATION-001.md:617](../../../specifications/science-contracts/contracts/SC-VEGETATION-001.md#L617)). Migration requires caller-supplied complete warm starts and does not copy or broadcast V1 numerical state. |
| `B-HIGH-004` | `REMEDIATED` | V2 now binds final-capped occupancy aggregation for GPP, leaf respiration, and transpiration, then runs shared tissue maintenance, turnover, allocation, and growth respiration once. Mineral N is explicitly stratum/layer/species keyed after aggregation and retains NH4/NO3 identity ([SC-VEGETATION-001.md:624](../../../specifications/science-contracts/contracts/SC-VEGETATION-001.md#L624)). |
| `B-HIGH-005` | `REMEDIATED` | The V2 JSON is byte-identical to Python `json.dumps(..., sort_keys=True, separators=(",", ":")) + "\n"`; both copies hash to `e62d448b045db1577fe9367b5b531fcd7b1cfc9b544800c11c4ed305d14da10a`. It normatively imports immutable V1 digest `003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157` with explicit merge semantics and binds current contract-section/full-contract digests ([openwepp_c3_woody_v2_definition.json:1](openwepp_c3_woody_v2_definition.json#L1)). |

## Remaining Finding

### B-REREVIEW-CRITICAL-001: The claimed complete independent oracle envelope is still materially tautological

The Stage-A directive requires real independent vectors for the same stratum
beneath different upper columns, authorization-sensitive final column rebuild,
tile-local failure rollback, shared C/N and mineral-N identity, plus poisons that
demonstrate the rejected route or identity fails
([c3-hold-lift-guidance.md:553](../../20260811-coupled-c3-forest-vegetation-state-machine-implementation-001/artifacts/c3-hold-lift-guidance.md#L553)). The current calculator instead includes the following non-evidence:

- `heterogeneous_upper_columns` tests only that two columns have different
  occupancy counts; the lower stratum occurs in `tile-a` only, so no same
  stratum is placed beneath different upper-canopy columns
  ([reference_calculator.py:88](reference_calculator.py#L88),
  [reference_calculator.py:163](reference_calculator.py#L163));
- `capped_upper_changes_lower` manually changes the upper occupancy's `vapor`
  from evaporation to condensation. It performs no potential water request,
  hydrology arbitration, authorization conversion, or authorization-capped
  coupled re-solve, so it does not test the newly admitted two-pass algorithm
  ([reference_calculator.py:137](reference_calculator.py#L137),
  [reference_calculator.py:175](reference_calculator.py#L175));
- `rollback_after_tile_failure` now mutates and discards a candidate, but its
  beginning object contains only generic lane stores and one transaction scalar.
  It does not serialize the admitted occupancy warm-start schema or water, BGC,
  and energy owner states, inject failures across the owner transaction, or
  demonstrate that a leaked alias cannot mutate beginning state
  ([reference_calculator.py:86](reference_calculator.py#L86),
  [reference_calculator.py:167](reference_calculator.py#L167));
- `mineral_n_after_aggregation` now distinguishes three keyed request literals
  from a collapsed dictionary, but performs no same-snapshot request
  aggregation, supply-limited proportional arbitration, finalized-use debit, or
  wrong-species/wrong-layer receipt rejection
  ([reference_calculator.py:176](reference_calculator.py#L176),
  [reference_calculator.py:225](reference_calculator.py#L225));
- `wrong_authorization_poison` now reads two keyed values, but only asserts that
  the unequal values differ. It never applies one occupancy's authorization to
  the other occupancy or executes the required identity rejection
  ([reference_calculator.py:184](reference_calculator.py#L184),
  [reference_calculator.py:220](reference_calculator.py#L220));
- several routing poisons reduce to positivity or inequality with an unrelated
  scalar. The positive stemflow-bypass identity itself is now explicit, but its
  corresponding poison remains algebraically
  `incident + positive_stemflow != incident`; wrong-tile drainage compares a
  lower incident amount with another tile's ground total rather than executing
  the prohibited route
  ([reference_calculator.py:193](reference_calculator.py#L193),
  [reference_calculator.py:213](reference_calculator.py#L213)).

The Rust authority test now correctly asserts the exact inventory of 31 names,
all-true values, fixture digest, and independent stand/water arithmetic
([vegetation_boundary_authority_contract.rs:722](../../../../tests/integration/vegetation_boundary_authority_contract.rs#L722)).
That strengthening detects missing labels and fixture drift, but cannot turn a
tautological producer boolean into an independent scientific vector. The
artifact claim that all 31 are executable and bind rollback, mineral-N identity,
and authorization-sensitive routing is therefore materially overstated
([test-vector-ledger.md:13](test-vector-ledger.md#L13)).

Disposition recommendation: `ACCEPT`. Replace the predicates above with modeled
positive and negative cases:

1. Put the same lower stratum in at least two tiles under distinct upper-column
   structures and compare its independently calculated incident liquid/state.
2. Implement an abstract potential-request/arbitration/fixed-cap final rebuild
   in the independent calculator; change the upper solution through a cap, not
   an unrelated prescribed vapor value, and prove the descendant consumes the
   final release.
3. Execute a phase-injected candidate transaction over serialized vegetation,
   water, BGC, and energy owner states and compare exact pre/post bytes.
4. Implement layer/species mineral-N request aggregation and proportional
   arbitration, then execute wrong-layer and wrong-species poisons.
5. Execute each rejected routing/area/authorization alternative and compare its
   independently reconstructed result or typed rejection with the accepted
   result.

Regenerate the fixture and digest, update its evidence ledger, rerun focused
gates, and repeat this reviewer after stable bytes.

## Full-Envelope Assessment

No additional material authority defect was found in the current occupancy
ownership, one-time area conversions, shared C/N ordering, field-level warm
starts, migration, normative V1 import, V2 serialization/digests, or lifecycle
posture. The proposed contracts and index correctly remain `in_review/draft`;
the final disposition remains in progress and Stage B remains prohibited.

Ran evidence on the reviewed bytes:

- independent Python oracle output was byte-identical to the committed fixture;
- both V2 definition copies were byte-identical;
- the transaction-contract SHA-256 matched the V2 binding
  `bbe498113e3130825b03e0e0a0a6134fa708c37326a3663f994dc44e3422f725`;
- canonical V2 JSON serialization comparison: PASS;
- `cargo nextest run --test vegetation_boundary_authority_contract --profile quick`:
  PASS, `14/14`;
- unit-compliance checks for `SC-VEGETATION-001` and
  `SC-VEGETATIONTRANSACTION-001`: PASS;
- `git diff --check`: PASS.

These successful gates verify current bytes and structural bindings; they do not
close the scientifically ineffective oracle cases identified above.

## Final Verdict

`HOLD`

Do not promote the V2 contracts, dispatch heavy gates, release Stage-A
implementation authority, or resume Stage B until
`B-REREVIEW-CRITICAL-001` is corrected and both independent science rereviews
pass the same stable bytes.
