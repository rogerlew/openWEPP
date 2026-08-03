# Review Agent A

Status: complete / `HOLD` recommended

Evidence mode: **Static + Ran**

Static: reviewed the terminal tracked diff, package authority/evidence, both
amended contracts, the shared partition implementation, the direct-production
and snowbench consumer call paths, and affected tests. Ran: `git diff --check`
passed. I did not repeat the retained full-profile run; package evidence reports
`2,195/2,195` passed.

## Findings

1. **High / closure-blocking — direct production still bypasses the corrected
   typed partition through a duplicated old activation predicate.**
   `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs:52-82`
   retains the pre-fix rule: with zero prior SWE, coupling is active only when
   daily mean temperature is below zero. At lines `382-387`, that predicate is
   evaluated *before* `direct_winter_hourly_forcing` and before the corrected
   `compute_direct_snow_liquid_partition_from_typed` call at lines `389-473`.
   A warm-mean day that an hourly phase provider would classify partly as snow
   therefore returns the manually constructed inactive partition at lines
   `481-512`, with zero accumulation and without the new closure validator.
   This is the old compatibility path duplicated outside
   `runoff_reconciliation.rs:298-305`, so the package has not satisfied
   `package.md:105-106`, `package.md:130-132`,
   `SC-SNOWFREEZE-001#INV-SNOWFREEZE-089`, or the consumer-path negative-proof
   rule. The static call site cited by `real-consumer-proof.md` proves only that
   direct production sometimes calls the shared API, not that the target warm
   new-snow case reaches it. Remove or redesign the runner pre-gate so typed
   hourly forcing is resolved before activation and route the result through
   the shared partition; add a real direct-production warm-mean/zero-pack/hourly-
   snow regression that proves state and published outputs consume the result.

2. **Medium — snowbench collapses kernel/closure failures into the wrong typed
   error class.** `crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs:743-752`
   stringifies every structured `Wb11HydrologyKernelGuardError` into
   `SnowbenchError::InvalidForcing`, and the new independent consumer closure at
   lines `836-844` also emits `InvalidForcing`. Consequently a non-finite or
   material *computed storage residual* is reported as `SNOWBENCH-E-006 invalid
   forcing`; the original `HKERNEL-WB14-RUNOFF-E-002/003` class, symbol, bounds,
   and error source are unavailable. `snowbench.rs:170-179` confirms that this
   variant has no typed source. The new tests at `snowbench_coe_melt.rs:1178-1189`
   assert only message substrings, so they bless the taxonomy loss. Preserve the
   kernel source in a typed snow/closure variant and distinguish source-forcing
   disagreement from an internally computed conservation failure.

3. **Medium — the shared fail-closed closure branch has no direct validation
   evidence.** The new integration vectors at
   `tests/integration/snow_surface_eb04w_accumulation_melt_diagnostics_contract.rs:123-156`
   independently reconstruct only successful outputs. No test names
   `snow.daily_storage_closure_residual_m`, asserts the shared error variant/code,
   or drives `validate_direct_snow_storage_closure` at an over-tolerance or
   non-finite residual (`runoff_reconciliation.rs:432-458`). The snowbench-local
   helper test does not exercise this orchestrator guard. The reported full
   `2,195/2,195` regression is valuable no-regression evidence but does not prove
   this new critical error branch. Add same-module boundary tests for positive
   and negative residuals above `1e-9 m`, exact-tolerance acceptance, non-finite
   rejection, and the expected typed symbol/code.

4. **Low — line-count governance is recorded against the wrong threshold.**
   `artifacts/line-count-governance-checklist.md:7-12` calls the 2,531-line
   `runoff_reconciliation.rs` below the review trigger and supplies no follow-on
   split intent. `crates/AGENTS.md` sets `>= 2,000` lines to `WARN` and requires
   decomposition rationale plus follow-on split intent; only `>= 3,000` is the
   mandatory pre-closure refactor threshold. Record the required warning and
   follow-on disposition.

## Residual Risk And Missing Tests

- The frozen eight-cell W2A rerun is real snowbench evidence, but it cannot
  establish the still-bypassed direct-production warm-new-snow path.
