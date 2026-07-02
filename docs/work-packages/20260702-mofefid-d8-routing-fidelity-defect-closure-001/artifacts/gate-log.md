# D8 Gate Log

Evidence class: `Ran`.

## Focused Diagnostics

| Command | Result |
|---|---|
| `sha256sum .../Figure_4.xlsx .../3.1_Validation_Input.docx` | PASS; hashes matched D01 manifest. |
| `cargo nextest run -p openwepp-hillslope-orchestrator dval` | PASS before edits; 2/2. |
| `cargo nextest run -p openwepp-hillslope-orchestrator ofe_routing` | PASS after edits; 43/43. |
| `.venv/bin/python tools/dval/compare_dval.py --case 1 --fig4 ... --crate-dir .` | PASS; emitted derived scalar metrics only. |
| `.venv/bin/python tools/dval/compare_dval.py --case 2 --fig4 ... --crate-dir .` | PASS; emitted derived scalar metrics only. |
| `.venv/bin/python tools/dval/compare_dval.py --case 3 --fig4 ... --crate-dir .` | PASS; emitted derived scalar metrics only. |
| `.venv/bin/python tools/dval/compare_dval.py --case 4 --ko 200 --fig4 ... --crate-dir .` | PASS; emitted derived scalar metrics only. |

## Required Gates

| Command | Result |
|---|---|
| `cargo fmt --check` | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS |
| `cargo nextest run -p openwepp-hillslope-orchestrator` | PASS; 191/191. |
| `cargo nextest run --workspace --profile full` | PASS; 1249/1249 passed, 1 skipped, 1 slow (`snowdensity05e_melt_adjudication coe_melt_snowbench_runs_both_models_as_diagnostic_only`). |
| `cargo deny check` | PASS; advisories, bans, licenses, sources ok. |
| `python tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` | PASS-DEFERRED; 4 BEI rows, 2 science-review-follow-on rows. |
| `bash tools/release/check_authority_suite_antievasion.sh` | PASS |
| `cargo nextest run --test auth11_required_suite_obligation_guards_contract` | PASS; 2/2. |
| `git diff --check` | PASS |
| `wctl doc-lint --path docs/work-packages/20260702-mofefid-d8-routing-fidelity-defect-closure-001` | PASS, but tool reported `0 files validated`; not a substantive Markdown validation. |

## Shadow-First Verification

Ran:

```console
rg -n "ofe_routing" crates/openwepp-runner \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime \
  crates/openwepp-hillslope-orchestrator/src/runtime_inputs \
  crates/openwepp-hillslope-orchestrator/src/hydrology
```

Result: no matches (`rg` exit 1). No production runner/direct-runtime/hydrology
path calls the shadow `ofe_routing` subsystem.
