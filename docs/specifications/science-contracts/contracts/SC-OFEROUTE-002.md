---
contract_id: SC-OFEROUTE-002
title: Hybrid Implicit-Explicit Kinematic-Wave Stepping Contract
status: approved
maturity: active
owner: openWEPP maintainers + hydrology reviewer
contract_version: 4
producer_scope:
  - Implicit backward-Euler upwind kinematic-wave stepping on smooth sample bins (per-cell scalar equilibrium solves, machine-exact ledger)
  - Hybrid span composition over a routed day window (implicit/explicit span partition, state seams, cross-span deficit carry, composed outlet-bin series)
  - Hybrid selector plumbing and solve-cost diagnostics counters
consumer_scope:
  - SC-OFEROUTE-001 routing surfaces (the hybrid produces the same outlet-bin series, ledger, and handoff surfaces the plain path produces)
  - Rev-27 active day-closure hard-fails (apply verbatim under the selector)
evidence_level: static
last_reviewed: 2026-07-07
supersedes: []
superseded_by: []
---

# SC-OFEROUTE-002 Hybrid Implicit-Explicit Kinematic-Wave Stepping Contract

Status: `approved`
Maturity: `active` (lifecycle vocabulary; the SUBSYSTEM posture remains EXPERIMENTAL/unpromoted — selector-gated evidence-gathering, see INV-OFEHYB-008)
Evidence mode: `static` (consolidation) over an executed evidence chain

## Purpose

This contract is the single normative authority for the EXPERIMENTAL hybrid
stepping subsystem selected by `OPENWEPP_LANED_ACTIVE_IMPLICIT=1` on the
Lane-D active path: the implicit backward-Euler upwind stepper, the
equilibrium-rating solve chain and its determinism rules, the switching
predicate, the hybrid span composition (including the cross-span deficit
carry), the selector/diagnostics plumbing, and the acceptance posture that
gates any promotion.

It CONSOLIDATES authority previously fragmented across `SC-OFEROUTE-001`
revision-history entries 28-31, that contract's hybrid Branch-and-Guard /
Test-Vector / BEI rows, and the LANED-T3 package design record
(`docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/artifacts/i0-scheme-design.md`).
Those sources remain the historical/evidence record; from rev 1 of this
contract forward, hybrid-subsystem changes amend THIS contract's normative
sections. `SC-OFEROUTE-001` rev 32 re-points its hybrid rows here.

## Scientific Scope

In scope:

- The implicit stepper as a NUMERICAL METHOD for the same physics
  `SC-OFEROUTE-001` owns (KWE continuity with the Papanicolaou friction
  closure): discrete form, existence/uniqueness/positivity, exact-ledger
  property, residual guard.
- The structure of the equilibrium rating induced by the
  `SC-OFEROUTE-001#INV-OFEROUTE-002` skin-regime dispatch (Z-shape, basin
  split, jump geometry) and the deterministic solve rules built on it.
- The hybrid switching predicate and span composition over a routed day
  window, including all fail-closed dispositions.
- Selector semantics, provenance surfacing, and solve-cost diagnostics.
- The acceptance/promotion posture (currently HELD — see the gap register).

Out of scope (owned elsewhere):

- The explicit TVD-MacCormack scheme, friction menu, CFL policy, oracle
  construction, cascade/handoff semantics, day windows, erosion coupling,
  and the rev-27 day-closure identities: `SC-OFEROUTE-001`.
- Default activation (D16-class promotion) and watershed scope.
- Subsurface coupling (`SC-SUBHYD-001` seams via `SC-OFEROUTE-001`).

## Authority Anchors

| Anchor | Role |
|---|---|
| `SC-OFEROUTE-001` (rev 32 pointer rows; revs 28-31 provenance) | Parent process contract: physics, explicit scheme, routing surfaces, day closure, Case-4 oracle acceptance surface (`INV-OFEROUTE-011`, rev 24-26). |
| `SC-OFEROUTE-001#INV-OFEROUTE-002` | The skin-regime dispatch whose `Re = 1000` crossover induces the Z-shaped equilibrium rating this contract's solve rules are built around. |
| WP `20260706-laned-router-t3-hybrid-implicit-stepping-001` | Design + I0/I1/I2 evidence (scheme derivation, prize measurement, dt-refinement ladder, rev-29 review hardening). |
| WP `20260706-laned-router-t3-aggressive-deficit-carry-001` | Rev-30 composition rules + executed H2637 carry evidence + dual review. |
| WP `20260707-laned-router-t3-ratification-solve-cost-001` | Rev-31 warm seeding + counters; the executed Case-4 hybrid ladder FAIL that holds ratification. |
| WP `20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001` | Rev-4 exact bare-skin branch evaluator for the solve-cost gap, with before/after H2637 timing and retained ratification gates. |

Legacy WEPP has no counterpart subsystem; no legacy anchor exists or is
sought (ADR-0011/ADR-0017 posture).

## Variables and Units (Externally Relevant)

| Symbol | Meaning | Units |
|---|---|---|
| `h_i` | cell flow depth | m |
| `q_i` | cell unit-width discharge | m^2 s^-1 |
| `q_up` | upstream boundary unit-width discharge (interval mean per step) | m^2 s^-1 |
| `v_i` | source (rainfall-excess) rate per cell | m s^-1 |
| `Δt` | implicit step size = outlet sample-bin width | s |
| `Δx` | cell length | m |
| `Q_c` | rating basin-split discharge `= 1000·ν` (Re crossover × kinematic viscosity) | m^2 s^-1 |
| `rhs_i` | cell solve right-hand side `h_i^n + (Δt/Δx)·q_in,i + v_i·Δt` | m |
| bin mass | conservative outlet-bin outflow (per unit width) | m^2 |

