# COLD-CANOPY-M1 — fully wet structural null directions and G4 completion

**DRAFT FOR ROGER'S ADOPTION.** This proposes a narrowly bounded numerical-representation amendment and an implementation allowance. It does not authorize itself, supply either independent review, erase the G4 failure, or accept M1. Incorporate adopted direction into the existing `docs/work-packages/20260920-cold-canopy-m1-001/package.md`; retain that file as the only maintained narrative.

## 1. Objective, evidence and exact start

Pin this checkpoint to `rogerlew/openWEPP` commit `d7ad407e956e3ecea716db6744f4fd2bbe14ed61`. Preserve the completed G4 attribution: the captured raw and weighted 21-by-21 natural matrices have exact rank 19, with zero columns for upper shaded-leaf and stem temperatures; scaling and Jacobi error do not explain the failure. Preserve G4 physical FAIL, earlier G1/G5 positives, all callable interface bindings, and every source-specific validation, timing, review and negative result.

Deliver a prospectively reviewed structural null-direction policy, corresponding controls and confined Rust implementation, then execute the unchanged G4 physical phase/capacity-tie control. Conditional downstream scope remains the existing G2/G3/G6 physical obligations and native harness, not another solver search or a new physics model.

Current detached runtime: `/home/roger/openwepp-experiments/cold-canopy-m1-20260920`, 764 canonical entries, source-tree SHA-256 `2d67a9bb338c9b4dcb683c35f86bfe0c8cf1bce251d6b6821643a9e36ba8b1d6`. Captured release executable SHA-256 `ae22132f99bb85a44d0c4037f752a80ef6bdbc3f26a5e532409f89feb70f36c0`. Recovery: `artifacts/g4-rank-terminal-recovery.json`, archive SHA-256 `1e7395de7e8aece4b985aed3677a335ad3f1f7219e5ab134133b98b5086c5aad`. Verify actual bytes before mutation; reconstruct only if necessary. Never overlay a complete recovery patch on an already modified tree or substitute main Rust.

Read the current diagnostic disposition, `g4-rank-correctness-terminal01.json`, `g4-rank-qa-terminal01.json`, `g4-rank-capture.json`, independent rank/reference evidence, final ledger, and the actual affected source. Capture SHA-256: `edfc1df63996608ef015e36822e7e94b3971b7e2c3d956d77bf1f671ec0a4291`. Reuse the established rank/spectrum/custody findings; do not rerun the closed diagnostic just to reproduce its conclusion. Apply root/nearest/package/science/numerical/testing/bounded instructions and relevant vegetation, derivative and physical-acceptance authority.

## 2. Proposed allowance and ownership

Carry at least **414465.393795 charged seconds**. Propose **14400 new charged seconds**, giving a fixed cumulative ceiling of **428865.393795 seconds**. Do not add the old 3681-second balance or refund any conservative closing allocation. Verify `g4-rank-final-ledger.json` and any actual greater later closing receipt once; greater consumption reduces the balance under this ceiling.

After adoption, anchor the verified balance at first resumed work, including reading and capacity checks, to one fixed UTC work cutoff and hard deadline. Retain at least **1800 seconds** for independent disposition, preservation and publication. Charge elapsed concurrent work/waits once with no new deductions or mid-session re-anchoring. Every command must fit its full supported bound before the reserve. Preserve the 180-second physical bound and only the existing conditional 900-second Critical command exception.

Astra owns grouped implementation, internal sequencing and supported corrections. Use one functioning writer and distinct independent correctness/QA reviewers under the current repository routes and at-most-two concurrent-child constraint. No nested workaround, reviewer-to-author conversion or saturated retry loop. Reuse accepted findings and reviewer-owned fix verification. Two unsuccessful corrections or 60 charged minutes prompt a supported internal continue/reassign/escalate decision, not a fresh owner prompt for each error. Explicit owner/time/run, integrity, indispensable independence/source, completed-disposition and out-of-scope-policy stops remain binding.

## 3. Prospective method: hold certified null increments, not physical equations

Proposed experiment identifier: **COLD-CANOPY-M1-TR-SVD-BVLS-05**. Preserve BVLS-03/BVLS-04 arithmetic and previous evidence. This amendment permits only structural elimination of named, exactly unidentifiable dry-temperature *step directions* in the current linearized subproblem. It does not introduce a temperature anchor, remove energy equations, or allow arbitrary rank-deficient matrices through the existing guard.

Canonical authority must explicitly reconcile the current vegetation clause permitting only exact dark-sun elimination with this isolated numerical exception. Keep the physical M/H, phase, wetness, radiation, physiology, drainage and conservation equations unchanged. The owning LSE numerical contract must specify mask provenance, representation, lifespan, reconstruction, acceptance and work before body release. Obtain complementary independent reviews of that exact amendment and its controls. The adviser has not supplied those reviews.

### Eligibility and provenance

