# Review Finding Disposition

Status: `terminal candidate / every accepted material finding corrected / final GO/PASS`

The former heterogeneous E04 liquid-topology omission was lifted by V2.
Increment 2B independently exposed additional load-bearing omissions and did
not relabel any Review-B finding remediated. See
`potential-pass-hold-legitimacy-audit.md`.

Evidence mode: `Static + Ran`

The historical findings and failed reviews remain unchanged in
`review_agent_b.md`, `review_agent_b_remediation.md`, and
`review_agent_a_remediation.md`. Every material finding was accepted; none was
deferred or rejected.

| Finding | Correction and authority | Focused evidence | Current status |
|---|---|---|---|
| B-CRITICAL-001 | Rebuilt `transaction::{execute_candidate_with_failure,prepare_stratum,finish_physical}` from SC-VEGETATION E01--E22; diagnostic consumes the real path. | real four-owner transaction plus 27 failure/malformed-owner phases | remediated; final GO/PASS |
| B-CRITICAL-002 | Replaced RK4/fixed-`kd` optics with `radiation::{two_stream,solve_two_stream}` exact matrix exponential, analytic particular/resonance branches, directional identities, and ordered topology traversal. | oracle absorption, band/direction closure, Beer poison, A0 | remediated; final GO/PASS |
| B-CRITICAL-003 | Added exact Brent and damped-Newton numerics, separate leaf/wet/stem/canopy-air energy nodes, four-potential hydraulics, cap complementarity, and outer `beta_hyd` equality solve. | coupled public path; full and cap-active hydraulic oracle vectors; rollback | remediated; final GO/PASS |
| B-CRITICAL-004 | Added persistent six-tissue display/storage/transfer state, exact C/N allocation, maintenance reserve, phenology, turnover, mortality, LAI ownership, and C/N/DM transfers. | six-tissue and phenology oracle vectors; public state transition | remediated; final GO/PASS |
| B-CRITICAL-005 | Added typed `SoilLayerId`, `MineralNitrogenSpecies`, `MineralNitrogenKey`, amount basis, owner, and transaction identity through request/auth/use/debit/receipt. BGC arbitrates exact keys proportionally. | layer/species, competition, unused authorization, wrong/duplicate receipt coverage | remediated; final GO/PASS |
| B-HIGH-006 | Replaced residual scalars with explicit five-ledger operand structures and external reconstruction; added diagnostic water, BGC, vegetation, and energy owner candidates. | five-ledger public commit and 27 byte-identical serialized rollback/malformed-owner phases | remediated; final GO/PASS |
| B-HIGH-007 | Corrected peaked response, stable log evaluation, finite/domain guards, smaller-root selection, and exact numerical tolerances/limits. | biochemical vectors, NaN/zero-capacity guards, strict Clippy | remediated; final GO/PASS |

## Remediation-review findings

| Finding | Correction | Focused evidence | Current status |
|---|---|---|---|
| RB-CRITICAL-001 / A-CRITICAL-002 | Clumped plant-area optics now consumes leaf/stem optical parameters, then partitions photosynthetic leaf area and wet/dry leaf/stem energy. | stem-only optics poison and directional/topology closure tests | corrected; final GO/PASS |
| RB-CRITICAL-002 | Condensation is capacity-bounded or typed unsupported, q1 paths use stem vulnerability, and gas/hydraulic equality uses the admitted scale-aware tolerance. | condensation-cap, four-node/cap oracle, and coupled solve tests | corrected; final GO/PASS |
| RB-CRITICAL-003 | Removed the Atkin clamp, applied its source-unit conversion, made GSI crossings strict at equality, enforced litter C:N donor sufficiency, and advances both previous offset fluxes. | phenology equality, trajectory, C/N, and persistent-state tests | corrected; final GO/PASS |
| RB-CRITICAL-004 / A-CRITICAL-001 | Sealed `CoupledCandidate`, bound commit to exact beginning identities, added typed exact proposal/receipt matching and the energy owner, and reconciles all owner candidates. | 27 phase/malformed-owner failures compare serialized owner bytes exactly | corrected; final GO/PASS |
| RB-HIGH-005 / A-HIGH-006 | Fixed Brent rotation, requires residual and step convergence, uses physical residual scales, and distinguishes energy/hydraulic/coupled/radiation failures. | Brent root, singular taxonomy, hydraulic oracle, strict Clippy | corrected; final GO/PASS |
| RB-HIGH-006 / A-MEDIUM-007 | Centralized deterministic, owner-sorted compensated proportional arbitration and exact layer/species debit/receipt identity. | order reversal, species/layer, unused authorization, and duplicate proposal tests | corrected; final GO/PASS |
| A-CRITICAL-003 | Added column rain routing, one top shortwave boundary, nonterminal zero-ground branches, and terminal-only water/energy output accounting. | two-rank rain and independent top-boundary shortwave reconstruction | corrected; final GO/PASS |
| A-HIGH-004 | Canonical configuration/state digests, exact stratum sets, and topology/domain rules now fail before Stage A. | mutated-digest poison and strict parsing | corrected; final GO/PASS |
| A-HIGH-005 | C/N/dry-material closure now uses `1e-14 + 64*epsilon*operand_sum`; water and energy use explicit scale-aware bounds. | public five-ledger commit and rollback suite | corrected; final GO/PASS |

