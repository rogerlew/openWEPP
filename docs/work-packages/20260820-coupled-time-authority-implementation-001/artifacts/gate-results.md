# Gate Results

Status: focused closure PASS / unrelated workspace failures recorded

Evidence mode: Ran

Historical failures retained: initial authority test 3/4 (missing error alias),
unit-lint header failure, two authority verification FAIL rounds, initial
semantic-validator invocation without required argument, and initial fmt-check
diff. Each was corrected and rerun.

Terminal focused results:

- independent reference: 114/114 PASS.
- semantic schema: 76/76 expected outcomes PASS.
- contract Nextest: 5/5 PASS.
- strict binding exposure and SC unit compliance: PASS.
- `cargo fmt --all -- --check`: PASS after formatting correction.
- `cargo check -p openwepp-coupled-time -p openwepp-hillslope-orchestrator`: PASS.
- `cargo clippy -p openwepp-coupled-time --all-targets -- -D warnings`: PASS.
- `cargo clippy -p openwepp-hillslope-orchestrator --lib -- -D warnings`: PASS.
- coupled-time crate Nextest: 13/13 PASS.
- orchestrator `coupled_time_reference` filter: 3/3 PASS.
- `git diff --check`: PASS.

The required heavy runner and cargo-deny ran. Workspace quick stopped on nine
pre-existing snow assurance identity failures involving `SC-SNOWENERGY` and
`SC-SNOWFREEZE`. Broad Clippy retains unrelated snow/WB14 findings after the
coupled-time lint was corrected. These failures are recorded but are not
coupled-time acceptance evidence.