For each actual natural or selected raw-Jacobian assembly, derive eligible coordinate roles from the existing occupancy/component map and owning physical preparation, not fixture IDs or hard-coded coordinate numbers. Limit eligible roles to explicitly reviewed dry leaf/stem temperature components whose structural area is positive and whose canonical effective dry area is exactly zero because the admitted wet fraction is exactly one. Existing structural-zero-area anchors are unchanged, not merged into this rule.

A coordinate may be held only when its *complete* current raw-Jacobian column and corresponding weighted column are exactly zero (either signed zero), with the ordinary finite/scaling guards satisfied. Physical/source justification is mandatory: a zero column in an unrelated coordinate, or one arising from unsupported arithmetic or derivative omissions, is not an inactivity certificate. No near-zero threshold, small-area cutoff, rank estimate, singular-vector truncation or observed-error trigger selects this policy. Ineligible/unproven coordinates retain the ordinary path and rank requirement.

Compute the mask only after the existing full assembly. Do not change the finite-difference stencil, freeze wetness during mass probes, or skip derivative/evaluator calls. In particular the centered mass stencil at the full-wetness join and its nonzero dry-energy mass couplings must survive unchanged.

### Reduced subproblem with complete residuals

Let D be the certified coordinate indices and R their complement. In the current scaled subproblem choose `p[D] = +0` and solve using the original A's retained columns, keeping **all 21 rows and the complete residual f**. For the captured G4 matrix this is a 21-by-19 problem, not a 19-equation physical model. Preserve both drainage coordinates, their columns and capacity rows, every dry-energy row, W/S, physical/scaled bounds and all residual normalizers.

Keep a distinct assembly-owned held-direction mask. Do not rewrite physical bounds to equalities, masquerade held directions as physical bound activations, or overwrite the raw matrix. The full coordinate vector and physical state remain 21-dimensional. Reconstruct held coordinates by preserving their current base values, not assigning canopy-air/wet-reservoir temperatures or altering the original seed.

The local mathematical basis is exact: when `A[:,D]=0`, deleting a candidate's D components leaves `f+A*p` unchanged and cannot increase its Euclidean norm. At a valid base, zero step is inside each original coordinate bound. Consequently an optimizer with zero D components exists for this linearized box/trust-ball problem. This selects the zero-null-component representative; it does not establish nonlinear convergence or independence of the residual at other physical states.

Use the existing rectangular free-column Jacobi/SVD and active-face machinery with retained original-index traversal. Apply the unchanged `sigma_min <= 2^-40*sigma_max` refusal to the *remaining* free columns. Any unexplained remaining rank loss still refuses. No general pseudoinverse, singular-value clipping, diagonal damping, alternative side selection, new factorization algorithm or fallback is permitted. The reduced matrix may retain dependent rows; least squares must still see all of them.

Preserve the original full-vector norms, box/radius checks, all-coordinate BVLS-02 optimality assessment and all nonlinear/root/materializer predicates. A held direction is not exempt from final physical validation. At the certified assembly, its zero column and zero step imply zero first-order stationarity contribution; verify that rather than omitting its acceptance row. Structural holds must not be released as physical active bounds within that same assembly.

### Mask lifetime, phase selection and reactivation

Use the reconstructed full physical direction in the existing phase/capacity selector and selected-side probe. Preserve natural predictor first, at most one selected reassembly, its verification and all existing errors. Derive a fresh mask from each actual selected assembly; never reuse the natural mask merely because the base values are unchanged.

Radius retries retaining the identical admitted assembly may retain its mask. A new base or new Jacobian requires fresh eligibility. If an accepted mass/phase change restores any effective dry area or a temperature-sensitive column, that coordinate must become an ordinary unknown again. No mask persists across supports, owner states or restart by implication.

The physical evaluator must always recompute its original wet/dry fractions and all 21 residuals at trial candidates. A step that begins at full wetness may create dry area; it does not inherit a residual exemption from its linearization mask. Retaining a dry temperature for one null step is not a claim that physical tissue stays at that temperature while drying. Any unresolved need for new physical temperature/enthalpy history is an out-of-scope authority finding, not permission to invent an anchor.

## 4. Controls, implementation and the real G4 endpoint

Keep corresponding controls-before-body stages internal to this assignment. Reuse existing SVD, mask, refinement, observer and work structures rather than creating a second solver or diagnostic framework. Compact independent controls must discriminate:

- Retained G4 columns/mask and full-row reduction/reconstruction: full versus reduced objective and norm, exact held increments/base-coordinate bits, both drainage columns, complete energy/mass/capacity rows, and rank of retained columns. An independent oracle must not call the candidate to obtain expected results. A genuinely inconsistent residual or failed physical acceptance must remain failed even when the reduced subproblem has a direction.
- Eligibility from actual component/occupancy identities, full wetness and complete zero raw/weighted columns. Partially wet/near-fully-wet, structurally zero-area anchored, unexpected zero-column, unexpected remaining dependency, nonfinite/scaling and active-bound cases must not acquire a relaxed path. Prespecify test operands from the owning formulas, not from trial-and-error searches.
- Full-to-dry and dry-to-full mask changes, natural/selected assembly distinction, unchanged mask on same-J radius retry, physical trial residuals when dry area reappears, full-vector governed-step and root checks, and error-safe state restoration. Source/call evidence must cover actual mask consumers; a constant-mask report is not proof.
- Correct joins for all included roles/occupancies and deterministic ordering, no extra assemblies or residual calls, unchanged BVLS-03/BVLS-04 calculations when D is empty, exact zero/nonzero-lambda factor and refinement routing, and honest work/cap accounting with the reduced free dimension. Remaining unexplained rank deficiency must still produce its owning refusal.

