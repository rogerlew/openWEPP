# Review Finding Disposition

Status: `PASS / every finding accepted and remediated; dual final reviews pass`

Evidence mode: `Static + Ran`

The parent accepted every first-pass finding. No finding was rejected,
deferred, or converted to follow-up.

| Findings | Disposition | Remediation |
|---|---|---|
| `RA-001`, `REVIEW-B-001` | `accepted` | canonical v5 now contains executable radiation, interception, gas exchange, energy, hydraulics, C/N, phenology and solver definitions; invalid `080` reference removed |
| `RA-002` | `accepted` | selected exact `alpha_liq*tanh(L+S)` CLM liquid sequence; stemflow is separately classified canonical selection |
| `RA-003` | `accepted` | v1 explicitly selects interval-equilibrium hydraulics, complete path factors, authorization-constrained final solve and no capacitance |
| `RA-004` | `accepted` | neutral forcing domain, log-law/boundary resistance, signed leaf residual and independent ground ownership made explicit |
| `RA-005`, `REVIEW-B-003` | `accepted` | oracle now independently solves two-stream, energy and hydraulic systems and exercises fixed expected, poison, trajectory, receiver and real rollback vectors |
| `RA-006` | `accepted` | exact mutable-leaf-N capacity equations and gas-unit conversions admitted |
| `RA-007` | `accepted` | deterministic brackets, finite differences, scaling, pivots, backtracking, limits and typed nonconvergence selected; no empirical temperature box |
| `RA-008`, `REVIEW-B-004` | `accepted` | exhaustive canonical field/initial-state inventory, no consumed aliases, exact model-definition bytes and SHA-256 binding added |
| `REVIEW-B-002` | `accepted` | N retranslocation precedes request; maximum authorization and finalized use are distinct and only finalized use is debited atomically |
| `REVIEW-B-005` | `accepted` | deterministic GSI edge/hysteresis/timer state machine, onset recurrence, deciduous offset and evergreen bounded turnover admitted |
| `REVIEW-B-006` | `accepted` | typed litter partition fields and donor routing admitted; named soil-transformation successor scaffolded with zero/fail boundary |
| `REVIEW-B-007` | `accepted` | canonical readiness vocabulary restored; every unavailable empirical/runtime surface is `BLOCKED` |

Ran after remediation: the package-local reference calculator returned
`"all_pass": true`. Fresh reviewer judgments and final gate reruns remain
required before disposition.

## Second-Pass Findings

Both reviewers correctly retained `HOLD`; every second-pass finding is
`accepted` and was remediated in-package.

| Findings | Disposition | Remediation |
|---|---|---|
| `RA2-001` | `accepted` | retained direct/diffuse transmission across strata, added numerical sun/shade absorption, exact Sellers/CLM direct-upscatter integral, and `phi2=0`, zero-scattering, zero-direct, and zero-LAI branches |
| `RA2-002` | `accepted` | corrected CLM `Cv` to `0.01 m s^-1/2`; admitted canopy-air heat/vapor nodes, all area factors, liquid saturation polynomial, air properties and enthalpy |
| `RA2-003` | `accepted` | added four-node/layer/gravity/series-resistance reconstruction, authorization-active re-solve, dry/frozen exclusion and exact `mm`--mass identity |
| `RA2-004`, `B2-005` | `accepted` | model-definition bytes now bind canonical variable, equation/algorithm, invariant, schema, numerical, and complete BGC section hashes plus all fixed constants; Rust independently checks every bound section digest |
| `RA2-005` | `accepted` | removed stale authority-missing labels and defined wet/dry area, evaporation, condensation, second drainage and subfreezing rejection order |
| `RA2-006` | `accepted` | gross `Ag` alone enters GPP while net `An` drives stomata; leaf respiration is debited once; leaf-N/class-area capacity and zero-LAI branches are explicit |
| `RA2-007` | `accepted` | added removable radiation branches, direct/diffuse mixed-stratum and sun/shade vectors, nested FvCB--Medlyn--ci--energy solve, active-cap/four-node hydraulics, selected-process failure rollback and changed-canopy/unchanged-floor poison vector |
| `B2-001` | `accepted` | removed circular `eta/Nuse`; potential demand precedes arbitration and final use is a unique authorization-proportional amount after water-limited GPP |
| `B2-002`, `B2-006` | `accepted` | every mineral-N competitor now has request/maximum-authorization/finalized-use objects; only finalized amounts debit inventory; BGC readiness vocabulary corrected |
| `B2-003` | `accepted` | N-limited carbon persists in `NSC_C`; exact six-tissue N credits/internal-N debit, persistent `T10`, and root/wood turnover ownership were added |
| `B2-004` | `accepted` | deciduous vector now reaches onset, active, offset and dormant with exact litter C/N/DM receipt; N bucket/finalization and selected-process rollback poison vectors execute |