The focused evidence invalidates the checkpoint implementation conclusions but
does not rewrite them. Final science/closure and Rust correctness reviews have
independently accepted the implementation bytes; the benchmark-evidence
correction recorded later was the only active rereview item at that checkpoint
and is superseded by the corrected benchmark matrix and final rereviews.

## HOLD adjudication and current disposition

The repeat review's targeted adjudication found a canonical omission for
heterogeneous-tile E04 liquid-store routing. It is accepted and recorded in
`hold-legitimacy-audit.md`; no constitutive workaround is implemented. The
public path fails closed for that topology.

The subsequent Rust findings concerning complete failure diagnostics,
multirank final liquid handoff, genuinely independent owner reconstruction,
all-owner-only atomic commit, duplicate request identity, unclamped energy
operand handling, and line-count decomposition are accepted and unresolved.
They are not deferred to a new package: they remain in this package for the
first continuation after the authority lift. No original or new finding is
marked remediated without passing focused review evidence.

## V4 Runtime Review Disposition

The V4 authority lift resolves the scoped shared-C/N state questions recorded
in `cn-state-hold-legitimacy-audit.md`: displayed leaf C owns LAI/area caches,
displayed leaf N owns positive-LAI capacity and leaf Rd inputs, and both
unconsumed previous-offset fields are removed. V1/V2/V3 HOLD and review
artifacts remain unchanged.

| Review finding | Disposition | Evidence / remaining action |
|---|---|---|
| recursive V4 JSON shape and typed structural mutation coverage | accepted / corrected | exact recursive key sets, duplicate rejection, typed identity/domain poisons, and all 155 independent mutation digests pass |
| leaf displayed/storage/transfer ownership | accepted / corrected | displayed leaf C/N are the only area/capacity/Rd owners; storage/transfer poisons and no-double-leaf-maintenance test pass |
| V3-to-V4 migration must validate, not merely reshape | accepted / corrected | source/config/state/lineage/membership/area/transfer validation, exhaustive issue report, target revalidation, and no direct V1/V2 shortcut pass |
| duplicated validation across transaction, migration, state-shape, and canonical serializer | accepted Medium / non-blocking at this increment | `review_v4_runtime` GO: intentional independent boundary validation has 155-mutation parity and target revalidation; extract shared structural validation before terminal closure to reduce drift risk |
| post-remediation runtime QA | PASS | `qa_v4_runtime` found no material issue after recursive-shape and typed-mutation corrections |
| authorization-capped solver acceptance | accepted blocker / unresolved | canonical minimum complementarity alone does not bind the cap-equality active set or independent `q_law`/cap operands; obtain digest-bound fully coupled cap-active vectors before connecting the draft or passing `STAGE_B_E11_E15_EXACT_ORACLE` |

No review labels the package, Milestone 2, Milestone 3, or E01--E22 public
transaction complete. The capped acceptance gap is not deferred or waived; it
is the current fail-closed boundary.

## V5 Authority-Lift Disposition

