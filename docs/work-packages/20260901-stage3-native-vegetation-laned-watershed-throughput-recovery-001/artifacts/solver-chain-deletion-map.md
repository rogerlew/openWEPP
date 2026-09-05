# Solver-chain deletion map

Status: `IMPLEMENTED — TERMINAL ANTI-ACCRETION EVIDENCE RETAINED; PACKAGE HOLD`

Evidence mode: `Static`

The opening inventory below is historical pre-deletion evidence. On the
current source the v33--v57 production cascade is deleted, the retained
historical test vectors are production-unreachable, and one canonical covered
solver remains.

The production path reaches `open_snow.rs`, which first performs ordinary
Picard iteration and then dispatches history-dependent v33--v57 triggers,
separate enthalpy/frozen solvers, root polishing, receipt cycles, Q-lattice
search, a post-root solver handoff, and finally a v34 recovery back to ordinary
Picard after a pre-root failure. This is the ADR-0044 quarantine.

## Exhaustive pre-iteration physical regime partition

Regime selection reads only the immutable beginning owner, sealed forcing,
represented snow mass, and event support before iteration. Frozen, mixed,
thaw/refreeze, and terminal-crossing states are active sets of the same
covered conservation equations; they are not solver-selection history.

| Beginning physical predicate | Canonical treatment | Numerical solver |
| --- | --- | --- |
| no represented snow owner | snow-free persistent WB/LSE/vegetation support | no covered solver |
| represented snow with total ice `<=1 kg m^-2` | execute the existing version-22 terminal one-volume enthalpy/adaptive process as the sole non-CoE thin-pack process; retain the cold solid Stage-3 owner or transfer the accepted terminal event exactly once | V22 thin-pack physical process, never the covered solver and never CoE |
| represented snow with total ice `>1 kg m^-2` and no accepted solid-exhaustion event inside the support | one covered water/enthalpy/LSE/soil conservation system; cold, mixed, melt/refreeze, density, and layer-count predicates are complementarity/constitutive branches inside that system | one canonical covered solver |
| the canonical covered trial proves an earliest solid-exhaustion event inside the support | reject the unsplit parent and invoke the existing canonical adaptive/event partition; solve the covered child with the same covered solver and the successor snow-free child without a covered solver | same covered solver plus existing event partition, never a recovery solver |
| any regime has nonfinite, out-of-domain, discontinuous topology, or unsealed source/custody input | fail before iteration | typed error; no fallback |

The initial canonical covered iterate is the immutable beginning owner projected
once into total-water/enthalpy/soil endpoint coordinates. A bounded
deterministic safeguarded solve evaluates only authentic physical maps. It may
use analytic constitutive derivatives and secant information from already
charged authentic maps, but cannot finite-difference by coordinate, enumerate
ULPs, search receipt cycles, or replay a second algorithm. Budget exhaustion
requests the existing smaller time support; exhaustion at the exact floor is a
typed nonconvergence.

## Reachable symbol and call-site classification

| Current symbol/seam | Production call site | Classification |
| --- | --- | --- |
| ordinary `for iteration in 0..COVERED_FIXED_POINT_POLICY.max_iterations` authentic-map Picard loop | `open_snow.rs:652` with evaluation/finalization calls through the same function | replace as the sole covered control loop; retain authentic physical map and closure guards, delete historical relaxation/cycle control |
| `covered_stable_monotone_*` | `open_snow.rs:875-898`, `2282-2294`, `2532+`, `2728+`, `3020+` | delete production selector, fallback, and reset surfaces |
| `phase_consistent_coupled_active_set_transition_window_v1` and parity/phase-history selectors | `open_snow.rs:1042-1123` | delete; retained captures become physical-boundary tests only |
| `covered_frozen_temperature_primary_eligibility_v1` | `open_snow.rs:1231-1259` | delete; strict-frozen is an active set of the canonical covered equations |
| `covered_frozen_temperature_primary_solve_v1` | `open_snow.rs:2052-2069`, `2128+` | delete solver and post-root handoff; migrate physical frozen equations/carry custody only |
| `phase_consistent_coupled_physical_solve_v1` | `open_snow.rs:2072-2090` | delete historical finite-difference/trust cascade; migrate W/H/CN residual equations |
| `covered_frozen_temperature_primary_post_root_transition_v1` | `open_snow.rs:2096-2146` | delete solver-history transition |
| `phase_consistent_coupled_root_polish_v1` | `open_snow.rs:2150-2167` | delete separate polish/recovery algorithm |
| exact receipt-cycle and private-Q lattice helpers | `open_snow.rs:2179-2252` through included solver files | delete continuous exact-cycle/ULP search; preserve exact discrete receipt envelope only |
| stable-monotone pre-root refusal back to raw Picard | `open_snow.rs:2281-2296` | delete fallback |
| snow-free kernel and terminal event partition | attachment/adaptive execution and `open_snow.rs:344-360` | retain physical regime/event control; never a numerical fallback |

