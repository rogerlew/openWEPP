# Review - Codex Code Correctness

Status: **EXECUTED** (2026-07-07).

Evidence mode: **Static**. Reviewed current `main` at `64d57f51`, execution
commit `bd64d2c8`, package artifacts, `SC-OFEROUTE-001` rev 28 text, and the
LANED-T3 implementation. Subagent evidence: `rust_code_reviewer` and a narrow
`explorer` for Filippov test coverage. No tests or gates were run by this
review.

Package verdict: **NO-GO** for settling LANED-T3 / rev-28 as closure-grade.
The strict hybrid selector can remain experimental evidence only, but the
primary code-correctness lane finds two High issues in the branch selection
and Filippov closure chain.

## Findings

### High

- **T3-H1 - Filippov jump can still mask a real branch-solve failure.**
  `crates/openwepp-hillslope-orchestrator/src/ofe_routing/implicit_recession.rs:347`
  returns `CellSolve::Jump` when the depth bracket collapses, but it does not
  prove the collapse is at the rating discontinuity, evaluate both branches at
  `h_jump`, or prove `q_jump` lies inside the branch-equilibrium convex hull.
  `crates/openwepp-hillslope-orchestrator/src/ofe_routing/implicit_recession.rs:267`
  then commits the low-branch jump if the high branch also reports `Jump`.
  A continuous branch root missed by tolerance, fixed-point jitter, or bracket
  management can therefore become a mass-exact but physically wrong Filippov
  commit. Existing retained tests cover ledger/positivity, upstream inflow,
  steady state, and recession ladders, but no direct vector forces and asserts
  the LOW -> HIGH -> Filippov no-root case.

- **T3-H2 - Steffensen basin determinism is seed-biased, not branch-locked.**
  `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs:268`
  accepts only `q_seed`; the requested LOW/HIGH branch is not an input to
  `equilibrium_discharge_converged`. The acceleration guard at
  `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs:329`
  compares the accelerated point to `q2`'s side of `Q_c`, not to the requested
  branch or initial seed side. If `q1`/`q2` cross basins, a low-branch call can
  converge to the high branch, invalidating the LOW -> HIGH -> Filippov guard
  chain.

### Medium

- **T3-M1 - Package text overclaims I2 closure/fidelity ratification.**
  `docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/package.md:7`
  says all closure gates are green, and
  `docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/package.md:56`
  requires the full hybrid Case-4 oracle ladder. The evidence states the
  selector is evidence-gathering and tolerances remain UNRATIFIED in
  `docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/artifacts/i2-hybrid-evidence.md:55`,
  matching
  `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md:480`.
  Amend the package status/acceptance wording before treating rev 28 as
  settled.

- **T3-M2 - Direct Filippov regression vector is missing.**
  The high-risk path is
  `crates/openwepp-hillslope-orchestrator/src/ofe_routing/implicit_recession.rs:253`
  through
  `crates/openwepp-hillslope-orchestrator/src/ofe_routing/implicit_recession.rs:359`.
  Existing unit tests at
  `crates/openwepp-hillslope-orchestrator/src/ofe_routing/implicit_recession.rs:421`,
  `:443`, `:460`, and `:581` cover surrounding properties but not a direct
  LOW -> HIGH -> Filippov branch. H2637/hybrid evidence exercises the code
  only implicitly.

### Low

- **T3-L1 - Dust-floor residual guard is plausible but under-pinned.**
  `crates/openwepp-hillslope-orchestrator/src/ofe_routing/implicit_recession.rs:171`
  floors the residual scale at `DRY_DEPTH_M * dx * n`. That is defensible as
  a per-step dust floor, but no retained long dry-tail or short-mesh
  accumulated-residual regression proves repeated dust steps cannot hide a
  material leak class.

## Adversarial Questions

1. **Filippov closure correctness:** not complete. The implementation needs
   branch-gap validation and direct no-root/jump tests before rev-28 closure.
2. **Basin-split determinism:** not proven. The current API is seed-biased;
   it does not branch-lock the converged equilibrium.
3. **Hybrid span composition:** the strict current path looks coherent:
   `set_state` / `discharge_state`, global bin offsets, final storage, and the
   all-explicit bit-identity scope are sane.
4. **Dust floor:** acceptable only as a dust-scale rule; add accumulation
   coverage.
5. **Aggressive-rule defect:** confirmed. Deficit carry across span boundaries
   is the right fix shape if it preserves exact totals and keeps
   `NegativeOutletBin` fail-closed for unabsorbed material deficits.
6. **No perturbation:** static dispatch isolation holds. Hybrid is behind
   `OPENWEPP_LANED_ACTIVE_IMPLICIT=1`, and `route_single_ofe` remains the
   non-hybrid path.
7. **Fidelity posture:** contract text is honest about EXPERIMENTAL /
   UNRATIFIED posture; the package artifacts need amendment where they imply
   ratified closure.
