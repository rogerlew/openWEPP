# Line-count governance

Status: PASS

Evidence mode: Ran

Ran: `wc -l tests/integration/snow_stage3_shared_carrier_authority_contract.rs`
returned `546` lines. No production Rust is touched. This is below the
repository `WARN` threshold of 2,000 and `BLOCK` threshold of 3,000 lines.