Older LSE implementation variants are not solver regimes. The canonical
Stage-3 runner is required to construct only the current native-vegetation LSE
owner and must reject legacy/CoE generation in the qualification configuration;
static negative tests bind this before a production claim.

| Version | Current production role | Disposition |
| --- | --- | --- |
| v33 | active-set reset and reduced W/H/soil solver | Delete trigger/dispatch; migrate physical residual equations. |
| v34 | eight-map stable-monotone selector and failure fallback | Delete selector, trace, and fallback. |
| v35 | exact authentic receipt/artifact stabilization | Delete continuous exact-fixed-point admission; retain discrete receipt custody only. |
| v36--v38 | density, derived thickness, finalization-equivalent map | Migrate density/geometry equations and one authentic map without version dispatch. |
| v39 | source/soil transaction separation | Migrate exact custody invariant. |
| v40--v41 | parity and phase-history selectors | Delete selectors; retain useful physical-boundary vectors as regime tests. |
| v42 | cold-content export operand | Migrate unchanged physical ledger operand. |
| v43--v44 | projected-coordinate custody and provisional LSE closure | Migrate typed custody, exact-once CN consumption, and strict final closure. |
| v45--v46 | root polish, reserve, and step preflight | Delete cascade; migrate only bounded canonical-step/evaluation-budget safety. |
| v47--v50 | atomic install and source/resident/target custody | Migrate exact ownership invariants unchanged. |
| v51 | post-crossing history selector | Delete selector; retain boundary vector as a regime/guard test. |
| v52--v53 | CN heat coordinate and endpoint seed | Migrate the CN equation and exact-once operand; delete version seed dispatch. |
| v54 | exact receipt-cycle witness | Delete production cycle search. |
| v55 | binary64 private-Q lattice enumeration | Delete production ULP search. |
| v56 | frozen temperature-primary specialization | Delete historical specialization dispatch; migrate frozen equations/carry into the one canonical covered solver. |
| v57 | bounded-liquid eligibility and post-root handoff | Delete solver-history eligibility/handoff; retain the exact liquid ledger and contract-authorized domain guard only. |

## File disposition

- `phase_consistent_coupled_solve.rs`,
  `phase_consistent_temperature_primary.rs`,
  `phase_consistent_private_q_lattice.rs`, and `stable_monotone.rs`:
  production algorithms are deleted after required equations/guards move into
  the canonical replacement.
- `open_snow.rs`, `fixed_point.rs`, `receipt_sets.rs`, and `regime.rs`: migrate
  one pre-iteration regime selector, one bounded algorithm per physical
  regime, tolerance-based continuous admission, exact discrete joins, and one
  typed nonconvergence route.
- `open_snow_convergence_vNN_tests.rs`: delete historical algorithm obligations
  or rewrite them as equation/regime/guard vectors. Dead implementations do
  not retain test authority.
- persisted restart V4/V5 readers: `schema-compatibility`; retain decoding and
  migration, but no schema field may select historical numerical behavior.
- snow-free support and terminal-event physical kernels: `migrate/retain`;
  these are physical regimes/events, not numerical recovery versions.
- version-22 thin-pack terminal one-volume enthalpy/adaptive process:
  `migrate/retain` as the sole non-CoE `<=1 kg m^-2` physical process. It
  reevaluates support fluxes on each admitted microstep, retains a cold solid
  Stage-3 owner or transfers one accepted terminal event exactly once, and is
  never a numerical fallback from the represented-covered solver. CoE is a
  typed poison in this regime.

## Required terminal negative proof

Source and call-path scans must find no production symbol or error recovery
that selects v33--v57, a prior solver, an exact receipt cycle, or Q-lattice
search. Failure injection must reach only the same canonical solver's bounded
response, canonical adaptive support retry, or typed error. Real runner proof
must show the accepted canonical result flows into native vegetation, Stage 3,
Lane D, and final ledgers.
