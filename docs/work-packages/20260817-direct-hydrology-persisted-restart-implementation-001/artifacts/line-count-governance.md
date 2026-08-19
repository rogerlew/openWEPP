# Line Count Governance

No production Rust file reaches 2,000 lines. The largest promoted files are
`checkpoint.rs` (about 1,330), `evidence_fixture.rs` (about 1,010), and
`scientific_owners.rs` (about 1,000). The evidence fixture is feature-gated;
all remain below both the WARN and mandatory split thresholds.