The V4 capped acceptance blocker is resolved at the authority layer by commit
`b7e6f08b655452c5c59a498ac9becd1439dd21ef`. V5's two independent science
reviews returned GO and both terminal verifiers returned PASS on definition
`0ee6a50d...`, vectors `6f5e9554...`, and generator `4c3a1cfc...`.

| Finding | Disposition | Remaining implementation evidence |
|---|---|---|
| missing cap amount/rate and `q_law` operands | authority corrected / implementation active | exact Rust operand reconstruction and basis poisons |
| unresolved equality-active convention and derivative | authority corrected / implementation active | equality/near-tie active set and frozen generalized-Jacobian vectors |
| incomplete capped failure diagnostics | authority corrected / implementation active | failed-iterate typed payload, singular/limit/backtracking, null candidate/use |
| authorization substituted for finalized use | authority corrected / implementation active | `F<=A<=D`, hydrology debit of `F` only, receipt and closure |
| incomplete V4-to-V5 identity transition | authority corrected / implementation active | exact payload copy, distinct identities, stale-V4 rejection |

No capped implementation review has passed yet. The earlier V1--V4 findings
and HOLD records remain immutable, and no Milestone 2/3 or public-path claim is
made.

## V5 Bounded-Core Review Disposition

Both repeat reviewers found no remaining material Rust defect in the bounded
V5 capped core after remediation. They independently rejected
`STAGE_B_E11_E15_EXACT_ORACLE` because the digest-bound CPython failed-iterate
`step_norm` is not reproduced by Rust and V5 supplies no cross-runtime
comparison rule. The locally observed `3e-6` comparison is provisional only and
does not remediate or pass the finding. The exact blocker, attempted routes,
and lift action are recorded in
`v5-failure-payload-portability-hold-legitimacy-audit.md`.

Disposition: bounded checkpoint accepted; Stage B, public path, Milestones 2/3,
and the original Review-B completion claims remain unresolved and fail-closed.

## V6 Portability-Lift Review Disposition

The contract-first V6 package at commit `b326173e2` lifted only the frozen
rejected-failure evidence-comparison HOLD. The first correctness and QA reviews
of the implementation found the following material defects; every finding was
accepted and corrected in this package.

| Finding | Decision and correction | Focused evidence | Status |
|---|---|---|---|
| runtime comparison cloned exact fields from the reference | accepted; actual record now derives all 21 fields from Rust diagnostics, typed context/result posture, configuration bytes, and independently serialized full owner/attempted-transaction snapshots | 21 seam poisons plus 10 boundary, 20 firewall, and 4 nonfinite authority cases | corrected; repeat review pending |
| V5-to-V6 migration omitted diagnostic identity/payload | accepted; added atomic configuration, initial/current state, and typed-category identity-bound diagnostic migration with distinct V6 digests and byte-preserved non-identity payload | frozen V6 `identity_transition` payload/digests, stale diagnostic identity/category/digest/lineage poisons, no-candidate source immutability | corrected; repeat review pending |
| historical source lineage was incomplete | accepted; complete V5 config/initial/current state, shared/occupancy/pending-transfer lineage and error precedence now validate before candidate construction; transaction-zero current state must be byte-identical to initial state | initial receipt, transaction-zero divergence, nonzero transaction, shared/occupancy/transfer poisons | corrected; repeat review pending |
| `migration.rs` exceeded 3,000 lines | accepted; extracted cohesive V5-to-V6 implementation/tests; parent is 2,890 lines | strict Clippy and line-count recount | corrected; repeat review pending |
| positive test fixture could launder historical V5 drift | accepted; shared exact V5 configuration/initial-state validation now precedes test-only identity rebinding | full vegetation 179/179 and historical V1--V5 rejection | corrected; repeat review pending |
| package evidence and V5 public wording were stale | accepted; package/evidence rows now preserve the failed retry and identify V6 fail-closed posture | Markdown and source checks | corrected; repeat review pending |
| repeat reviews found a harness-authored failure category and incomplete rollback evidence | accepted; `VegetationError::NumericalFailure` now carries `NumericalFailureCategory`, producers preserve the category, and V6 comparison derives it from the returned error while hashing the complete beginning owner plus attempted transaction/occupancy and absent candidate before and after rejection | vegetation 179/179; implementation 13/13; authority 23/23; strict Clippy | corrected; repeat review pending |
| repeat reviews found migration category/fixture gaps | accepted; the migration envelope preserves and validates the typed failure category and its payload relationship, directly consumes the frozen V6 transition vector hashes, and enforces source-side transaction-zero precedence | focused migration 3/3 and frozen V6 vector hash assertions | corrected; repeat review pending |
| third QA review found re-digested category aliases could pass migration | accepted; V5-to-V6 evidence migration now admits only the authorized capped hydraulic `backtracking_limit` seam with positive backtracking count and finite nonnegative `step_norm` | explicit domain, iteration, singular, bracket, and outer-solve alias poisons; focused migration 3/3; strict Clippy | corrected; repeat review pending |
| third correctness review found unrelated rollback/configuration identities and indirect authority-vector consumption | accepted; the comparator snapshot now combines the validated five-layer V6 configuration receipt, full `CoupledOwnedState`, exact constitutive case/caps, Stage-A state, fixed authorization identity, attempted transaction/occupancy, and absent candidate; the production boundary helper attaches authorization identity; migration source and target projections both equal the frozen V6 scientific payload | focused V6 5/5; migration 3/3; strict Clippy | corrected; repeat review pending |
| third review exact line-count evidence drift | accepted; recounted stable formatted bytes | `migration.rs` 2,890; V5-to-V6 implementation 378; migration tests 651; comparator tests 1,070 | corrected; repeat review pending |

