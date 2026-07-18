# Release Gate Automation

This directory hosts repository-local automation for openWEPP release gates.

Required authority lane runs by default.

The adjudicated CRAP gate requires repo-local `.venv/bin/python`,
`cargo-llvm-cov 0.8.7`, and `cargo-crap 0.2.2`. The metric-helper versions and
report schema are pinned; the active `cargo` and `rustc` versions are recorded
in the sealed acquisition provenance.

## Scripts

- `run_adjudicated_crap_gate.sh`
  - Collects fresh full-workspace LCOV and `cargo-crap` JSON by default, or
    assesses an explicitly supplied retained CRAP artifact.
  - `--scope affected --package <name>... --nextest-profile affected
    --base-ref <ref>` is the shadow increment path. Repeated packages are the
    terminal plan's exact affected/reverse-dependent closure. One instrumented
    Nextest run emits JUnit and LCOV, then CRAP is evaluated for that package
    set. The mode is fresh-only and cannot substitute for global critical,
    campaign, or release closure.
  - Applies the exact production filter and deduplication tuple established by
    the completed CQR pre-integration campaign.
  - Fresh closure snapshots every production Rust source, Rust/Cargo/gate
    measurement input (including `rust-toolchain.toml`), HEAD, and the Git index
    before, after, and immediately after report generation; any drift fails the
    run. The canonical adjudication registry cannot be overridden in this mode.
  - Preserves raw rows above 30, matches only exact current adjudications from
    `adjudicated_crap_exceptions.json`, and fails unless the actionable
    workspace set is empty.
  - With `--base-ref`, reports every touched production Rust file while still
    blocking actionable regressions in source-untouched files. Status records
    distinguish additions, modifications, deletions, and both rename paths.
  - `--crap-json` is retained assessment mode. It requires a repository-local
    provenance artifact, cannot report current touched files, and emits
    `ASSESSMENT-PASS` rather than a closure `PASS`.
  - Every non-help invocation clears known generated outputs before semantic or
    prerequisite checks. Its exit trap writes a fresh PASS/FAIL run envelope
    and checksum manifest, so a reused directory cannot retain an apparent
    prior success after a failed run.
- `check_adjudicated_crap.py`
  - Fail-closed evaluator used by the shell driver. It validates registry
    schema, current and historical whole-file hashes, exact symbols,
    classifications, evidence hashes/content bindings, production-crate census,
    source manifest, and path containment before generating JSON and Markdown
    reports.
- `run_release_candidate_gates.sh`
  - Requires `--mode validate` or `--mode release`; ambiguous invocation fails.
  - Validation mode runs workspace gates (`fmt`, `clippy`, full-profile `nextest`, `deny`, the
    adjudicated-CRAP unit tests, and a fresh adjudicated CRAP measurement),
    but exits before assurance snapshotting, binary staging, sidecar emission,
    and release lint. Ordinary CI uses only this mode and names its upload
    validation evidence.
  - Release mode first passes the fail-closed assurance transition preflight,
    then runs the same closure gates, creates an immutable zero-report assurance
    snapshot, builds/stages release binaries, emits sidecars, and runs
    `open_wepp_runner release lint`. Full-profile nextest supplies the required
    process-per-test isolation for environment-mutating integration tests.
  - An approved v2 assurance realization is supplied only as the complete set
    `--v2-assurance-snapshot`, `--v2-assurance-receipt`,
    `--v2-assurance-release-commit`, and
    `--v2-assurance-release-configuration`. The production verifier
    reconstructs the content-addressed snapshot, receipt, public tree, all
    source-derived authorization roots, and independently supplied release
    identity before any release directory is created. The supplied commit must
    equal checkout `HEAD`; configuration must equal the driver's actual build
    configuration. Validation mode, partial sets, and `test_only` artifacts
    fail. Release mode then calls `materialize_assurance_v2_release.sh` to copy
    and reverify the artifacts under
    `RELEASE_DIR/assurance-v2/`, emits an
    `assurance-v2-publication.json` discovery sidecar, and records checksums.
  - The transition preflight admits only the exact typed zero-report catalog
    bytes. A retired v1 path must be absent or a real, non-symlink, completely
    empty directory; duplicate catalog keys, files, symlinks, sockets, FIFOs,
    nested directories, and other special entries fail before candidate-
    directory creation.
  - Evaluates external-authority suite lanes from
    `docs/specifications/external-authority/registry.yaml`:
    - verifies fixture integrity for all active suites before lane execution:
      - `fixtures.sha256` (`sha256sum --check --strict`)
      - `fixtures.provenance.yaml` (required per-fixture provenance keys)
    - includes required/hard-fail suite
      `cas_l4_infile_soil_producer_contract_001` to block `.sol`
      producer-contract symbol/order/arity or fixture-integrity drift.
    - `required` lane runs by default (blocking on `hard-fail`).
    - `periodic` lane runs when `--run-authority-periodic` is set.
    - `manual` lane runs when `--run-authority-manual` is set.
    - `investigation` failures are recorded and surfaced as non-blocking.
  - Runs the stability cohort unless `--skip-stability` is passed. That flag is
    valid for bounded transition-route verification or when a separately bound
    stability job has already passed; an invocation using it is not, by itself,
    a conformant release candidate.
- `run_hillstab_gate.sh`
  - Executes the HILLSTAB01 cohort harness and applies pass/fail assertions.
