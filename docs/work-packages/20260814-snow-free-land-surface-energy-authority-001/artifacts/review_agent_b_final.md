# Final Hydrology And Ownership Review

Evidence class: `Static + Ran`, independent exact-worktree review.

Verdict: **NO-GO / FAIL**. The canonical ownership text and strict identity
surfaces are materially stronger, but the frozen evidence still does not prove
the complete positive owner transaction, shared-source finalized-use lineage,
routed multi-OFE custody, condensation receipt, or five-owner rollback required
for release.

## Exact Bytes Reviewed

The repository was on local `main` at
`0db1960129ad4f8fc4e292b20574dfe7229d5fe1` with the uncommitted Child-1
worktree. Exact reviewed hashes were:

- `SC-LANDSURFACEENERGY-001.md`:
  `7917d02a66c4ecefa70cf566b1057df9b990deae95a2daef512efa877855f5fc`;
- `SC-VEGETATION-001.md`:
  `1d7ec3699085fdf5d2f29e01b3c1d76b8a2a5ad8ce22340df2e066cb39f1fb1a`;
- `SC-VEGETATIONTRANSACTION-001.md`:
  `44b77e5d9854ce9c4b64214f2a669e1b09e2fdc5bdb6d1391ef25a909f186e73`;
- `SC-WATBAL-001.md`:
  `c30b7c243a36f7fc2aec316c3ba590c8f7629759d36bf1f91b60c0cf0c419188`;
- LSE definition:
  `51280eecaebd02fcde9675fc6bb48f2b3afa9e251be57246762be62cb92e484a`;
- V8 definition:
  `622bc900a08bd4c70e67c09e1fa113a9de24c48afce3b145a494bb76f6dcbe9b`;
- top-level calculator:
  `00197518134ea1d6ce351ac7ff42a3cb1e89888bf349c33de7ea672b103ce9ce`;
- joint core:
  `a5bbad2e80a75864ddd69ecb35b08a11c59949fe7de8a663c6abae9e5ce3a87c`;
- committed vectors:
  `b462d1710ebb991e19ac5936cdda543e1d0a5d8c39cc84afca85c22479c571b7`.

The six reviewed schema hashes were configuration `6499b98c...`, coupled
transaction `e9cea670...`, diagnostics `df454622...`, forcing `f1fb785e...`,
state `91243e40...`, and water protocol `2e5ade75...`.

Ran:

- independent Python regeneration produced `b462d171...` and compared
  byte-identically with the committed fixture;
- `cargo nextest run --test land_surface_energy_balance_authority_contract
  --profile quick` passed 7/7;
- an independent structured audit found no finalized-use collection in the
  shared-layer vector, no upstream runoff/outlet record in the multi-OFE
  vector, zero typed condensation credits in both positive water-protocol
  instances, and only three of eleven failures carrying the complete
  five-owner-plus-envelope rollback map.

The first attempted regeneration used the nonexistent `--output` option and
failed before producing evidence; it was rerun with the calculator's documented
`--write` option.

## Material Findings

### `OWN3-CRITICAL-001` — Five owner candidates are opaque, not independently validated

`reference_calculator.py:541-553` checks only that five dictionary keys exist,
hashes each supplied payload, and writes `validated: True` unconditionally.
The positive hydrology candidate at lines 1321-1333 contains a protocol hash,
one aggregate post-ingress mass, and the literal assertion
`ending_stores_reconstructed_from_finalized_use: True`; it does not expose
beginning stores, per-source finalized debits, condensation credits, ingress,
infiltration/runoff partitions, or reconstructed ending stores. The soil,
LSE, vegetation, and BGC payloads are similarly partial.

The coupled-transaction schema validates five opaque candidate-state hashes and
owner-kind labels, not the candidate bodies or their cross-owner joins. The
separate strict positive instance at `reference_calculator.py:658-707` creates
candidates by cloning placeholder beginning dictionaries and adding a
transaction field; it is not the physically solved transaction. Consequently
the five receipts attest only that bytes were hashed, not that each owner
independently accepted its candidate.

