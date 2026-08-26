# Gate results

Status: `FINAL / PASS FOR DEFECT-SHAPED HOLD`.

- Ran: post-review trajectory matrix nextest
  `11375817-3189-4900-999b-22b1df2595ab`: 8/8 PASS.
- Ran: post-review real fixture nextest
  `df2735e6-b5d8-4988-9970-e407a7b209c4`: 1/1 PASS.
- Ran: `cargo fmt --all -- --check`: PASS.
- Ran: `cargo check -p openwepp-hillslope-orchestrator --tests`: PASS with
  pre-existing warnings.
- Ran: `cargo clippy -p openwepp-hillslope-orchestrator --tests --no-deps`:
  PASS with repository-baseline warnings; warnings-denied is not baseline-clean.
- Ran: `git diff --check`: PASS.
- Static: all new model code and module registration are cfg(test).
- Static: production code, canonical contracts, manifests/dependencies, public
  APIs, and the Assurance V2 package are unchanged.
- Static: independent science, ownership, Rust, and QA reviews all return NO-GO
  for candidate selection and GO for production isolation/HOLD.

The gates qualify the truthfulness and isolation of the rejected research
checkpoint. They do not qualify a physical candidate or production path.