Final independent disposition on the stable bounded increment: Rust
correctness **GO** and QA **PASS**, with no unresolved material finding. The
2,890-line `migration.rs` remains WARN-level decomposition debt below the
mandatory threshold. Neither review approves the still-fail-closed public
E01--E22 transaction or completes Milestones 2/3.

No finding is rejected, deferred, or used to activate the public candidate.

## V7 Increment 4A Review Disposition

The first correctness and QA reviews returned HOLD. All material findings were
accepted; their original text remains unchanged in `review_agent_a.md` and
`review_agent_b.md`.

| Finding | Decision and correction | Focused evidence | Status |
|---|---|---|---|
| signed `XS_C` was rejected or algebraically cancelled | accepted; physical vegetation C and finite signed reserve are separate beginning/ending operands, and the ledger consumes the directly retained final-maintenance operand | valid negative signed aggregate plus ending-XS corruption poisons; real two-ULP candidate corruption rejects | corrected; final review GO/PASS |
| nitrogen candidate could be paired with a different capped water phase | accepted; the nitrogen phase retains its exact source `UncommittedWaterPhase`, and candidate construction requires structural equality with the supplied phase | real two-ULP nitrogen phase rejects a distinct complete capped-water phase | corrected; final review GO/PASS |
| whole-ledger stratum/state identity was incomplete | accepted; validator requires exact configured stratum set, one expected transaction/beginning/ending digest, unique strata, and globally unique proposal IDs | duplicate/missing/mixed identity, global duplicate-ID, and coherent forged-ending-digest poisons | corrected; final review GO/PASS |
| whole-owner C/N tolerance was too loose | accepted; restored `1e-14 + 64*epsilon*operand_sum` | old-envelope-only residual rejects; vegetation 223/223 | corrected; final review GO/PASS |
| candidate failures used generic or invented taxonomy | accepted; capped rollback, V7 allocation/closure, and V7 candidate rollback now emit canonical `VEG-E-093`, `VEG-E-097`, and `VEG-E-100` variants | exact variant assertions, strict Clippy, and focused rejection tests | corrected; final review GO/PASS |
| displayed-area calculation had three production copies | accepted; runtime, candidate, and V3 migration use one exact-order helper | migration and full vegetation suites pass | corrected; final review GO/PASS |
| real-candidate provenance poisons were missing | accepted; the actual two-ULP candidate is revalidated after XS corruption, coherent digest forgery, DM=C substitution, deterministic reconstruction, and source-water replacement | vegetation 223/223; beginning bytes remain exact | corrected; final review GO/PASS |

No finding authorizes public candidate publication, receiving-owner mutation,
atomic commit, heavy gates, activation, or calibration claims.

