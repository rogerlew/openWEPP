# Pre-Execution Review - Claude

Status: COMPLETE. Evidence mode: Static.

Reviewer lane: Claude Code (cross-WP continuity). Reviewed: this package's
`package.md`, kickoff prompt, and fixture-cohort plan; the current
`SC-OFEROUTE-001` mesh/acceptance surfaces (INV-OFEROUTE-006/011, the rev-27
window/reset and erosion-shape rows); and the production mesh construction
(`laned_active.rs`, `laned_shadow.rs`, `kinematic_wave.rs`,
`d10b_reconciliation_tests.rs`). All cited required-reading paths were
verified to exist. No commands beyond file/git reads were run.

## Verdict

The scaffold's governance shape is right: contract-first tolerance
proposal is sequenced BEFORE evidence (T2R-C ahead of T2R-D/E), H2637 is
quarantined as synthetic stress, fleet-topology surveys are excluded, the
old package is properly superseded, and the bounded clamp form is named.

It is NOT ready to execute as-is. Three High findings would each
independently produce a re-litigable adjudication — the exact "third
re-do" this review exists to prevent. All three are package-text fixes,
not design unknowns. Mediums are execution-discipline constraints that
should be in the package text before kickoff; Lows can ride along.

## Findings

### T2R-H1 (High): Case-4 at absolute candidate `dx` is not a valid rung —
### the oracle transfers only dimensionlessly

- Evidence: Case-4 is the Iwagaki experiment-(B) flume — 24 m total, three
  8 m reaches. The RATIFIED acceptance (`SC-OFEROUTE-001#INV-OFEROUTE-011`,
  rev 25/26; `d10b_reconciliation_tests.rs`, `TOL_PEAK_REL = 0.05`) was
  validated on a 120..960-cell ladder, i.e. `dx` 0.2 m down to 0.025 m.
  Candidate policies are `dx ~ 5/10/20 m`: `ceil(8 / 10) = 1` cell per
  reach. Running Case-4 "at candidate policies" (package §Gates, T2R-D as
  written) is therefore structurally meaningless, with BOTH failure modes
  live:
  - spurious fail: 1-2 cells per reach cannot resolve the shock; the rung
    fails and the failure says nothing about a 20 m cell on a 300 m OFE;
  - vacuous pass: if `min_cells` (e.g. 10) rescues the flume geometry,
    every "candidate" runs the identical mesh and the ladder passes
    without testing anything.
- Required before execution: restate the oracle's role. Case-4 evidence is
  expressed in cells-per-reach (dimensionless resolution), and its job is
  to validate the convergence MACHINERY and error-vs-resolution order in
  the shock regime. The candidate `dx` adjudication rests on real-member
  self-convergence (T2R-H2). The package must explicitly forbid
  interpreting Case-4-at-candidate-`dx` results, in either direction, as
  candidate evidence.

### T2R-H2 (High): the acceptance basis must be candidate-vs-REFERENCE,
### never candidate-vs-baseline — and the baseline is itself a judged rung

- Evidence: T2R-D/E name the surfaces to record but never define what
  "error" is measured against. The current fixed `10 cells/OFE` is a
  working setting with no fidelity authority (it appears nowhere in
  `SC-OFEROUTE-001`; `LANED_ACTIVE_CELLS = 10` is a code constant). On
  long-OFE members the baseline is the COARSE end (`n_idaho`: one 300 m
  OFE, `dx = 30 m`), so treating baseline-vs-candidate deltas as
  "candidate error" inverts authority.
- Required before execution: define error as candidate-vs-fine-reference
  (self-convergence limit) per judged surface, and require the BASELINE be
  judged on the same tolerances as a rung. The adjudication must be able
  to return the bidirectional answer — including "10 cells/OFE is
  fidelity-INADEQUATE on long real OFEs and the ratified policy costs
  MORE on some members." Timing is then priced, not gating (or an
  explicit timing budget is predeclared in T2R-C); the package currently
  leaves the accept/reject decision rule implicit.

### T2R-H3 (High): the fine reference has no adequacy criterion

- Evidence: "Fine/reference: target `dx` about `2.5 m` or finer when
  Case-4 oracle evidence requires it" — "about/or finer" is not a rule. An
  arbitrary reference makes every downstream conclusion re-litigable: this
  is the single likeliest path to a third scaffold.
- Required before execution: a reference-independence check, e.g.: the
  reference is adequate when one further halving of its `dx` moves every
  judged surface by less than a named fraction (suggest <= 1/3) of that
  surface's tolerance; otherwise refine and repeat. Record the check's
  numbers in `artifacts/oracle-ladder.md` / the cohort evidence. (Cost is
  acceptable: even `n_idaho` at `dx = 1.25 m` is 240 cells; with cost
  between `n` and `n^2` that is minutes, not hours.)

### T2R-M1 (Medium): predeclare the judged-surface list, and include the
### production-integration surfaces the oracle cannot see

- The rev-27 active rows make several counted classes mesh-sensitive, and
  none are named in the package: `routed_end_window_storage_m3` (window
  end storage is BOOKED into day closure and RESET — coarser meshes drain
  differently, so this is a mass-relevant surface, not a diagnostic);
  `routed_tail_fold_m3`; `lane_days_erosion_source_shape_degenerate` (the
  wet-gate/dry-floor inconsistency class — mesh changes can flip discrete
  lane-days across this cliff); `days_uniform_shape`.
