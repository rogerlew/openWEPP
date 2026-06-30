# Gates

Evidence class: Ran

## Commands

| Gate | Result | Evidence |
|---|---|---|
| Format | PASS | `cargo fmt --check` |
| Clippy | PASS | `cargo clippy --workspace --all-targets -- -D warnings` |
| Runner focused lib suite | PASS | `cargo test -p openwepp-runner --lib`: `150 passed` |
| Full Rust suite | PASS | `cargo nextest run --workspace --profile full`: `1856 passed`, `1 skipped`, one slow diagnostic |
| Deny | PASS | `cargo deny check`: advisories, bans, licenses, sources all OK |
| Authority anti-evasion | PASS | `bash tools/release/check_authority_suite_antievasion.sh`: PASS |
| Required-suite obligation guard | PASS | `cargo test --test auth11_required_suite_obligation_guards_contract`: `2 passed` |
| Forbidden-source scan | PASS | `rg` scan in `static-callgraph-audit.md`: no matches outside the source-guard test |
| Markdown lint | PASS | `markdown-doc lint --path ... --no-ignore` over the package, work-package README, ADR-0030, ADR README, array-native spec, ROADMAP, and work-package AGENTS |

## Nextest Environment Note

The first full `nextest` attempt failed because the worktree lacked the
repo-local `.venv/bin/python` expected by existing integration tests. An
untracked worktree symlink to the repo `.venv` was added locally; the failed
tests then passed in a focused rerun, and the second full `nextest` run passed.

## Output Identity / No-Regression

No separate base-vs-worktree file byte-diff harness was run outside the test
suite. The no-regression evidence is:

- production selector and manifest tests passed under full `nextest`, proving
  no-env default and legacy sidecar-discovery runs select
  `direct-production-executor` with no fallback;
- multi-OFE/Wave-2 and public per-OFE WAT tests passed under full `nextest`;
- the deleted selectors/adapters are statically unreachable from runner
  production/API/CLI sources;
- direct runtime source guards passed under full `nextest`.

Because the deleted modes were transition-only and are now statically absent
from production selection, this is sufficient for the package's scoped deletion.
If the next package deletes the explicit replay seam or setup-time carriers, it
must run a true base-vs-after output-byte comparison across H2637, multi-OFE,
and Wave-2 fixtures.

## Counter Proof

Counter proof is covered by full `nextest` execution of:

- `r7e_default_candidate_uses_direct_production_manifest`;
- `r7e_default_candidate_activation_selects_direct_runtime_manifest`;
- `r7e_default_candidate_legacy_sidecar_discovery_uses_direct_manifest`;
- `r7c_direct_production_executor_reports_no_day_input_compatibility_edges`;
- `r7f_typed_day_input_hot_loop_excludes_runtime_surface_reads`;
- `cli03_mofe03_multiofe_runfile_executes_wave2_without_manual_symbol_injection`;
- `cli03_mf_multiofe_publication_emits_public_per_ofe_wat_rows`.

These tests assert `compatibility_edge_invocations = 0` where the manifest or
direct audit counter is the relevant evidence surface.

## Static Guards

Added
`compatibility_runtime_deletion_removes_obsolete_transition_modes`, which scans
runner production/API/CLI files for the deleted selectors, flags, and adapter
helpers. It passed in the focused runner lib suite and the full `nextest` suite.
