# Review Agent B: Coupled State and Resource Transaction

Status: `HOLD / material authority and evidence gaps remain`

Evidence mode: `Static + Ran`

Review scope: exact current worktree bytes for occupancy/shared-state ownership,
typed water identity and area conversion, hydrology arbitration, shared C/N,
warm starts and rollback, migration, model identity/digests, local/tile/stand
closure, independent oracle/poison coverage, and canonical admission posture.
This review was performed independently and did not consult the other science
reviewer's artifact.

## Findings

### B-CRITICAL-001: Adjacent resource and energy owners do not admit the V2 occupancy identity they must reconstruct

`SC-VEGETATION-001@6` defines the V2 request as
`(tau,stratum,occupancy,layer,resource,amount_basis)` and requires the
multiply/divide/multiply area conversions
([SC-VEGETATION-001.md:574](../../../specifications/science-contracts/contracts/SC-VEGETATION-001.md#L574)).
The digest-bound hydrology owner still admits only stratum-keyed `U_s,l` and
`D_s,l`, with an aggregate `sum_s` bound
([SC-WATBAL-001.md:296](../../../specifications/science-contracts/contracts/SC-WATBAL-001.md#L296)).
It does not admit occupancy identity, the `D_W/A_W/F_W` distinction, the
tile-to-stand conversion receipts, duplicate occupancy-key rejection, or a
`sum_(s,t)` same-snapshot arbitration bound. Similarly, the digest-bound energy
owner retains only stratum-level latent identity and recipient-level radiation
([SC-LANDSURFACEENERGY-001.md:218](../../../specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L218)); it has no occupancy/tile area identity or local-to-stand reconstruction rule.

The package intended bounded amendments to both owner contracts
([package.md:93](../package.md#L93)), but the amendment evidence instead declares
the unchanged contracts sufficient
([contract-amendment-evidence.md:11](contract-amendment-evidence.md#L11)). That
leaves each shared boundary specified by the producer contract alone and makes
the promised independent two-owner reconstruction underdetermined. The problem
is not that the selected equations are wrong; it is that the accepting owners
do not share the new typed identity and basis.

Disposition recommendation: `ACCEPT`. Before release, amend the applicable
owner contracts (or a canonical shared resource-transaction authority consumed
by both) to bind the exact occupancy key, stand-area amount, proportional
same-layer arbitration over all occupancy requests, one-time conversions,
final-use-only debit, energy operand identity, and independent reconstruction.
Regenerate every affected section/full-contract digest and its tests.

### B-CRITICAL-002: The independent oracle and authority test do not cover the binding Stage-A vector and poison set

The lift directive requires fifteen independent vector families and thirteen
specific poisons, including homogeneous reduction, tile permutation, exact
rollback, wrong routing, wrong area basis, omitted/double `f_t`, authorization
swap, and missing/duplicate occupancy lanes
([c3-hold-lift-guidance.md:553](../../20260811-coupled-c3-forest-vegetation-state-machine-implementation-001/artifacts/c3-hold-lift-guidance.md#L553)).
The oracle has one heterogeneous two-column example, one empty column, a single
scalar water conversion, one aggregate-first poison, and migration literals
([reference_calculator.py:82](reference_calculator.py#L82)). It contains no
executable homogeneous reduction, tile-order permutation, rollback/state
serialization, identity swap, duplicate/missing lane, wrong routing, wrong
area conversion, wet-fraction/PAR preaggregation, or shared-owner arbitration
poison. The test merely searches committed JSON for eight literals and its
digest
([vegetation_boundary_authority_contract.rs:717](../../../../tests/integration/vegetation_boundary_authority_contract.rs#L717)); it neither invokes this oracle nor independently reconstructs the omitted cases.

The statement that remaining coupled-physiology and rollback vectors are Stage-B
obligations
([test-vector-ledger.md:21](test-vector-ledger.md#L21)) conflicts with the
current Stage-A requirement and the package's own completion rule that every
state/resource/migration join and oracle/poison vector pass
([package.md:135](../package.md#L135)). A numerically closing happy-path fixture
cannot reject the prohibited alternatives.

Disposition recommendation: `ACCEPT`. Extend the independent calculator and
committed fixtures to execute every required Stage-A vector and poison, add
explicit expected failures/results rather than `all_pass` supplied by the
producer, and make the authority gate regenerate and compare the committed
fixture byte-for-byte. Retain Stage-B Rust conformance/rollback tests as a
separate implementation obligation.

### B-HIGH-003: Occupancy state and migration are not an exact executable schema

The contract gives each occupancy a liquid store, derived wet fraction, an
undifferentiated collection of warm starts, and a last accepted transaction
identity
([SC-VEGETATION-001.md:549](../../../specifications/science-contracts/contracts/SC-VEGETATION-001.md#L549)).
The V2 definition likewise encodes all leaf/stem/wet/canopy-air/hydraulic warm
starts as one descriptive string rather than typed, cardinality-defined state
([openwepp_c3_woody_v2_definition.json:31](openwepp_c3_woody_v2_definition.json#L31)).
Neither surface defines sun/shade cardinality, root-potential cardinality and
layer identity, canonical ordering/serialization, finite domains, or the
precise relationship between a lane's stored transaction identity and the
incoming transaction.

Migration specifies only liquid: zero expansion, single-tile division, and a
nonzero multi-tile unresolved report
([SC-VEGETATION-001.md:587](../../../specifications/science-contracts/contracts/SC-VEGETATION-001.md#L587)).
It does not say how the newly required warm starts and last-accepted identity
are initialized for either supposedly automatic migration. Consequently a V1
state cannot be transformed into the complete V2 lane schema without an
implementation-chosen reset, copy, broadcast, or caller requirement. That is
especially material because broadcasting accepted occupancy solutions is
explicitly prohibited.

Disposition recommendation: `ACCEPT`. Freeze a field-level typed occupancy
schema, cardinalities, deterministic ordering/serialization, domains and stale
identity rule. Define every field's zero-store and single-tile migration, or
require caller-supplied complete V2 lanes consistently. Add complete-schema,
migration, alternate-warm-start, permutation, and byte-rollback oracle vectors.

### B-HIGH-004: The shared C/N aggregation and mineral-N identity decision remain implicit

The canonical amendment says only that accepted occupancy fluxes are weighted
and update shared stratum C/N once
([SC-VEGETATION-001.md:567](../../../specifications/science-contracts/contracts/SC-VEGETATION-001.md#L567)).
The package addendum similarly names an "exact weighted" input without defining
the operand set or sequence
([equation-authority-addendum.md:10](equation-authority-addendum.md#L10)).
It does not bind the requested explicit identities such as
`GPP_s=sum_t(f_t*GPP_s,t)` and `Rm_s=sum_t(f_t*Rm_s,t)`, identify which
respiration terms are local versus shared, or decide whether mineral-N demand
remains stratum/layer/species-level after aggregation. The lift directive
requires that mineral-N decision to be stated explicitly
([c3-hold-lift-guidance.md:394](../../20260811-coupled-c3-forest-vegetation-state-machine-implementation-001/artifacts/c3-hold-lift-guidance.md#L394)).

Without the exact aggregation boundary, Stage B can legitimately choose
different orderings for water-limited occupancy carbon, maintenance
respiration, N demand, and the single persistent C/N transition. Those choices
need not produce the same state.

Disposition recommendation: `ACCEPT`. Add the complete local-to-stand operand
map and ordering, explicitly select the mineral-N request identity, preserve
NH4/NO3 and layer identity, and add asymmetric-tile C/N/N-demand reconstruction
and preaggregation poisons.

### B-HIGH-005: The V2 definition is not serialized or content-bound as its canonical schema requires

The canonical contract says model-definition JSON uses lexicographically
ordered keys and shortest round-trip numbers
([SC-VEGETATION-001.md:817](../../../specifications/science-contracts/contracts/SC-VEGETATION-001.md#L817)).
The frozen V2 file begins with `model_version`, `supersedes_model_version`, and
`historical_v1_sha256`, so its keys are not lexicographically ordered
([openwepp_c3_woody_v2_definition.json:1](openwepp_c3_woody_v2_definition.json#L1)).
Running `jq -S -c` produces digest
`a4d016cea44578a0e294681e865dbb2f7402e3ebb9c208c5499ee37dbe2c76b7`,
not the recorded
`fc5fa57bf84d3e409f9663c3b5beebee0cdfd87ec1be1dc224efe030749bfbc8`.

The successor also replaces V1's enumerated equation families and complete
fixed-constant map
([openwepp_c3_woody_v1_definition.json:14](../../20260811-coupled-c3-forest-vegetation-model-stack-authority-001/artifacts/openwepp_c3_woody_v1_definition.json#L14))
with the descriptive token `v1_complete_e01_e22_constitutive_stack` and has no
`fixed_constants` object
([openwepp_c3_woody_v2_definition.json:67](openwepp_c3_woody_v2_definition.json#L67)).
Although the historical V1 digest gives useful provenance, the V2 schema does
not state that it imports the exact V1 definition as a normative base, so the
required fixed constants, algorithms, and tolerances are only inferred. This
does not meet the package's claim that V2 bytes themselves freeze the model.

Disposition recommendation: `ACCEPT`. Define an explicit immutable
`base_model_definition_sha256` import with normative merge semantics or repeat
the exact V1 algorithm/fixed-constant/numerical maps, encode the V2 additions,
serialize by the declared canonical algorithm, then regenerate both copies,
all section/model digests, fixtures, tests, and evidence.

## Positive Assessment

The selected tile-resolved occupancy state is scientifically capable of lifting
the original E04 ambiguity. Conditional `LAI/WAI`, same-tile top-to-bottom
routing, stemflow bypass, explicit second drainage, and weighted local/column/
stand liquid closure are mutually consistent in the reviewed happy-path oracle.
The historical V1 file remains byte-identical at
`003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157`.

Ran evidence on the reviewed bytes:

- independent Python oracle regenerated a byte-identical committed fixture;
- `cargo nextest run --test vegetation_boundary_authority_contract --profile quick`:
  PASS, `14/14`;
- `check_sc_unit_compliance.sh` for `SC-VEGETATION-001`: PASS;
- `check_science_contract_admission.sh --base-ref 02631ae92... --worktree`:
  PASS, `A0_ADMITTED contracts=44 science_surfaces=0`;
- `git diff --check`: PASS.

These gates prove syntax, current hashes, and the implemented happy-path
fixture. They do not close the missing cross-owner authority or absent poison
families above.

## Final Verdict

`HOLD`

Do not release `OPENWEPP_C3_WOODY_V2` authority or begin Stage B yet. Accept and
remediate B-CRITICAL-001 through B-HIGH-005, regenerate all invalidated digests
and fixtures, rerun focused gates, and repeat both independent science reviews
against the resulting stable bytes. The hold is an in-package authority/evidence
defect, not a rejection of tile-resolved occupancy as the selected scientific
rule.