- Required in T2R-C's predeclared surface list: per-day routed outlet
  mass, hourly-weight shape feeding the D13 erosion consumer, annual
  pass-sediment sums, conservation closure (expected exact — rev 25/26
  holds it identically zero at any resolution, so it discriminates
  nothing; say so), plus the four counters above with named bounds on
  their drift across the ladder.

### T2R-M2 (Medium): clamp bounds are scheme-regime constraints, not
### conveniences

- The TVD-MacCormack limiter/dissipation machinery (rev-24/26 form:
  face-based dissipation, material-interface zero-flux, boundary-adjacent
  monitor mirroring) degenerates at very small cell counts — a 2-3-cell
  OFE has no interior away from boundary/interface treatment. At
  `dx = 20 m`, H2637-class 26 m OFEs get `ceil(26.11/20) = 2` cells.
- Required: justify `min_cells` against the scheme's stencil regime (not
  merely divide-by-zero safety), include a short-OFE-at-the-floor rung in
  the ladder, and make the clamp explicit and typed in the policy —
  `KinematicWaveMesh::uniform` already silently clamps to `>= 1`
  (`kinematic_wave.rs:256`), and the policy must not inherit that silent
  floor as behavior.

### T2R-M3 (Medium): the shadow lane is undecided and absent from the
### write set

- `LANED_SHADOW_CELLS = 10` is a SEPARATE constant in
  `crates/openwepp-runner/src/hillslope/laned_shadow.rs:38`; the package's
  write set names only `direct_runtime/`, `ofe_routing/`, and the runner
  `day_input_and_helpers/`. If the active policy changes and the shadow
  lane keeps a fixed 10, the two lanes silently run different meshes.
- Required: an explicit decision (shadow follows the policy, or shadow is
  frozen with a recorded rationale), and `laned_shadow.rs` added to the
  conditional write set if T2R-G is reached.

### T2R-M4 (Medium): hold the `dt` constants fixed across the ladder, and
### correct the cost expectation

- Production runs `LANED_ACTIVE_SAMPLE_DT_S = 900`,
  `LANED_ACTIVE_MAX_DT_S = 300` with CFL-adaptive stepping underneath. The
  ladder must vary ONE variable: mesh. Do not co-tune `dt` caps in this
  package (a `dt`-cap adjudication would be its own contract question).
- Consequence to record: the backlog's `cost ∝ n²` assumed CFL-bound
  stepping; where the 300 s cap binds (the T3 record already showed smooth
  phases partly cap-bound), coarsening yields only the linear work-per-step
  factor. Expected savings are sub-quadratic; predeclare that the timing
  outcome is measured against no named multiplier, so an undershoot vs the
  backlog estimate is not misread as a failed package.

### T2R-L1 (Low): record the uniform-coefficient premise in the ratified
### contract text

- Each lane's mesh runs ONE parameter set: `lane_config.mean_gradient`
  collapses the OFE's internal slope profile
  (`laned_active.rs:432-443`; `n_idaho`'s single OFE carries 14 slope
  points in `p1.slp`, flattened to one mean gradient). `dx`-convergence
  under this premise is NUMERICS fidelity only, not terrain fidelity.
  The ratified policy text should state the premise and that a future
  within-OFE parameter-profile change reopens the mesh question; also
  scope the ratification envelope to the evidenced OFE-length range
  (~26 m to ~300 m), with clamp behavior as the recorded posture outside
  it.

### T2R-L2 (Low): pre-implementation constant/pin inventory

- `LANED_ACTIVE_CELLS` has no consumers outside `laned_active.rs` and no
  test pins active-output byte hashes (checked: no `21c54bf2`-class pins
  in `crates/`/`tests/`), so the implementation surface is cleanly
  single-sited plus the shadow twin (T2R-M3). Verify this inventory again
  at T2R-G time, and note the ADR-0037 plain-identity artifacts are
  historical evidence of the 10-cell era — a ratified mesh change
  legitimately changes active outputs and must not be "reconciled" against
  those artifacts.

## Positive Notes

- T2R-C before T2R-D/E is the correct tolerance-predeclaration sequencing
  (the CL-M1 discipline, honored structurally this time).
- H2637 quarantine, fleet-survey exclusion, supersession note, ROADMAP
  re-point, and the explicit bounded clamp form are all correct.
- All Core required-reading paths exist, including both materialization
  JSONs; the kickoff prompt carries the byte budget, the subagent
  authorization phrase, and gate non-deferral.
- Cohort topology coverage is reasonable for the decision: single-OFE
  medium (81 m), single-OFE long (300 m), multi-OFE medium (5 x 108 m),
  plus the short-OFE synthetic stress case — the floor rung of T2R-M2
  completes the envelope.

## Disposition

Fix T2R-H1/H2/H3 in `package.md` (and mirror in the kickoff prompt) before
dispatch; fold T2R-M1..M4 into the phase text or the T2R-C predeclaration;
carry T2R-L1/L2 as execution notes. Whether the H-findings are resolved by
Claude amending the scaffold or by Codex as a pre-flight phase is the
operator's call — the required end-state is that the error basis, the
oracle's dimensionless role, and the reference-adequacy rule are written
down before any ladder run.