Freeze a finite list of necessary new non-target assembly/control cases before result-bearing execution. Existing stopped physical measurements and target slots are not silently reused inside broad filters. Obtain affected debug/release/all-feature inventories, format/default/feature checks and source-bearing inherited-lint reconciliation. Correct introduced findings without suppression; the inherited strict populations stay failures under only their existing bounded policy. Applicable A0/A1/A3, conservation and Critical-trigger obligations remain.

After reviewed bodies, controls, quality and exact source/manifest readiness, this adoption permits **one planned release execution of the unchanged G4 endpoint under the new policy**:

`m1_coupled_tests::m1_trust_region_retained_phase_capacity_tie_uses_actual_model`

Preserve the original input and trial, canonical upper capacity preparation, upper `M=Cliq`, `H=+0`, `D=+0`, other coordinates, forcing and every required assertion. This is a new prospectively authorized method test, not a retroactively valid retry of BVLS-04. It must reach the intended real phase/capacity model, actual natural/selected assemblies and probes, both drainage columns, selected M/H/D/capacity/duration evidence and returned full direction. Merely passing the first rank check or producing a reduced direction is not G4 PASS.

Record mask lifecycle, full predictor/selected-direction custody, exact reached/unreached events, typed refusal and all induced work. No alternative seed, wetness nudge, different side, rank-cutoff change or weaker assertion after observing the result. A source-conforming numerical negative ends this fixed effort. An affected corrected verification requires an independently demonstrated implementation/recording deviation, with failed source and result preserved.

## 5. Cost, conditional continuation and preservation

Runtime cost remains a design requirement. Identify inactivity from existing preparation and bounded column checks, not runtime exact-rank arithmetic, a second SVD, probe sweeps or repeated full-state serialization. Count mask/scatter work, actual free dimensions, SVD sweeps/pivots, refinements and all owning evaluator/hydraulic/materializer operations. Keep full residual checks and existing ceilings. A smaller SVD is not itself a measured speedup; no extra physical evaluation is justified by mask discovery.

Measure the actual release G4 proposal/control interval, with initialization, build and observation cost separated and CPU resolution explicit. Preserve overlapping-ledger semantics. The historical millisecond results remain tied to their original source/inputs; this endpoint alone does not qualify seasonal or watershed scale.

Only after G4 and its prerequisites pass, continue the previously unrun, reviewed G2 and five G6 physical-owner cases plus G3 physical/harness obligations within the envelope. Revalidate affected method/source bindings without repeating unchanged investigations. Existing interfaces are already callable; do not restore the obsolete claim that 18 references are undefined. All required physical assertions remain real requirements. The original canonical target prefix/baseline/treatment/conditional repeat stay unspent, unreplenished and separately gated; no direct release follows from this amendment.

Preserve all historical failures, root/capacity/rank attribution, prototype cadence disclosure, archives and unrelated dirty/staged/untracked work. Scoped reviewed authority, package/evidence, necessary recorder changes and exact detached recovery may be published to already-authorized main. No main Rust adoption, branch switch, new dependencies, general cold-physics expansion, physiological/domain relaxation, state/reset/seed substitution, WAT5/Grid40/E008/old-reader work, live parent/receiver/cycle progression or M1 acceptance.

Return the exact adopted structural policy, reduction/activation evidence, real G4 outcome and measured work first, then final-source validation, reviewer scope verdicts, any conditional progress, recovery/publication and conservative ledger. A policy review or matrix PASS is not a completion milestone while authorized implementation and allowance remain.

## Adviser evidence boundary

The adviser inspected the pinned G4 package, complete independent correctness/QA reviews, canonical vegetation M1 wetness/conservation text, current numerical KKT rules and matching prior predictor/rank authority, closing ledger and preceding G4 diagnostic authorization. The captured matrices and detached runtime were not independently reconstructed or executed. Direct raw-file transfer to the calculation environment failed on DNS resolution; GitHub connector reads supplied the cited evidence. The original draft patch was read only to locate canonical M1 text, not treated as authority. The structural reduction above is a prospective design requiring independent review, not already validated implementation. Only retrieval, elementary budget arithmetic and this handoff creation ran.

Pinned package: https://github.com/rogerlew/openWEPP/blob/d7ad407e956e3ecea716db6744f4fd2bbe14ed61/docs/work-packages/20260920-cold-canopy-m1-001/package.md

Numerical background only: LAPACK's linear least-squares guide distinguishes full-rank and minimum-norm rank-deficient formulations. This proposal does not import LAPACK, its rank tolerance or a general pseudoinverse: https://www.netlib.org/lapack/lug/node27.html
