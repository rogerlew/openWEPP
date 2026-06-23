# Verification

Status: executed-held.

## Static

- Static: `SC-HYDRAULICS-001` WB16 peak-flow coupling requires finite,
  non-negative `peakro`/`watdur` and deauthorizes fallback reconstruction when
  WB16 peak surfaces are missing or invalid.
- Static: R7D6 artifacts proved direct WB16 peak-duration values were
  producer-authoritative and must not be suppressed to match prior
  compatibility zero serialization.
- Static: Compatibility HBP serialization already consumed runtime
  `peakro`/`watdur` from `HillslopeWritebackSurface`; compatibility PASS
  hardcoded `peakro = 0.0`.
- Static: Direct HBP already consumed
  `runoff.peak_runoff_m3_s.or(erosion.peak_runoff_m3_s)`, while direct PASS
  consumed only the erosion copy.
- Static: R7D7 threaded explicit `HillslopePassPublicationScalars` into
  compatibility PASS construction and made direct PASS use runoff peak before
  the erosion copy.

## Ran

- Ran: `cargo fmt --check` passed.
- Ran: `git diff --check` passed.
- Ran: `cargo test -p openwepp-runner per_ofe --lib` passed after threading
  `HillslopePassPublicationScalars` through the per-OFE PASS helper.
- Ran: `cargo test -p openwepp-runner direct_production --lib` passed.
- Ran: `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`
  passed after the test-only `HillslopePassPublicationScalars::zero()` warning
  was guarded with `#[cfg(test)]`.
- Ran: aborted an accidental full-H2637 R7D7 compatibility run after it was
  identified as the long runfile rather than the R7D6 5-day fixture; no
  evidence was taken from that run.
- Ran: fresh H2637 5-day compatibility run
  `/tmp/r7d7-h2637-5day` label `r7d7-compat-current` exited `0`:
  `compat elapsed=0.71 rss_kb=50740`.
- Ran: fresh H2637 5-day direct production run
  `/tmp/r7d7-h2637-5day` label `r7d7-direct-current` exited `0`:
  `direct elapsed=1.12 rss_kb=63704`.
- Ran: output comparison:
  `wat_cmp=0`, `pass_cmp=0`, `hbp_cmp=1`.
- Ran: direct manifest counters:
  `phase_span_runs = 2509`, `direct_phase_entries = 4791`,
  `direct_compute_operations = 2623`, `direct_state_mutations = 2731`,
  `downstream_operand_productions = 2617`, `shadow_projections = 2509`,
  and `compatibility_edge_invocations = 0`.
- Ran: HBP payload parse showed peak/duration parity and narrowed the residual
  to HBP sediment export aliases: compatibility
  `total_detachment_kg = 0.6`, direct `0.0`; compatibility
  `sediment_concentration_kg_m3 = 6.816136920064195`, direct `0.0`.

## Not Run

- Full workspace `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check` were not run because R7D7
  is executed-held at the narrower HBP EROD15 sediment-export alias blocker,
  and the dirty tree already contains broader R7D in-flight changes.
