# Focused Verification

Evidence class: **Ran + Static**

Status: `PASS`

## Commands And Results

| Command | Result |
| --- | --- |
| `.venv/bin/python -m unittest -v tests.python.test_adjudicated_crap_gate` | PASS, 17/17 |
| `.venv/bin/python -m py_compile tools/release/check_adjudicated_crap.py tests/python/test_adjudicated_crap_gate.py` | PASS |
| `bash -n tools/release/run_adjudicated_crap_gate.sh tools/release/run_release_candidate_gates.sh` | PASS |
| `jq empty tools/release/adjudicated_crap_exceptions.json` | PASS |
| `ruby -e 'require "yaml"; YAML.load_file(".github/workflows/release-gates.yml")'` | PASS |
| scoped `markdown-doc lint` over ten changed documentation paths | PASS, 0 errors and 0 warnings |
| `git diff --check` | PASS |

The focused driver reproduction used the completed campaign artifact:

    bash tools/release/run_adjudicated_crap_gate.sh \
      --crap-json docs/work-packages/cqr-pre-integration-campaign-evidence/low/final/final-crap.json \
      --retained-provenance docs/work-packages/cqr-pre-integration-campaign-evidence/low/campaign-final-assessment.md \
      --output-dir /tmp/openwepp-acrap-focused

Observed summary:

    status=ASSESSMENT-PASS raw=2 adjudicated=2 actionable=0 touched_files=0

The machine report records `closure_eligible=false`; immutable historical
reproduction cannot be mistaken for current-source closure.

## Focused Failure Coverage

The 17 tests directly cover exact adjudication, workspace regression outside
the touched set, current and historical source hashes, evidence content hashes,
the CQR production filter and exact deduplication tuple, canonical-registry
enforcement, retained assessment-only status, production-crate census mismatch,
production-source and Rust-toolchain manifest mutation,
tracked/untracked/deleted/renamed worktree discovery, stale-PASS replacement by
failure envelopes both before acquisition and during evaluation,
malformed/nonproduction input, and exact reproduction of the final CQR census.
