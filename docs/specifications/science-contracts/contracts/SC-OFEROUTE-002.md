---
contract_id: SC-OFEROUTE-002
title: Hybrid Implicit-Explicit Kinematic-Wave Stepping Contract
status: draft
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 1
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

Status: `draft` (pending dual-agent contract review)
Maturity: `draft` (lifecycle vocabulary; the SUBSYSTEM posture is EXPERIMENTAL/unpromoted — selector-gated evidence-gathering, see INV-OFEHYB-008)
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

### 4. Switching predicate (rev 30, AGGRESSIVE; one named open gap)

The day window is partitioned into outlet sample bins. A bin is
IMPLICIT-ELIGIBLE iff its seam SOURCE rate is zero on every cell of the
lane (zero-source-only; upstream inflow does NOT force explicit stepping —
the implicit step books the interval-mean upstream mass exactly).
Properties this contract binds:

- The predicate is FORCING-derived, deterministic, and hysteresis-free —
  knowable at bin start from the seam series alone.
- The bin-start point sample is exact ONLY for bin-constant source, so the
  composition FAILS CLOSED unless the sample cadence partitions the seam
  hour exactly (rev-30 C-M1 guard).
- **Named open design gap (GAP-OFEHYB-001, ratification HOLD):** the
  predicate proxies "smooth" by "source-quiet", but a kinematic front can
  OUTLIVE its source — the Case-4 (Iwagaki) post-cutoff phase carries the
  shock through source-quiet bins, where the implicit scheme's first-order
  diffusion smears the peak beyond the ratified oracle tolerance. Any
  predicate change (e.g. a wave-quiet term) amends THIS section
  contract-first.

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
| Acceleration basin lock | plain triple or accelerated point crosses `Q_c` | Reject the accelerated point; continue plain iteration. | Deterministic rejection. | `INV-OFEHYB-003` |
| Step-residual guard | booked step identity off by > `1e-9` relative (dust-floor scale) | Typed failure; no unvalidated state commits. | Typed runtime hard fail. | `INV-OFEHYB-001` |
| Material terminal deficit | end-of-window carry beyond the noise floor | Fail closed. | `NegativeOutletBin`. | `INV-OFEHYB-006` |
| Sub-noise terminal remainder | carry within the noise floor | Backward absorption; bounded all-dry drop. | Approved bounded disposition (documented + test-pinned). | `INV-OFEHYB-006` |
| Promotion | any attempt to ratify/promote the selector | Must meet the full acceptance surface (§Tolerances). | HOLD — currently failing (gap register). | `INV-OFEHYB-008` |

## Invariants

| Invariant ID | Statement | Guard | Failure posture | Authority | Evidence |
|---|---|---|---|---|---|
| INV-OFEHYB-001 | Implicit-step ledger exactness BY CONSTRUCTION: booked inflow/rain/outflow/storage equal the scheme's actual fluxes to the solve residual tolerance; no clamp class; the step-residual hard guard (relative `1e-9`, dust-floor absolute scale) rejects any violation. | runtime guard + I1 exactness vectors | Typed runtime hard fail. | This contract §Algorithm 1/3 | `[DIRECT][Ran]` (I1 ladder residuals ≤ 1.7e-14) |
| INV-OFEHYB-002 | Unconditional positivity: no negative depth/discharge for any `Δt`; no `max(0)` clamps anywhere in the implicit path. | construction + I1 vectors | Typed failure (a negative would indicate a solve defect). | §Algorithm 1 | `[DIRECT][Static] + [DIRECT][Ran]` |
| INV-OFEHYB-003 | Determinism: the converged cell state is a pure function of (cell parameters, `rhs`, `Δt/Δx`, branch) — independent of run history. Warm seeds and acceleration must provably converge to the same value (warm-seed acceptance requires FINITE + POSITIVE + evaluated-branch-side, else cold fallback; acceleration is basin-locked). | seed/acceleration guards + regression vectors | Deterministic fallback/rejection; divergence is a defect. | §Algorithm 2/3; rev-29/31 rules | `[DIRECT][Ran]` (rev-29 bit-identical books; rev-31 evidence) |
| INV-OFEHYB-004 | Branch closure: LOW→HIGH preference; a both-branches-jump outcome is unreachable for genuine physics (double-collapse theorem) and fails closed. No mass-exact filled-jump commit exists. | solve-chain structure + LOW-jump→HIGH-root vector | Typed non-convergence hard fail. | §Algorithm 2 theorem | `[DIRECT][Static] (proof) + [DIRECT][Ran]` |
| INV-OFEHYB-005 | Switching-predicate soundness: forcing-derived, deterministic, hysteresis-free; implicit only on zero-source bins; the cadence must partition the seam hour (fail closed) so the bin-start sample is provably bin-constant. | preflight guards + C-M1 vector | Typed degenerate-configuration failure. | §Algorithm 4 | `[DIRECT][Ran]` |
| INV-OFEHYB-006 | Composition exact-total: the composed published bin series is non-negative and sums to the booked outflow EXACTLY EXCEPT for the single approved C-L1 disposition (an un-absorbable sub-noise remainder on an all-dry/insufficient-gross series is dropped, bounded by the noise floor); deficits move ATTRIBUTION forward in time, never mass; material end-of-window deficits fail closed. | carry algebra + disposition guards + rev-30 vectors | `NegativeOutletBin` / bounded documented disposition. | §Algorithm 5 | `[DIRECT][Ran]` (H2637 6-event carry evidence) |
| INV-OFEHYB-007 | Non-perturbation: selector unset ⟹ plain/default byte identity; all-explicit windows bit-identical to the plain path; the deficit-returning solver variant is composition-scoped and every other caller keeps the fail-closed wrapper. | bit-identity vector + parquet pin + call-site audit | Byte/bit diff blocks; wrapper regression is a defect. | §Algorithm 5/6 | `[DIRECT][Ran]` (parquet `21c54bf2…` pinned) |
| INV-OFEHYB-008 | Acceptance/promotion gate: the hybrid must meet the `SC-OFEROUTE-001#INV-OFEROUTE-011` Case-4 oracle surface at EVERY ladder rung AND carry ratified fidelity tolerances before any promotion beyond evidence-gathering. CURRENTLY FAILING (GAP-OFEHYB-001) — promotion is HELD. | Case-4 hybrid ladder vector (retained, ignored-with-reason) + ratification process | HOLD; no promotion path exists while failing. | Parent oracle surface | `[DIRECT][Ran]` (`22.8/15.5/10.2 %` vs 5 %) |
| INV-OFEHYB-009 | Day-closure inheritance: the rev-27 `SC-OFEROUTE-001` day-closure hard-fails (supply, router-internal, seam cross-ledger, day identity) apply VERBATIM under the selector at their named tolerances. | live rev-27 guards | Typed runtime hard fail. | SC-OFEROUTE-001 rev 27 | `[DIRECT][Ran]` (H2637 machine-exact under hybrid) |
| INV-OFEHYB-010 | Diagnostics obligations: implicit step and solve-cost counters are surfaced on every profiled run; subsystem timing claims must cite them. | profile plumbing + evidence convention | Missing counters invalidate timing evidence. | §Algorithm 6 | `[DIRECT][Ran]` |

