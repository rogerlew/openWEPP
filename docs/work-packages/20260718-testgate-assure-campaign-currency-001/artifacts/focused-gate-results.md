# Focused Gate Results

Ran: PASS on the implementation tree before independent review.

| Command | Result |
| --- | --- |
| `cargo check -p openwepp-gate-planner --all-targets` | PASS |
| `cargo clippy -p openwepp-gate-planner --all-targets -- -D warnings` | PASS |
| `cargo nextest run -p openwepp-gate-planner -E 'test(/assurance\|registry_authority/)'` | PASS, 9/9; 45 filtered |
| `cargo nextest run --test testgate_assure_campaign_currency_contract` plus the corrected failed-case filter | PASS, 3/3 total; final corrected discovery case 1/1 in 74.582 seconds |
| `cargo nextest run --test testgate_align_authority_contract` and corrected schema-case filter | PASS, 10/10 initial except one stale expected-error pointer; corrected case PASS 1/1 |
| `cargo fmt --check` | PASS |
| `git diff --check` | PASS |

Ran after final review remediation: the null-principal unresolved-role
substitution test passed 1/1, focused Clippy passed, and `git diff --check`
passed.

Ran after cargo-deny remediation: `cargo deny check licenses` PASS. Static:
the MIT-0 dependency chain and versions are identical to the frozen base; only
the direct workspace dependency projections changed in `Cargo.lock`.

The contracts prove deterministic repeated planning, complete catalog/registry
equality, all eight watch kinds, multi-watch coalescing, explicit delete/add
rename expansion, all-report unknown escalation, exact derived impact IDs,
dirty-tree and committed-terminal target binding, lifecycle-selected authority,
subject-keyed history folding, fail-closed disposition events, assessed/source
root binding, blocked transfer, strict fixtures, and no target-free `CURRENT`.

Ran: two development assertions required correction after the first remediated
execution: the schema negative fixture expected the obsolete historical-rule
pointer, and the integration watch set omitted the newly governed domain and
contract matches. Each failed assertion was corrected and only its affected
case was rerun. These are recorded as development feedback, not PASS evidence.

Ran: one earlier full crate development run is not closure evidence. It passed
45/46 and correctly rejected its own receipt reconstruction after source edits
continued concurrently (`GATE-RECEIPT-SOURCE-MUTATION`). The run was not
repeated because terminal full-workspace Nextest will cover the stable tree.