Final exact-byte review: Rust correctness `GO`; QA `PASS`. No material
Increment 4A finding remains unresolved. Milestone 4 is closed; Milestone 5 is
still open.

## E19 Ordering-Remediation Review Disposition

| Finding | Decision and correction | Status |
|---|---|---|
| potential/final monotonicity errors and guards were noncanonical | accepted; removed `FinalDemandExceedsPotential`, `NitrogenDemandOrdering`, and both direct monotonicity checks | corrected |
| receipt-bound growth still rejected rounded `Nused>demand` | accepted; removed aggregate ordering guard and retained canonical eta clamp | corrected |
| exact aggregate-rounding poison missing | accepted; bound a binary64 case with `Nused` exactly one ULP above demand, `eta=1`, and zero NSC | corrected |
| lifecycle test/count evidence stale after review correction | accepted; preserved initial HOLD, recorded 215/215, and refreshed 2,214-line WARN | corrected |
| six-tissue assertions and source-substring guards could be stronger | accepted as Low/non-blocking QA debt; existing exact C/N closure and behavioral tests support only the bounded uncommitted claim | visible debt |

Final fresh review: Rust correctness `GO`; QA `PASS`. No material finding is
unresolved. The reviews explicitly confirm that neither canonical contract
requires `Ndem_final<=Ndem_pot` and trace immutable requests, one arbitration,
typed `F_N`, common eta, NSC retention, and beginning-state immutability for the
two-ULP fixture. No finding authorizes public candidate publication, BGC debit,
atomic commit, activation, or calibration claims.

## V6 Public Water-Phase Review Disposition

The first correctness review returned HOLD and the first QA review returned
FAIL. All findings were accepted; none was deferred or rejected.

| Finding | Decision and correction | Focused evidence | Status |
|---|---|---|---|
| authorization/candidate validation could span different water snapshots | accepted; `WaterArbitration` carries one typed immutable `WaterOwnerSnapshot`, and the same exact snapshot is required in `WaterOwnerCandidate` | snapshot-drift poison and aggregate snapshot overbooking validation | corrected; repeat review pending |
| per-request authorization reason absent | accepted; every exact request key carries one of the six canonical `WaterAuthorizationReason` variants and reason/amount consistency is independently validated | zero/full/competing paths and wrong-reason poison | corrected; repeat review pending |
| shared-layer owner and validator used different floating aggregation order | accepted; both call `reconstruct_water_ending`, which sorts exact resource keys, sums once per layer, and subtracts once | two-occupancy same-layer exact-bit vector | corrected; repeat review pending |
| evidence did not distinguish finalized use from authorization | accepted; independent vector fixes `F<A` and rejects a candidate whose ending store debits exact `A` instead of `F` | exact authorization-as-debit poison | corrected; repeat review pending |
| rollback test omitted receiving-owner bytes | accepted; every injected public water failure compares serialized vegetation and water-owner beginning stores before/after | seven phase failures, no returned phase | corrected; repeat review pending |
| first repeat review found frozen/rooting/competition reason aliases | accepted; exact per-request owner reason facts now live in the immutable snapshot and must equal the arbitration reason map | frozen-to-rooting and competing-to-exclusion alias poisons | corrected; second repeat review pending |
| first repeat QA found shared-layer poison arithmetic aliased | accepted; changed finalized operands to `0.01/0.07` and assert the canonical result differs by one ULP from sequential subtraction | explicit `assert_ne` against rejected arithmetic | corrected; second repeat review pending |

The full `CoupledCandidate` and commit remain fail-closed at E16--E22. Final
repeat review on stable bytes returned correctness GO and QA PASS with no
unresolved material finding; the bounded public water increment is accepted.

## E16/E17 Operand-Retention Review

| Finding | Disposition | Correction |
|---|---|---|
| accepted-carbon API allowed potential columns | accepted / corrected | crate-private aggregation requires capped diagnostics and is exposed only by sealed final water-phase output |
| production `Ag` versus `An` retention lacked independent poison | accepted / corrected | V3 uncapped and V5 capped fixtures assert sun/shade `Ag`, distinct `An`, and exact Rd |
| interval could be substituted by accessor caller | accepted / corrected | validated configuration interval is stored in `UncommittedWaterPhase`; accessor takes no duration |
| aggregation rejection seams lacked direct tests | accepted / corrected | absent operands, duplicate/wrong identity, inconsistent T10, negative class Ag, and potential-pass inputs reject |

