# Independent Review A: Growth-State CRAP Regression

Evidence class: **Static + Ran**

Initial recommendation: **PASS** for the bounded growth extraction.

## Findings

No blocking or nonblocking implementation finding.

## Contract And Numeric-Identity Review

The terminal diff moves one complete root mass/root depth candidate block from
`DirectGrowthInputs::compute_equation_growth_state` into the private
`compute_root_mass_and_depth_candidates` helper
([growth.rs](../../../../crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs):658).
The moved comparison, branches, tuple order, arithmetic expressions, and
grouping are text-identical.

Specifically:

- SC-PLANT-001 `INV-PLANT-027` remains first in the root-update block: perennial
  `root_mass_kg_m2 >= rtmmax` is tested before any incremental root-mass
  expression or division by `rtmmax`.
- The saturated branch still returns `(rtmmax, min(rdmax, soil_depth))`, because
  `rtd_upper` is computed and positively validated before helper invocation.
- The incremental mass expression, perennial clamp, annual nonnegative floor,
  root-depth division, multiplication, addition, and `.max(rtd_floor)` retain
  their original evaluation grouping and branch order.
- Error order is unchanged: `growth.root_depth_upper_bound_m` is validated
  before the candidate computation; `growth.root_depth_candidate_m` is
  validated after it; final state validation and publication order remain in
  the caller.
- The helper adds no allocation, clone, `Result`, fallback, tolerance,
  conversion, or public surface. Passing the `Copy` input value into the private
  method does not alter computed operands.

## Ran Evidence

- Independent focused Nextest selection passed `3/3`: the exact-zero perennial
  cap vector, the ordinary annual production path, and the ordinary positive-cap
  perennial production path (run id
  `ea860040-b386-4a99-a87e-753ad48268ff`).
- The fresh post-extraction artifact
  `/tmp/openwepp-acrap-terminal-20260713/workspace-crap.json` is newer than the
  source file and reports:
  - `compute_equation_growth_state`: CC `27`, coverage `97.22222222222221%`,
    CRAP `27.015625`;
  - `compute_root_mass_and_depth_candidates`: CC `5`, coverage `100%`, CRAP
    `5`.
- Independent adjudicated assessment of that artifact returned `PASS` with
  `2` raw, `2` adjudicated, `0` actionable, and the growth file in the touched
  production set.
- The terminal source SHA-256 is
  `1ce345e533159d7317f8c7d1a5f41b292a27896aa53d8e10d693d6366a6eb041`,
  matching the implementation artifact. `git diff --check` and the focused gate
  unit suite also pass.

## Recommendation

The growth refactor itself satisfies `CQR-GR-002` through `CQR-GR-004` and is
safe to retain. Package closure still depends on durable terminal heavy-gate
evidence and disposition/verification of the separate gate-package findings;
those dependencies do not reveal a defect in this extraction. No source edits
were made by Reviewer A.
