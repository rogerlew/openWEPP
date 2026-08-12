# Review Agent A

Status: `HOLD / material authority and acceptance gaps`

Evidence mode: `Static + Ran`

Review role: independent canopy-interception/topology/energy science reviewer.

Reviewed exact current worktree bytes for the V2 canonical amendment, both V2
definition copies, V1 identity, the independent Python oracle and committed
fixture, authority-suite bindings, adjacent LSE/hydrology ownership contracts,
and the Stage-A package evidence. This review did not inspect or coordinate with
the other science reviewer.

Ran:

- regenerated the topology fixture with the package Python oracle and obtained
  an empty diff against the committed JSON;
- verified both V2 definition copies have SHA-256
  `fc5fa57bf84d3e409f9663c3b5beebee0cdfd87ec1be1dc224efe030749bfbc8`;
- verified the historical V1 definition remains
  `003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157`;
- ran the vegetation authority suite: PASS `14/14`;
- ran science-contract admission: PASS,
  `A0_ADMITTED contracts=44 science_surfaces=0`;
- ran `SC-VEGETATION-001` unit compliance: PASS; and
- ran `git diff --check`: PASS.

The selected occupancy-local state and same-tile routing direction is
scientifically suitable. The findings below concern missing execution authority
and evidence needed to make that selection implementable without inference.

## Findings

### A-CRITICAL-001: The final E04 column is not ordered through water authorization and capped re-solve

The amendment requires complete local E04 followed by same-tile routing at
`docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md:553`,
while it separately requires occupancy-local coupled solves and capped water
re-solves at
`docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md:567` and
`docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md:574`.
It never states whether the entire top-to-bottom column is rerun after
authorization, which run supplies accepted second drainage, or how a changed
upper-occupancy vapor solution invalidates and recomputes every descendant
occupancy.

This is load-bearing. Upper wet evaporation or condensation depends on the
coupled canopy-air/leaf/hydraulic solution. Authorization can change that
solution, hence upper second drainage, hence lower incident liquid, wet
fraction, energy state, water request, and final flux. A merely local capped
re-solve can leave a stale lower-canopy candidate; a naive whole-column rerun can
change lower demand after arbitration. Neither behavior is authorized. The
oracle sidesteps the issue by accepting vapor as an externally fixed scalar at
`artifacts/reference_calculator.py:24` and routing its resulting second drainage
immediately at `artifacts/reference_calculator.py:70`.

Scientific impact: the same admitted inputs and authorizations can produce
different accepted stores, ground receipt, energy, and transpiration depending
on an implementation-selected solve schedule. Local and stand closure can still
pass while the causal state is wrong.

Disposition recommendation: `accepted`. Amend the canonical algorithm with an
exact potential-column/request/arbitration/final-column execution rule,
including how any authorization-induced request change is resolved, which
candidate owns final routed liquid, convergence/failure semantics for the
column-level coupling, and a vector in which the upper capped solve changes
second drainage and the lower accepted state. Digest-bind that amendment.

### A-CRITICAL-002: Adjacent owners cannot independently validate occupancy identity or basis

V2 requires occupancy-preserving water identities and occupancy energy closure
in `SC-VEGETATION-001.md:562`--`581`, but the digest-bound adjacent contracts are
unchanged. `SC-LANDSURFACEENERGY-001.md:218`--`219` binds recipient/stratum
lineage, not `(stratum,tile)` occupancy or tile-ground-to-stand conversion.
`SC-WATBAL-001.md:296` still defines hydrology Stage B by `D_s,l`/`U_s,l` on one
area basis, not by occupancy, amount-basis identity, or the required divide-back
operation. Nevertheless, `artifacts/contract-amendment-evidence.md:8`--`17`
claims those unchanged contracts are sufficient, and the V2 definition binds
them as the receiving-owner authority at
`artifacts/openwepp_c3_woody_v2_definition.json:6`--`18`.

Scientific impact: vegetation can assert the correct occupancy locally, but the
water or energy owner cannot independently reject an occupancy swap, omitted or
double `f_t`, or aggregation of two occupancy latent debits under one stratum
identity. That defeats the package's independent-owner and anti-tautology
acceptance claim.

Disposition recommendation: `accepted`. Add bounded, canonical adjacent-owner
invariants/obligations for occupancy identity, local and stand amount basis,
exact conversion direction, independent local/aggregate reconstruction, and
duplicate/swap rejection. Update their versions/digests and add owner-side
poison vectors. If a shared typed resource contract is the repository-authorized
owner instead, bind that exact contract from both domains and prove both owners
consume it; a vegetation-only assertion is insufficient.

### A-HIGH-003: The V2 occupancy state schema is descriptive rather than executable