## Algorithm State Surfaces

Required inputs: the routing mesh (`Δx`, per-cell friction parameters), the
committed state `(h_i, q_i)` at the window/span start, the hourly seam
source-rate series and intensity series, the upstream conservative bin
series (optional), the forcing breakpoints, the day window `[0, T]`, the
sample cadence, and the explicit-path `max_dt`.

Required outputs: committed `(h_i, q_i)` at window end, the composed
non-negative outlet-bin series with `Σ bins == booked outflow` (exact
except the approved C-L1 bounded all-dry/insufficient-gross attribution
drop, `<=` the noise floor — §Algorithm 5.4), the mass
ledger (`inflow / rainfall-excess / outflow / storage-change / clamp`), the
bin-mean hydrograph, peak diagnostics, and the diagnostics counters
(§Algorithm item 6).

Mutated state: the mesh depth/discharge vectors and the composed bin/ledger
accumulators only. No state may persist across day windows beyond what
`SC-OFEROUTE-001`'s window/reset row already defines.

## Algorithm Specification

### 1. Implicit step (backward-Euler + first-order upwind)

Information propagates strictly downstream (`dq/dh > 0`), so the implicit
system is lower-triangular and solves by a single downstream march — one
scalar nonlinear solve per cell, no matrix. For cells `i = 0..n-1` over
`Δt`:

```text
F_i(h) = h − h_i^n + (Δt/Δx)·(q(h) − q_in,i) − v_i·Δt = 0
q_in,0 = q_up            (prescribed interval-mean boundary flux)
q_in,i = q(h_{i−1}^{n+1}) (already solved — downstream march)
```

Preconditions: finite non-negative `h_i^n`, `q_up`, `v_i`; validated cell
parameters. Postconditions: `h_i^{n+1} >= 0` (INV-OFEHYB-002); the summed
discrete equations ARE the mass balance (flux terms telescope), so the
ledger is exact BY CONSTRUCTION with no clamp class and no dissipation term
(INV-OFEHYB-001). Degenerate states: a zero-mass step on a dry mesh is a
valid no-op; near-dry residual noise is governed by the dust floor
(§Constants, Branch/Guard).

Within one friction branch `q(h)` is continuous and strictly increasing, so
`F_i' >= 1`: the in-branch root is unique, exists for any `Δt`
(unconditional stability), and is non-negative.

### 2. Equilibrium-rating structure (the Z-shape)

The `INV-OFEROUTE-002` regime dispatch (Shen & Li at `Re <= 1000`, Hirsch
above) makes the FULL equilibrium rating Z-SHAPED: an overlap depth band
exists where both laminar and turbulent equilibria are defined, fixed-point
basins split exactly at `Q_c = 1000·ν`, and each branch rating carries
exactly one UPWARD jump — at the laminar edge `h_b` for the LOW rating and
the turbulent edge `h_a < h_b` for the HIGH rating. Consequences this
contract binds:

- "The converged equilibrium" is seed-dependent; any warm-start or
  acceleration scheme must be proven to preserve the deterministic value
  (INV-OFEHYB-003).
- Double-collapse theorem (rev-29): the cell line
  `q = (rhs − h)/(Δt/Δx)` is strictly decreasing; crossing the LOW jump
  requires `line(h_b) > Q_c` while crossing the HIGH jump requires
  `line(h_a) < Q_c` — a contradiction. Whenever the LOW rating jumps over
  the root, the HIGH rating hosts a genuine root; a both-branches-jump
  outcome is unreachable for genuine physics and MUST fail closed as a
  solve failure (INV-OFEHYB-004). No filled-jump (Filippov-style) commit is
  permitted.

### 3. Deterministic solve chain (per cell)

Ordered branch priority:

1. Solve the LOW branch from its deterministic seed; if it converges to an
   in-branch root, accept.
2. Else solve the HIGH branch from its deterministic seed; if it converges,
   accept.
3. Else (both branches report bracket collapse): typed solve failure —
   fail closed (§2 theorem).

Solve rules, all normative:

- **Deterministic basin-split seeding (rev 29):** cold seeds are fixed per
  branch (`Q_c·1e-3` LOW / `Q_c·1e3` HIGH) — pure functions of the branch,
  never of run history.
- **Branch-local warm seeding (rev 31, cost-only lever):** a warm seed may
  be derived ONLY from the same implicit downstream march's own
  already-solved upstream discharge/equilibrium result, is accepted
  ONLY when FINITE, POSITIVE, and on the evaluated branch's side of `Q_c`;
  any candidate failing any of the three conditions falls back to the cold
  seed. It may not change the converged value, the residual form, the
  branch preference, or the fail-closed posture.
