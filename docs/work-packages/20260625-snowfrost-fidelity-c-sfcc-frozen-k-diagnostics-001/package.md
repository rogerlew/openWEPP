# SNOWFROST-FIDELITY-C SFCC and Frozen-K Diagnostics

Status: complete

Package type: diagnostic implementation and contract guard.

Primary gap: `GAP-SNOWFREEZE-002`.

Objective: add diagnostic-only SFCC/unfrozen-water and frozen hydraulic
conductivity comparison surfaces without changing production snow/frost
physics. Closure requires a runnable diagnostic tool, contract tests proving
bounded/monotonic behavior, source scans proving no production runtime
coupling, and artifact disposition that blocks promotion until a later
contract ratifies a selected model, parameter source, and texture defaults.

This package follows `AGENTS.md`, `docs/work-packages/AGENTS.md`,
`docs/specifications/science-contracts/AGENTS.md`,
`docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`,
`tests/AGENTS.md`, and the literature synthesis in
`docs/work-packages/20260625-snowfreeze-frost-depth-literature-annotation-001/artifacts/literature-synthesis.md`.

Subagent authorization: this package explicitly authorizes
spawning/delegating to read-only Rust-review, science-review, and verification
subagents for diagnostic-only boundary review, formula/provenance review,
source-scan review, and final evidence review. Expected outputs are compact
findings summarized into `artifacts/review-disposition.md` and
`artifacts/verification.md`; subagents may not edit files. Current execution
uses local reviews unless the operator separately requests subagent dispatch.

## Purpose

SNOWFROST-FIDELITY-A showed field residuals cannot yet be attributed to frost
physics because modeled snow depth is absent. SNOWFROST-FIDELITY-B added
no-migration heat-flow benchmark gates. C now adds a non-production diagnostic
surface for comparing candidate SFCC/unfrozen-water and frozen-conductivity
families before any runtime physics choice.

## Non-Goals

- Do not change production Rust frost/snow execution.
- Do not introduce a production runtime switch.
- Do not tune field residuals or classify observed sites as defective.
- Do not enable, port, approximate, or promote `Qwet`.
- Do not ratify texture-class defaults, salinity handling, impedance, or any
  candidate model as production authority.

## Authority Envelope

In scope:

- diagnostic formulas drawn from Kurylyk and Watanabe (2013), Watanabe and
  Flury (2008), Azmatch et al. (2012), Ming et al. (2020), Cheng et al. (2023),
  Amankwah et al. (2021), and Devoie et al. (2022);
- generic Clapeyron/SFCC liquid-water curve using diagnostic van Genuchten
  parameters supplied by the CLI fixture;
- diagnostic SFCC-Mualem frozen-K and impedance-scaled frozen-K ratios;
- offline JSON/Markdown diagnostic output for a small fixed parameter grid;
- tests for monotonic liquid-water decrease, conductivity bounds, impedance
  ordering, salinity sensitivity, provenance labels, and no production coupling.

Out of scope:

- production `crates/**` physics edits;
- observational acceptance or calibration;
- integration into WB12/WB14/WB18/WB19, WAT/HBP/PASS, direct runtime, or
  compatibility runtime;
- external network data acquisition.

## Intended Write Set

- `docs/work-packages/20260625-snowfrost-fidelity-c-sfcc-frozen-k-diagnostics-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `Cargo.toml`
- `tools/snowfreeze_observed/README.md`
- `tools/snowfreeze_observed/frozen_k_diagnostics.py`
- `tests/integration/snowfrost_fidelity_c_diagnostics_contract.rs`

## Phase Plan

### Phase 0: Scaffold and Authority Lock

- Create package scaffold and prompt.
- Record required reading and diagnostic-only scope.
- Define no-production-coupling gates.

Exit criteria:

- Package artifacts exist.
- The package explicitly blocks runtime promotion and field residual tuning.

### Phase 1: Diagnostic Tool

- Add a CLI/library under `tools/snowfreeze_observed/` that emits deterministic
  JSON and Markdown diagnostic curves.
- Include model/provenance labels for Clapeyron/SFCC, SFCC-Mualem,
  Watanabe/Flury capillary-bundle screening, Cheng-style impedance, and
  Amankwah salinity sensitivity.
- Keep all parameter sets marked `diagnostic_fixture`, not texture defaults.

Exit criteria:

- Tool runs offline with no external dependencies beyond the Python standard
  library.
- Tool output contains model labels, parameter provenance, and non-promotion
  status.

### Phase 2: Contract Tests

- Add integration tests that execute the CLI and parse JSON output.
- Assert liquid water is bounded and non-increasing with colder temperature.
- Assert frozen conductivity ratios are bounded by unfrozen `Ksat`, decrease
  with colder temperature, and impedance-scaled ratios do not exceed the
  unimpeded SFCC-Mualem ratios.
- Assert salinity diagnostic sensitivity shifts liquid water upward at a
  fixed subzero temperature without becoming a production model.
- Assert production `crates/` do not import or reference the diagnostic.

Exit criteria:

- Focused test passes.
- Source scan proves no production coupling.

### Phase 3: Closure

- Run validation gates.
- Complete review, review disposition, verification, line-count, worker
  handoff, and disposition artifacts.
- Update ROADMAP and package catalog.

Exit criteria:

- Package closes complete only as diagnostic tooling.
- Any production promotion remains separately scoped under later
  SNOWFROST-FIDELITY-D/E authority.

## Validation Commands

Run from `/home/workdir/openWEPP`.

- `.venv/bin/python tools/snowfreeze_observed/frozen_k_diagnostics.py --output-json target/snowfrost_fidelity_c/diagnostics.json --output-md target/snowfrost_fidelity_c/diagnostics.md`
- `cargo test --test snowfrost_fidelity_c_diagnostics_contract`
- `rg -n "frozen_k_diagnostics|sfcc_mualem|clapeyron_unfrozen|diagnostic_fixture" crates -S || true`
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
- `git diff --check`

## HOLD Boundaries

Close as `HOLD` if the diagnostic cannot be implemented without production
runtime coupling, if formulas cannot be labeled clearly enough to avoid
promotion-by-accident, if monotonic/bound tests fail, or if source scans find a
production `crates/` dependency on the diagnostic.
