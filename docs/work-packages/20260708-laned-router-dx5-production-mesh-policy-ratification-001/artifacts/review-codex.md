# Codex Review

Static: read root/work-package/science-contract/crates governance, `SC-OFEROUTE-001`, package docs, promotion/runtime/consumer/implementation/verification artifacts, and the relevant active runner/orchestrator code paths. Ran: focused mesh-policy tests, JSON consistency checks, `git diff --check`, package trailing-whitespace scan, and `cargo fmt --check`.

## Findings

### HIGH - Canonical contract metadata still advertises rev 43

Path: `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md:7`

The contract body now contains rev 45 active `dx5` production mesh authority (`SC-OFEROUTE-001.md:127`, `:232`, `:250`, `:353`, `:521`, `:586`), but the YAML front matter still says `contract_version: 43`. That breaks the contract-first authority surface: tooling or reviewers that consume canonical metadata still see rev 43 even though the package, registry text, and implementation claim rev 45 authorization.

Required disposition: update the front matter to `contract_version: 45` and rerun/record the contract/profile/BEI checks before any completion claim.

### HIGH - Completion is claimed while required closure gates remain open

Paths:
- `docs/work-packages/README.md:15`
- `docs/ROADMAP.md:277`
- `docs/work-packages/20260708-laned-router-dx5-production-mesh-policy-ratification-001/package.md:3`
- `docs/work-packages/20260708-laned-router-dx5-production-mesh-policy-ratification-001/package.md:135`
- `docs/work-packages/20260708-laned-router-dx5-production-mesh-policy-ratification-001/package.md:153`

The work-package README and roadmap already claim `EXECUTED-COMPLETE-DX5-PRODUCTION-MESH-POLICY`, but `package.md` still says `EXECUTING`, required artifacts are still missing (`gate-results.md`, `disposition.md`, `final-disposition.md`, `worker-handoff.md`), and `verification-codex.md` itself is `QA-HOLD`. I also ran `cargo fmt --check`; it fails in `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs:1392` on the new mesh-policy test formatting. This violates the package gate non-deferral and truthfulness rules: the package cannot be recorded as complete while required gates are absent or failing.

Required disposition: either move catalog/roadmap claims back to active/hold status, or complete and record every required gate/artifact truthfully, including fixing the formatting failure.

### LOW - Required-reading/provenance artifact still names rev 44

Path: `docs/work-packages/20260708-laned-router-dx5-production-mesh-policy-ratification-001/artifacts/required-reading-map.md:15`

The required-reading map describes `SC-OFEROUTE-001` as "Rev-44 active mesh policy" even though this package is rev 45 production-promotion work. This is not the core authority blocker because the contract body has rev 45 text, but it is package evidence drift.

### LOW - Package-local trailing whitespace is present in an untracked artifact

Path: `docs/work-packages/20260708-laned-router-dx5-production-mesh-policy-ratification-001/artifacts/verification-comparator.md:41`

`rg -n '[ \t]+$' docs/work-packages/20260708-laned-router-dx5-production-mesh-policy-ratification-001` reports a trailing-space line. `git diff --check` is clean, but these files are untracked, so package-local scans are still needed until staging.

## Verified Evidence

- Contract body authority is otherwise coherent for rev 45: production active default is `target_dx_m = 5.0`, `min_cells = 10`, `max_cells = 4096`, `LANED_ACTIVE_SAMPLE_DT_S = 900`, and `LANED_ACTIVE_MAX_DT_S = 300`; shadow mesh remains separate.
- Runtime implementation matches that authority: `DirectLanedActiveMeshPolicy::production_default()` returns `TargetDx { target_dx_m: 5.0, min_cells: 10, max_cells: 4096 }`, and the runner returns that default when `OPENWEPP_LANED_ACTIVE_MESH_TARGET_DX_M` is absent.
- Fail-closed selector semantics held in focused tests:
  - `cargo test -p openwepp-hillslope-orchestrator mesh_policy_resolves_production_dx5_target_floor_and_cap`
  - `cargo test -p openwepp-runner mesh_policy_parser_defaults_parses_and_rejects_invalid_target_dx`
  - `cargo test -p openwepp-runner diagnostic_max_dt_selector_requires_active_trace`
- Runtime evidence JSON is internally consistent: `status=PASS`, `runs=12`, zero failed run/identity/mesh/closure assertions, active no-env runs all serialize `target_dx_m=5.0`, and off runs have no `laned_active` provenance block.
- Promotion matrix JSON is internally consistent: `row_count=21`, `blockers=0`, `missing_annual_count=0`, `gate_nonpass=0`, and three fixed-300 rows are report-only under rev 43/44 evidence interpretation.

## Residual Risk And Missing Tests

I did not rerun the full selected-cohort runtime evidence, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo nextest run --workspace --profile full`, `cargo deny check`, markdown/doc lint, or contract/profile/BEI checks. Those must be recorded in `gate-results.md` before closure.

`DirectLanedActiveMeshPolicy::FixedCells` remains a public enum variant, but I found no current runner selector path that constructs it. If direct orchestrator API callers are treated as production mesh selectors, that retained variant needs explicit non-production justification or retirement in a follow-on.

## Disposition

Not approved for package closure until the high-severity findings are fixed. I found no numerical or runtime-path blocker in the active `dx5` default implementation itself.