- **Exact bare-skin branch evaluator (rev 4 / GAP-OFEHYB-002):** when the
  active cell's additive friction closure is effectively ONLY skin
  resistance (`k_o` plus optional rain term) with no active form, wave,
  vegetation, or Manning addend, the implementation may replace fixed-point
  map iteration with the algebraic fixed points of the exact same Shen-Li
  and Hirsch branch equations. "No active addend" follows the same guards as
  the friction equations: e.g. zero roughness concentration or zero element
  height disables form/wave; `LAI = 0`, zero canopy height, or zero
  vegetation drag disables vegetation. These are exact zero component-absence
  guards, not near-zero thresholds, and the cell must still pass the finite,
  non-negative roughness-domain validation before the direct path can execute.
  Seed-side selection remains deterministic: prefer the seed's branch side
  when that side has an in-regime fixed point, otherwise migrate only to the
  single valid in-regime fixed point. This is a solve-method optimization, not
  a byte-identity promise: it may not alter LOW→HIGH outer preference,
  residual equations, tolerance guards, publication schema/ownership surfaces,
  or fail-closed behavior, and any active-output numeric deltas must remain at
  the branch-equilibrium tolerance scale recorded for the ratified H2637
  vector. It records no `implicit_equilibrium_map_evaluations` because no map
  applications are executed.
- **Basin-locked Steffensen acceleration (rev 29):** an accelerated point
  is accepted only when the whole plain iteration triple sits in one basin
  and the accelerated point stays on that side of `Q_c`, so the accelerated
  sequence converges to the plain-iteration limit from the given seed.
- **Safeguarded outer solve with bisection interleave:** false position
  stalls one-sided on the convex `q ~ h^3`-class rating; a forced bisection
  every other iteration guarantees bracket halving.
- **Step-residual hard guard:** after the march, the booked step identity
  is re-checked at relative `1e-9` with the dust-floor absolute scale
  (§Constants); violation is a typed failure. No exit path returns `Ok`
  with an unvalidated `(h, q)` pair. TRANSACTIONALITY NOTE (review B-M2):
  the low-level step API mutates the caller's working buffers cell-by-cell
  DURING the march; on a typed failure those buffers are UNDEFINED and
  must not be consumed — the production path fails the routing window
  closed without publishing them. Staging the commit behind the guard is a
  recorded non-blocking hardening candidate.

### 4. Switching predicate (rev 30/33, source-memory cooldown)

The day window is partitioned into outlet sample bins. A bin is
IMPLICIT-ELIGIBLE iff its seam SOURCE rate is zero on every cell of the
lane AND the bin is outside the source-memory cooldown window
(zero-source-after-cooldown; upstream inflow does NOT force explicit
stepping — the implicit step books the interval-mean upstream mass exactly).
Properties this contract binds:

- The predicate is FORCING-derived and deterministic — knowable from the
  seam series and bin cadence alone before routing. It carries source-history
  memory only; it is not state-derived and cannot chatter on solver state.
- The bin-start point sample is exact ONLY for bin-constant source, so the
  composition FAILS CLOSED unless the sample cadence partitions the seam
  hour exactly (rev-30 C-M1 guard).
- **Source-memory cooldown (rev 33 / GAP-OFEHYB-001 lift candidate):** after
  a contiguous source-active burst ends, the next `2 * burst_duration` of
  source-free bins remain EXPLICIT. Only after that cooldown expires may
  source-free bins route implicitly. A later source-active burst resets the
  rule for its own post-source cooldown. The multiplier `2` is a numerical
  switching constant, not process physics; it was selected by the
  package-local Case-4 cooldown scan (`10 s` after the `10 s` Iwagaki source
  still failed, `20 s` passed).

Implicit bins step once per bin at the bin cadence (`Δt` = bin width; no
CFL constraint applies). Contiguous same-class bins form spans.

### 5. Hybrid span composition

1. **Explicit spans** run the unchanged `SC-OFEROUTE-001` scheme as a
   sub-window: state installed via the solver state seam, forcing closures
   shifted to global time, breakpoints span-clipped. An all-explicit window
   MUST be bit-identical to the plain path (INV-OFEHYB-007).
2. **State seam:** depth carries; entering explicit installs the implicit
   solve's own converged equilibrium discharges; entering implicit consumes
   committed depths (discharge is re-derived in-solve). The seam moves no
   mass.
3. **Cross-span deficit carry (rev 30):** the explicit recorder's
   forward-redistribution rule continues ACROSS span boundaries. An
   explicit span's terminal front-arrival attribution deficit (the
   `2q_{n-1} − q_{n-2}` step-scale dip class; the booked ledger stays
   exact) is returned by the composition-scoped solver variant
   (`run_with_options_deficit_carry`) and absorbed by subsequent composed
   bins under the exact-total, non-negative rule
   (`booked + new_carry == mass + old_carry` at every bin). The public
   windowed run remains a fail-closed wrapper; ONLY the hybrid composition
   may consume the deficit-returning variant.
4. **End-of-window disposition:** a MATERIAL remaining deficit fails closed
   (`NegativeOutletBin`); a sub-noise remainder (`1e-9` of series gross,
   absolute floor `1e-12 m^2` on the gross scale) is absorbed BACKWARD from
   trailing positive bins; on an all-zero/insufficient-gross series the
   un-absorbable sub-noise remainder is DROPPED as a bounded attribution
   slack (never a mass-ledger change, never a negative bin).
5. **Composed surfaces:** `Σ published bins == booked outflow` exactly,
   with the SINGLE approved exception of the §5.4 C-L1 bounded all-dry
   drop (INV-OFEHYB-006) — the property the inter-lane handoff injection and the
   erosion hourly-weight mapping consume; ledger accumulators book actual
   fluxes from both schemes; peak diagnostics use physical (pre-carry)
   values while exported bins are post-carry conservative attribution.

### 6. Selector and diagnostics

