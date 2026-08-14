# Review Agent A — Closure Rust Correctness Review

Evidence class: `Static exact-commit + Ran exact-commit`

Reviewed commit: `ab703c83abd4af22cecd956f37daf1fdf0b8152d`

Verdict: `HOLD / NO-GO`.

## Findings

### High — Critical exact-head full-workspace correctness evidence remains unmet

The package is a critical cross-owner, restart, serialization, conservation,
and shared-WB14 change. Its own acceptance language and
`docs/standards/testing-and-gate-strategy.md` therefore require current
campaign-strength full-workspace correctness evidence. The only retained
full-workspace Nextest attempt remains the failed run at `e82ba462a`: it stopped
after 84 passes when the advisory authority-map expectation had not yet been
updated, leaving 2,710 selected tests unexecuted. The package records that the
binding was corrected, but no later exact-head full-workspace Nextest PASS is
present, and no prospective campaign deferral assigned this current-scope
critical obligation before implementation.

Focused tests cannot replace that evidence. Under the gate-evidence
non-deferral rule, exact commit `ab703c83a` cannot close until the canonical
full-workspace correctness regression passes at exact head (or the package is
truthfully held on a demonstrated external blocker).

### Medium — Exact-head workspace strict Clippy fails

Ran:

```text
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Result: `FAIL` at exact commit `ab703c83a` with three warnings-denied errors in
`tests/integration/land_surface_energy_real_hydrology_shadow_contract.rs`:

- line 1090: `unified_entry_rejects_frozen_snow_and_duplicate_legacy_surface_custody`
  is 113 lines (`clippy::too_many_lines`);
- line 1192: `.as_ref().map(String::as_str)` violates
  `clippy::option_as_ref_deref`; and
- line 1444: `unified_receiver_join_poisons_return_no_partial_candidate` is
  111 lines (`clippy::too_many_lines`).

This is not a science-arithmetic defect, but it is a direct required source-
quality failure on package-owned regression evidence. The package's current
claims of strict Clippy PASS cover the affected crate command, not the required
workspace/all-target/all-feature command, and therefore do not close the
critical boundary.

## Accepted-finding re-evaluation

Static review found no remaining material Rust/science defect in the two latest
corrections or in the retained public custody path:

- E004 preflight occurs before authorization and before the fixed-cap callback.
  It rejects snow runtime state, retained-snow-liquid-only state, snow carry,
  frost/thaw runtime state, frost carry, and any positive production-layer
  `frozen_depth_m` or `frozen_water_m`. The first offending production lane is
  retained as hydrology owner/OFE context, with configured tile/surface/source
  identity and the beginning snapshot hash. Candidate work is not entered.
- Same-store authorization now fails closed on nonfinite demand accumulation,
  supply multiplication, proportional numerator, division, remainder, and
  allocated-total accumulation. The retained arbitration validator calls the
  same inner authorization reconstruction, so a forged resource candidate
  cannot bypass the finite-intermediate checks. The finite-overflow and adjacent
  large-finite control vectors exercise both boundaries.
- Proportional authorization is re-derived from immutable beginning supply and
  exact demand identity; resource application retains and validates `D/A/F`,
  checks `0 <= F <= A <= D`, and debits finalized use only.
- Configuration and state canonical bytes are deterministic, strict,
  configuration-bound, digest-sensitive, and validated before emission.
  Initial versus accepted continuation/lineage combinations fail closed.
- Arbitration, resource, ingress, finalization, and unified candidates are
  externally sealed. Ingress candidates reconstruct from their immutable
  inputs; the unified candidate joins its duplicated surface ending to the
  sealed ingress ending before exposure.
- Signed condensation is validated with exact transaction/owner/basis,
  positive mass, admitted temperature, and bit-identical specific liquid
  enthalpy. Credit precedes capacity overflow; overflow becomes post-resource
  timed ingress and cannot satisfy same-interval authorization.
- The shared WB14 interval transition is used by both the production daily
  wrapper and the persistent 1,800-second continuation. No second Green-Ampt
  transcription or duplicated constitutive transition remains.
- Independent surface closure reconstructs complete store equations and parcel
  mass/enthalpy/routing joins from frozen beginning/input operands. Receiver
  closure reconstructs ordered production-layer infiltration, production
  aggregate water including residual water over unfrozen depth, soil-thermal
  enthalpy, and retained LSE enthalpy.
- E011 receiver-expectation, final receiver, and exact three-owner rollback
  validators retain the first actual offender, or the exact expected missing
  receiver/owner. Missing non-terminal rollback rows are detected by expected
  membership before shifted-row comparison. Applicable OFE/tile context is
  present for tile-scoped failures and typed absence is retained for owner-wide
  rollback failures.
- All candidate operations use clones; successful and failed shadow attempts do
  not mutate the supplied production frame. Both normal `DirectRunFrame`
  constructors set `surface_liquid_shadow=None`. Repository search found no
  runner selector, production scheduler/default, publication, or activation
  consumer for `execute_unified_real_hydrology_shadow`.

## Ran at exact commit

```text
cargo nextest run --profile quick \
  --test surface_liquid_hydrology_custody_authority_contract \
  --test land_surface_energy_real_hydrology_shadow_contract
27 passed / 0 skipped

cargo nextest run --profile quick \
  -p openwepp-hillslope-orchestrator -E 'test(/surface_liquid/)'
32 passed / 507 skipped

cargo clippy --workspace --all-targets --all-features -- -D warnings
FAIL / 3 warnings-denied integration-test findings
```

## Line-count governance

Exact counts match the retained artifact: `runoff.rs` 2,852;
`00_core_frames.rs` 2,783; `surface_liquid_owner.rs` 2,990;
`vegetation_real_hydrology_shadow.rs` 2,157;
`surface_liquid_ingress.rs` 1,959;
`land_surface_energy_shadow/mod.rs` 2,582;
`surface_liquid_closure.rs` 769; and `surface_liquid_wb14.rs` 303. No affected
Rust file reaches the mandatory 3,000-line threshold. Every 2,000-line WARN
file has a package disposition. `surface_liquid_owner.rs` remains only ten
lines below the hard threshold and is a near-term decomposition risk.

## Residual risk and missing tests

- A passing exact-head full-workspace Nextest run is missing after the retained
  authority-map failure was corrected.
- Workspace strict Clippy must pass after the three package-owned integration-
  test findings are corrected.
- The E004 matrix combines positive frozen depth and frozen water in one vector;
  the independent single-field branches are statically explicit, but separate
  positive-depth-only and positive-water-only public-entry poisons would make
  regression localization stronger.
- This closure review did not run workspace doctests, `cargo deny`, or a legacy
  comparator. Retained earlier doctest/deny evidence predates the terminal
  implementation corrections; comparator agreement is not standalone
  correctness authority.

## Approval statement

`NO-GO`: the accepted numerical, domain, serialization, sealing, restart,
ingress, D/A/F, condensation, receiver, rollback, error-context,
nonactivation, and line-governance findings are materially closed on static
inspection and focused execution. Package closure remains blocked by the
unmet exact-head critical full-workspace regression and the exact-head strict
workspace Clippy failure.
