# HOLD Legitimacy Audit

Status: **EXECUTED-HOLD-TIMING-ACTIVE-PATH**.

Evidence mode: Ran + Static.

## Hold Conditions

### HOLD-1: D10B-corrected H2637 shadow timing path fails before endpoint

Ran:

- Release build completed.
- Default/off native H2637 run completed: user `2.58 s`, wall `0:02.60`.
- `OPENWEPP_LANED_SHADOW=1` native H2637 run failed: exit `1`, user
  `20.05 s`, wall `0:20.06`, `NegativeOutletBin`.
- `OPENWEPP_LANED_SHADOW=1 OPENWEPP_LANED_SHADOW_PROFILE=1` failed the same
  way before emitting `laned_shadow_profile`.
- Focused ignored H2637 evidence test failed the same way:
  `cargo nextest run --test laned_shadow_h2637 --run-ignored ignored-only --no-capture`.

Additional Ran evidence from a temporary diagnostic-only error-context patch
(not retained): the failing buffered event is day `88`, full one-day window
`86400 s`, source active through 1-based hour `24`.

Why this is legitimate: D15's first phase requires a valid endpoint timing
refresh after D10B. The candidate path currently cannot complete. Extending,
truncating, backward-redistributing, or reshaping the terminal hydrograph to
make timing pass would change routed-hydrograph timing/closure semantics and
the future 24-slot D13 consumer surface. That needs contract-authorized
day-boundary / inter-day carry handling, not a timing-package shortcut.

First actionable follow-on: open a Lane D terminal-bin/day-boundary hold-lift
package. It must close the H2637 day-88 `NegativeOutletBin` on the D10B
conservative bin-series path with contract-backed semantics for a source active
in hour 24, prove non-negative exact-total handoff/publication, rerun the
H2637 timing refresh, and state how any over-day tail or carry maps to the
24-hour active erosion shape.

### HOLD-2: Active production owner path is absent

Static:

- `ofe_routing.rs:5-7` states the current routed subsystem has diagnostics-only
  shadow wiring and no production phase-span wiring.
- `05_runner_execution_and_outputs.rs:89-98` creates only the optional
  `OPENWEPP_LANED_SHADOW` collector.
- `runoff.rs:205-209` still unconditionally calls
  `apply_dc01_runon_supply_admission()` in the production WB14 path.
- The DC01 mutual-exclusion and closure helpers exist in `seam.rs`, but the
  active production runtime does not invoke them.
- The D13 consumer exists, but production builder inputs still set
  `Dc01SourceShape` and `routed_hydrograph_runoff_fraction: None`.

Why this is legitimate: The package cannot truthfully claim opt-in production
activation while the real active downstream consumer does not exist. Building
that consumer while the timing path fails would create a partial flip and would
violate the package's no-partial-activation rule.

First actionable follow-on after HOLD-1 is lifted: open the active-owner
implementation package that wires an explicit opt-in active selector, moves
surface-water ownership to the routed path, disables DC01 daily-lump runon on
active lanes, constructs active closure operands (`ui_SCrunf`, `latqcc` bypass,
storage/ET/deep-perc terms), hard-fails material closure residuals, feeds the
D13 routed-hydrograph shape, and proves default/off byte identity.

## Considered In-Envelope Route

The direct route would be to implement Phase C in this package. That route was
rejected because Phase A fails before endpoint timing and Phase B proves the
real active consumer path is absent. Proceeding would either ignore the timing
blocker or create a shadow/producer-only activation claim, both explicitly
disallowed by `docs/work-packages/AGENTS.md`.
