# SNOWDENSITY-10.3.5b - Opt-In Hourly Partition And Jennings Validation

Status: complete (executed by Codex, 2026-06-27).

Package type: defect-closure style physics-adjudication package; opt-in
production runtime wiring plus observed-phase validation; no default activation.

Closure target: `COMPLETE-10-3-5B-HOURLY-PARTITION-JENNINGS-VALIDATED`.

## Objective

Wire the `openwepp-meteorology` Harder-Pomeroy hourly rain/snow partition into
the openWEPP hourly winter partition seam as an explicit opt-in candidate, then
validate it against the Jennings et al. observed precipitation-phase corpus
without site calibration. The real direct snow consumer must read the opt-in
partition when selected, while the default WEPP `RST` path remains unchanged.

## Primary Authority

- `docs/planning/snow-frost-fidelity-strategy.md` section 10.3 step 5:
  SNOWDENSITY-10.3.5 robust rain/snow partition is the lead candidate after
  10.3.4 isolated near-0 degC phase partition as the top defect-eligible
  maritime over-accumulation mechanism.
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`:
  canonical snow/freeze process authority. SNOWDENSITY-10.3.5a v91 explicitly
  deferred production wiring and Jennings validation to this package.
- `crates/openwepp-meteorology`: checked psychrometric primitives and
  Harder-Pomeroy candidate core from 10.3.5a.
- Jennings et al. (2018) observed-phase corpus under
  `tests/fixtures/precip_phase_observed/jennings2018/`.
- ADR-0011 and ADR-0017: contract-first new physics and observed-data/
  comparator-as-flag posture.

## Correction Authority Envelope

In scope:

- Candidate selection at the openWEPP hourly winter rain/snow partition seam.
- A typed opt-in selector for `legacy_rst` versus `harder_pomeroy_hourly`.
- Direct-production snow consumer evidence proving the opt-in hourly forcing
  changes the real `DirectSnowHourlyForcing` read by
  `Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed`.
- Runtime-symbol projection evidence for the same hourly partition family where
  current `snow.hourly.stmtim.*` surfaces are produced.
- Jennings observed-phase validation tooling and compact artifacts.
- Contract, contract-derived tests, focused runtime tests, and final gates.

Protected boundaries:

- Default behavior must remain `legacy_rst`.
- No parser/runfile/user CLI activation.
- No fixture input edits.
- No public WAT/HBP/PASS schema changes.
- No snow density, melt, canopy, radiation, frost, or compatibility-runtime
  physics changes.
- No site-specific calibration or fitted per-station thresholds.
- No adoption of Jennings temp/RH logistic as production physics in this
  package; it is a validation comparator only if used.

## Scope

- Amend `SC-SNOWFREEZE-001` from v91 to the next revision to authorize
  candidate opt-in runtime wiring and Jennings validation.
- Add contract-derived tests for the new invariant, obligation, rollback
  boundary, and package package-status artifacts.
- Add `openwepp-meteorology` as a dependency only where the opt-in candidate is
  implemented or validated.
- Extend hourly winter forcing with an explicit partition model selector:
  `legacy_rst` (default) and `harder_pomeroy_hourly` (opt-in).
- Use daily dew point from climate forcing to derive hourly relative humidity
  from each synthesized hourly air temperature. If dew point implies
  supersaturation, behavior must match the amended contract: either fail closed
  or use an explicitly bounded, cited saturation normalization. Silent clamping
  is forbidden.
- Add a diagnostic CLI/tool path to score the Harder-Pomeroy candidate against
  Jennings file2/file3. The full local file2 corpus is gitignored; compact
  JSON/Markdown evidence artifacts are committed under this package.
- Run default versus opt-in checks proving default `legacy_rst` output is
  unchanged and opt-in output can differ at near-freezing humid/dry cases.

## Non-Scope

- Default activation or promotion of Harder-Pomeroy.
- New `.run`, `.snow`, parser, or user-facing selector syntax.
- Snow-depth rubric reruns or default snow-depth remediation.
- Calibrating Harder-Pomeroy constants, fitting a local threshold, or optimizing
  against Jennings stations.
- Susong fallback implementation unless required as a validation comparator; if
  missing fallback blocks decision, close with a named hold rather than inserting
  an ad hoc substitute.

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/index.md`
- `Cargo.toml` / crate manifests as required for dependency edges or tests.
- `crates/openwepp-hillslope-orchestrator/**` for opt-in hourly partition seam.
- `crates/openwepp-runner/**` for diagnostic validation CLI/tooling and direct
  opt-in runtime selection.