The contract says every occupancy owns warm starts for leaf, stem, wet,
canopy-air, and hydraulic nodes at `SC-VEGETATION-001.md:549`--`552` and
`SC-VEGETATION-001.md:582`--`586`. The supposedly frozen schema represents all
of them as the single string
`leaf_stem_wet_canopy_air_hydraulic_warm_starts` at
`artifacts/openwepp_c3_woody_v2_definition.json:31`--`36`. Neither that file nor
`artifacts/state-schema-amendment.md:10`--`16` defines field names, units,
sun/shade cardinality, root-layer indexing, finite domains, optional/initial
transaction semantics, canonical serialization order, or digest treatment for
each value.

Scientific impact: Stage B cannot implement exact V2 bytes or strict
missing/duplicate/extra-state rejection from this authority. Different
implementations can persist different node sets and still claim the same V2
digest. Initial state also has no exact representation for an occupancy that has
no prior accepted transaction.

Disposition recommendation: `accepted`. Replace the aggregate label with an
exhaustive typed occupancy-state manifest. Define every scalar/vector and unit,
sun/shade and layer indexing, finite/domain rules, initial/no-prior-transaction
encoding, deterministic key and serialization order, and which values enter the
state digest. Add schema poisons for every missing, duplicate, extra, stale,
wrong-unit, and wrong-cardinality form.

### A-HIGH-004: The independent fixture does not cover most required vectors or poisons

The committed oracle contains one heterogeneous pair of columns, one empty
tile, one scalar water conversion, one nonlinear aggregate-first comparison,
and three migration outputs (`artifacts/reference_calculator.py:82`--`145`). It
does not exercise:

- the same lower stratum beneath two different upper columns;
- single-tile reduction to V1;
- homogeneous two-tile reduction;
- tile-order permutation invariance;
- exact rollback after a tile-local failure;
- replicate-store, wrong-area, wrong-tile drainage, stemflow-through-foliage,
  omitted-second-drainage, missing/double-`f_t`, wrong-occupancy authorization,
  or missing/duplicate-lane rejection;
- averaged wet fraction, aggregated PAR, or another tile-local wet-energy/
  nonlinear physiology poison.

Those are explicit Stage-A obligations in the hold-lift directive and canonical
test table at `SC-VEGETATION-001.md:979`--`988`. The authority test only checks
selected JSON substrings and a fixture digest at
`tests/integration/vegetation_boundary_authority_contract.rs:716`--`738`; it
does not parse and independently reconstruct the local/column/stand operands or
prove any rejected alternative differs. The oracle's `all_pass` value is set
unconditionally at `artifacts/reference_calculator.py:144`.

Scientific impact: the current passing fixture proves one arithmetic example,
not the advertised routing, invariance, migration, rollback, energy, physiology,
or poison envelope. Several wrong implementations would consume the fixture and
pass the current A0 test.

Disposition recommendation: `accepted`. Add all mandated positive and poison
families with deliberately distinct values, expected typed failures, and
independently reconstructed operands. Make the ordinary Rust authority test
parse committed fixture values and assert the required case inventory/digests;
retain a separate regeneration gate that compares exact Python output to the
committed fixture. Wet-energy/physiology authority poisons must be bound in
Stage A even when production execution remains Stage B.

### A-HIGH-005: Version 6 is labeled approved and admitted before its required review cycle

The exact canonical bytes declare `status: approved` and `maturity: active` at
`SC-VEGETATION-001.md:5`--`7`, while the registry says V2 authority is admitted
at `docs/specifications/science-contracts/index.md:70`. The package itself still
records both science reviews, disposition, heavy gates, and verification as
pending (`package.md:28`--`34`; `artifacts/final-disposition.md:3`). The
science-contract authoring procedure permits promotion only after both reviews,
finding disposition, fixes, and both verifications pass.

Governance impact: downstream worktree consumers can read an approved/active V2
contract and passing A0 test even though the package explicitly prohibits Stage
B until review closure. The package placeholder does not undo the canonical
registry claim.

Disposition recommendation: `accepted`. Keep the proposed V2 amendment visibly
in review until the promotion gate passes, or use the repository's reviewed
staging mechanism that preserves V1 as the currently approved version. Promote
the exact final V2 bytes and registry row only after disposition and verification.

## Final Recommendation

`HOLD`

The tile-resolved occupancy selection itself should be retained. Stage A is not
ready to release implementation authority because the authorization-sensitive
column order is still underdetermined, adjacent owners cannot independently
enforce occupancy identity and basis, the persistent schema is not executable,
and the required independent vector/poison envelope is materially incomplete.
All are in-scope authority-package corrections; none should be deferred to
production implementation.