## Invariant Guard Map

| Invariant ID | Enforcement path | Guard class | Failure behavior | Evidence artifact |
|---|---|---|---|---|
| `INV-OFEHYB-001..002` | `ofe_routing::implicit_recession` step guard + `implicit_step_ledger_is_exact_and_positive`, `implicit_step_books_upstream_inflow_exactly`, `dust_scale_steps_do_not_accumulate_a_material_leak` | runtime + test | typed hard fail | T3 `i1-implicit-stepper-evidence.md` |
| `INV-OFEHYB-003` | seed/acceleration acceptance rules + `steady_state_is_a_fixed_point_of_the_implicit_step`, `branch_warm_seed_preserves_solution_and_reduces_or_matches_map_work`, `branch_warm_seed_acceptance_is_basin_locked` | runtime + test | deterministic fallback; divergence = defect | T3 rev-29 disposition; rev-31 WP |
| `INV-OFEHYB-004` | solve-chain ordering + double-collapse typed error + `low_jump_recovers_high_branch_root_and_never_commits_filippov` | runtime + test | typed hard fail | rev-29 disposition |
| `INV-OFEHYB-005` | hybrid preflight guards + `hybrid_rejects_cadence_that_does_not_partition_the_seam_hour`, `hybrid_rejects_non_integral_windows` | runtime + test | typed degenerate-configuration | T3-AGG WP |
| `INV-OFEHYB-006` | `absorb_deficit`/`dispose_terminal_carry` + the `rev30_deficit_carry_tests` functions + `bin_recorder_returns_material_terminal_deficit_exactly` | runtime + test | `NegativeOutletBin` / bounded documented drop | T3-AGG `fix-evidence.md` |
| `INV-OFEHYB-007` | `hybrid_is_bit_identical_on_all_explicit_windows` + plain-parquet pin + `pub(super)` scoping of the deficit variant | test + run evidence | bit/byte diff blocks | T3/T3-AGG gate results |
| `INV-OFEHYB-008` | `case4_hybrid_manning_ladder_meets_iwagaki_oracle` (retained, ignored-with-reason; ignored-only reproduction command recorded) | validation | HOLD | rev-31 `ratification-evidence.md` |
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
  fidelity tolerances for the implicit phase. **Current executed result:
  FAIL — peak `22.8 / 15.5 / 10.2 %` (improving under refinement; every
  rung out of tolerance). Ratification and promotion are HELD
  (GAP-OFEHYB-001).**
- Timing evidence convention: endpoint claims must cite the rev-31 counters
  (H2637 record: `36.61 s` user hybrid vs `37.9 s` plain; `274.7 M` map
  evaluations / `37.2 M` branch evaluations).

