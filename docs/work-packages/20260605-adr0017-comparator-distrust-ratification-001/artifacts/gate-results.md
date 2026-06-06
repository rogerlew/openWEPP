# Gate Results

Status: complete

Evidence mode: Ran

Ran: `cargo fmt --check`

Result: PASS.

Ran: `cargo test --test hphys0313_snowpack_settling_carry_recursion_contract -- --nocapture`

Result: PASS, `6 passed`.

Ran: `bash tools/release/check_authority_suite_antievasion.sh`

Result: PASS.

Ran: `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture`

Result: PASS, `2 passed`.

Ran: `git diff --check`

Result: PASS.

Ran: `cargo test --test adr0017_comparator_distrust_ratification_contract -- --nocapture`

Result: PASS, `4 passed`.

Ran: `markdown-doc lint --path docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`

Result: PASS.

Ran: `markdown-doc lint --path docs/work-packages/20260605-adr0017-comparator-distrust-ratification-001`

Result: PASS.
