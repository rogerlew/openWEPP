# Gate Results

Status: complete

Evidence mode: ran

Static:

- Package status is `hold` because HPHYS0298 localized the first divergence
  but did not apply a production physics correction.
- No openWEPP production kernel/runtime physics file was changed.

Ran:

| Gate | Result | Notes |
| --- | --- | --- |
| `.venv/bin/python -m py_compile docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/hphys0298_paired_lineage_partition.py` | pass | Syntax check for package-local paired lineage runner. |
| `cargo fmt --check` | pass | Rust formatting accepted. |
| `cargo clippy --workspace --all-targets -- -D warnings` | pass | Workspace lint accepted with warnings denied. |
| `cargo test --workspace` | pass | Workspace tests passed. |
| `cargo deny check` | pass | `advisories ok, bans ok, licenses ok, sources ok`; existing warnings for duplicate crates `getrandom`, `hashbrown`, `twox-hash`, and unmatched allowances `ISC`, `Unicode-DFS-2016`. |
| `bash tools/release/check_authority_suite_antievasion.sh` | pass | Authority-suite anti-evasion guard passed. |
| `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture` | pass | Required-suite obligation guard tests passed. |
| `cargo test --test hphys0298_paired_lineage_partition_contract -- --nocapture` | pass | HPHYS0298 contract-derived guard tests passed after review fixes. |
| `.venv/bin/python docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/hphys0298_paired_lineage_partition.py --run-root /tmp/hphys0298_full_20260605T000000Z` | pass | Full H1..H39 suite, target traces, baseline observe identity, and partition ledger generated. |
| `.venv/bin/python docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/hphys0298_paired_lineage_partition.py --run-root /tmp/hphys0298_full_20260605T000000Z --skip-full-suite --skip-targeted-traces` | pass | Regenerated corrected ledger and observe identity after review fixes without rerunning unchanged heavy traces. |
| `wctl doc-lint --path docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001` | pass | Tool reported `0 files validated, 0 errors, 0 warnings` for the scoped package path. |
| `.venv/bin/python -m py_compile docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/hphys0298_paired_lineage_partition.py && cargo fmt --check && cargo test --test hphys0298_paired_lineage_partition_contract -- --nocapture && wctl doc-lint --path docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001` | pass | Post-Claude-review validation passed after porting-fidelity authority, summary-generator, and reference-binary capability amendments. |

## Diagnostic Build Gate

Ran:

```text
git -C /workdir/wepp-forest_260430_baseline worktree add --detach /tmp/hphys0298_wepp_forest_obs dac3c950d8b16cc73774bf5ce2e7e11f80baac70
make clean && make COMPILER=gfortran wepp_hill
```

Result:

- Instrumented detached baseline build passed.
- Pinned baseline worktree remained clean.
- Detached diagnostic worktree remains dirty by design and is recoverable with
  `git -C /workdir/wepp-forest_260430_baseline worktree remove --force /tmp/hphys0298_wepp_forest_obs`.
