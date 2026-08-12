# Review Agent B Second Rereview: Coupled State and Resource Transaction

Status: `PASS / no unresolved material finding`

Evidence mode: `Static + Ran`

Review scope: second independent full repeat review of the exact current
worktree bytes for all original B findings, `B-REREVIEW-CRITICAL-001`, and the
complete shared-owner/resource/schema/C/N/oracle envelope. Historical review
artifacts were preserved unchanged.

## Finding Reassessment

| Finding | Assessment | Current evidence |
|---|---|---|
| `B-CRITICAL-001` | `REMEDIATED` | `SC-VEGETATIONTRANSACTION-001@1` binds exact `(tau,s,t,layer,resource,amount_basis)` identity, stand-ground requests, one-time tile conversion, proportional same-layer arbitration, finalized-use-only debit, independent energy reconstruction, and all-owner atomicity ([SC-VEGETATIONTRANSACTION-001.md:44](../../../specifications/science-contracts/contracts/SC-VEGETATIONTRANSACTION-001.md#L44), [SC-VEGETATIONTRANSACTION-001.md:60](../../../specifications/science-contracts/contracts/SC-VEGETATIONTRANSACTION-001.md#L60)). `SC-VEGETATION-001@6` consumes it through `REF-VEGETATION-030` and `INV-VEGETATION-077` ([SC-VEGETATION-001.md:127](../../../specifications/science-contracts/contracts/SC-VEGETATION-001.md#L127), [SC-VEGETATION-001.md:721](../../../specifications/science-contracts/contracts/SC-VEGETATION-001.md#L721)). BGC retains independent `(layer,species)` NH4/NO3 arbitration authority. |
| `B-CRITICAL-002` | `REMEDIATED` | The independent calculator now executes all 31 named Stage-A positive/poison checks and publishes operands/results, not labels alone ([reference_calculator.py:150](reference_calculator.py#L150), [reference_calculator.py:333](reference_calculator.py#L333)). The Rust authority test checks the exact inventory and independently reconstructs local/stand closure, water conversion/arbitration, exact finalized N keys, rollback/state digests, nonlinear alternatives, and shared-C/N single-transition separation ([vegetation_boundary_authority_contract.rs:722](../../../../tests/integration/vegetation_boundary_authority_contract.rs#L722), [vegetation_boundary_authority_contract.rs:771](../../../../tests/integration/vegetation_boundary_authority_contract.rs#L771), [vegetation_boundary_authority_contract.rs:801](../../../../tests/integration/vegetation_boundary_authority_contract.rs#L801), [vegetation_boundary_authority_contract.rs:852](../../../../tests/integration/vegetation_boundary_authority_contract.rs#L852)). |
| `B-HIGH-003` | `REMEDIATED` | The canonical occupancy schema enumerates every warm-start field, finite domain, root-layer cardinality/order, previous-transaction rule, recursive lexical serialization, state-digest inclusion, and `mm H2O` potential basis. Migration requires caller-supplied complete numerical lanes and never copies/broadcasts V1 warm starts ([SC-VEGETATION-001.md:601](../../../specifications/science-contracts/contracts/SC-VEGETATION-001.md#L601), [SC-VEGETATION-001.md:620](../../../specifications/science-contracts/contracts/SC-VEGETATION-001.md#L620)). |
| `B-HIGH-004` | `REMEDIATED` | Shared C/N advances once after final capped columns using exact weighted GPP/leaf-respiration and finalized transpiration. Shared maintenance, turnover, allocation, and growth respiration execute once; mineral N remains stratum/layer/species keyed after occupancy aggregation ([SC-VEGETATION-001.md:627](../../../specifications/science-contracts/contracts/SC-VEGETATION-001.md#L627)). The oracle executes distinct layer/species supply groups, proportional authorization, finalized-use-only inventory debit, and wrong-layer/species rejection ([reference_calculator.py:259](reference_calculator.py#L259)). |
| `B-HIGH-005` | `REMEDIATED` | Both canonical V2 copies are byte-identical at SHA-256 `b2b01f965f83a52f4c800c489079c88d97179ed6a8191734b541115308b97a5c`. The file is recursively lexicographically serialized with shortest Python round-trip numbers, normatively imports immutable V1 digest `003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157`, and binds exact current section/full-contract digests ([openwepp_c3_woody_v2_definition.json:1](openwepp_c3_woody_v2_definition.json#L1)). |
| `B-REREVIEW-CRITICAL-001` | `REMEDIATED` | The same lower stratum now occurs beneath distinct upper states in both tiles and receives different same-column incident liquid ([reference_calculator.py:152](reference_calculator.py#L152), [reference_calculator.py:337](reference_calculator.py#L337)). Typed potential requests undergo proportional same-layer arbitration; the fixed authorization fraction drives the controlled final-pass response and the descendant consumes the recomputed final release ([reference_calculator.py:206](reference_calculator.py#L206), [reference_calculator.py:213](reference_calculator.py#L213), [reference_calculator.py:408](reference_calculator.py#L408)). Failure injection mutates isolated candidate vegetation warm starts/shared C/N, water, BGC, energy, and transaction state, then proves beginning-owner bytes unchanged ([reference_calculator.py:88](reference_calculator.py#L88), [reference_calculator.py:225](reference_calculator.py#L225)). Routing and water/N swaps execute typed rejection; wet-state and FvCB locality poisons execute nonlinear alternatives ([reference_calculator.py:281](reference_calculator.py#L281), [reference_calculator.py:295](reference_calculator.py#L295), [reference_calculator.py:303](reference_calculator.py#L303), [reference_calculator.py:313](reference_calculator.py#L313)). |

## Full-Envelope Assessment

No material defect remains in the reviewed Stage-A authority envelope:

- occupancy/shared-state ownership is singular and deterministic;
- water identity survives request, authorization, final use, owner debit, and
  receipt on the exact layer and stand-ground amount basis;
- hydrology arbitration is same-snapshot, equal-status proportional, and
  mutation-free until validated final use;
- NH4 and NO3 remain distinct within exact layers;
- potential and final columns are separate top-to-bottom passes, with final
  descendant routing rebuilt from beginning state under immutable caps;
- shared C/N is aggregated after local nonlinear solves and advanced once;
- warm starts and migration are typed, unit-bound, serialized, digest-bound,
  and rollback-safe;
- local occupancy, tile-column, and weighted stand liquid closure are
  independently reconstructible;
- V2 has an immutable canonical identity with an explicit normative V1 import;
  and
- lifecycle posture remains truthful: both proposed contracts are
  `in_review/draft`, final disposition is in progress, and Stage B is not yet
  released.

The controlled cap-response function in the Stage-A oracle is explicitly a
schedule/causality fixture, not substitute E11--E15 constitutive authority. The
V2 definition normatively retains the complete V1 coupled equations; Stage B
must conform its Rust solver to those exact equations and the admitted schedule.

## Ran Evidence

- independent Python oracle regeneration: PASS, byte-identical fixture,
  SHA-256 `e487413142c463a81a4e29d4887cdf4fa339eadeaeeda0a4cf92ffbf2ceb76a7`;
- canonical V2 recursive lexical serialization check: PASS;
- both V2 definition copies: byte-identical;
- transaction-contract digest binding: PASS,
  `bbe498113e3130825b03e0e0a0a6134fa708c37326a3663f994dc44e3422f725`;
- `cargo nextest run --test vegetation_boundary_authority_contract --profile quick`:
  PASS, `14/14`;
- `cargo clippy --test vegetation_boundary_authority_contract -- -D warnings`:
  PASS;
- unit-compliance checks for `SC-VEGETATION-001` and
  `SC-VEGETATIONTRANSACTION-001`: PASS;
- authority-suite anti-evasion: PASS;
- `auth11_required_suite_obligation_guards_contract`: PASS, `3/3`;
- `cargo fmt --all -- --check`: PASS;
- `git diff --check`: PASS.

## Final Verdict

`PASS`

No unresolved material science, ownership, resource-identity, schema,
migration, C/N, oracle-independence, digest, or lifecycle finding remains in
this review scope. Stage A may proceed to the other independent rereview and,
only after both reviews pass the same stable bytes, its heavy and terminal
gates.