No finding is deferred or rejected. The bounded operand-retention increment
does not remediate `GAP-VEGETATION-027` or enable persistent E16--E22 execution.

## Increment 4B / Milestone 5 Review Disposition

The first correctness and QA reviews returned HOLD. Every material finding was
accepted; none was deferred or rejected.

| Finding | Decision and correction | Focused evidence | Status |
|---|---|---|---|
| empty stand rejected by nonempty water/C/energy assumptions | accepted; admitted empty configuration now runs an exact zero-demand water/N transaction, retains mineral and water stores, closes tile radiation, and advances all owner lineage | real public empty-stand integration vector | corrected; repeat review pending |
| BGC owner omitted from science admission | accepted; classifier includes biogeochemistry and separate impact entries bind the crate to `SC-BIOGEOCHEM-001` and `SC-VEGETATIONTRANSACTION-001` | admission now covers 18 surfaces | corrected; repeat review pending |
| prior energy ledger accepted without validation | accepted; genesis/continuation shape, identity, finite operands, and stand closure are validated before construction | corrupted accepted-ledger rollback poison | corrected; repeat review pending |
| owner-envelope failures used a generic vegetation receipt | accepted; typed `DiagnosticError::OwnerEnvelopeIdentity` carries canonical `VEGTXN-E-007` | exact variant assertions | corrected; repeat review pending |
| cross-owner comparisons lacked behavioral poisons | accepted; validly constructed water, N, energy, transaction, beginning-state, and material-receipt mismatches reach the real envelope validator | 27-point rollback/malformed-owner matrix with error-origin assertions | corrected; repeat review pending |
| envelope owner-validation injection was shadowed by the water phase | accepted; the diagnostic reserves `OwnerValidation` for the envelope while the water phase retains its own focused unit failure | exact error-origin assertions | corrected; repeat review pending |
| stale lifecycle and line-count evidence | accepted; current headers/status, counts, and historical qualifiers reconciled | package Markdown lint and exact recount | corrected; repeat review pending |

The first review text remains historical evidence. Final stable-byte
disposition: Rust correctness **GO** and QA **PASS**, with no unresolved
material finding.

## Milestone 6 Central Arbitration and Error-Taxonomy Disposition

The first final-campaign correctness review returned HOLD with two material
findings. Both were accepted and corrected; neither was deferred or rejected.

| Finding | Decision and correction | Focused evidence | Status |
|---|---|---|---|
| `V7-M6-A-001`: the diagnostic duplicated proportional water arbitration and could drift in summation/order semantics | accepted; added one kernel `authorize_proportionally_by` allocator that groups by projected supply identity, preserves the complete request key, uses compensated totals, and sorts by owner/transaction/key/basis; diagnostic water delegates to it | shared-layer `2:6` against supply `4` produces exact `1:3`; four-request reversal-sensitive binary64 vector returns bit-identical per-key authorizations under caller-order reversal | corrected; final GO/PASS |
| `V7-M6-A-002`: canonical outer VEGTXN resource error families were unreachable through real water/N boundaries | accepted; kernel classifies identity/operand/bound once, vegetation maps those categories to `VEGTXN-E-001/002/003`, and BGC retains its distinct admitted `BGC-E-001/010` owner boundary | real duplicate/wrong-owner, nonfinite, `A>D`, `F>A`, inventory-overdraw, material-closure, and wrong-species tests assert exact variants/codes and rollback | corrected; final GO/PASS |

The final taxonomy delta review confirms the authority precedence: the kernel
owns shared exhaustive classification, the coupled envelope emits VEGTXN
families, and the BGC owner emits its own BGC families. Kernel 50/50,
vegetation 225/225, BGC 6/6, implementation 16/16, strict affected Clippy,
formatting, diff hygiene, and package Markdown passed. Final Rust correctness
is **GO** and final science/closure QA is **PASS**. No material review finding
is unresolved.
