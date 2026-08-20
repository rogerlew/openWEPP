# Gate Results

Status: implementation candidate gates PASS / broad closure pending

Evidence mode: Ran

Historical failures retained: initial authority test 3/4 (missing error alias),
unit-lint header failure, two authority verification FAIL rounds, initial
semantic-validator invocation without required argument, and initial fmt-check
diff. Each was corrected and rerun.

Current focused results:

- independent reference: 108/108, SHA
  `4540951f70f9de0846669f8f955e7eeca425dd831108997f50009d6ec002df95`.
- semantic schema: 31/31 expected outcomes.
- contract Nextest: 5/5 PASS.
- strict binding exposure and SC unit compliance: PASS.
- `cargo fmt --all -- --check`: PASS after formatting correction.
- `cargo check -p openwepp-coupled-time -p openwepp-hillslope-orchestrator`: PASS.
- `cargo clippy -p openwepp-coupled-time --all-targets -- -D warnings`: PASS.
- `cargo clippy -p openwepp-hillslope-orchestrator --lib -- -D warnings`: PASS.
- coupled-time crate Nextest: 5/5 PASS.
- orchestrator `coupled_time_reference` filter: 2/2 PASS, 724 skipped.
- `git diff --check`: PASS.

The required broad runner, cargo-deny, final review, and terminal gates remain.