This leaves `OWN-CRITICAL-009` and `OWN2-CRITICAL-003` unresolved. Recommended
disposition: `accepted`. Construct complete typed candidate operands for all
five owners, independently validate each ending state and join, and derive
receipt validation from those checks rather than a literal boolean.

### `OWN3-CRITICAL-002` — Shared soil-layer competition stops at authorization

`execute_shared_layer_root_ground_competition()` constructs 19 typed requests,
performs one source-group arbitration, and reruns three capped solvers, but
returns no typed `finalized_uses`, no complete water protocol, no D/A/F
validation, and no hydrology ending-store reconstruction
(`reference_calculator.py:987-1009`). The committed fixture therefore has
`requests` and `authorizations` but no `finalized_uses` or
`water_protocol_validation` for the decisive five-way `soil-1` competition.

The individual capped solver details are not a substitute for preserving each
request key through finalized use and owner debit. In particular, the evidence
cannot independently prove aggregate finalized use for the shared layer is
bounded by the immutable beginning inventory or that unused authorization
remains.

This leaves `OWN2-CRITICAL-002` unresolved and prevents complete confirmation
of `OWN-CRITICAL-004`. Recommended disposition: `accepted`. Build finalized
records from each accepted capped result, validate the exact D/A/F key sets and
bounds, reconstruct every physical source store, and freeze that complete
protocol in the shared-layer vector.

### `OWN3-HIGH-003` — The multi-OFE runon parcel is not derived from upstream outflow

The routed case creates a hard-coded `0.6 kg m^-2` runon parcel at 289.6 K
(`reference_calculator.py:1306-1312`). After executing an upstream covered
water transaction, it changes only `source_state_sha256` and declares route
identity when that hash matches the upstream final-candidate hash (lines
1340-1355). The upstream transaction contains no runoff or outlet mass/energy
record at all; an independent path scan found zero upstream `runoff` or
`outlet` operands.

Thus neither the routed mass nor its enthalpy is paired to an upstream debit,
and the downstream closure can pass for a parcel the upstream owner never
produced. This leaves `OWN2-HIGH-006`, the routed part of
`OWN-CRITICAL-005`, and cross-OFE evidence for `OWN-HIGH-006` unresolved.
Recommended disposition: `accepted`. Produce an upstream typed runoff/outlet
crossing, construct downstream runon from the exact accepted mass and enthalpy,
and validate equal mass, energy, route sequence, source/destination OFE, tile,
interval, and area conversion.

### `OWN3-HIGH-004` — Condensation is solved but never enters a typed owner transaction

The mandatory condensation solve yields a signed vapor flux and scalar
`condensation_credit_kg_m2_stand_ground`, but both the complete water protocol
and the strict positive water-protocol instance contain an empty
`condensation_credits` array. No positive vector constructs the schema-defined
typed credit, applies it to the exact hydrology-owned source, updates the LSE
enthalpy with the same mass/temperature, or validates the paired receipt.

Canonical text and schema shape correct the original omission, but executable
owner evidence does not. `OWN-CRITICAL-002` therefore remains unconfirmed.
Recommended disposition: `accepted`. Bind the accepted condensation case to a
complete typed water protocol and five-owner candidate, then reconstruct the
mass and signed vapor-energy credit independently.

### `OWN3-HIGH-005` — Natural numerical failures do not prove whole-owner rollback

The snow/domain failures compare vegetation, hydrology, LSE, soil thermal,
BGC, and envelope hashes. The natural singular, iteration-limit, and
backtracking-limit records instead compare only one joint-column
`beginning_sha256` with one `rollback_sha256`
(`reference_calculator.py:1359-1362`). They contain no complete owner or
envelope hash map. The aggregate `all_validation_failures_rollback_exact`
therefore combines two materially different rollback envelopes and overstates
the result.

