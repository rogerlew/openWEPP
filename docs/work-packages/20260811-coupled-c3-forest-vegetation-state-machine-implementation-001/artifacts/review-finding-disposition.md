# Review Finding Disposition

Status: `executing / V4 runtime reviews PASS / capped acceptance gap retained`

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
