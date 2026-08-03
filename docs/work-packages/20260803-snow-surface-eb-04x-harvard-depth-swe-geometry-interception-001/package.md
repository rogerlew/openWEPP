# SNOW-SURFACE-EB-04X Harvard Depth-SWE Geometry And Interception Investigation

Status: `complete / technical, review, and verification pass / no promotion`

Date: `2026-08-03`

Campaign: `SNOW-SURFACE-EB`

Plan class: `Retained-evidence scientific diagnostic / authority-bound characterization`

## Purpose

Use the paired Harvard HF237 open and hardwood observations plus the retained
EB-04 B/L/S/LS outputs to separate algebraic depth-SWE-density consistency,
density trajectory, snowfall-input identity, and residual forest/open energy
contrast. Determine whether the evidence identifies canopy snow interception,
unloading, drip, or sublimation, without inventing absent production physics.

## Implementation Intent

Intent: `diagnostic characterization + successor-gate design`. No production
implementation, calibration, coefficient fitting, threshold change, or default
activation is authorized.

## Authority Boundary

- HF237 open and hardwood observations are empirical diagnostic evidence.
- The committed EB-04 factorial result is the retained model evidence; ignored
  target traces may be consumed only when their committed hashes reproduce.
- `SC-SNOWFREEZE-001` governs current ground-snow behavior. openWEPP has no
  canopy-snow-load state, interception, unloading, or canopy sublimation path.
- Harvard hardwood is winter leaf-off. A paired residual may motivate a future
  process package but cannot uniquely identify or calibrate interception.
- The Harvard observed SWE field has prior provider-identity concerns. Algebraic
  SWE-density-depth closure is tested before any SWE-based scientific claim.

## Included Scope And Write Set

- this package tree;
- `docs/ROADMAP.md`;
- `docs/planning/snow-surface-energy-balance-roadmap.md`;
- `docs/work-packages/README.md`.

All production Rust, contracts, tests, fixtures, observations, predecessor
packages, retained results, and target traces are read-only.

## Frozen Diagnostic Rules

1. Analyze only `harvard_open` and `harvard_hardwood`, with B/L/S/LS meanings
   inherited unchanged from EB-04.
2. Observed algebraic density is `observed_swe_mm / observed_depth_m` in
   `kg m^-3`; nonzero rows close only within `1 kg m^-3` of supplied density.
3. Modeled algebraic density is `1000 * SWE_m / depth_m`; closure tolerance is
   `1e-6 kg m^-3`.
4. Open is the geometry/forcing control. Compare paired dates only; report
   hardwood-minus-open deltas without treating association as causation.
5. Trace snowfall identity uses daily `accumulation_m` and precipitation/phase
   fields from hash-matched B and LS traces. Different ground snowfall under
   identical meteorological precipitation is reported as model behavior, not
   canopy interception, because no canopy-snow state exists.
6. Component promotion predicates are frozen before results: longwave requires
   a residual hardwood improvement with open noninferiority and valid geometry;
   sublimation requires open and hardwood noninferiority plus closed latent
   mass/energy; combined interaction requires both component predicates and a
   non-compensating interaction. Invalid observed SWE geometry makes every
   SWE-bearing promotion predicate `NOT_EVALUABLE`.
7. No result-aware rule change, new model run, or new process equation.

## Phase Plan

1. Scaffold and hash-bind instructions, observations, factorial evidence, and
   target trace identities.
2. Reconstruct observed and modeled algebraic closure and paired trajectories.
3. Compare B/L/S/LS ground-snow inputs, storage, loss, sublimation, and energy;
   bound what can and cannot be attributed to interception.
4. Apply the frozen component promotion predicates and produce synthesis.
5. Run deterministic regeneration, independent reconstruction, documentation,
   exact-diff, security, review, verification, and final disposition gates.

## Acceptance Criteria

1. Input identities are frozen before analysis and every consumed file matches.
2. Every nonzero observed HF237 open/hardwood row is algebraically classified;
   invalid SWE identity is quantified rather than silently normalized.
3. Every retained modeled sample pair closes SWE-density-depth geometry or the
   package holds with the exact failing rows.
4. B/L/S/LS paired forest/open differences and trace totals are reconstructed
   without changing model inputs or rerunning production.
5. Interception/storage/unloading/drip/sublimation claims match available state:
   absent operands remain `NOT_OBSERVED` and cannot carry causal conclusions.
6. Longwave, sublimation, and interaction gates are applied prospectively and
   reported as `PASS`, `FAIL`, or `NOT_EVALUABLE`.
7. Dual independent reviews, finding disposition, dual verification, prompt
   lifecycle, exact-diff, line-count, security, and roadmap/catalog gates pass.

Any unmet current-scope criterion forces `HOLD`.

## Validation And Delegation

No Rust regression is selected because production and test paths are read-only.
Run Python syntax/self-check, deterministic regeneration, independent summary
reconstruction, JSON/CSV/schema checks, scoped Markdown lint, `git diff --check`,
and protected-path empty-diff checks.

Subagent authorization: this package explicitly authorizes spawning/delegating
to two independent read-only science/governance review agents and two read-only
terminal verification agents. Expected outputs are their named package-local
artifacts; write access is restricted to that single artifact per agent.

## Security Impact

No network, secret, authentication, dependency, unsafe Rust, subprocess model
execution, external write, or public schema change is authorized.

## Progress

- [x] (2026-08-03) User authorized scaffold and autonomous execution.
- [x] Freeze retained inputs and execute the diagnostic.
- [x] First dual review found missing trajectory/phase work, asynchronous peak
  aliases, incomplete guards/protocol, and bytecode; all findings accepted.
- [x] Executed isolated terminal-v2 with prospective quantitative screens,
  profile/trajectory analysis, daily phase-input identity, guarded pairing,
  same-day extrema, rejected aliases, and fail-closed output handling.
- [x] Fresh dual re-review accepts every correction with no remaining finding.
- [x] Dual terminal verification independently reproduces all hashes,
  inventories, operators, provider contradictions, causal limits, and
  no-promotion disposition; final lifecycle and exact-diff gates pass.
- [x] Complete dual review, dual verification, and final disposition.

## Outcomes

All eight retained model traces close SWE-depth-density geometry. The HF237
provider fields do not: all 336 complete open rows and all 410 complete hardwood
rows violate the supplied SWE/depth/density identity. The model exposes ground
snowfall, pack state/loss, ground sublimation, and energy, but no canopy load,
intercepted snowfall, canopy sublimation, unloading, or drip. B/L pair over the
full record; S/LS have asymmetric early termination and are common-prefix only.

Consequently, Harvard does not provide an identical-forcing, operand-complete
interception experiment, and all three frozen component promotion gates are
`NOT_EVALUABLE`. No promotion or process implementation is authorized.
