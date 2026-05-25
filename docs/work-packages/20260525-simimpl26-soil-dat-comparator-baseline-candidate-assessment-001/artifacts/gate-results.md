# Gate Results

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- Required non-doc gates from SIMIMPL26 exit criteria were executed.
- All required gates passed.

## Ran
1. `cargo test -p openwepp --test infile_soil_parser_contract`
   - result: pass (`8 passed; 0 failed`)
2. `cargo test -p openwepp --test pl14s_tier_a_candidate_emission_and_replay_contract`
   - result: pass (`8 passed; 0 failed`)
3. `cargo test --workspace`
   - result: pass
4. `cargo deny check`
   - result: pass (warnings only; advisories/bans/licenses/sources all ok)

Warning capture (`cargo deny check`):
- duplicate lock entries:
  - `getrandom`
  - `hashbrown`
  - `twox-hash`
- unmatched license allowlist entries:
  - `ISC`
  - `Unicode-DFS-2016`