`OPENWEPP_LANED_ACTIVE_IMPLICIT=1` composes with `OPENWEPP_LANED_ACTIVE=1`;
unset means plain rev-27 active behavior (byte-identical). The run manifest
records `hybrid_implicit_stepping`; the profile line surfaces
`solver_steps_implicit`, `implicit_equilibrium_map_evaluations`, and
`implicit_branch_evaluations` (rev-31 counters; endpoint-timing claims for
this subsystem MUST include them).

## Branch and Guard Table

| Branch / guard | Trigger | Required behavior | Failure posture | Invariants |
|---|---|---|---|---|
| Selector | `OPENWEPP_LANED_ACTIVE_IMPLICIT=1` + active selector | Hybrid composition per §Algorithm; provenance + counters surfaced. | Unset ⟹ plain path byte-identical; no default/shadow surface touched. | `INV-OFEHYB-007` |
| Hour-partition cadence | sample cadence does not partition the seam hour | Fail closed before routing. | Typed degenerate-configuration failure. | `INV-OFEHYB-005` |
| Non-integral window | window not an integral bin count | Fail closed. | Typed degenerate-configuration failure. | `INV-OFEHYB-005` |
| Branch closure | LOW converges / LOW jumps / both jump | Accept LOW root; else accept HIGH root; else typed solve failure (double-collapse theorem). | Typed non-convergence failure; no filled-jump commit. | `INV-OFEHYB-004` |
| Warm-seed validity | candidate seed non-finite, non-positive, OR off the evaluated branch's basin side | Fall back to the cold basin-split seed (all three acceptance conditions must hold). | Deterministic fallback (cost-only lever). | `INV-OFEHYB-003` |
| Exact bare-skin evaluator | no active form/wave/vegetation/Manning addend; exact Shen-Li/Hirsch branch fixed points exist | Use the algebraic branch value selected by the same seed-side rule; otherwise use the generic fixed-point path. | Typed non-convergence if the exact value is non-finite or no valid branch value exists. | `INV-OFEHYB-003` |
| Acceleration basin lock | plain triple or accelerated point crosses `Q_c` | Reject the accelerated point; continue plain iteration. | Deterministic rejection. | `INV-OFEHYB-003` |
| Step-residual guard | booked step identity off by > `1e-9` relative (dust-floor scale) | Typed failure; no unvalidated state commits. | Typed runtime hard fail. | `INV-OFEHYB-001` |
| Material terminal deficit | end-of-window carry beyond the noise floor | Fail closed. | `NegativeOutletBin`. | `INV-OFEHYB-006` |
| Sub-noise terminal remainder | carry within the noise floor | Backward absorption; bounded all-dry drop. | Approved bounded disposition (documented + test-pinned). | `INV-OFEHYB-006` |
| Source-memory cooldown | bin is source-free but inside `2 * preceding_source_burst_duration` after source shutoff | Route explicitly; implicit remains ineligible until cooldown expires. | Test failure or ratification failure blocks closure/promotion. | `INV-OFEHYB-005`, `INV-OFEHYB-008` |
| Promotion | any attempt to ratify/promote the selector | Must meet the full acceptance surface (§Tolerances). | HOLD until every acceptance subgate passes. | `INV-OFEHYB-008` |

## Invariants