- `assert_hillstab_success.py`
  - Validates HILLSTAB01 JSON suite summaries and exits non-zero on failures.
- `check_authority_suite_antievasion.sh`
  - Runs diff-based anti-evasion checks for suites declared in
    `docs/specifications/external-authority/required-suite-obligations.json`.
  - Guards against:
    - required-case anchor removal,
    - cohort cardinality shrinkage,
    - threshold loosening,
    - lane/failure posture changes without control-path updates,
    - non-blocking Level-4 suites without queued/in-progress closure package
      linkage in `docs/work-packages/README.md`.
- `check_sc_unit_compliance.sh`
  - Lints canonical `SC-*` contracts for unit-governance readiness:
    `Variables and Units` coverage, alias-map `Units check` coverage, and
    cross-checks against the executable boundary-symbol unit registry where
    symbols are registered.
  - Current full-contract runs are expected to report HOLD inventory until
    legacy `SC-INFILE-*` and registry-linkage gaps are remediated.
- `check_hillslope_schedule_export.sh`
  - Regenerates the code-derived hillslope phase schedule artifacts from
    `HillslopePhaseGraph::canonical()` into a temporary directory and compares
    them against the committed Mermaid, JSON, and DOT artifacts under
    `docs/architecture/generated/`.
  - Fails on documentation drift without writing repository files.
- `check_assurance_dossier_exports.sh`
  - Validates/checks the typed zero-report catalog, requires exactly one tracked
    public assurance page, requires an empty dormant export with vendoring
    prohibited, and runs the release transition preflight.
- `check_assurance_release_transition.sh`
  - Separates non-assembly validation from release authorization.
  - Release mode fails on the transition marker, nonempty/ambiguous legacy
    catalog, or any retired v1 source/public route.
  - With a complete optional v2 snapshot/receipt/commit/configuration set,
    binds commit/configuration to the selected checkout/build and invokes the
    reconstructing `openwepp-assurance verify-release`; otherwise preserves the
    exact established zero-report release path.
- `materialize_assurance_v2_release.sh`
  - Performs the bounded release-path copy after preflight, verifies source and
    copied production authority, emits the discovery sidecar, and writes and
    checks content-addressed manifest/receipt checksums.
  - Is called by the release runner and directly exercised by the ASSURE-04D
    integration contract; it is not a substitute for the transition preflight
    or the remaining release gates.

## Typical Usage

For implementation-package closure from its frozen base:

```bash
bash tools/release/run_adjudicated_crap_gate.sh \
  --base-ref <frozen-base> \
  --output-dir <package-artifacts>/adjudicated-crap
```

For planner-selected bounded-package feedback during shadow observation:

```bash
bash tools/release/run_adjudicated_crap_gate.sh \
  --scope affected \
  --package openwepp-gate-planner \
  --nextest-profile affected \
  --base-ref <frozen-base> \
  --output-dir target/affected-crap
```

The terminal plan, not a human shortcut, must select every repeated package and
the exact covering-test inventory. Under executor control, the output and Cargo
target are relocated beneath the external artifact root. Critical or unknown
coverage contribution continues to use the global command above.

To reproduce the completed CQR adjudication against its retained immutable
CRAP JSON without claiming current-source closure:

```bash
bash tools/release/run_adjudicated_crap_gate.sh \
  --crap-json docs/work-packages/cqr-pre-integration-campaign-evidence/low/final/final-crap.json \
  --retained-provenance docs/work-packages/cqr-pre-integration-campaign-evidence/low/campaign-final-assessment.md \
  --output-dir /tmp/openwepp-adjudicated-crap-reproduction
```

That command returns success when the retained debt assessment passes, but its
machine status is `ASSESSMENT-PASS` and `closure_eligible` is `false`.

Ordinary validation (never assembles a candidate):

```bash
bash tools/release/run_release_candidate_gates.sh \
  --mode validate \
  --skip-stability
```

```bash
bash tools/release/check_authority_suite_antievasion.sh --base-ref HEAD~1 --head-ref HEAD
```

```bash
bash tools/release/check_sc_unit_compliance.sh
```

```bash
bash tools/release/check_hillslope_schedule_export.sh
```

```bash
bash tools/release/check_assurance_dossier_exports.sh
```

```bash
bash tools/release/run_release_candidate_gates.sh \
  --mode release \
  --run-authority-periodic \
  --cohort-seeds-csv /workdir/wepp-forest/docs/work-packages/20260503-wb05b-forest-hillslope-closure-sweep/artifacts/audits/_meta/defect_seeds.csv \
  --watchlist-csv /workdir/wepp-forest/docs/ablation/hillslope_watchlist.csv \
  --expect-suite wb05b_1166=1166 \
  --expect-suite release_gate_watchlist=19
```

## Authority Lane Flags

- `--skip-authority-required`
  - Skip the default required authority lane.
- `--run-authority-periodic`
  - Execute `gate_lane=periodic` suites from the authority registry.
- `--run-authority-manual`
  - Execute `gate_lane=manual` suites from the authority registry.
- `--authority-registry <path>`
  - Override registry location (default:
    `docs/specifications/external-authority/registry.yaml`).
- `--authority-report <path>`
  - Override report destination (default:
    `<release_dir>/authority_suite_results.md`).