Ran after second-pass remediation: reference calculator exit `0` with every
check true; focused nextest `12/12`; both canonical unit-compliance checks pass.
A third exact-byte review is required; no earlier `HOLD` is silently treated as
acceptance.

## Third-Pass Findings

Every third-pass finding is `accepted`; none is rejected, deferred, or left as
an undispositioned follow-up.

| Findings | Disposition | Remediation |
|---|---|---|
| `RA3-001` | `accepted` | corrected the selected two-stream sign/coefficient convention; reconstructed `mubar` and `beta0` by the admitted adaptive quadrature; separated terminal direct/diffuse and local sunlit/shaded absorption; added black, zero-direct, zero-LAI, and leaf-angle-domain poisons |
| `RA3-002` | `accepted` | coupled oracle now solves boundary-layer `cs/ci`, surface VPD, canopy-air temperature/humidity, leaf energy, Medlyn conductance, and the four-node hydraulic demand from one immutable state |
| `RA3-003` | `accepted` | replaced the surrogate root solve in the coupled vector with the exact four-potential nonlinear path, authorization-active common residual, gravity/path factors, dry/frozen exclusions, and explicit hydraulic-redistribution rejection |
| `RA3-004` | `accepted` | canonical wet area is `fwet*(leaf+stem)` with a common wet node, separate signed residual, caller wet dimension/emissivity, exact area-proportional energy ownership, and an executable omitted-stem poison |
| `RA3-005` | `accepted` | hard-vector claims now bind the amended calculations, fixed independent numbers, resistance/node poisons, active caps, redistribution rejection, and real iteration-limit rollback |
| `REVIEW-B3-001` | `accepted` | specified bounded fine-root turnover, live-to-dead wood internal transfer, and subsequent background mortality of every remaining tissue with exact litter/CWD C/N/dry-matter receivers |
| `REVIEW-B3-002` | `accepted` | renamed potential and final N-demand surfaces unambiguously and removed the unused `leaf_n_retrans_fraction` consumed field |
| `REVIEW-B3-003` | `accepted` | added fine-root, live/dead wood, CWD, receiver-N, wrong-full-competitor-debit, and exact C/N/dry-matter reconstruction vectors |

Ran after third-pass remediation: the independent calculator returned every
check true; focused nextest passed `12/12`; canonical definition section hashes
and the definition-file digest were recomputed and independently asserted.
Fresh final reviewer judgments remain mandatory.

## Fourth-Pass Findings

The parent accepted and remediated every fourth-pass finding.

| Findings | Disposition | Remediation |
|---|---|---|
| `RA4-001` | `accepted` | admitted `beta_hyd` coupling of Medlyn conductance and solved it as an equivalent nested fixed point whose gas/energy transpiration equals the four-node hydraulic leaf flux to the canonical water tolerance; the former one-pass value is now a failing poison |
| `RA4-002` | `accepted` | added one `dt`-bound, store-limited wet leaf/stem plus dry-leaf/dry-stem vector; the identical vapor amount closes store and latent energy, all surface energy partitions reconstruct the original stratum operand, and rate/amount plus omitted-stem aliases fail |
| `RA4-003` | `accepted` | removed the scalar/clipped hydraulic oracle; all positive root-profile, dry/frozen, active-cap, and coupled vectors now use the selected four-node path; authorizations are interval amounts converted by explicit `dt` and an amount-as-rate poison executes |
| `REVIEW-B4-001` | `accepted` | emitted fine-root start/end/loss and receiver C/N/DM plus independent wood/CWD donor-DM ledgers; Rust parses and asserts all six closure residuals and explicit wrong-C/C-as-DM poisons |
| `REVIEW-B4-002` | `accepted` | corrected the strict initial-state manifest to six C/N display/storage/transfer fields per tissue, 36 total |

Ran after remediation: independent oracle exit `0`, every check true; focused
nextest `12/12`; Markdown lint clean for both amended canonical/package trees.
The final stable-byte reviews described below completed this requirement.

## Final Review Result

Reviewer A and reviewer B each independently re-read the stable post-remediation
bytes and returned `PASS / no material findings`. Their exact identities,
commands, numeric checks, and snapshot hashes are recorded in their respective
review artifacts. Both repeated the review after approved/active lifecycle and
digest rebinding; no finding remains undispositioned.