| Invariant ID | Statement | Guard | Failure posture | Authority | Evidence |
|---|---|---|---|---|---|
| INV-OFEHYB-001 | Implicit-step ledger exactness BY CONSTRUCTION: booked inflow/rain/outflow/storage equal the scheme's actual fluxes to the solve residual tolerance; no clamp class; the step-residual hard guard (relative `1e-9`, dust-floor absolute scale) rejects any violation. | runtime guard + I1 exactness vectors | Typed runtime hard fail. | This contract §Algorithm 1/3 | `[DIRECT][Ran]` (I1 ladder residuals ≤ 1.7e-14) |
| INV-OFEHYB-002 | Unconditional positivity: no negative depth/discharge for any `Δt`; no `max(0)` clamps anywhere in the implicit path. | construction + I1 vectors | Typed failure (a negative would indicate a solve defect). | §Algorithm 1 | `[DIRECT][Static] + [DIRECT][Ran]` |
| INV-OFEHYB-003 | Determinism: the converged cell state is a pure function of (cell parameters, `rhs`, `Δt/Δx`, branch) — independent of run history. Warm seeds, acceleration, and exact direct evaluators must provably converge to the same branch value (warm-seed acceptance requires FINITE + POSITIVE + evaluated-branch-side, else cold fallback; acceleration is basin-locked; bare-skin direct evaluation is algebraically exact and seed-side valid). | seed/acceleration/direct-evaluator guards + regression vectors | Deterministic fallback/rejection; divergence is a defect. | §Algorithm 2/3; rev-29/31/GAP002 rules | `[DIRECT][Ran]` (rev-29 bit-identical books; rev-31 evidence; GAP002 exactness vector) |
| INV-OFEHYB-004 | Branch closure: LOW→HIGH preference; a both-branches-jump outcome is unreachable for genuine physics (double-collapse theorem) and fails closed. No mass-exact filled-jump commit exists. | solve-chain structure + LOW-jump→HIGH-root vector | Typed non-convergence hard fail. | §Algorithm 2 theorem | `[DIRECT][Static] (proof) + [DIRECT][Ran]` |
| INV-OFEHYB-005 | Switching-predicate soundness: forcing-derived, deterministic, source-memory bounded; implicit only on zero-source bins after the `2 * preceding_source_burst_duration` cooldown; the cadence must partition the seam hour (fail closed) so the bin-start sample is provably bin-constant. | preflight guards + C-M1/cooldown vectors | Typed degenerate-configuration failure or test failure. | §Algorithm 4 | `[DIRECT][Ran]` |
| INV-OFEHYB-006 | Composition exact-total: the composed published bin series is non-negative and sums to the booked outflow EXACTLY EXCEPT for the single approved C-L1 disposition (an un-absorbable sub-noise remainder on an all-dry/insufficient-gross series is dropped, bounded by the noise floor); deficits move ATTRIBUTION forward in time, never mass; material end-of-window deficits fail closed. | carry algebra + disposition guards + rev-30 vectors | `NegativeOutletBin` / bounded documented disposition. | §Algorithm 5 | `[DIRECT][Ran]` (H2637 6-event carry evidence) |
| INV-OFEHYB-007 | Non-perturbation: selector unset ⟹ plain/default byte identity; all-explicit windows bit-identical to the plain path; the deficit-returning solver variant is composition-scoped and every other caller keeps the fail-closed wrapper. | bit-identity vector + parquet pin + call-site audit | Byte/bit diff blocks; wrapper regression is a defect. | §Algorithm 5/6 | `[DIRECT][Ran]` (parquet `21c54bf2…` pinned) |
| INV-OFEHYB-008 | Acceptance/promotion gate: the hybrid must meet the `SC-OFEROUTE-001#INV-OFEROUTE-011` Case-4 oracle surface at EVERY ladder rung AND carry ratified fidelity/timing tolerances before any promotion beyond evidence-gathering. GAP-OFEHYB-001 is the Case-4 subgate; closing it does not by itself promote the selector. | Case-4 hybrid ladder vector + ratification process | HOLD until every promotion subgate passes. | Parent oracle surface | `[DIRECT][Ran]` |
| INV-OFEHYB-009 | Day-closure inheritance: the rev-27 `SC-OFEROUTE-001` day-closure hard-fails (supply, router-internal, seam cross-ledger, day identity) apply VERBATIM under the selector at their named tolerances. | live rev-27 guards | Typed runtime hard fail. | SC-OFEROUTE-001 rev 27 | `[DIRECT][Ran]` (H2637 machine-exact under hybrid) |
| INV-OFEHYB-010 | Diagnostics obligations: implicit step and solve-cost counters are surfaced on every profiled run; subsystem timing claims must cite them. | profile plumbing + evidence convention | Missing counters invalidate timing evidence. | §Algorithm 6 | `[DIRECT][Ran]` |

## Invariant Guard Map

| Invariant ID | Enforcement path | Guard class | Failure behavior | Evidence artifact |
|---|---|---|---|---|
| `INV-OFEHYB-001..002` | `ofe_routing::implicit_recession` step guard + `implicit_step_ledger_is_exact_and_positive`, `implicit_step_books_upstream_inflow_exactly`, `dust_scale_steps_do_not_accumulate_a_material_leak` | runtime + test | typed hard fail | T3 `i1-implicit-stepper-evidence.md` |
| `INV-OFEHYB-003` | seed/acceleration/direct-evaluator acceptance rules + `steady_state_is_a_fixed_point_of_the_implicit_step`, `branch_warm_seed_preserves_solution_and_reduces_or_matches_map_work`, `branch_warm_seed_acceptance_is_basin_locked`, `bare_skin_direct_equilibrium_matches_iterated_branch_values`, `bare_skin_direct_equilibrium_avoids_fixed_point_map_work` | runtime + test | deterministic fallback; divergence = defect | T3 rev-29 disposition; rev-31 WP; GAP002 WP |
| `INV-OFEHYB-004` | solve-chain ordering + double-collapse typed error + `low_jump_recovers_high_branch_root_and_never_commits_filippov` | runtime + test | typed hard fail | rev-29 disposition |
| `INV-OFEHYB-005` | hybrid preflight guards + source-memory cooldown mask + `hybrid_rejects_cadence_that_does_not_partition_the_seam_hour`, `hybrid_rejects_non_integral_windows`, `hybrid_source_memory_cooldown_keeps_post_source_bins_explicit` | runtime + test | typed degenerate-configuration / test failure | GAP001 WP |
| `INV-OFEHYB-006` | `absorb_deficit`/`dispose_terminal_carry` + `absorb_deficit_exact_total_and_non_negative`, `dispose_terminal_carry_material_deficit_fails_closed`, `dispose_terminal_carry_subnoise_absorbs_backward_exactly`, `dispose_terminal_carry_all_dry_subnoise_drop_is_bounded`, `bin_recorder_returns_material_terminal_deficit_exactly` | runtime + test | `NegativeOutletBin` / bounded documented drop | T3-AGG `fix-evidence.md` |
| `INV-OFEHYB-007` | `hybrid_is_bit_identical_on_all_explicit_windows` + plain-parquet pin + `pub(super)` scoping of the deficit variant | test + run evidence | bit/byte diff blocks | T3/T3-AGG gate results |
| `INV-OFEHYB-008` | `case4_hybrid_manning_ladder_meets_iwagaki_oracle` | validation | HOLD until all promotion subgates pass | GAP001 `design-evidence.md` / `gate-results.md` |
| `INV-OFEHYB-009` | rev-27 live closure guards (unchanged) | runtime | typed hard fail | H2637 evidence blocks |
| `INV-OFEHYB-010` | `ofe_routing::profile` counters + manifest/profile line | diagnostics | evidence-invalidating | rev-31 timing artifact |