This is another unresolved part of `OWN2-CRITICAL-003` and
`OWN-CRITICAL-009`. Recommended disposition: `accepted`. Wrap every natural
failure in the same complete five-owner transaction envelope and compare exact
serialized bytes for all owners and pending transaction state.

## Finding Reassessment

| Finding | Final assessment |
|---|---|
| `OWN-CRITICAL-001` | **Corrected.** Canonical authority, state schema, and model definition give hydrology all water mass, LSE one surface enthalpy node, and soil thermal all soil temperature/enthalpy. |
| `OWN-CRITICAL-002` | **Canonical/schema correction complete; positive owner evidence incomplete.** See `OWN3-HIGH-004`. |
| `OWN-CRITICAL-003` | **Corrected.** Authorization uses immutable beginning stores only; current ingress is post-final and cannot alter caps. The complete single-column vector reconstructs the beginning-state rebuild and single call. |
| `OWN-CRITICAL-004` | **Canonical interface corrected; decisive shared-resource final-use/debit evidence incomplete.** See `OWN3-CRITICAL-002`. |
| `OWN-CRITICAL-005` | **Single-OFE ingress largely corrected; routed cross-OFE custody incomplete.** See `OWN3-HIGH-003`. |
| `OWN-HIGH-006` | **Canonical OFE-local area rules corrected; routed multi-OFE evidence incomplete.** |
| `OWN-HIGH-007` | **Corrected for the six declared strict surfaces.** Full candidate-body validation remains part of `OWN3-CRITICAL-001`. |
| `OWN-HIGH-008` | **Corrected.** Forcing schema conditionally requires Harder--Pomeroy precipitation or accepted-upstream runon provider lineage, and missing positive-mass enthalpy fails. |
| `OWN-CRITICAL-009` | **Not corrected.** Five-owner positive validation and whole-owner numerical rollback remain absent. |
| `OWN2-CRITICAL-001` | **Corrected.** Complete water keys preserve transaction, requester owner/component, OFE, request tile, occupancy or surface/class, physical source/type/tile/layer, and basis. |
| `OWN2-CRITICAL-002` | **Not corrected for the shared competition endpoint.** Requests and authorizations are typed; finalized uses and source-store debit are missing. |
| `OWN2-CRITICAL-003` | **Not corrected.** See `OWN3-CRITICAL-001` and `OWN3-HIGH-005`. |
| `OWN2-CRITICAL-004` | **Corrected for the single-OFE ingress vector.** Retained ingress changes surface enthalpy and infiltration produces a soil-thermal receipt. |
| `OWN2-HIGH-005` | **Corrected.** Source-specific tile/layer/basis conditions and exactly one named receipt per owner are schema-bound. |
| `OWN2-HIGH-006` | **Not corrected.** See `OWN3-HIGH-003`. |

## Real-Hydrology Feasibility And Boundary

The Stage-0 trace truthfully identifies `DirectFrameExecutor` and the immutable
pre-hydrology day-frame point, and it records that production currently lacks
persistent snow-free pond/litter mass, runon enthalpy lineage, and a soil
thermal `-G` receiver. Those are later implementation obligations, not
evidence supplied by this authority oracle.

The Python `arbitrate()` routine is an independent authority fixture. Despite
the string `hydrology-real-owner`, it is not the actual production hydrology
owner and must not be borrowed as Child-2 real-owner or Child-4 real-consumer
evidence. Child 2 still must expose or extract the actual hydrology state and
candidate logic and prove production byte invariance.

## Conclusion

The acyclic definition/schema identities, OFE-local area convention, complete
water-key shape, beginning-only authorization rule, post-ingress thermal update,
and provider lineage are acceptable. The remaining findings are evidence and
owner-boundary defects inside Child 1's required terminal envelope, not reasons
to defer them to runtime implementation.

**Result: FAIL. Do not release `OPENWEPP_SNOW_FREE_LSE_V1` implementation
authority until every accepted finding above is corrected and independently
re-reviewed against new stable exact bytes.**
