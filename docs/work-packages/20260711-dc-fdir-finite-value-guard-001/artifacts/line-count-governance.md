# Line-count governance

Status: PASS
Evidence mode: Ran

Ran: `wc -l crates/openwepp-input-contract/src/parsers/irrigation_fixeddate.rs`
reported 915 lines after the finite guard and before decomposition. Final line
count is 934 after extracting two named helpers. Both are below the 2,000-line
WARN and 3,000-line blocker.