## Producer Obligations

| Obligation ID | Producer | Obligation |
|---|---|---|
| OBL-OFEHYB-P-001 | Hybrid composition | Publish only non-negative composed bin series that are exact-total up to the approved C-L1 bounded all-dry drop; never expose an unresolved MATERIAL deficit beyond the composition boundary. |
| OBL-OFEHYB-P-002 | Implicit stepper | Book actual fluxes; return `Ok` only for validated `(h, q)` states — on typed failure the working buffers are undefined and callers must fail the routing window closed without consuming them; surface counters. |
| OBL-OFEHYB-P-003 | Runner plumbing | Record selector provenance in the manifest; neutralize the selector in harness helpers that claim plain-path evidence. |

## Consumer Obligations

| Obligation ID | Consumer | Obligation |
|---|---|---|
| OBL-OFEHYB-C-001 | Inter-lane handoff / erosion weight mapping | May rely on non-negativity and `Σ bins == booked outflow` up to the approved C-L1 bounded all-dry drop (`<=` the noise floor; zero on any series with material gross); must NOT assume bins equal raw scheme fluxes (attribution may shift within the window under the carry rule). |
| OBL-OFEHYB-C-002 | Any future caller of the deficit-returning solver variant | Must implement the full carry obligation (absorb or fail closed) — currently prohibited outside the hybrid composition. |

## Symbol Alias Map

| Canonical symbol | Boundary/API name | Scope | Units check |
|---|---|---|---|
| implicit step operator | `ofe_routing::implicit_recession::implicit_step_with_discharges` | orchestrator kernel tier | SI (m, m^2 s^-1, s) — same surfaces as `SC-OFEROUTE-001` |
| hybrid composition | `ofe_routing::cascade::route_single_ofe_hybrid` | orchestrator cascade tier | SI; composed bin masses m^2 per unit width |
| deficit-returning windowed run | `KinematicWaveSolver::run_with_options_deficit_carry` | composition-scoped (`pub(super)`) | SI; returned deficit m^2 (`<= 0`) |
| selector | `OPENWEPP_LANED_ACTIVE_IMPLICIT` | runner environment | dimensionless flag |

## Constants and Parameters

| Constant | Value | Provenance |
|---|---|---|
| Basin-split discharge `Q_c` | `1000·ν` | `INV-OFEROUTE-002` crossover; rev 28 |
| Cold branch seeds | `Q_c·1e-3` (LOW) / `Q_c·1e3` (HIGH) | rev 29 determinism rule |
| Implicit `Δt` | outlet sample-bin width (900 s active cadence) | rev 28 dt policy |
| Source-memory cooldown multiplier | `2 * preceding_source_burst_duration` | rev 33 GAP-OFEHYB-001 design increment; Case-4 scan bracket (`10 s` fail, `20 s` pass) |
| Step-residual guard | relative `1e-9`; absolute dust floor `DRY_DEPTH_M · Δx · n` (`DRY_DEPTH_M = 1e-9 m`) | rev 28 dust-floor rule |
| Terminal-carry noise floor | `1e-9 ×` series gross, floor `1e-12 m^2` | rev 30 (recorder noise rule at composed level) |
| All-dry drop bound | `<=` noise floor (absolute floor class `1e-21 m^2` at the degenerate gross floor) | rev 30 C-L1 disposition |

## Unit-Governance Map

All quantities are SI (m, s, m^2 per-unit-width masses, m^3 volumes at the
lane boundary) and flow through the same boundary-symbol surfaces as
`SC-OFEROUTE-001` (no new published symbols; the counters are dimensionless
counts). No new registry entries are required; any future published hybrid
symbol must register per `docs/specifications/unit-governance.md`.

## Tolerance and Numeric Notes

- Implicit-step exactness: ledger residuals measured `<= 1.7e-14` across the
  I1 ladder; the binding guard is relative `1e-9` with the dust floor.
- Recession fidelity (I1 ladder, DIAGNOSTIC — NOT ratified): per-900 s-bin
  L1 vs the explicit reference ~`0.43` at `Δt = 900 s` on pure recession,
  first-order in `Δt`, cross-scheme gap `O(1/n)`.
- H2637 fidelity deltas (DIAGNOSTIC): outlet `−0.24 %` (strict, rev 28) /
  `−0.84 %` (aggressive, rev 30) vs plain active; ledgers exact throughout.
- **Acceptance surface (promotion-gating, INV-OFEHYB-008):** the
  `SC-OFEROUTE-001#INV-OFEROUTE-011` rev-25/26 ratified Case-4 oracle
  tolerances at every rung (peak `<= 5 %`, `t_peak <= 1.5 s`, rise
  `<= 2.0 s`, non-diverging) run with the FULL hybrid, plus named ratified
  fidelity/timing tolerances for the implicit phase. **Rev-33 design evidence:
  the old zero-source-only rule failed at peak `22.8 / 15.5 / 10.2 %`; a
  `10 s` cooldown after the `10 s` Iwagaki source still failed
  (`13.1 / 8.1 / 5.03 %`), while `20 s` passed.**
- Timing evidence convention: endpoint claims must cite the rev-31 counters
  (H2637 record: `36.61 s` user hybrid vs `37.9 s` plain; `274.7 M` map
  evaluations / `37.2 M` branch evaluations). Rev-33 GAP-OFEHYB-001
  source-memory record: `37.96 s` user, `980804` implicit steps, `151.4 M`
  map evaluations, `20.1 M` branch evaluations; `+3.69 %` vs rev-31 hybrid
  and `+0.16 %` vs plain active.
