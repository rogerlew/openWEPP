# Phase 0 Residue-Mass Characterization

Evidence mode: Ran + Static.

## Result

Branch: `MASS-NOT-SEASONAL-NO-INPUT-ZERO-DECAY`.

The existing Hubbard Brook `Dec_*` fixture does not already provide a seasonal
surface-residue mass trajectory:

- Static: `tests/fixtures/cancov_forest/hubbardbrook_deciduous_nh/p10.man:9`
  identifies `Dec_4899`.
- Static: `p10.man:18` has `oratea=0.0` and `orater=0.0`.
- Static: `p10.man:41` seeds `sumrtm=0.5 kg/m2` and
  `sumsrm=0.5 kg/m2`.
- Static: `hydrology/07_decomposition_equations.rs:139-160` only decays and
  annual action-adjusts the existing surface pool.
- Static: `hydrology/07_decomposition_equations.rs:161-194` only applies
  perennial grazing removal; it has no recurring litter input.
- Ran: `.venv/bin/python
  docs/work-packages/20260629-frost-residue-cover-implementation-001/artifacts/phase0_residue_mass_characterization.py`
  emitted `phase0_residue_mass_summary.json` and
  `phase0_residue_mass_monthly.csv`.

## Monthly Surface Residue

The package-local characterization records the first-year monthly producer mass
as constant at `0.5 kg/m2`; no autumn peak, winter/spring decline, or recurring
leaf-drop input exists in the current fixture/producer path.

## Failed Real-Run Attempts

Ran: `cargo build -p openwepp-runner --bin openwepp-cli-hill` passed after the
diagnostic trace enrichment.

Ran: a 45-year compatibility run with
`OPENWEPP_R7H_COMPAT_LAYER_TRACE_PATH` was started against the Hubbard Brook
deciduous fixture. It produced no trace file before being terminated as a
stalled evidence attempt.

Ran: a shortened 3-year copy was rejected by the management parser because
`sim_years` must match the declared 45-year schedule. The fixture was not
mutated.

## Branch Decision

The package must implement the litter-input limb before or with dynamic
mass-to-depth wiring. Pure wiring would propagate a flat residue mass and would
not close the Step 3 branch C root cause.

