# Implementation

Static: the inline `#[cfg(test)]` body moved mechanically to
`planner_coverage_tests.rs`; `planner.rs` retains only its include wiring. A
virtual reconstruction of the pre-split file has the exact original SHA-256
`5eb14b31c277babd74ec5e77a5c88a22537bbe3652753c5dbe4cefdcf93ebd26`.
The first 2,401 production lines also have identical before/after SHA-256
`5cea2ee375dd805296f91fbc46494cd98227e9084d55bbe1eadef4bf80dd9319`.
No production logic changed.

Ran: format and diff hygiene passed. The exact planner test namespace passed
12/12 with 133 skipped in 78.422 seconds after the split.

Status: mechanical split complete; characterization additions precede the
private helper extraction.
