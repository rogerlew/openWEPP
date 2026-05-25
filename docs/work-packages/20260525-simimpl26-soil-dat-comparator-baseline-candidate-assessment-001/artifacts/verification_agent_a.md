# Verification Agent A

Status: complete
Evidence mode: ran
Date: 2026-05-25

## Static
- Verification objective: independently re-check executed gate and file-identity
  claims recorded by SIMIMPL26.

## Ran
Re-verified outcomes from command execution logs:
- `cargo test -p openwepp --test infile_soil_parser_contract` -> pass
- `cargo test -p openwepp --test pl14s_tier_a_candidate_emission_and_replay_contract` -> pass
- `cargo test --workspace` -> pass
- `cargo deny check` -> pass (warnings only)

Re-verified soil-file claims:
- PL08 baseline/candidate `p5.sol` sha256 hashes match exactly.
- `cmp -s` confirms byte-identical PL08 baseline/candidate `p5.sol`.
- PL14R candidate `runs/p5.sol` absence confirmed.
