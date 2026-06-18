# BASECOND01 - Vertical `ssc` Harmonic Normalization Defect Closure

Status: complete 2026-06-18

Package type: Defect-Closure ExecPlan.

Defect ID: `BASECOND01-SSC-HARMONIC-NORMALIZATION`

## Objective

Close the source-intent defect found by
`STAGE2-BASE-CONDUCTIVITY-H2637-MAGNITUDE`: vertical
`wb18_perc_ssc_####` 200 mm runtime-layer normalization is currently arithmetic
where pinned baseline source intent is inverse-conductivity/harmonic. Preserve
modern hourly `wb19_lateral_ssh_####` as arithmetic `ssc2*ui_anisrt`.

## Rationale

H2637 base `ksat` is byte-live. The adjudication package proved WAT/PASS
checksums and magnitude outputs change when `.sol` layer `ksat` is perturbed.
It also found a non-aliased source-intent mismatch:

- current split-layer vertical `wb18_perc_ssc_0003` on H2637:
  `270.8259 mm/h`;
- source-intent vertical `ssc`:
  `117.955408163210 mm/h`;
- source-intent hourly `ui_ssh`:
  `270.8259 mm/h`.

This package lands the correction rather than relaying another diagnostic step.

## Correction Authority Envelope

Observed violation:

- `BASECOND01-SSC-HARMONIC-NORMALIZATION`: vertical percolation conductivity
  projection (`wb18_perc_ssc_####`) is arithmetic over split source layers, but
  `/workdir/wepp-forest_260430_baseline/src/input.for:760,843,926` accumulates
  `ksinv += thickness/ssc2` and finalizes `ssc1 = slayth/ksinv`.

In-scope write set:

- `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests/soil.rs`
- `tests/integration/parser_runtime_seam_integration/common.rs`
- this package directory under
  `docs/work-packages/20260618-basecond01-ssc-harmonic-normalization-defect-closure-001/`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`

Allowed production edit:

- Change only the soil parser-to-runtime conductivity projection so
  `ssc_m_s` is harmonic/inverse-conductivity normalized while
  `lateral_ssh_m_s` remains arithmetic from `ksat*anisotropy`.

Protected boundaries:

- Do not change the WB19 lateral equation, lateral withdrawal, active-layer
  selection, `drfc`, or `ksatadj`.
- Do not make hourly `wb19_lateral_ssh_####` harmonic.
- Do not loosen typed guards or silently default missing/non-positive
  conductivity.
- Do not chase legacy comparator parity as a target.

Authority:

- `SC-INFILE-SOIL-001` owns parser-to-runtime soil projection.
- `SC-PERC-001` maps vertical `Ksi` to `wb18_perc_ssc_####`.
- `SC-SUBHYD-001` HPHYS0257 owns modern hourly `ui_ssh` /
  `wb19_lateral_ssh_####` as separate horizontal conductivity.
- Pinned baseline source at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` provides the source-intent
  formulas.

Conversion rule:

If the package establishes the defect remains reproducible inside this envelope
and the expected behavior is supported by the canonical contract amendment plus
pinned baseline provenance, it must proceed through contract amendment,
contract-derived tests, production correction, validation, review, verification,
and disposition in this package. It may close `HOLD` only at a named boundary
outside the envelope.

Seven-gate bar:

1. Reproduction: the split-layer fixture exposes arithmetic vertical `ssc`.
2. Mechanism: vertical `ssc` and horizontal `ui_ssh` share one arithmetic
   projection path.
3. Ownership: runtime soil projection is inside this package.
4. Authority: baseline `input.for` and `SC-*` contracts support the split.
5. Safety: the fix changes a projection formula only, without clamping/defaults.
6. Testability: a split-layer test can prove `ssc != ssh`.
7. Validation: focused tests, full Rust gates, and H2637 rerun evidence are
   measurable in this package.

## Phase Plan

1. Scaffold package and record required reading / authority map.
2. Amend `SC-INFILE-SOIL-001` to make vertical `ssc` harmonic and hourly
   `ui_ssh` arithmetic explicit at the parser-to-runtime seam.
3. Add failing contract-derived tests for split-layer vertical/horizontal
   divergence.
4. Implement the projection fix.
5. Run focused tests, formatting, workspace gates, and H2637 rerun evidence.
6. Complete review, verification, line-count governance, disposition, and
   roadmap/work-package index updates.

## Exit Criteria

- Contract authority updated or confirmed with explicit citation.
- Non-aliased test proves split-layer `wb18_perc_ssc` differs from
  `wb19_lateral_ssh` and matches the harmonic expected value.
- Production projection publishes harmonic vertical `ssc` and arithmetic
  horizontal `ssh`.
- H2637 no-UI rerun records WAT/PASS checksum and aggregate deltas against the
  pre-BASECOND01 baseline.
- Required closure loop run and recorded:
  `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, `cargo deny check`.
- Markdown lint and `git diff --check` clean.
- Dual reviews and dual verifications completed with finding disposition.
- `.rs` line-count governance recorded.

## Deliverables

- `artifacts/required-reading-map.md`
- `artifacts/contract-implementation-evidence.md`
- `artifacts/contract-test-implementation-evidence.md`
- `artifacts/pre-implementation-contract-gate.md`
- `artifacts/implementation-test-evidence.md`
- `artifacts/h2637-rerun-evidence.md`
- `artifacts/kernel-profile-compliance-checklist.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-results.md`
- `artifacts/line-count-governance.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
- `artifacts/basecond01_disposition.md`
- `artifacts/worker-handoff.md`

## Subagent Authorization

This package explicitly authorizes spawning/delegating to read-only review and
verification subagents for bounded review of this package's diff, evidence, and
gate legitimacy. Expected outputs are compact findings suitable for
`review_agent_a.md`, `review_agent_b.md`, `verification_agent_a.md`, and
`verification_agent_b.md`. Write access is not authorized for subagents.

## Required Reading Budget

Local required-reading bytes total: `366583`.

Disposition: `OK` (`<=400000` bytes).

See `artifacts/required-reading-map.md`.
