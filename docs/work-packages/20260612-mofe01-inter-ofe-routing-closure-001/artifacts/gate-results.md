# gate results

Status: M-A executed

Evidence mode: Ran + Static

## Ran

| Gate/check | Result | Notes |
| --- | --- | --- |
| `tools/owcmp/owcmp env --manifest tools/owcmp/suites/wa-cascades-mofe-ksflag0.json` | PASS | Confirmed `.venv/bin/python`, pyarrow 24.0.0, and arboreal-dendrite H1-H36 legacy outputs. |
| `cargo build -p openwepp-runner --bin openwepp-cli-hill` | PASS | Built current hillslope binary for isolated batch. |
| `cargo build -p openwepp-runner --bin open_wepp_runner` | PASS | Built launcher boundary used by wrapper contract checks. |
| Isolated current H1-H36 batch | RAN, expected evidence failure for MOFE | 7/7 single-OFE surfaces passed; 29/29 multi-OFE surfaces failed before output publication. |
| Local legacy H1-H36 WAT parse | PASS | Parsed 271,808 rows and produced per-OFE-count closure/routing calibration. |

## Not run

| Gate/check | Reason |
| --- | --- |
| `cargo fmt --check` | M-A made documentation/evidence edits only; no production Rust edits. |
| `cargo clippy --workspace --all-targets -- -D warnings` | M-A made documentation/evidence edits only; no production Rust edits. |
| `cargo test --workspace` | M-A made documentation/evidence edits only; no production Rust edits. |
| `cargo deny check` | M-A made documentation/evidence edits only; no dependency edits. |
| `bash tools/release/check_authority_suite_antievasion.sh` | No external-authority suite posture, cohort fixture binding, or required-case binding was edited. |
| `cargo test --test auth11_required_suite_obligation_guards_contract` | Same anti-evasion non-trigger as above. |
