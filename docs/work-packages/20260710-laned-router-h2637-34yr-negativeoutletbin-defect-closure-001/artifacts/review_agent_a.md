# Independent Review A

Static: reviewed the complete current working-tree diff, `SC-OFEROUTE-001`
rev 51, the package plan and execution artifacts, the solver update/ledger/bin
path, the active direct-runtime call chain, applicable `AGENTS.md` files, and
the DC/kernel-package governance standards.

Ran: `cargo nextest run -p openwepp-hillslope-orchestrator
source_quiet_dry_front_outlet_flux_stays_nonnegative_and_conservative
bin_recorder_retains_material_terminal_deficit_signal --no-capture` passed
`2/2` (run `88b7b26e-30c3-4caa-a465-eeb419d97040`); `git diff --check`
passed; the SC binding-exposure checker returned its defined
`PASS-DEFERRED` posture; SC unit-compliance lint passed. I independently read
both retained 34-year manifests, confirmed binary SHA-256
`a822036fd327c2f54d877ab51dc6c2e9aae13accff2ad4a61c154cbd730a131d`,
effective UI modes `0/0` and `1/1`, direct-production selection, active closure
values, and zero skeleton/compatibility-edge counts. I also reran `cmp` and
SHA-256 checks for all five retained disabled-path pre/post outputs; all five
are byte-identical. I did not duplicate the recorded full workspace run.

## Findings

### Medium — the contract regression does not observe the stage-face invariant

`SC-OFEROUTE-001.md:616` requires the rev-51 vector to complete with **every
scheme-actual face** nonnegative. The test at
`crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs:2173`
uses `run_with_options`, which disables step trace
(`kinematic_wave.rs:1501-1509`), then asserts only aggregate booked outflow,
bins, hydrograph samples, and closure (`kinematic_wave.rs:2176-2209`). Those
checks can still pass if a future negative predictor face is offset by a
positive corrector face, because both the update and ledger use their mean.
The raw negative candidate computed at `kinematic_wave.rs:2150-2151` proves the
fixture setup but does not observe the scheme-actual bounded face.

Before closure, run the vector through
`run_with_options_and_step_trace(..., true)` and assert every retained
`pred_out_face_m2_s` and `corr_out_face_m2_s` is finite/nonnegative. Pin the
first predictor face to exact positive zero for this constructed raw-negative
state. This closes the amended invariant directly without weakening the
existing independent storage reconstruction.

### Medium — consumer-path evidence omits the required exact call-site map

`docs/work-packages/AGENTS.md:74-83` requires an endpoint package artifact to
name the producer source, in-memory frame, runner handoff, downstream consumer
call site, output/API surface, and a “what still reads the old path?” negative
check. `artifacts/fidelity-and-byte-identity.md:9-16` gives a conceptual chain
and strong manifest counters, but it does not name those concrete source/frame/
consumer bindings or record the static old-path check. Counters and successful
outputs substantiate execution, but they do not replace the required map.

Amend that artifact with the concrete chain already present in source:

- `DirectRunFrame::run_laned_active_publication_stream` selects the active
  two-phase path (`direct_runtime/03_executor.rs:312-318,427-448`);
- `DirectDayFrame` carries the source and receives
  `DirectLanedActiveDayRouting`; the executor calls `laned_active_route_lane`
  (`03_executor.rs:502-522,577-593`);
- `laned_active_route_lane` calls `route_single_ofe_with_step_trace`, then sets
  `DirectErosionHydrographShapeAuthority::RoutedHydrograph`
  (`direct_runtime/laned_active.rs:942-977`);
- `route_single_ofe_with_step_trace` constructs `KinematicWaveSolver` and calls
  its production run entry point (`ofe_routing/cascade.rs:247-268`);
- the routed weights are consumed at
  `direct_runtime/erosion.rs:402-445`, after which the executor builds and
  consumes `DirectPublicationDayRow` and commits the frame
  (`03_executor.rs:634-687`).

Also record the negative check over the DC01/compatibility selectors and cite
`laned_active_assert_no_dc01_surface_feed` plus the retained manifest evidence
(`selected=direct-production-executor`, `compatibility_edge_invocations=0`,
`skeleton_runs=0`).

## Review Assessment

- Mechanism attribution is specific and internally consistent: the captured
  source-quiet terminal steps isolate a negative
  `2 q[n-1] - q[n-2]` predictor face, negative booked outflow, compensating
  storage increase, and the correct terminal-bin fail-closed site.
- Rev 51 is proximate authority and preserves the physical one-way boundary.
  The implementation checks finiteness, applies the exact-zero boundary domain
  before the rev-41 available-water cap, and uses the same face in the state
  update and ledger. I found no clamp-mass injection, tolerance widening,
  damping, surrogate physics, guard weakening, future-bin borrowing, hybrid
  revival, or publication-only masking.
- The `NegativeOutletBin` branch remains unchanged. The separate recorder test
  retains its material deficit signal; static inspection confirms the public
  run path still maps that signal to the typed error.
- Operand lineage distinguishes raw extrapolation, scheme-actual faces,
  committed outlet discharge, bins, ledger outflow, and independently
  reconstructed storage. The focused reconstruction plus real endpoint/day
  closure and five-output off-path identity are not a producer-formula-only or
  one-sided acceptance case.
- The correction authority envelope includes all plausible routing, seam,
  snow-boundary, runtime, and test surfaces. Attribution lands inside it; the
  protected snow/hybrid/off-path boundaries do not shield an in-scope fix.
  The package is not claiming `HOLD`, so no hold-legitimacy artifact is needed.
- Required repository, contract, oracle/19-OFE, selected-cohort, endpoint, and
  byte-identity gates all have direct current evidence. No gate is deferred to
  a later increment. Final doc lint appropriately remains to be rerun after
  review/verification artifacts land.
- `kinematic_wave.rs` is `2548` lines: `WARN`, not the `3000+` closure blocker.
  `line-count-governance.md` supplies a proportionate decomposition rationale,
  owner class, and follow-on split intent.

## Verdict

`PASS-WITH-FINDINGS`.

I found no production-correction or science-authority defect. The two findings
are current-scope test/evidence obligations and must be accepted and fixed (or
rejected with specific counter-evidence), then independently verified before
final package disposition.
