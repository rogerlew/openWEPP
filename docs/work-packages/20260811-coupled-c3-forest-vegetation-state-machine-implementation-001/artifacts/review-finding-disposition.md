# Review Finding Disposition

Status: `executed-hold / bounded capped core accepted / Stage B not passed`

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
| B-CRITICAL-001 | Rebuilt `transaction::{execute_candidate_with_failure,prepare_stratum,finish_physical}` from SC-VEGETATION E01--E22; diagnostic consumes the real path. | real three-owner transaction plus 13 failure phases | implemented; re-review pending |
| B-CRITICAL-002 | Replaced RK4/fixed-`kd` optics with `radiation::{two_stream,solve_two_stream}` exact matrix exponential, analytic particular/resonance branches, directional identities, and ordered topology traversal. | oracle absorption, band/direction closure, Beer poison, A0 | implemented; re-review pending |
| B-CRITICAL-003 | Added exact Brent and damped-Newton numerics, separate leaf/wet/stem/canopy-air energy nodes, four-potential hydraulics, cap complementarity, and outer `beta_hyd` equality solve. | coupled public path; full and cap-active hydraulic oracle vectors; rollback | implemented; re-review pending |
| B-CRITICAL-004 | Added persistent six-tissue display/storage/transfer state, exact C/N allocation, maintenance reserve, phenology, turnover, mortality, LAI ownership, and C/N/DM transfers. | six-tissue and phenology oracle vectors; public state transition | implemented; re-review pending |
| B-CRITICAL-005 | Added typed `SoilLayerId`, `MineralNitrogenSpecies`, `MineralNitrogenKey`, amount basis, owner, and transaction identity through request/auth/use/debit/receipt. BGC arbitrates exact keys proportionally. | layer/species, competition, unused authorization, wrong/duplicate receipt coverage | implemented; re-review pending |
| B-HIGH-006 | Replaced residual scalars with explicit five-ledger operand structures and external reconstruction; added diagnostic water, BGC, vegetation, and energy owner candidates. | five-ledger public commit and 13 byte-identical serialized rollback phases | implemented; re-review pending |
| B-HIGH-007 | Corrected peaked response, stable log evaluation, finite/domain guards, smaller-root selection, and exact numerical tolerances/limits. | biochemical vectors, NaN/zero-capacity guards, strict Clippy | implemented; re-review pending |

## Remediation-review findings

| Finding | Correction | Focused evidence | Current status |
|---|---|---|---|
| RB-CRITICAL-001 / A-CRITICAL-002 | Clumped plant-area optics now consumes leaf/stem optical parameters, then partitions photosynthetic leaf area and wet/dry leaf/stem energy. | stem-only optics poison and directional/topology closure tests | corrected; repeat review pending |
| RB-CRITICAL-002 | Condensation is capacity-bounded or typed unsupported, q1 paths use stem vulnerability, and gas/hydraulic equality uses the admitted scale-aware tolerance. | condensation-cap, four-node/cap oracle, and coupled solve tests | corrected; repeat review pending |
| RB-CRITICAL-003 | Removed the Atkin clamp, applied its source-unit conversion, made GSI crossings strict at equality, enforced litter C:N donor sufficiency, and advances both previous offset fluxes. | phenology equality, trajectory, C/N, and persistent-state tests | corrected; repeat review pending |
| RB-CRITICAL-004 / A-CRITICAL-001 | Sealed `CoupledCandidate`, bound commit to exact beginning identities, added typed exact proposal/receipt matching and the energy owner, and reconciles all owner candidates. | 13 phase failures compare serialized owner bytes exactly | corrected; repeat review pending |
| RB-HIGH-005 / A-HIGH-006 | Fixed Brent rotation, requires residual and step convergence, uses physical residual scales, and distinguishes energy/hydraulic/coupled/radiation failures. | Brent root, singular taxonomy, hydraulic oracle, strict Clippy | corrected; repeat review pending |
| RB-HIGH-006 / A-MEDIUM-007 | Centralized deterministic, owner-sorted compensated proportional arbitration and exact layer/species debit/receipt identity. | order reversal, species/layer, unused authorization, and duplicate proposal tests | corrected; repeat review pending |
| A-CRITICAL-003 | Added column rain routing, one top shortwave boundary, nonterminal zero-ground branches, and terminal-only water/energy output accounting. | two-rank rain and independent top-boundary shortwave reconstruction | corrected; repeat review pending |
| A-HIGH-004 | Canonical configuration/state digests, exact stratum sets, and topology/domain rules now fail before Stage A. | mutated-digest poison and strict parsing | corrected; repeat review pending |
| A-HIGH-005 | C/N/dry-material closure now uses `1e-14 + 64*epsilon*operand_sum`; water and energy use explicit scale-aware bounds. | public five-ledger commit and rollback suite | corrected; repeat review pending |

The focused evidence invalidates the checkpoint implementation conclusions but
does not rewrite them. Repeat science/closure and Rust correctness reviews must
independently accept the exact current bytes before any heavy gate.

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
