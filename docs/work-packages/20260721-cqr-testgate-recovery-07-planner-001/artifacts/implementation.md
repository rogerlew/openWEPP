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

Static: after the test-first commit, the complete base/head Cargo-graph load
and union block moved to private `load_source_graph`. The call remains in the
same statement position after request validation and policy load. Base graph is
still acquired before head graph; each workspace/current/committed branch and
every `?` propagation remains in the original order. The union still occurs
only after both loads succeed. No clone, allocation, public surface, error, or
selection behavior changed.

Ran: the expanded planner namespace passed 18/18 with 133 skipped in 79.835
seconds after extraction, including canonical isolated terminal reconstruction
and real narrow package inventory.
