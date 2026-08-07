# Contract-Test Implementation Evidence

Status: `complete`.

Evidence mode: `Ran`.

Added `tests/integration/snow_stage3_wind_source_custody_contract.rs` and its
explicit `Cargo.toml` target. Three tests require v10/v132 literals, source/raw/
PMET/virtual-height separation, `AUTHORITY_MISSING`, anti-fit/canopy/correction
guards, and actual parser -> runtime -> Stage 3 custody with negative
`fwv_m_s` reachability. Ran: `3 passed; 0 failed`.

The follow-on updates the same affected tests to v11/v133 and adds literals
that distinguish direct retained centroids/values from statically reconstructed
request/serialization semantics, plus the modeled-forest-versus-physical-
exposure claim limit.
