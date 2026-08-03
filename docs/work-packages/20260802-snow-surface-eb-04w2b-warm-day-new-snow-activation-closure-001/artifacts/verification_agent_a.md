# Independent Terminal Verification — Agent A

Status: **PASS / HOLD_CROSS_DOMAIN_CORRECTNESS_GATE**

Evidence mode: **Static + Ran**

## Verdict

The terminal disposition is truthful. The snow-scoped activation and daily
storage-closure corrections are contract-backed, typed, exercised through the
real production day-input/frame consumer and the snowbench consumer, and pass
the focused current-diff gates. Closure remains prohibited because the required
quick correctness surface exposes the independently reproducible EROD16
continuity-instrument failure. Neither reverting the snow correction nor
weakening the erosion bound is admissible in this package.

The earlier W2A result is correctly retained only as prerequisite-ineligible
audit material. No terminal W2A rerun, corrected-diff full profile, promotion,
or EB-04X admission is claimed.

## Static Verification

- **Contract and tolerance:** `INV-SNOWFREEZE-089` makes material typed hourly
  snowfall independently sufficient for activation after an authorized phase
  provider while retaining the existing snow/frost/cold provider triggers.
  `TOL-SNOWFREEZE-014` uses strict `> 1e-12 m` presence thresholds and says
  equality is sub-resolution. Daily closure binds to
  `TOL-SNOWFREEZE-006 = 1e-9 m` water equivalent and the independently
  reconstructed signed equation. `INV-RUNOFFPART-033` carries the same
  fail-closed shared-consumer obligation. The wording is sufficient rather
  than exclusive and does not change phase equations or values.
- **Real direct consumer:** the regression loads the production fixture,
  constructs `DirectProductionSeedAuthority` and the production run frame,
  invokes `DirectProductionDayInputBuilder::build`, and asserts active coupling,
  a published storage-gain handoff, positive after-day SWE, and positive
  hydrology-projection SWE. Production SIMIMPL28 resolves typed phase rows for
  material precipitation; warm dry/no-pack suppression and warm all-rain/no-pack
  inactivity remain covered. No wrapper, skeleton, shadow, or compatibility-only
  path carries this claim.
- **Closure and typed errors:** the shared partition reconstructs
  `SWE_before + sum(hourly snowfall_m * 0.1) + rain_retained - snowpack_loss -
  sublimation - SWE_after` without trusting `accumulation_m`. Exact
  `+/-1e-9 m` is accepted, both next-representable magnitudes are rejected with
  `HKERNEL-WB14-RUNOFF-E-003`, and non-finite residuals are rejected with
  `HKERNEL-WB14-RUNOFF-E-002`. Snowbench maps kernel failures to
  `SnowKernel { source }` and uses the distinct `SnowStorageClosure` variant for
  its independent consumer reconstruction.
- **Assurance:** three checked `scientific-full` transactions form the exact
  generation chain `4d83e2a9... -> a26a0352... -> f2b8a335... ->
  9e64c4c7...`; each records zero invalidated authority, and the identity lock
  names terminal generation `9e64c4c7...`.
- **Disposition surfaces:** `package.md`, both roadmaps, the package catalog,
  scientific synthesis, disposition, and prompt lifecycle consistently state
  the cross-domain HOLD, withdraw the first rerun from terminal adjudication,
  withhold full/rerun closure, and keep EB-04X held. The kickoff is archived and
  `prompts/active/` contains no active kickoff.
- **Line count and write set:** the two files above 2,000 lines are correctly
  marked `WARN` (`runoff_reconciliation.rs` 2,598; runner `03_tests.rs` 2,891),
  both remain below 3,000, and decomposition intent is recorded. Production,
  contract, mechanically version-pinned tests, assurance locks/transactions,
  roadmap/catalog, new runner test, and package paths match the declared
  authority envelope.

## Commands Run Independently

From `/home/workdir/openWEPP` on the current working diff:

| Command | Result |
|---|---|
| orchestrator exact two-test filter from `gate-results.md` | **PASS**, 2/2 |
| runner exact three-test filter from `gate-results.md` | **PASS**, 3/3 |
| `cargo nextest run --test snow_surface_eb04w_accumulation_melt_diagnostics_contract` | **PASS**, 6/6 |
| `cargo run --quiet -p openwepp-assurance -- validate --all` | **PASS**, 3/3 reports; terminal generation retained |
| `cargo fmt --check` | **PASS** |
| `git diff --check` | **PASS** |
| `cargo nextest run --test erod16_wave1_continuity_fixture_conservation` | **EXPECTED BLOCKER**, exit 100; 231 storms, 170 clean, 61 `flux_closure` refusals (`26.4% > 20%`) |

The isolated failure exactly matches the retained quick-profile failure. The
historical `37/227` old-trigger reversal remains explicitly labeled an
unretained supporting observation and carries no closure claim.

## Exact Finding

- **LOW — terminal manifest wording:** `owned-file-manifest.md` calls
  tracked-only `git diff --name-only` the authoritative exact expansion, but
  the package, three assurance transactions, and new runner test are currently
  untracked. Use `git status --short --untracked-files=all` for final write-set
  reconciliation or qualify that sentence before commit. The local generated
  `tools/__pycache__/run_frozen_w2a_rerun.cpython-312.pyc` is correctly ignored
  by `.gitignore` and cannot enter the change normally. This evidence-wording
  cleanup does not alter the verified HOLD or either science authority.

Final verification: **PASS for the HOLD disposition only.** Resume terminal
full validation and the exact frozen W2A rerun only after a separately
authorized erosion-governed hold-lift restores the quick correctness gate.