## Test-Vector Obligations

| Obligation | Minimum vectors | Expected evidence |
|---|---|---|
| Implicit stepper (I1 family) | Exactness + positivity at every dt/mesh rung; steady-state fixed point; recession dt-refinement ladder vs the upwind/characteristics reference; 10k-step dust accumulation; LOW-jump→HIGH-root recovery. | Retained `implicit_recession` suites; T3 I1 evidence. |
| Solve determinism | Warm-seed fallback and basin-lock regressions (converged values unchanged vs plain iteration). | rev-29/31 retained vectors + evidence. |
| Switching predicate | Hour-partition fail-closed (C-M1 scenario); non-integral window; aggressive coverage pin (zero-source + upstream ⟹ all-implicit, upstream mass booked exactly). | `rev30_deficit_carry_tests` + rejection vectors. |
| Composition deficit carry | Recorder deficit-return identity; absorb/dispose exact-total + non-negativity; material end-of-window fail-closed (incl. all-dry); sub-noise backward absorption; bounded all-dry drop. | rev-30 vector family. |
| Non-perturbation | All-explicit bit-identity; plain-parquet pin under selector-off. | T3/T3-AGG gate artifacts. |
| Acceptance (HELD) | Case-4 FULL-hybrid oracle ladder at the parent ratified tolerances (retained, ignored-with-reason while failing; ignored-only reproduction command recorded). | rev-31 `ratification-evidence.md` + `case4-hybrid-ignored-ratification.log`. |

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
| GAP-OFEHYB-001 | **Shock-outlives-source: the forcing-derived switching predicate routes shock-carrying source-quiet bins implicitly.** The Case-4 hybrid ladder fails the parent ratified peak tolerance at every rung (`22.8 / 15.5 / 10.2 %` vs `5 %`, improving under refinement — the implicit phase's first-order diffusion smearing an in-mesh front). ONE recorded design lever exists for the lift, to be adjudicated contract-first in a design increment: the I0-recorded **explicit cool-down** — remain explicit for K bins after source-off until the homogeneous TV(q) transient is below the rev-25 bound (provenance: T3 `i0-scheme-design.md` §2 residual-risk fallback). NON-BINDING ASSESSMENT CANDIDATES (no provenance beyond this WP's authoring session, 2026-07-07; NOT authority until a contract-first design increment adopts one): a spatial wave-quiet predicate (e.g. max relative inter-cell depth jump below a named threshold), noting that a q-vs-equilibrium departure test cannot discriminate because kinematic state sits ON the rating — any state-derived discriminator must be spatial or transit-time-based. The same smearing class plausibly contributes to the H2637 aggressive fidelity delta (`−0.84 %`), so the lift feeds both held gates. | **Blocks INV-OFEHYB-008**: no ratification, no promotion; the selector stays evidence-gathering. | OPEN — next design increment on the T3 queue. | `[DIRECT][Ran]` (rev-31 ladder) |
| GAP-OFEHYB-002 | **Implicit solve cost remains the endpoint bottleneck.** H2637: `274.7 M` equilibrium map evaluations ≈ 12 per cell-solve after rev-31 warm seeding (endpoint `36.61 s` vs `37.9 s` plain — first net win, modest). Remaining levers: Newton on the composed cell residual (replacing nested fixed-point iteration), Tier-1 friction-evaluation cost cuts (compose multiplicatively), further deterministic seeding improvements under INV-OFEHYB-003. | Bounds the subsystem's endpoint value; does not block correctness. | OPEN — optimization increments; each must re-prove INV-OFEHYB-003. | `[DIRECT][Ran]` (rev-31 counters) |

## Revision History

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-07-07` | `1` | `Claude Code` | Initial authority CONSOLIDATION (WP `20260707-laned-router-hybrid-contract-authority-001`): normative content assembled from `SC-OFEROUTE-001` revs 28 (scheme + selector + Z-rating discovery), 29 (double-collapse theorem, basin-locked Steffensen, dust floor), 30 (aggressive mask, hour-partition guard, cross-span deficit carry + dispositions), 31 (branch-local warm seeding, solve-cost counters), the T3 design record (`i0-scheme-design.md` — including the recorded explicit cool-down fallback now carried in GAP-OFEHYB-001), and the executed evidence chain (T3 / T3-AGG / rev-31 WPs). New in this document relative to that provenance: stable invariant IDs (`INV-OFEHYB-001..010`), obligation/BEI/guard-map organization, and the gap register carrying the Case-4 HOLD (`22.8/15.5/10.2 %` vs `5 %`) with the one I0-recorded design lever (explicit cool-down) plus clearly-labeled non-binding assessment candidates. Status `draft` pending dual-agent contract review; lifecycle maturity `draft` (the EXPERIMENTAL selector posture is carried in the body/INV-OFEHYB-008, not the lifecycle field). `SC-OFEROUTE-001` rev 32 re-points its hybrid rows here. |