- The two closure equations are intentionally independently reconstructed, but
  their private `1e-9 m` constants are duplicated across crates. Preserve
  independent operands while adding a shared contract constant or an explicit
  parity binding so their acceptance thresholds cannot drift silently.
- I did not independently rerun the full profile. The retained `2,195/2,195`,
  frost, quick, crate, and frozen-rerun results were reviewed as package evidence.

## Review Outcome

Not approved for complete disposition. The shared API's new activation
predicate and storage equation are arithmetically consistent with the amended
contract, and the warm all-rain negative control is preserved there, but the
direct real consumer bypass is a high-severity science/contract divergence.
Findings 1-3 require correction and rerun evidence before closure.

## Terminal Re-review — 2026-08-02

Status: complete / `HOLD` remains required; the cross-domain boundary is real,
but the package is not yet cleanly terminal because two in-envelope issues
remain.

Evidence mode: **Static + Ran**

### Findings

1. **Medium — `TOL-SNOWFREEZE-014` overstates the new precipitation trigger as
   an exclusive provider-availability rule and therefore conflicts with the
   retained production implementation.**
   `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md:1256`
   says production phase rows resolve **iff** finite daily precipitation is
   greater than `1e-12 m`. Production correctly retains additional established
   triggers for prior SWE, frost state/configuration, and a subzero daily mean
   at
   `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs:233-239`.
   `INV-SNOWFREEZE-089` itself uses the compatible sufficiency wording
   "whenever daily precipitation exceeds" the threshold. The tolerance row's
   biconditional would instead prohibit dry cold/pack/frost row resolution and
   makes the implementation nonconforming to the canonical text. Clarify the
   row as the material-*precipitation* sufficiency threshold; do not remove or
   narrow the pre-existing snow/frost/cold triggers.

2. **Medium — A1's implementation bypass is corrected, but its required real
   downstream proof remains incomplete.**
   The new test
   `crates/openwepp-runner/src/hillslope/tests03/eb04w2b_warm_snow.rs:2-97`
   invokes the private direct-publication `snow_liquid_partition` helper and
   asserts only the returned partition's active flag, accumulation, and SWE.
   It does not drive `DirectProductionDayInputBuilder` through the state/frame
   handoff at
   `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs:199-209,340-364,398-401`
   or verify the produced trace/output fields at lines `1184-1239`. Static
   inspection shows that chain now consumes the corrected result, but
   `docs/work-packages/AGENTS.md:130-141` explicitly classifies
   direct-runtime-internal evidence as insufficient for a direct/publication
   closure claim. This also falls short of the first-round requested regression
   proving state and published outputs. Extend the warm-mean fixture through
   the actual day-input/frame or production trace consumer and assert positive
   carried/published SWE and accumulation plus the old-path negative proof.

### A1-A4 Correction Audit

- **A1 arithmetic/control: corrected.** Material daily precipitation now makes
  the existing SIMIMPL28 typed phase provider available without changing its
  phase equations (`06_simimpl28_hourly_forcing.rs:233-243`); the runner's old
  daily-mean pre-gate is gone, and material precipitation reaches the shared
  partition (`00a_snow_frost_authority_impl.rs:52-72,360-467`). Warm dry/no-pack
  suppression remains tested. The remaining issue is consumer-path evidence,
  not the repaired activation arithmetic.
- **A2: corrected.** `SnowbenchError::SnowKernel` retains
  `Wb11HydrologyKernelGuardError` as a typed source, while
  `SnowStorageClosure` separately reports the independent snowbench residual;
  kernel and consumer-closure errors are no longer mislabeled as invalid
  forcing.
- **A3: materially corrected.** The shared guard uses the exact signed equation
  and symmetric absolute `1e-9 m` tolerance, with distinct typed non-finite and
  range failures. Its same-module test accepts exact `+/-1e-9 m`, rejects both
  next-representable magnitudes, and rejects NaN. Residual test gap: it asserts
  variants and symbol text but not the expected `error.code()` values
  (`HKERNEL-WB14-RUNOFF-E-002/003`).
- **A4: corrected.** The checklist now marks both 2,000+ line files `WARN`,
  records decomposition rationale and follow-on intent, and has already
  extracted the new runner regression into `tests03/`.