- Rev-4 exact bare-skin H2637 record (GAP-OFEHYB-002): `33.37 s` user,
  `0:33.43` wall, `980804` implicit steps, `0` equilibrium map evaluations,
  and `20.1 M` branch evaluations. Active output is not byte-identical to the
  iterative baseline: `H2637.loss.json` remains byte-identical; `H2637.hbp`
  differs by 54 bytes; `H2637.pass.parquet` keeps the same shape, columns, and
  index with sparse numeric movement only (`tdet` one row, max absolute
  `3.48e-9`, max relative `1.54e-10`; `sedcon_1..5` three rows each, max
  relative `3.84e-10`, max absolute `1.89e-11`). Manifest closure remains at
  the existing machine-precision ledger surface (`max_day_identity_residual_rel
  <= 4.5e-13`, WB13 identity maxima `0.0`). These deltas are ratified for the
  exact evaluator as branch-equilibrium numeric dust; they are not a
  publication schema, publication ownership, default, or tolerance change.

## Test-Vector Obligations

| Obligation | Minimum vectors | Expected evidence |
|---|---|---|
| Implicit stepper (I1 family) | Exactness + positivity at every dt/mesh rung; steady-state fixed point; recession dt-refinement ladder vs the upwind/characteristics reference; 10k-step dust accumulation; LOW-jump→HIGH-root recovery. | Retained `implicit_recession` suites; T3 I1 evidence. |
| Solve determinism | Warm-seed fallback, basin-lock regressions, exact bare-skin evaluator equivalence (converged values unchanged vs the branch-defining iterative map where iteration is still the authority fallback), composed bare-skin cell solves for rain-term / zero-`k_o` / near-crossover edges, and invalid inactive-operand fail-closed validation before any direct path. | rev-29/31 retained vectors + GAP002 exactness evidence. |
| Switching predicate | Hour-partition fail-closed (C-M1 scenario); non-integral window; source-memory cooldown pin; zero-source/no-prior-source upstream-fed bins remain implicit and book upstream mass exactly. | `rev30_deficit_carry_tests` + rejection/cooldown vectors. |
| Composition deficit carry | Recorder deficit-return identity; absorb/dispose exact-total + non-negativity; material end-of-window fail-closed (incl. all-dry); sub-noise backward absorption; bounded all-dry drop. | rev-30 vector family. |
| Non-perturbation | All-explicit bit-identity; plain-parquet pin under selector-off. | T3/T3-AGG gate artifacts. |
| Acceptance (Case-4 subgate CLOSED; selector promotion still HELD) | Case-4 FULL-hybrid oracle ladder at the parent ratified tolerances; retained unignored after the rev-33 source-memory cooldown amendment. | GAP001 `design-evidence.md`, `gate-results.md`, and `verification-case4-cooldown-scan.md`. |

## Binding Exposure Index

Status: `experimental-unpromoted`
Evidence mode: `Static`

| Entry ID | Source | Status | Binding classification | Canonical binding IDs | Review gate | Notes |
|---|---|---|---|---|---|---|
| `OFEHYB-IMPLICIT-STEPPER` | `SC-OFEROUTE-002.md#algorithm-specification` §1-3 | `active` | `unpromoted-binding` | `INV-OFEHYB-001, INV-OFEHYB-002, INV-OFEHYB-003, INV-OFEHYB-004` | `science-review-follow-on` | `ofe_routing::implicit_recession` + the equilibrium solve rules in `ofe_routing::kinematic_wave`. |
| `OFEHYB-SWITCHING-COMPOSITION` | `SC-OFEROUTE-002.md#algorithm-specification` §4-5 | `active` | `unpromoted-binding` | `INV-OFEHYB-005, INV-OFEHYB-006, INV-OFEHYB-007` | `science-review-follow-on` | `route_single_ofe_hybrid` + `run_with_options_deficit_carry` + carry/disposition helpers. |
| `OFEHYB-SELECTOR-DIAGNOSTICS` | `SC-OFEROUTE-002.md#algorithm-specification` §6 | `active` | `unpromoted-binding` | `INV-OFEHYB-009, INV-OFEHYB-010` | `science-review-follow-on` | Runner selector/provenance/counter plumbing; rev-27 closure inheritance. |
| `OFEHYB-ACCEPTANCE-HOLD` | `SC-OFEROUTE-002.md#tolerance-and-numeric-notes` | `active` | `maps-to-existing-INV` | `INV-OFEHYB-008` | `science-review-follow-on` | The HELD promotion gate; lifts only via GAP-OFEHYB-001 adjudication + ratified tolerances. |

## Gaps

