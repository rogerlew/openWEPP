# gate-results

Status: complete  
Evidence mode: Ran

Executed:
- `bash -n tools/release/run_release_candidate_gates.sh`
- `bash -n tools/release/run_hillstab_gate.sh`
- `python3 -m py_compile tools/release/assert_hillstab_success.py`
- `bash tools/release/run_release_candidate_gates.sh --release-tag 260529ci --release-dir /tmp/openwepp_release_260529ci_relproc03 --skip-stability`
- `bash tools/release/run_hillstab_gate.sh --openwepp-binary /home/workdir/openWEPP/target/release/openwepp-cli-hill --cohort-seeds-csv /workdir/wepp-forest/docs/work-packages/20260503-wb05b-forest-hillslope-closure-sweep/artifacts/audits/_meta/defect_seeds.csv --watchlist-csv /workdir/wepp-forest/docs/ablation/hillslope_watchlist.csv --output-json /tmp/openwepp_relproc03_hillstab_sample.json --jobs 1 --limit-1166 1 --limit-watchlist 1 --expect-suite wb05b_1166=1 --expect-suite release_gate_watchlist=1`
- `set +e; python3 tools/release/assert_hillstab_success.py --results-json /tmp/openwepp_relproc03_hillstab_sample.json --expect-suite wb05b_1166=2; echo $?`
- `markdown-doc lint --path docs/governance/openwepp-release-procedure-draft.md --format plain`
- `markdown-doc lint --path docs/work-packages/README.md --format plain`
- `markdown-doc lint --path docs/work-packages/20260529-relproc03-release-gate-ci-automation-001 --format plain`
- `markdown-doc lint --path tools/release/README.md --format plain`

Result:
- all listed gates passed.
- negative assertion probe produced expected non-zero exit for mismatched suite
  expectation.
