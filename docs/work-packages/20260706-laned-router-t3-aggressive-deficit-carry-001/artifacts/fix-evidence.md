# LANED-T3-AGG — Fix evidence: aggressive-rule deficit-carry composition

Status: **EXECUTED** (2026-07-06/07). Evidence mode: **Ran** (unit vectors,
H2637 release runs, instrumented diagnostic run — all executed this session)
+ Static (design deltas). Contract authority: `SC-OFEROUTE-001` rev 30
(amendment authored BEFORE the behavior change).

## What landed

1. **Solver-API extension** (`kinematic_wave.rs`):
   `run_with_options_deficit_carry(...) -> (RoutingResult, f64)` returns the
   recorder's material terminal deficit (`<= 0`, m²) instead of failing
   closed; documented as composition-scoped (the returned bin series
   over-counts by exactly `|deficit|`; the caller must continue the forward
   redistribution or fail closed itself). The public `run_with_options` is
   now a thin wrapper that keeps the exact fail-closed posture
   (`NegativeOutletBin` on material deficit) — the plain/default paths are
   behavior-identical (H2637 plain-active parquet hash unchanged, below).
2. **Cross-span deficit carry** (`cascade.rs`, `route_single_ofe_hybrid`):
   a running carry (`<= 0`) continues the recorder's rev-24
   forward-redistribution rule across span boundaries — each composed bin
   (implicit outcome or explicit-span bin) books
   `absorb_deficit(mass, &mut carry)` (exact total by construction:
   `booked + new_carry == mass + old_carry`); an explicit span's terminal
   deficit joins the carry after its own bins. End-of-window disposition
   (`dispose_terminal_carry`): a MATERIAL remaining deficit fails closed
   (`NegativeOutletBin`); a sub-noise remainder (recorder noise rule at the
   composed level, `1e-9` of series gross) is absorbed BACKWARD from the
   trailing positive bins (exact total, non-negative bins).
3. **Aggressive mask** (rev 30, supersedes rev-28 strict): a sample bin is
   implicit-eligible when its SOURCE rate is zero — the zero-upstream-mass
   condition is dropped; the implicit step books the interval-mean upstream
   inflow exactly (machinery already present from T3-I2).

## Unit vectors (Ran, retained)

- `bin_recorder_returns_material_terminal_deficit_exactly`
  (kinematic_wave.rs): pins the seam the composition consumes — a material
  terminal-bin deficit is RETURNED (not folded), bins stay non-negative,
  and `Σ bins == booked total − deficit` holds exactly.
- `absorb_deficit_exact_total_and_non_negative` (cascade.rs): full/partial
  absorption, pass-through on zero bins, identity booking — the exact-total
  property at every step.
- `dispose_terminal_carry_material_deficit_fails_closed`: material
  end-of-window deficit → `NegativeOutletBin`; includes the all-dry-series
  degenerate case.
- `dispose_terminal_carry_subnoise_absorbs_backward_exactly`: sub-noise
  remainder absorbed backward from trailing bins — exact total, leading
  bins untouched, zero-carry no-op.
- `hybrid_aggressive_routes_upstream_fed_zero_source_bins_implicitly`:
  the rev-30 mask pin — a zero-source window with nonzero upstream inflow
  steps 8/8 bins implicitly (profile counters), books the upstream mass
  exactly, ledger and bin-series totals close at machine precision.
- Rev-28 vectors unchanged and green: all-explicit bit-identity
  (`to_bits` per bin), event-day ledger/fidelity, non-integral-window
  rejection (all use `upstream: None`, where strict ≡ aggressive).

**Honest coverage note:** there is no synthetic end-to-end vector that
forces the SOLVER to produce a terminal deficit (the front-arrival /
boundary-ripple attribution class needs a step-scale negative dip in the
final bin of a window; scans over rain-grown and prescribed-flux fronts did
not reproduce it at unit scale). The seam is instead pinned on both sides
(recorder-return identity; composition absorption) and the REAL class is
exercised by the instrumented H2637 run below.

## Executed H2637 evidence (native-patched fixture, `taskset -c 4`, release)