| Gap ID | Statement | Impact | Disposition | Evidence |
|---|---|---|---|---|
| GAP-OFEHYB-001 | **Shock-outlives-source: the old zero-source-only predicate routed shock-carrying source-quiet bins implicitly.** The rev-31 Case-4 hybrid ladder failed the parent ratified peak tolerance at every rung (`22.8 / 15.5 / 10.2 %` vs `5 %`, improving under refinement — the implicit phase's first-order diffusion smearing an in-mesh front). Rev 33 selects the recorded explicit cool-down lever as a source-memory predicate: after a source-active burst, route the next `2 * burst_duration` source-free bins explicitly before implicit recession is eligible. The Case-4 scan proved `10 s` cooldown still failed and `20 s` passed for the `10 s` Iwagaki source; the retained Case-4 ladder now passes unignored under the same parent tolerances. | Case-4 subgate no longer blocks the selector; closing this gap does not by itself promote the selector, which still requires the full `INV-OFEHYB-008` fidelity/timing ratification process. | RESOLVED — WP `20260707-laned-router-gap-ofehyb-001-hold-lift-design-001`. | `[DIRECT][Ran]` (cooldown scan + retained ladder + H2637 timing/profile) |
| GAP-OFEHYB-002 | **Implicit solve cost was the endpoint bottleneck for the H2637 source-memory hybrid path.** Rev-33 source-memory H2637 carried `151.4 M` equilibrium map evaluations and `38.39 s` user in this package's refreshed baseline. Rev-4 exact bare-skin evaluation removes the nested map work on the H2637-active cell class (`0` map evaluations, `33.37 s` user, `0:33.43` wall) while preserving branch rules, fail-closed validation, and closure surfaces. | No longer blocks the H2637 endpoint value for the current active source-memory hybrid vector. It does not by itself promote the selector; generic non-bare solve optimization remains optional Tier-2/Tier-1 performance work. | RESOLVED-FOR-H2637-SOLVE-COST — WP `20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001`; selector remains experimental/unpromoted under `INV-OFEHYB-008`. | `[DIRECT][Ran]` (before/after H2637 counters, output-delta audit, retained tests) |

## Revision History

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-07-07` | `4` | `Codex` | GAP-OFEHYB-002 solve-cost amendment and execution (WP `20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001`): authorizes and ratifies an exact bare skin-only branch evaluator for cells with no active form/wave/vegetation/Manning addend under exact-zero component-absence guards plus the ordinary finite/non-negative cell-parameter validation. The evaluator computes the algebraic Shen-Li and Hirsch fixed points of the same branch equations, preserves deterministic seed-side selection and LOW→HIGH outer preference, records no map applications because no fixed-point map is executed, and falls back to the generic basin-locked iterative path for every non-bare operand class. H2637 source-memory hybrid timing moves from `38.39 s` user / `151.4 M` map evaluations to `33.37 s` user / `0` map evaluations. Active outputs are not byte-identical but are bounded to sparse branch-equilibrium numeric dust as recorded in the tolerance notes; no physics, selector posture, publication schema/ownership, default activation, or promotion change is made here. GAP-OFEHYB-002 is resolved for the current H2637 solve-cost bottleneck; `INV-OFEHYB-008` remains the selector promotion gate. |
| `2026-07-07` | `3` | `Codex` | GAP-OFEHYB-001 hold-lift design amendment (WP `20260707-laned-router-gap-ofehyb-001-hold-lift-design-001`): replaces the old zero-source-only switching predicate with a deterministic source-memory cooldown. After any contiguous source-active burst, the next `2 * burst_duration` source-free bins remain explicit; only later source-free bins are implicit-eligible. This is a numerical switching rule, not process physics; upstream inflow remains allowed after cooldown because implicit steps book interval-mean upstream mass exactly. Design evidence: Case-4 cooldown scan bracketed the transition (`10 s` after the `10 s` source still failed; `20 s` passed), the retained Case-4 hybrid ladder now passes unignored at the parent tolerances, and H2637 active hybrid timing/profile is recorded at `37.96 s` user with `980804` implicit steps. GAP-OFEHYB-001 is resolved as the Case-4 subgate; no selector promotion/default change is made. |
| `2026-07-07` | `2` | `Codex` | Approval-lift after dual review, accepted finding disposition, and dual verification in WP `20260707-laned-router-hybrid-contract-authority-001`: Agent A verification GO; Agent B initial verification found one remaining Low guard-map shorthand, the row was amended to name the retained deficit-carry tests directly, and Agent B follow-up verification returned GO. Lifecycle status is now `approved` / `active`; no behavior, tolerance, selector posture, or promotion change is made. `INV-OFEHYB-008` remains HELD on `GAP-OFEHYB-001`; `GAP-OFEHYB-002` remains an optimization gap. |
| `2026-07-07` | `1` | `Claude Code` | Initial authority CONSOLIDATION (WP `20260707-laned-router-hybrid-contract-authority-001`): normative content assembled from `SC-OFEROUTE-001` revs 28 (scheme + selector + Z-rating discovery), 29 (double-collapse theorem, basin-locked Steffensen, dust floor), 30 (aggressive mask, hour-partition guard, cross-span deficit carry + dispositions), 31 (branch-local warm seeding, solve-cost counters), the T3 design record (`i0-scheme-design.md` — including the recorded explicit cool-down fallback now carried in GAP-OFEHYB-001), and the executed evidence chain (T3 / T3-AGG / rev-31 WPs). New in this document relative to that provenance: stable invariant IDs (`INV-OFEHYB-001..010`), obligation/BEI/guard-map organization, and the gap register carrying the Case-4 HOLD (`22.8/15.5/10.2 %` vs `5 %`) with the one I0-recorded design lever (explicit cool-down) plus clearly-labeled non-binding assessment candidates. Status `draft` pending dual-agent contract review; lifecycle maturity `draft` (the EXPERIMENTAL selector posture is carried in the body/INV-OFEHYB-008, not the lifecycle field). `SC-OFEROUTE-001` rev 32 re-points its hybrid rows here. |
