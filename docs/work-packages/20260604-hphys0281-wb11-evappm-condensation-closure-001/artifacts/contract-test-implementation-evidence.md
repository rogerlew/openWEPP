# Contract-Test Implementation Evidence

Status: completed
Evidence mode: static + ran

Static: Added contract-derived tests:
- `crates/openwepp-runner/src/hillslope/mod.rs`: `hphys0281_wb11_evappm_seed_publishes_condensation_storage_return`.
- `crates/openwepp-runner/src/hillslope/mod.rs`: `hphys0281_wb13_publication_canonicalizes_roundoff_negative_es_without_evappm_clamp`.
- `crates/openwepp-hillslope-orchestrator/src/tests.rs`: `hphys0281_pmet_evapotranspiration_applies_condensation_storage_return`.

Review-driven assertions added:
- The producer seed test covers nonzero residue interception, positive
  `pmet.es_storage_return_m`, typed `m` units, and non-negative published
  `pmet.ep_m`/`wb11_et_demand` under active-canopy condensation.
- The WB17 consumer test covers residue plus condensation return by asserting
  the top layer increases by both returned depths while published `Es`/`Er`
  remain zero.

Ran before production edits: the three tests failed for missing producer return, missing WB17 consumption, and WB13 branch-specific roundoff behavior.

Ran after production edits:
- `cargo test -p openwepp-runner hphys0281 -- --nocapture`: 2 passed.
- `cargo test -p openwepp-hillslope-orchestrator hphys0281 -- --nocapture`: 1 passed.
- `cargo test --test pl14s_tier_a_candidate_emission_and_replay_contract simimpl18 -- --nocapture`: 2 passed.