Binary provenance note: the first evidence attempt ran a STALE rev-29
binary (the workspace-level `cargo build --release` does not relink the
runner bins; its books were bit-identical to the rev-29 strict record,
which exposed the staleness). All numbers below are from the rebuilt
rev-30 binary (`cargo build --release -p openwepp-runner --bins`).

| Case | User time (3 runs) | Books |
|---|---|---|
| plain active (rev 27 path) | sanity only | parquet `21c54bf2…` UNCHANGED — plain-path invariance holds; outlet `374,463.08 m³` |
| HYBRID aggressive (rev 30) | `38.28 / 38.32 / 38.04 s` | outlet `371,322.66 m³`; closures: supply `7.3e-16`, cascade `3.26e-13`, seam `1.65e-14`, identity `3.30e-13`; clamp `340.3 m³` (vs plain `3,207`, strict `2,439`); deterministic across runs (identical books to the last bit); parquet `a5fb9233…` |
| (record) HYBRID strict (rev 29) | `37.02 / 37.15 / 37.22 s` | outlet `373,581.06 m³` (T3 package record) |

- **The former failure coordinates pass**: the full 731-day / 610-routed-day
  year completes under the aggressive rule (rev 28 observed
  `NegativeOutletBin` at lane 17 day 54); all four rev-27 day-closure
  hard-fails ran LIVE on every routed day and passed at machine precision.
- **The deficit-carry path FIRED — instrumented diagnostic run** (temporary
  `eprintln` at the span-boundary carry, reverted after the run; separate
  non-timed build): **6 carry events** over the year, span-terminal
  deficits `3.06e-9 … 1.23e-5 m²` (material vs the recorder noise floor),
  every one absorbed by subsequent composed bins; end-of-window fail-closed
  never triggered. Under the rev-29 code each of these six would have
  failed the run closed. Books identical between instrumented and clean
  binaries.
- **Fidelity (UNRATIFIED, evidence-gathering)**: aggressive outlet differs
  from plain by `−0.84 %` and from strict by `−0.60 %` (both end-state
  differences; every ledger is exact). Ratification remains the parent
  package's open gate; this widens the evidence base for it.

## Timing disposition — the ~1.9x prize did NOT materialize (honest)

Profiled decomposition (`OPENWEPP_LANED_SHADOW_PROFILE=1`, one run each):

| Counter | plain active | aggressive hybrid |
|---|---|---|
| solver_steps (explicit) | 10,479,200 | 4,660,296 |
| solver_steps_implicit | 0 | 1,146,432 |
| alpha_evaluations (explicit slots) | 173.8 M | 88.4 M |
| solver_cfl_ns | 24.6 s | 14.6 s |
| solver_step_ns | 10.4 s | 5.4 s |

The aggressive mask removed exactly the I0-measured share of explicit work
(−55.5 % explicit steps; explicit slot time roughly halved) — the coverage
arithmetic was right. The win is consumed by the IMPLICIT CELL SOLVES,
which these slots do not instrument: ~1.15 M implicit bin-steps × 20 cells
= ~23 M scalar equilibrium solves, each cold-started from the rev-29
deterministic basin-split seeds (`Q_c·1e-3` / `Q_c·1e3`) — the I0 prize
estimate assumed implicit-step cost comparable to explicit steps, and that
assumption is now measured FALSE at current solve cost. Net endpoint:
`38.0-38.3 s` vs `37.9 s` plain (~wash) and `37.0-37.2 s` strict.

**Named follow-on lever (composes with Tier-1):** reduce implicit
cell-solve cost under the rev-29 determinism constraint — candidates:
deterministic within-step warm seeding (the downstream march can seed cell
`i` from its own already-solved upstream neighbor's converged `q`, which is
deterministic state of the SAME step, not run history), Newton on the
composed cell residual instead of nested fixed-point iteration, and the
Tier-1 `pow`/friction-eval reductions which cut the cost of every
equilibrium iteration. Until that lands, the aggressive selector's value is
EVIDENCE (fidelity-ratification coverage + live deficit-carry exercise),
not endpoint time.

## Scope check

- Default path: untouched (selector-gated; workspace suite green).
- Plain active path: `run_with_options` wrapper is behavior-identical;
  parquet hash `21c54bf2…` reproduced exactly.
- Strict rule: superseded by rev 30 (recorded in the contract changelog;
  the rev-28/29 strict evidence remains in the parent package record).