- `tests/integration/**` for contract/runtime/tool guards.
- Package-local files under
  `docs/work-packages/20260627-snowdensity-10-3-5b-hourly-partition-jennings-validation-001/**`.

Any production edit outside the hourly partition seam, runner selector, or
validation tooling must be stopped and dispositioned before continuing.

## Required Evidence Artifacts

- `artifacts/required-reading-map.md`
- `artifacts/contract-implementation-evidence.md`
- `artifacts/contract-test-implementation-evidence.md`
- `artifacts/pre-implementation-contract-gate.md`
- `artifacts/runtime-wiring-evidence.md`
- `artifacts/jennings-validation-report.json`
- `artifacts/jennings-validation-report.md`
- `artifacts/default-rollback-evidence.md`
- `artifacts/no-scope-creep-scan.md`
- `artifacts/implementation-test-evidence.md`
- `artifacts/kernel-profile-compliance-checklist.md`
- `artifacts/gate-results.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/review-disposition.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
- `artifacts/line-count-governance-checklist.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/worker-handoff.md`
- `artifacts/disposition.md`

Evidence must label `Static:` versus `Ran:`. Placeholder artifacts must be
updated during execution; do not close on stale placeholders.

## Acceptance Gates

- Contract-first sequence is followed: canonical contract amendment, then
  contract-derived tests, then pre-implementation contract gate, then runtime
  code.
- Default `legacy_rst` behavior remains unchanged by focused tests that compare
  old threshold semantics and by no-scope-creep scans.
- The opt-in Harder-Pomeroy model is selected only through the package-authorized
  runtime selector and is not the default.
- The real direct-production snow consumer reads the opt-in partition result
  when selected; producer-only evidence is insufficient.
- Jennings validation runs against the available corpus:
  - if full gitignored file2 exists locally, run it and record row count;
  - if unavailable, close `HOLD-JENNINGS-FILE2-ABSENT` unless the package was
    amended before implementation to accept a committed subset.
- Validation reports must include rows scored, stations scored, accuracy,
  rain/snow confusion counts, per-station predicted 50% air-temperature
  threshold summaries, and maritime-vs-continental threshold contrast where
  station metadata supports it.
- Conservation of precipitation partitioning: hourly rain depth plus snowfall
  water-equivalent depth divided by the legacy `10x` snowfall-depth scale must
  reconstruct active hourly precipitation within tolerance for both models.
- No site calibration, coefficient tuning, fixture input edits, public output
  schema changes, default activation, density/melt/canopy/frost changes, or
  compatibility-runtime deletion.
- Final gates:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`

Any failed, blocked, or unjustified not-run required gate prevents `COMPLETE`.

## Phase Plan

1. Orientation and package scaffold.
   - Read required instructions and record `required-reading-map.md`.
   - Confirm current package envelope and write set.
2. Contract-first authority.
   - Amend `SC-SNOWFREEZE-001` for SNOWDENSITY-10.3.5b opt-in wiring and
     Jennings validation.
   - Add/update contract-derived tests.
   - Record `pre-implementation-contract-gate.md`.
3. Runtime opt-in wiring.
   - Add a typed partition model selector at the hourly winter forcing seam.
   - Preserve default `legacy_rst`.
   - Prove the direct snow consumer receives opt-in rain/snow partitions.
4. Jennings validation.
   - Add a diagnostic validation command/tool.
   - Run full local Jennings file2 when present and write compact package
     artifacts.
5. Closure.
   - Run focused and full gates.
   - Complete review, disposition, verification, line-count governance,
     handoff, and final disposition.

## Subagent Authorization

Subagent authorization: none. This package does not explicitly authorize
spawning/delegating to subagents; required review and verification may be
performed locally with evidence labels.

## Downstream

If Harder-Pomeroy improves observed-phase validation and protected openWEPP
snow-depth signatures, a later package may decide promotion/default activation.
This package may recommend that route but cannot activate it.
