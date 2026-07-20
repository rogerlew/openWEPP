# CRAP Control-Envelope Correction

Static: the detailed `adjudicated-crap-report.json` remains unchanged and may
contain finite JSON floating-point CRAP, coverage, cyclomatic, and threshold
metrics. The runner's existing `run-status.json` now adds
`adjudicated_crap_report_sha256`; the envelope otherwise retains integer exit
status, PASS/FAIL result, acquisition mode, and timestamps.

Static: executor success validation reads `run-status.json` through the same
confined, regular-file, symlink-rejecting reader used for the report, strictly
parses only those control bytes, requires `result == PASS` and
`exit_status == 0`, reads the confined detailed report as raw bytes, and
requires its SHA-256 to equal the control envelope. Publication repeats that
paired read and passes the returned validated report buffer directly to atomic
write, closing the validation/publication race. Both files are reset before
each adapter attempt. Detailed report bytes remain the receipt artifact.

Ran:

- `bash -n tools/release/run_adjudicated_crap_gate.sh`: PASS.
- `cargo fmt --check`: PASS.
- `cargo clippy -p openwepp-gate-planner --all-targets -- -D warnings`: PASS.
- Planner focused regression: 1/1 PASS. It accepts a numeric detailed report,
  rejects report-byte tampering, rejects floating control JSON, rejects a
  non-PASS/nonzero control, and proves report/control cleanup.
- `python -m unittest tests.python.test_adjudicated_crap_gate`: 18/18 PASS in
  6.873 seconds. The success regression executes retained-mode CRAP assessment,
  preserves a floating threshold, and reconstructs the detailed report SHA.
- TESTGATE executor/authority contract pair: 13/13 PASS, 0 skipped, in 0.250
  seconds after 7.82 seconds compilation.
- Direct runner SHA and both policy bindings:
  `f7108868659c16fe1648dbb27cea573578db4e3bb86d75a6b29b63bb78b0f857`.
- `git diff --check`: PASS.

No GitHub workflow, forest1 runner, production host, kernel, science contract,
CRAP threshold, adjudication, or command-selection semantic changed.
