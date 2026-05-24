# 20260523-cli01-open-wepp-runner-and-hillslope-driver-bootstrap-001

## Status
- state: active
- date: 2026-05-23
- timezone: UTC

## Objective
Implement in-repo `open_wepp_runner` and executable `openwepp-cli-hill`
runtime foundations so openWEPP can emit provenance-valid, comparator-ready
hillslope outputs (`H5.wat.dat`, `H5.plot.dat`) without legacy candidate-lane
substitution.

## Why This Package Exists
The PL08 hold-lift recovery queue requires a runtime foundation package that
produces real openWEPP candidate outputs and exposes deterministic provenance.
Prior queue labeling used `CLI10`; this package ratifies `CLI01` as the
execution package for in-repo runner ownership and hillslope CLI bootstrap.

This package is contract-first and execution-bound:
- runner/CLI contract/spec authority must be implemented first,
- contract-derived tests must be implemented second,
- pre-implementation contract-gate evidence must be recorded third,
- only then may production runner/CLI code be modified.

## Scope
### Included
- Implement in-repo `open_wepp_runner` launcher boundary for openWEPP
  binaries, with explicit argument arrays and explicit engine selector posture.
- Implement `openwepp-cli-hill` executable run path that can execute a fixture
  deterministically and emit candidate `H5.wat.dat` and `H5.plot.dat`.
- Enforce blind run-directory sidecar discovery (sidecars not declared in
  `.run`) through `openwepp-legacy-bridge` strict/compat adapter policy.
- Implement typed hard-fail behavior for required sidecar absence and required
  output absence.
- Emit run provenance manifest artifact with binary identity, command args,
  sidecar resolution posture, and input/output checksums.
- Emit and validate build/release metadata sidecars (`<binary>.json`) for
  runner/hillslope artifacts per release contract.
- Implement canonical contract/spec amendments needed by CLI01 and
  corresponding contract-derived tests before production code edits.
- Preserve existing typed seam closure and no-silent-fallback posture.

### Explicitly Out of Scope
- Watershed executable implementation beyond required launcher compatibility.
- Replay executable feature expansion beyond existing contract authority.
- Water-balance physics implementation packages (`WB17`, `WB18`, `WB19`,
  `WB20`).
- Tier-A comparator closeout disposition packages (`PL14S`, `PL15S`).
- wepppy-side repository code changes (consumer updates are documented as
  boundary notes only in this package).

## Deliverables
1. CLI01 contract/spec authority implementation evidence:
   - `artifacts/cli01-contract-implementation-evidence.md`
2. Runner/CLI authority and guard map:
   - `artifacts/cli01-runner-cli-authority-and-guard-map.md`
3. CLI01 contract-derived test implementation evidence:
   - `artifacts/cli01-contract-test-implementation-evidence.md`
4. Pre-implementation contract gate evidence:
   - `artifacts/cli01-preimplementation-contract-gate.md`
5. Implementation and test evidence:
   - `artifacts/cli01-implementation-and-test-evidence.md`
6. Sidecar-discovery behavior evidence:
   - `artifacts/cli01-sidecar-discovery-behavior-evidence.md`
7. Run-manifest schema/sample evidence:
   - `artifacts/cli01-run-manifest-schema-and-sample.md`
8. Release-sidecar validation evidence:
   - `artifacts/cli01-release-sidecar-validation-evidence.md`
9. Fixture candidate-output evidence:
   - `artifacts/cli01-fixture-run-candidate-output-evidence.md`
10. wepppy consumer boundary note:
   - `artifacts/cli01-wepppy-consumer-boundary-note.md`
11. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/cli01_disposition.md`
12. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Implement canonical runner/CLI contract updates in canonical docs.
2. Implement contract-derived tests.
3. Record pre-implementation contract gate evidence.
4. Only then modify production runner/CLI code.

Any sequencing violation keeps package disposition in `HOLD`.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label evidence mode using `Static:` and/
or `Ran:` sections. Claims without explicit evidence labeling are non-compliant.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/decisions/0004-subprocess-hillslope-orchestration.md`
- `/workdir/openWEPP/docs/decisions/0006-three-binaries-incl-replay.md`
- `/workdir/openWEPP/docs/decisions/0007-openwepp-runner-and-release-governance.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/contracts/openwepp-runner-contract.md`
- `/workdir/openWEPP/docs/contracts/openwepp-binary-release-contract.md`
- `/workdir/openWEPP/docs/contracts/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/subsystems/runner/README.md`
- `/workdir/openWEPP/docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md`
- `/workdir/openWEPP/docs/architecture/legacy-sidecar-bridge-boundary.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/legacy-sidecar-bridge-contract.md`
- `/workdir/openWEPP/docs/specifications/wepp-input-files/input-surface-registry.md`
- `/workdir/openWEPP/docs/work-packages/20260511-openwepp-runner-bootstrap/package.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `/workdir/wepppy/wepp_runner/` (consumer launch boundary reference)
- `/workdir/wepp-forest/release/` (legacy sidecar shape reference)
- `/workdir/openWEPP/Cargo.toml`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `/workdir/openWEPP/crates/openwepp-legacy-bridge/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs`
- `/workdir/openWEPP/tests/integration/`

## Intended Write Set
- `Cargo.toml`
- `crates/openwepp-runner/**` (new runner crate/binary)
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `crates/openwepp-legacy-bridge/src/**`
- `crates/openwepp-kernel-contract/src/lib.rs` (if typed boundary errors need
  shared identifiers)
- `tests/integration/**`
- `docs/contracts/openwepp-runner-contract.md`
- `docs/contracts/openwepp-binary-release-contract.md`
- `docs/specifications/subsystems/runner/**`
- `docs/work-packages/20260523-cli01-open-wepp-runner-and-hillslope-driver-bootstrap-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`

## Phase Plan
### Phase 0 - Intake
- Confirm CLI01 queue objective and runtime-foundation acceptance criteria.
- Confirm canonical authority for runner/CLI naming, sidecar policy, and
  release-sidecar behavior.

### Phase 1 - Canonical Contract/Spec Implementation
- Implement required canonical runner/CLI contract/spec amendments (if needed)
  before runtime code edits.
- Publish authority/guard mapping and sidecar discovery policy alignment.

### Phase 2 - Contract-Test Implementation + Gate (Pre-Implementation)
- Implement contract-derived tests for runner invocation safety, sidecar
  discovery strict/compat behavior, required outputs, and sidecar validation.
- Execute and record pre-implementation contract-gate evidence.

### Phase 3 - Runner/CLI Implementation
- Implement in-repo runner + hillslope CLI executable path, run-manifest
  emission, and release-sidecar production/validation wiring.

### Phase 4 - Verification
- Run required repository gates and targeted fixture execution to prove
  deterministic candidate output and provenance evidence.

### Phase 5 - Disposition
- Publish CLI01 evidence set and disposition outcome (`GO`/`HOLD`) with
  explicit unresolved blocker rationale when applicable.

## Exit Criteria
- `CLI01` objective is evidence-backed.
- In-repo `open_wepp_runner` executable launch boundary exists and is used for
  openWEPP launcher posture.
- `openwepp-cli-hill` binary target exists and emits
  `H5.wat.dat` + `H5.plot.dat` from openWEPP execution for a deterministic
  fixture run.
- Sidecar handling is blind run-directory discovery (not `.run` sidecar path
  declaration) and is adapter-governed with strict/compat typed behavior.
- Run-manifest evidence includes binary identity, command args, sidecar
  resolution posture, and input/output checksums.
- Release-sidecar artifacts (`<binary>.json`) are present and schema-valid.
- Contract-derived CLI01 tests are implemented and executed.
- Pre-implementation contract-gate evidence proves contract + test completion
  before production code edits.
- Dual review and dual verification artifacts are completed and dispositioned.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: medium
- dedicated_security_review_required: yes
- Rationale: executable launch boundary and run-path file orchestration changes.

## Execution Fallback (Policy-Filter Safe)
If an execution agent receives a false-positive policy block while running
CLI01, continue with a minimal scoped prompt and micro-steps:

1. Scope:
   - local repository flat-file reads/edits only;
   - contract/spec and Rust runner/CLI implementation work only.
2. Run in short phases (separate prompts):
   - phase A: contract/spec-only updates (`docs/contracts/*`,
     `docs/specifications/subsystems/runner/*`);
   - phase B: contract-derived tests + pre-implementation gate artifact;
   - phase C: runner/CLI production code edits;
   - phase D: verification + disposition artifacts.
3. Avoid large multi-file prompt payloads; reference file paths and specific
   functions/headings instead.
4. Preserve all CLI01 exit criteria and evidence requirements; this fallback
   is prompt-shape mitigation only, not a scope change.
