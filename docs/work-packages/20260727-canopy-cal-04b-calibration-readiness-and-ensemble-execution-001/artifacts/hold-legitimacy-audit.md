# HOLD Legitimacy Audit

Status: `PASS / OUT-OF-ENVELOPE PRODUCTION DEFECT`

Evidence class: `Ran + Static`

## Boundary

The current package authorizes writes only to its documentation/scaffold and
the two roadmap catalogs. Production code, science contracts, protected
fixtures, and admitted source observations are read-only. Attempt 004 proves
that the frozen interior calibration vector cannot traverse the real production
consumer because phenology publishes positive LAI while the post-growth path
publishes missing or non-positive canopy height.

This is defect `CAL04B-NATIVE-001`: production phenology-to-post-growth
LAI/canopy-height coupling is not closed across the admitted calibration
domain.

## Evidence

- `execution-incident-004.md` binds the failed observed receipt, exact guard,
  copied configuration, trace extent, and hashes.
- The package requires representative interior, boundary, saturated, and
  invalid real-consumer proof before population execution.
- The package declares a broken real parameter path a hold boundary.
- `SC-PLANT-001` requires coherent LAI/canopy-height publication and real
  downstream consumption; the fail-closed guard must not be weakened.
- Source review confirms that the generalized-GSI override updates current
  foliar biomass, interception biomass, LAI, and cover after baseline growth
  state is computed, but does not recompute current canopy height. The exact
  anchors are `growth_state_for_build` and
  `native_forest_growth_state_for_build` in
  `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs`
  (current lines 413–426 and 600–607), plus the baseline height calculation in
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs`
  (current lines 781–786).
- Independent implementation, QA, and science disposition rejected
  candidate substitution or fixture manipulation as post-failure evasion.

## Considered In-Envelope Routes

The proof serializer, binary resolution, command observation, and copied-input
injection were inspected because earlier incidents occurred in those
package-local surfaces. They are not the cause here: native-default completes,
the interior production process itself returns the typed runtime failure, and
no proof receipt exists to compare.

Changing the interior selector, shortening the fixture, forcing a canopy
height, using only the direct GSI kernel, or suppressing the guard would make
the proof easier without proving the required production path. Those routes
are rejected.

## Why This Package Cannot Close the Defect

The supported correction must change and verify production runtime coupling
under the applicable science-contract authority. Those files and that edit
class are explicitly outside this package's declared write set. The failure is
therefore a legitimate out-of-envelope boundary, not deferred in-envelope
implementation.

The package closes in `HOLD`, with Harvard sealed and no population result.
The separately scaffolded
`20260727-cal04b-native-gsi-canopy-height-coherence-hold-lift-001` package
defines the defect-closure target and acceptance needed before CAL-04B can be
reopened or superseded.