### Cross-domain Gate Assessment

The proposed cross-domain hold is scientifically and procedurally genuine. I
independently ran
`cargo nextest run --test erod16_wave1_continuity_fixture_conservation` on the
current diff: it failed with exit `100`, `231` storms, `170` clean solves, and
`61` named `flux_closure` refusals (`26.4%`), exceeding the retained `<=20%`
bound at
`tests/integration/erod16_wave1_continuity_fixture_conservation.rs:515-521`.
This matches the package's corrected-diff reproduction.

The bound is not an incidental snow-package assertion. The owning erosion port
records the concave instrument and `37/227` baseline with a hard bounded tail
and no other permitted error class at
`docs/work-packages/20260703-erosion-sediment-continuity-port-001/package.md:177-183`.
`SC-ROUTE-001:547-567` explicitly assigns this hillslope sediment-continuity
authority to `SC-SED-001`. Repairing the solver's numerical/process mechanics,
or prospectively adjudicating whether its instrument population/bound remains
the right erosion gate under corrected hydrology, requires erosion contract,
source, test, and comparator authority outside EB-04W2B's declared write set.
The result-blind old-trigger reversal is useful causal evidence but cannot be
retained: it would reintroduce the canonical snow defect. The EROD16 gate must
likewise not be weakened here.

### Residual Risk And Missing Tests

- Add the full direct day-input/frame or production-trace warm-snow regression
  described above before claiming real-consumer closure.
- Pin the shared closure guard's exact typed error codes, and add a focused
  assertion that `SnowKernel` exposes its structured source and code through
  the snowbench boundary.
- `artifacts/real-consumer-proof.md` still cites the first frozen W2A run under
  a `passed` status without labeling that run prerequisite-ineligible and
  superseded, unlike `gate-results.md` and `disposition.md`; reconcile that
  evidence before terminal handoff.
- No terminal corrected-diff full profile or frozen W2A rerun exists, correctly,
  because the renewed quick prerequisite failed. They remain required after an
  authorized erosion hold-lift restores the quick gate.

### Terminal Verdict

No high-severity science implementation defect remains in the reviewed snow
arithmetic. The corrected snow behavior must be preserved, and the reproduced
EROD16 failure is a legitimate different-process-family hold boundary. However,
`disposition.md:7` currently overclaims that all in-envelope findings are
corrected: the contract biconditional and real-consumer evidence gap above can
still be fixed inside EB-04W2B. Therefore retain `HOLD`, do not approve
`complete` or EB-04X handoff, correct those in-envelope items, and then carry
the still-binding EROD16 blocker into a separately authorized erosion-governed
hold-lift package without weakening either contract.

## Terminal Re-review Addendum — 2026-08-02

Status: **PASS / HOLD**

### Findings

No remaining high-, medium-, or low-severity in-envelope findings.

`TOL-SNOWFREEZE-014` now states that material precipitation and material typed
snowfall are independently sufficient triggers while explicitly preserving the
existing snow, frost, and cold-forcing triggers. The real-path regression now
loads the production fixture, builds typed seed authority and the production
frame, invokes `DirectProductionDayInputBuilder`, and asserts the published
storage-gain handoff, after-day SWE, and hydrology-projection SWE. The shared
closure test pins exact `HKERNEL-WB14-RUNOFF-E-002/003` codes, and
`real-consumer-proof.md` correctly labels the earlier frozen W2A run as
prerequisite-ineligible with no terminal authority.

Ran independently on the corrected diff: orchestrator-focused `2/2`, runner
focused `3/3`, EB-04W integration `6/6`, and `git diff --check`; all passed.

### Residual Risk And Verdict

The independently reproduced EROD16 `61/231` failure remains a binding
cross-domain correctness blocker. Its mechanics and gate authority remain
outside EB-04W2B, while reverting the snow trigger or weakening the erosion
bound would violate protected authority. Approve the corrected snow diff for
review purposes, retain `HOLD_CROSS_DOMAIN_CORRECTNESS_GATE`, and do not admit
package completion, terminal W2A rerun, or EB-04X handoff until the separately
authorized erosion-governed hold-lift restores the quick gate and the deferred
terminal validations pass.
